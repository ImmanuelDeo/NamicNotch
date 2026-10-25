// src/notch/states.rs
use std::sync::{Arc, Mutex};
use chrono::{Local, Timelike};
use windows::core::PCWSTR;
use windows::Media::Control::{
    GlobalSystemMediaTransportControlsSessionManager,
    GlobalSystemMediaTransportControlsSessionPlaybackStatus,
};
use windows::Win32::Graphics::Direct2D::Common::D2D_RECT_F;
use windows::Win32::System::Power::{GetSystemPowerStatus, SYSTEM_POWER_STATUS};
use windows::Win32::UI::Shell::ShellExecuteW;
use windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;

use crate::mascot::chibi_liebe::ChibiLiebe;
use crate::mascot::eyes::DevilUnionEyes;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotchState {
    Pet,     // Closed mascot: Mata Asta Full Anime
    Idle,    // Closed notch: Jam Digital & Baterai iOS (muncul setiap 30 menit / saat baru tutup)
    Hub,     // Tab 0: Jam Besar, Baterai iOS & Scrollable Notification Center
    Music,   // Tab 1: Music/Video Player + Spotify-style Side-by-Side Synced Lyrics
    Control, // Tab 2: All-in-One Control (Volume Slider, Brightness Slider, Wi-Fi, Energy Saver)
    Pomo,    // Tab 3: Pomodoro Focus Timer
    Shelf,   // Tab 4: File Shelf (Real Drag & Drop Temporary Storage)
    Todo,    // Tab 5: To-Do List
}

#[derive(Debug, Clone, PartialEq)]
pub enum OverlayKind {
    Volume(u8),
    Brightness(u8),
    CapsLock(bool),
    Charging { percent: u8, plugged: bool },
}

#[derive(Debug, Clone)]
pub struct AppNotification {
    pub app_name: String,
    pub title: String,
    pub message: String,
    pub time_str: String,
    pub launch_target: String, // URL atau protokol/exe untuk dibuka saat diklik
}

#[derive(Debug, Clone, Default)]
pub struct LiveMediaSnapshot {
    pub title: String,
    pub artist: String,
    pub playing: bool,
    pub elapsed_secs: u32,
    pub total_secs: u32,
    pub lyrics: Vec<String>,
}

#[derive(Debug, Clone, Copy)]
pub struct SpringValue {
    pub value: f32,
    pub target: f32,
    pub velocity: f32,
    pub stiffness: f32,
    pub damping: f32,
}

impl SpringValue {
    pub fn new(initial: f32, stiffness: f32, damping: f32) -> Self {
        Self {
            value: initial,
            target: initial,
            velocity: 0.0,
            stiffness,
            damping,
        }
    }

    pub fn set_target(&mut self, target: f32) {
        self.target = target;
    }

    pub fn update(&mut self, dt: f32) {
        let step_dt = (dt * 0.5).clamp(0.0005, 0.016);
        for _ in 0..2 {
            let displacement = self.value - self.target;
            let spring_force = -self.stiffness * displacement;
            let damping_force = -self.damping * self.velocity;
            let acceleration = spring_force + damping_force;

            self.velocity += acceleration * step_dt;
            self.value += self.velocity * step_dt;
        }

        if self.is_settled() {
            self.value = self.target;
            self.velocity = 0.0;
        }
    }

    pub fn is_settled(&self) -> bool {
        (self.value - self.target).abs() < 0.15 && self.velocity.abs() < 0.8
    }
}

pub struct NotchController {
    pub state: NotchState,
    pub last_expanded_tab: NotchState,
    pub is_hovered: bool,
    pub minimal_mode: bool,
    pub mascot_enabled: bool,
    pub anim_time: f32,

    pub width: SpringValue,
    pub height: SpringValue,
    pub corner_radius: SpringValue,
    pub content_alpha: SpringValue,

    pub closed_quiet_timer: f32,
    pub hover_grace_timer: f32,
    pub last_half_hour_marker: (u32, u32), // (hour, minute) untuk chime setiap 30 menit

    // --- Pop-up Interactive Overlay ---
    pub active_overlay: Option<OverlayKind>,
    pub overlay_timer: f32,

    // --- All-in-One Control Center State ---
    pub volume_level: u8,
    pub brightness_level: u8,
    pub wifi_enabled: bool,
    pub energy_saver_enabled: bool,

    // --- Baterai & Power ---
    pub battery_percent: u8,
    pub is_charging: bool,
    power_poll_timer: f32,

    // --- Section 0: Hub & Scrollable Notifications ---
    pub notifications: Vec<AppNotification>,
    pub notif_scroll_offset: usize,
    pub show_notif_list_in_hub: bool,

    // --- Section 1: Music/Video + Integrated Spotify Lyrics ---
    pub media_title: String,
    pub media_artist: String,
    pub media_playing: bool,
    pub media_progress: f32,
    pub media_elapsed_secs: u32,
    pub media_total_secs: u32,
    pub show_side_lyrics: bool,
    pub lyrics_lines: Vec<String>,
    pub active_lyric_idx: usize,
    live_media_arc: Arc<Mutex<Option<LiveMediaSnapshot>>>,

    // --- Section 3: Pomodoro Timer ---
    pub pomo_remaining_secs: u32,
    pub pomo_total_secs: u32,
    pub pomo_running: bool,
    pub pomo_is_break: bool,
    pomo_accum: f32,

    // --- Section 4: File Shelf (Name, Meta, FullPath) ---
    pub shelf_files: Vec<(String, String, String)>,

    // --- Section 5: To-Do List ---
    pub todos: Vec<(String, bool)>,

    // --- Maskot ---
    pub devil_eyes: DevilUnionEyes,
    pub chibi_liebe: ChibiLiebe,
}

impl NotchController {
    pub fn new() -> Self {
        let (bat_pct, charging) = Self::read_windows_power();
        let live_media_arc: Arc<Mutex<Option<LiveMediaSnapshot>>> = Arc::new(Mutex::new(None));
        Self::spawn_gsmtc_media_poller(live_media_arc.clone());

        let mut ctrl = Self {
            state: NotchState::Pet,
            last_expanded_tab: NotchState::Music,
            is_hovered: false,
            minimal_mode: false,
            mascot_enabled: true,
            anim_time: 0.0,
            width: SpringValue::new(190.0, 310.0, 26.0),
            height: SpringValue::new(36.0, 310.0, 26.0),
            corner_radius: SpringValue::new(16.0, 310.0, 26.0),
            content_alpha: SpringValue::new(1.0, 240.0, 28.0),
            closed_quiet_timer: 0.0,
            hover_grace_timer: 0.0,
            last_half_hour_marker: (99, 99),

            active_overlay: None,
            overlay_timer: 0.0,

            volume_level: 72,
            brightness_level: 80,
            wifi_enabled: true,
            energy_saver_enabled: false,

            battery_percent: bat_pct,
            is_charging: charging,
            power_poll_timer: 0.0,

            notifications: vec![
                AppNotification {
                    app_name: String::from("WhatsApp"),
                    title: String::from("Grup Project Rust"),
                    message: String::from("Build Nusa Clone v4 sudah siap di-test!"),
                    time_str: String::from("Baru saja"),
                    launch_target: String::from("https://web.whatsapp.com"),
                },
                AppNotification {
                    app_name: String::from("Discord"),
                    title: String::from("Black Clover Server"),
                    message: String::from("Asta Devil Union animation looks sick 🔥"),
                    time_str: String::from("5m lalu"),
                    launch_target: String::from("https://discord.com/app"),
                },
                AppNotification {
                    app_name: String::from("YouTube"),
                    title: String::from("Media Session Active"),
                    message: String::from("Sinkronisasi pemutaran video & lirik aktif"),
                    time_str: String::from("12m lalu"),
                    launch_target: String::from("https://www.youtube.com"),
                },
                AppNotification {
                    app_name: String::from("Windows Security"),
                    title: String::from("System Status"),
                    message: String::from("Semua layanan berjalan normal tanpa kendala"),
                    time_str: String::from("25m lalu"),
                    launch_target: String::from("ms-settings:windowsdefender"),
                },
            ],
            notif_scroll_offset: 0,
            show_notif_list_in_hub: true,

            media_title: String::from("Menunggu Musik / Video YouTube..."),
            media_artist: String::from("Putar lagu di Spotify / Chrome / YouTube"),
            media_playing: false,
            media_progress: 0.0,
            media_elapsed_secs: 0,
            media_total_secs: 180,
            show_side_lyrics: true,
            lyrics_lines: vec![
                String::from("♪ Putar musik atau video di YouTube / Spotify"),
                String::from("Lirik otomatis tampil di panel kanan ini"),
                String::from("Klik baris lirik untuk scroll manual"),
                String::from("Atau klik tombol Lyrics untuk mode penuh"),
            ],
            active_lyric_idx: 0,
            live_media_arc,

            pomo_remaining_secs: 25 * 60,
            pomo_total_secs: 25 * 60,
            pomo_running: false,
            pomo_is_break: false,
            pomo_accum: 0.0,

            shelf_files: Vec::new(),

            todos: vec![
                (String::from("Bentuk notch menyatu ke bezel atas laptop"), true),
                (String::from("Coba drag & drop file ke tab Shelf"), false),
                (String::from("Atur Volume & Brightness dari tab Control"), false),
            ],

            devil_eyes: DevilUnionEyes::new(),
            chibi_liebe: ChibiLiebe::new(),
        };
        ctrl.chibi_liebe.sync_with_state(ctrl.state);
        ctrl
    }

    /// Background worker yang mem-poll Windows GSMTC secara langsung agar YouTube / Spotify selalu terdeteksi
    fn spawn_gsmtc_media_poller(shared: Arc<Mutex<Option<LiveMediaSnapshot>>>) {
        std::thread::spawn(move || {
            let mut last_fetched_title = String::new();
            let mut cached_lyrics: Vec<String> = Vec::new();

            loop {
                std::thread::sleep(std::time::Duration::from_millis(1200));

                let Ok(mgr_op) = GlobalSystemMediaTransportControlsSessionManager::RequestAsync() else {
                    continue;
                };
                let Ok(manager) = mgr_op.get() else { continue };
                let Ok(session) = manager.GetCurrentSession() else { continue };

                let playing = session
                    .GetPlaybackInfo()
                    .ok()
                    .and_then(|info| info.PlaybackStatus().ok())
                    .map(|st| st == GlobalSystemMediaTransportControlsSessionPlaybackStatus::Playing)
                    .unwrap_or(false);

                let (mut elapsed_secs, mut total_secs) = (0_u32, 180_u32);
                if let Ok(timeline) = session.GetTimelineProperties() {
                    if let (Ok(pos), Ok(end)) = (timeline.Position(), timeline.EndTime()) {
                        // TimeSpan.Duration adalah dalam satuan 100-nanosecond ticks
                        elapsed_secs = (pos.Duration / 10_000_000).max(0) as u32;
                        let end_s = (end.Duration / 10_000_000).max(1) as u32;
                        if end_s > 1 {
                            total_secs = end_s;
                        }
                    }
                }

                if let Ok(prop_op) = session.TryGetMediaPropertiesAsync() {
                    if let Ok(props) = prop_op.get() {
                        let title = props.Title().map(|s| s.to_string()).unwrap_or_default();
                        let artist = props.Artist().map(|s| s.to_string()).unwrap_or_default();

                        if !title.is_empty() && title != last_fetched_title {
                            last_fetched_title = title.clone();
                            cached_lyrics = Self::fetch_lrclib_Stage(&title, &artist);
                        }

                        if !title.is_empty() {
                            if let Ok(mut guard) = shared.lock() {
                                *guard = Some(LiveMediaSnapshot {
                                    title,
                                    artist: if artist.is_empty() {
                                        String::from("YouTube / Media Session")
                                    } else {
                                        artist
                                    },
                                    playing,
                                    elapsed_secs,
                                    total_secs,
                                    lyrics: cached_lyrics.clone(),
                                });
                            }
                        }
                    }
                }
            }
        });
    }

    fn fetch_lrclib_Stage(title: &str, artist: &str) -> Vec<String> {
        let query = format!(
            "https://lrclib.net/api/search?q={}%20{}",
            title.replace(' ', "%20"),
            artist.replace(' ', "%20")
        );
        let Ok(resp) = reqwest::blocking::get(&query) else {
            return Vec::new();
        };
        let Ok(json) = resp.json::<serde_json::Value>() else {
            return Vec::new();
        };
        if let Some(arr) = json.as_array() {
            if let Some(first) = arr.first() {
                let raw = first
                    .get("syncedLyrics")
                    .and_then(|v| v.as_str())
                    .or_else(|| first.get("plainLyrics").and_then(|v| v.as_str()))
                    .unwrap_or("");
                let lines: Vec<String> = raw
                    .lines()
                    .map(|l| {
                        // Bersihkan tag timestamp [00:12.34]
                        if let Some(pos) = l.find(']') {
                            l[pos + 1..].trim().to_string()
                        } else {
                            l.trim().to_string()
                        }
                    })
                    .filter(|l| !l.is_empty())
                    .collect();
                if !lines.is_empty() {
                    return lines;
                }
            }
        }
        vec![
            format!("♪ {}", title),
            format!("Artist: {}", if artist.is_empty() { "Unknown" } else { artist }),
            String::from("Menikmati alunan musik & video..."),
        ]
    }

    fn read_windows_power() -> (u8, bool) {
        unsafe {
            let mut status = SYSTEM_POWER_STATUS::default();
            if GetSystemPowerStatus(&mut status).is_ok() {
                let pct = if status.BatteryLifePercent <= 100 {
                    status.BatteryLifePercent
                } else {
                    100
                };
                let charging = status.ACLineStatus == 1;
                (pct, charging)
            } else {
                (100, false)
            }
        }
    }

    pub fn trigger_overlay(&mut self, kind: OverlayKind) {
        match kind {
            OverlayKind::Volume(v) => self.volume_level = v.min(100),
            OverlayKind::Brightness(b) => self.brightness_level = b.min(100),
            _ => {}
        }
        self.active_overlay = Some(kind);
        self.overlay_timer = 3.2; // Cukup waktu bagi pengguna untuk klik/geser slider langsung dari notch!
        if !self.is_hovered {
            self.width.set_target(290.0);
            self.height.set_target(46.0);
            self.corner_radius.set_target(18.0);
        }
    }

    pub fn ordered_tabs() -> [NotchState; 6] {
        [
            NotchState::Hub,
            NotchState::Music,
            NotchState::Control,
            NotchState::Pomo,
            NotchState::Shelf,
            NotchState::Todo,
        ]
    }

    pub fn next_tab(&mut self) {
        let tabs = Self::ordered_tabs();
        let idx = tabs.iter().position(|&s| s == self.state).unwrap_or(0);
        let next = tabs[(idx + 1) % tabs.len()];
        self.transition_to(next);
    }

    pub fn prev_tab(&mut self) {
        let tabs = Self::ordered_tabs();
        let idx = tabs.iter().position(|&s| s == self.state).unwrap_or(0);
        let prev = tabs[(idx + tabs.len() - 1) % tabs.len()];
        self.transition_to(prev);
    }

    pub fn target_dimensions_for(state: NotchState, minimal_mode: bool) -> (f32, f32, f32) {
        if minimal_mode && matches!(state, NotchState::Pet | NotchState::Idle) {
            return (130.0, 6.0, 3.0);
        }

        match state {
            NotchState::Pet => (190.0, 36.0, 16.0),
            NotchState::Idle => (216.0, 36.0, 16.0),
            NotchState::Hub => (470.0, 174.0, 24.0),
            NotchState::Music => (488.0, 172.0, 24.0),
            NotchState::Control => (460.0, 168.0, 24.0),
            NotchState::Pomo => (440.0, 162.0, 24.0),
            NotchState::Shelf => (465.0, 168.0, 24.0),
            NotchState::Todo => (450.0, 168.0, 24.0),
        }
    }

    pub fn transition_to(&mut self, next_state: NotchState) {
        if self.state == next_state {
            return;
        }

        if next_state == NotchState::Pet {
            self.devil_eyes.trigger_awakening();
        } else if !matches!(next_state, NotchState::Pet | NotchState::Idle) {
            self.last_expanded_tab = next_state;
        }

        self.state = next_state;
        self.chibi_liebe.sync_with_state(next_state);

        let (tw, th, tr) = Self::target_dimensions_for(next_state, self.minimal_mode);
        self.width.set_target(tw);
        self.height.set_target(th);
        self.corner_radius.set_target(tr);

        self.content_alpha.value = 0.25;
        self.content_alpha.set_target(1.0);
    }

    pub fn add_dropped_file(&mut self, full_path: String) {
        let path_obj = std::path::Path::new(&full_path);
        let file_name = path_obj
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Dropped File")
            .to_string();

        let size_str = std::fs::metadata(&full_path)
            .map(|m| {
                let kb = m.len() as f32 / 1024.0;
                if kb > 1024.0 {
                    format!("{:.1} MB", kb / 1024.0)
                } else {
                    format!("{:.0} KB", kb.max(1.0))
                }
            })
            .unwrap_or_else(|_| String::from("File"));

        if !self.shelf_files.iter().any(|(_, _, p)| p == &full_path) {
            self.shelf_files.insert(0, (file_name, size_str, full_path));
        }
        self.is_hovered = true;
        self.transition_to(NotchState::Shelf);
    }

    pub fn on_mouse_wheel(&mut self, delta: i16) {
        if self.state == NotchState::Hub && !self.notifications.is_empty() {
            if delta < 0 && self.notif_scroll_offset + 1 < self.notifications.len() {
                self.notif_scroll_offset += 1;
            } else if delta > 0 && self.notif_scroll_offset > 0 {
                self.notif_scroll_offset -= 1;
            }
        } else if self.state == NotchState::Music && !self.lyrics_lines.is_empty() {
            if delta < 0 {
                self.active_lyric_idx = (self.active_lyric_idx + 1) % self.lyrics_lines.len();
            } else if delta > 0 {
                self.active_lyric_idx = if self.active_lyric_idx == 0 {
                    self.lyrics_lines.len() - 1
                } else {
                    self.active_lyric_idx - 1
                };
            }
        }
    }

    pub fn update(&mut self, dt: f32, cursor_in_pill: bool, cursor_offset: Option<(f32, f32)>) {
        self.anim_time += dt.clamp(0.001, 0.05);

        // 1. Sinkronisasi hasil tangkapan GSMTC (YouTube / Spotify / Browser)
        if let Ok(mut guard) = self.live_media_arc.try_lock() {
            if let Some(snap) = guard.take() {
                let title_changed = self.media_title != snap.title;
                self.media_title = snap.title;
                self.media_artist = snap.artist;
                self.media_playing = snap.playing;
                if snap.total_secs > 0 {
                    self.media_elapsed_secs = snap.elapsed_secs;
                    self.media_total_secs = snap.total_secs;
                    self.media_progress =
                        (snap.elapsed_secs as f32 / snap.total_secs as f32).clamp(0.0, 1.0);
                }
                if !snap.lyrics.is_empty() && (title_changed || self.lyrics_lines.len() <= 4) {
                    self.lyrics_lines = snap.lyrics;
                }
                if !self.lyrics_lines.is_empty() && self.media_total_secs > 0 {
                    let lyr_idx = ((self.media_progress * self.lyrics_lines.len() as f32) as usize)
                        .min(self.lyrics_lines.len() - 1);
                    self.active_lyric_idx = lyr_idx;
                }
            }
        }

        // 2. Cek apakah waktu sekarang berada di menit :00 atau :30 (Auto 30-Minute Time Peek!)
        let now = Local::now();
        let hm = (now.hour(), now.minute());
        if (hm.1 == 0 || hm.1 == 30) && self.last_half_hour_marker != hm {
            self.last_half_hour_marker = hm;
            if !self.is_hovered && self.state == NotchState::Pet {
                self.closed_quiet_timer = -6.5; // Tampilkan Jam & Baterai iOS selama 10 detik penuh!
                self.transition_to(NotchState::Idle);
            }
        }

        // 3. Poll status baterai & charger setiap 2 detik
        self.power_poll_timer += dt;
        if self.power_poll_timer >= 2.0 {
            self.power_poll_timer = 0.0;
            let (pct, charging) = Self::read_windows_power();
            if charging != self.is_charging {
                self.is_charging = charging;
                self.battery_percent = pct;
                self.trigger_overlay(OverlayKind::Charging {
                    percent: pct,
                    plugged: charging,
                });
            } else {
                self.battery_percent = pct;
            }
        }

        // 4. Timer Overlay
        if self.active_overlay.is_some() {
            self.overlay_timer -= dt;
            if self.overlay_timer <= 0.0 {
                self.active_overlay = None;
                if !self.is_hovered {
                    let (tw, th, tr) = Self::target_dimensions_for(self.state, self.minimal_mode);
                    self.width.set_target(tw);
                    self.height.set_target(th);
                    self.corner_radius.set_target(tr);
                }
            }
        }

        // 5. Deteksi Hover
        if cursor_in_pill {
            self.hover_grace_timer = 0.15;
            if !self.is_hovered && self.active_overlay.is_none() {
                self.is_hovered = true;
                self.devil_eyes.trigger_wake_surge();
                self.transition_to(self.last_expanded_tab);
            }
        } else if self.is_hovered {
            self.hover_grace_timer -= dt;
            if self.hover_grace_timer <= 0.0 {
                self.is_hovered = false;
                self.closed_quiet_timer = 0.0;
                self.transition_to(NotchState::Idle);
            }
        } else if self.state == NotchState::Idle && self.mascot_enabled {
            self.closed_quiet_timer += dt;
            if self.closed_quiet_timer >= 3.5 {
                self.transition_to(NotchState::Pet);
            }
        }

        // 6. Countdown Pomodoro
        if self.pomo_running {
            self.pomo_accum += dt;
            if self.pomo_accum >= 1.0 {
                self.pomo_accum -= 1.0;
                if self.pomo_remaining_secs > 0 {
                    self.pomo_remaining_secs -= 1;
                } else {
                    self.pomo_is_break = !self.pomo_is_break;
                    self.pomo_total_secs = if self.pomo_is_break { 5 * 60 } else { 25 * 60 };
                    self.pomo_remaining_secs = self.pomo_total_secs;
                }
            }
        }

        self.width.update(dt);
        self.height.update(dt);
        self.corner_radius.update(dt);
        self.content_alpha.update(dt);

        if self.state == NotchState::Pet || self.height.value < 55.0 {
            self.devil_eyes.update(dt, cursor_offset);
        }

        if self.chibi_liebe.is_visible() {
            self.chibi_liebe.update(dt);
        }
    }

    fn send_system_media_command(cmd: &'static str) {
        std::thread::spawn(move || {
            if let Ok(op) = GlobalSystemMediaTransportControlsSessionManager::RequestAsync() {
                if let Ok(manager) = op.get() {
                    if let Ok(session) = manager.GetCurrentSession() {
                        match cmd {
                            "toggle" => {
                                let _ = session.TryTogglePlayPauseAsync();
                            }
                            "next" => {
                                let _ = session.TrySkipNextAsync();
                            }
                            "prev" => {
                                let _ = session.TrySkipPreviousAsync();
                            }
                            _ => {}
                        }
                    }
                }
            }
        });
    }

    pub fn apply_system_brightness(level: u8) {
        std::thread::spawn(move || {
            let script = format!(
                "(Get-WmiObject -Namespace root/WMI -Class WmiMonitorBrightnessMethods).WmiSetBrightness(1,{})",
                level.min(100)
            );
            let _ = std::process::Command::new("powershell")
                .args(["-WindowStyle", "Hidden", "-NoProfile", "-Command", &script])
                .spawn();
        });
    }

    pub fn apply_system_volume(level: u8) {
        std::thread::spawn(move || {
            // Gunakan PowerShell AudioEndpoint sederhana atau WScript SendKeys agar non-intrusif
            let script = format!(
                "$w = New-Object -ComObject WScript.Shell; 1..50 | % {{ $w.SendKeys([char]174) }}; 1..{} | % {{ $w.SendKeys([char]175) }}",
                (level.min(100) / 2).max(1)
            );
            let _ = std::process::Command::new("powershell")
                .args(["-WindowStyle", "Hidden", "-NoProfile", "-Command", &script])
                .spawn();
        });
    }

    fn open_target_url_or_path(target: &str) {
        let wide: Vec<u16> = target.encode_utf16().chain(std::iter::once(0)).collect();
        let op: Vec<u16> = "open\0".encode_utf16().collect();
        unsafe {
            let _ = ShellExecuteW(
                None,
                PCWSTR(op.as_ptr()),
                PCWSTR(wide.as_ptr()),
                PCWSTR::null(),
                PCWSTR::null(),
                SW_SHOWNORMAL,
            );
        }
    }

    /// Tangani klik pada Overlay Slider (ketika Volume / Brightness pop-up aktif)
    pub fn handle_overlay_click(&mut self, lx: f32, pill_w: f32) -> bool {
        if let Some(ref mut ov) = self.active_overlay {
            let bar_l = 56.0;
            let bar_r = pill_w - 48.0;
            if lx >= bar_l - 8.0 && lx <= bar_r + 8.0 {
                let pct = (((lx - bar_l) / (bar_r - bar_l).max(1.0)).clamp(0.0, 1.0) * 100.0)
                    .round() as u8;
                match ov {
                    OverlayKind::Volume(v) => {
                        *v = pct;
                        self.volume_level = pct;
                        Self::apply_system_volume(pct);
                        self.overlay_timer = 3.0;
                        return true;
                    }
                    OverlayKind::Brightness(b) => {
                        *b = pct;
                        self.brightness_level = pct;
                        Self::apply_system_brightness(pct);
                        self.overlay_timer = 3.0;
                        return true;
                    }
                    _ => {}
                }
            }
        }
        false
    }

    /// Tangani klik di dalam kartu fitur menggunakan koordinat logis `lx` & `ly` (relatif terhadap `pill_rect.left, pill_rect.top`)
    pub fn handle_panel_click(&mut self, lx: f32, ly: f32, pill_w: f32) -> bool {
        let card_l = 14.0;
        let card_t = 30.0;
        let card_w = pill_w - 28.0;

        // Koordinat relatif terhadap sudut kiri-atas Inner Card
        let cx = lx - card_l;
        let cy = ly - card_t;
        if cx < 0.0 || cy < 0.0 {
            return false;
        }

        match self.state {
            NotchState::Hub => {
                // Klik pada bagian kanan (daftar notifikasi) membuka aplikasi terkait!
                if cx >= 170.0 {
                    let row = (cy / 42.0) as usize;
                    let idx = self.notif_scroll_offset + row;
                    if let Some(notif) = self.notifications.get(idx) {
                        let target = notif.launch_target.clone();
                        Self::open_target_url_or_path(&target);
                        return true;
                    }
                } else {
                    self.show_notif_list_in_hub = !self.show_notif_list_in_hub;
                    return true;
                }
            }
            NotchState::Music => {
                // Sisi kiri: kontrol musik (cy: 84..126)
                if (80.0..=130.0).contains(&cy) && cx < card_w * 0.54 {
                    if (84.0..=116.0).contains(&cx) {
                        Self::send_system_media_command("prev");
                        self.media_progress = 0.0;
                        self.media_elapsed_secs = 0;
                        return true;
                    } else if (120.0..=156.0).contains(&cx) {
                        Self::send_system_media_command("toggle");
                        self.media_playing = !self.media_playing;
                        return true;
                    } else if (160.0..=192.0).contains(&cx) {
                        Self::send_system_media_command("next");
                        self.media_progress = 0.0;
                        self.media_elapsed_secs = 0;
                        return true;
                    } else if (196.0..=248.0).contains(&cx) {
                        self.show_side_lyrics = !self.show_side_lyrics;
                        return true;
                    }
                }
                // Sisi kanan: klik panel lirik untuk melompat ke baris berikutnya
                if cx >= card_w * 0.54 && !self.lyrics_lines.is_empty() {
                    self.active_lyric_idx = (self.active_lyric_idx + 1) % self.lyrics_lines.len();
                    return true;
                }
            }
            NotchState::Control => {
                // Baris 1 (cy: 10..56): Slider Volume (kiri) & Slider Brightness (kanan)
                let half_w = card_w * 0.5;
                if (10.0..=62.0).contains(&cy) {
                    if cx < half_w - 6.0 {
                        // Volume Slider bar
                        let sl_l = 14.0;
                        let sl_r = half_w - 18.0;
                        let pct = (((cx - sl_l) / (sl_r - sl_l).max(1.0)).clamp(0.0, 1.0) * 100.0)
                            .round() as u8;
                        self.volume_level = pct;
                        Self::apply_system_volume(pct);
                        return true;
                    } else {
                        // Brightness Slider bar
                        let sl_l = half_w + 10.0;
                        let sl_r = card_w - 16.0;
                        let pct = (((cx - sl_l) / (sl_r - sl_l).max(1.0)).clamp(0.0, 1.0) * 100.0)
                            .round() as u8;
                        self.brightness_level = pct;
                        Self::apply_system_brightness(pct);
                        return true;
                    }
                }
                // Baris 2 (cy: 68..124): Toggle Wi-Fi (kiri) & Toggle Energy Saver (kanan)
                if (68.0..=126.0).contains(&cy) {
                    if cx < half_w - 6.0 {
                        self.wifi_enabled = !self.wifi_enabled;
                        return true;
                    } else {
                        self.energy_saver_enabled = !self.energy_saver_enabled;
                        return true;
                    }
                }
            }
            NotchState::Pomo => {
                if (62.0..=112.0).contains(&cy) {
                    if (110.0..=202.0).contains(&cx) {
                        self.pomo_running = !self.pomo_running;
                        return true;
                    } else if (210.0..=280.0).contains(&cx) {
                        self.pomo_running = false;
                        self.pomo_remaining_secs = self.pomo_total_secs;
                        return true;
                    }
                }
            }
            NotchState::Shelf => {
                // Klik kartu file untuk membuka/menampilkan di Explorer (atau klik sudut kanan atas kartu untuk hapus)
                if cy >= 30.0 && !self.shelf_files.is_empty() {
                    let slot_w = (card_w - 76.0) * 0.5;
                    let idx = if cx < 14.0 + slot_w { 0 } else { 1 };
                    if idx < self.shelf_files.len() {
                        let card_right_edge = 14.0 + (idx as f32 + 1.0) * slot_w + idx as f32 * 10.0;
                        if cx >= card_right_edge - 24.0 && cy <= 54.0 {
                            self.shelf_files.remove(idx);
                        } else {
                            let path = self.shelf_files[idx].2.clone();
                            Self::open_target_url_or_path(&path);
                        }
                        return true;
                    }
                }
            }
            NotchState::Todo => {
                let row = ((cy - 8.0) / 32.0) as usize;
                if let Some(item) = self.todos.get_mut(row) {
                    item.1 = !item.1;
                    return true;
                }
            }
            _ => {}
        }
        false
    }
}