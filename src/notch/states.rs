// src/notch/states.rs
use std::os::windows::process::CommandExt;
use std::sync::{Arc, Mutex};
use chrono::{Datelike, Local, Timelike};
use windows::core::PCWSTR;
use windows::Media::Control::{
    GlobalSystemMediaTransportControlsSessionManager,
    GlobalSystemMediaTransportControlsSessionPlaybackStatus,
};
use windows::Win32::Media::Audio::Endpoints::IAudioEndpointVolume;
use windows::Win32::Media::Audio::{
    eConsole, eRender, IMMDeviceEnumerator, MMDeviceEnumerator,
};
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_MULTITHREADED,
};
use windows::Win32::System::Power::{GetSystemPowerStatus, SYSTEM_POWER_STATUS};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, VK_LWIN, VK_TAB,
};
use windows::Win32::UI::Shell::ShellExecuteW;
use windows::Win32::UI::WindowsAndMessaging::{
    FindWindowW, SetForegroundWindow, ShowWindow, SW_RESTORE, SW_SHOWNORMAL,
};

use crate::mascot::chibi_liebe::ChibiLiebe;
use crate::mascot::eyes::DevilUnionEyes;

const CREATE_NO_WINDOW: u32 = 0x08000000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotchState {
    Pet,     // Closed mascot: Asta Devil Union (Pedang + Petir Merah) / Cheer Mode (Healing Hijau)
    Idle,    // Closed notch: Jam Digital & Baterai iOS
    Hub,     // Tab 0: Jam, Tanggal, Baterai & Daily Quote
    Music,   // Tab 1: Clean Nusa Music Player + Auto-Synced Lyrics
    Control, // Tab 2: Clean Volume & Brightness Sliders
    Pomo,    // Tab 3: Pomodoro Timer (Focus 25m / Break 5m)
    Shelf,   // Tab 4: Dashed Dropzone File Shelf
}

#[derive(Debug, Clone, PartialEq)]
pub enum OverlayKind {
    Volume(u8),
    Brightness(u8),
    CapsLock(bool),
    Charging { percent: u8, plugged: bool },
}

#[derive(Debug, Clone)]
pub struct DailyQuote {
    pub word_of_the_day: &'static str,
    pub quote: &'static str,
    pub author: &'static str,
}

pub const MOTIVATIONAL_QUOTES: [DailyQuote; 7] = [
    DailyQuote {
        word_of_the_day: "RELENTLESS · PANTANG MENYERAH",
        quote: "\"My magic is never giving up — surpass your limits right here, right now!\"",
        author: "Asta · Black Clover",
    },
    DailyQuote {
        word_of_the_day: "CONSISTENCY · KONSISTENSI",
        quote: "\"Small disciplines repeated with consistency every day lead to great achievements.\"",
        author: "John C. Maxwell",
    },
    DailyQuote {
        word_of_the_day: "MASTERY · KETEKUNAN",
        quote: "\"Being weak is nothing to be ashamed of. Staying weak is!\"",
        author: "Fuegoleon Vermillion",
    },
    DailyQuote {
        word_of_the_day: "FOCUS · FOKUS PENUH",
        quote: "\"Starve your distractions, feed your focus, and build what matters today.\"",
        author: "Daily Mindset",
    },
    DailyQuote {
        word_of_the_day: "RESILIENCE · TANGGUH",
        quote: "\"It always seems impossible until it is done. Keep pushing forward.\"",
        author: "Nelson Mandela",
    },
    DailyQuote {
        word_of_the_day: "MOMENTUM · LANGKAH NYATA",
        quote: "\"Action cures fear. Start where you are, use what you have, do what you can.\"",
        author: "Arthur Ashe",
    },
    DailyQuote {
        word_of_the_day: "GRIT · TEKAD BAJA",
        quote: "\"Even without talent or magic, hard work beats everything in the end.\"",
        author: "Yami & Asta · Black Bulls",
    },
];

#[derive(Debug, Clone, Default)]
pub struct LiveMediaSnapshot {
    pub title: String,
    pub artist: String,
    pub playing: bool,
    pub raw_gsmtc_pos: f32,
    pub total_secs: f32,
    pub timed_lyrics: Vec<(f32, String)>,
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
    pub last_half_hour_marker: (u32, u32),

    pub active_overlay: Option<OverlayKind>,
    pub overlay_timer: f32,
    pub stagedock_flash_timer: f32,

    // --- Control State ---
    pub volume_level: u8,
    pub brightness_level: u8,

    // --- Baterai & Power ---
    pub battery_percent: u8,
    pub is_charging: bool,
    power_poll_timer: f32,

    // --- Section 0: Hub & Daily Quote ---
    pub quote_offset: usize,

    // --- Section 1: Music/Video + Auto-Synced Timed LRC Lyrics ---
    pub media_title: String,
    pub media_artist: String,
    pub media_playing: bool,
    pub media_progress: f32,
    pub media_elapsed_exact: f32,
    pub last_raw_gsmtc_pos: f32, // Mencegah reset ke detik ke-2 saat GSMTC membeku!
    pub media_total_secs: f32,
    pub show_side_lyrics: bool,
    pub timed_lyrics: Vec<(f32, String)>,
    pub active_lyric_idx: usize,
    live_media_arc: Arc<Mutex<Option<LiveMediaSnapshot>>>,

    // --- Section 3: Pomodoro Timer (Focus 25m vs Break 5m) ---
    pub pomo_remaining_secs: u32,
    pub pomo_total_secs: u32,
    pub pomo_running: bool,
    pub pomo_is_break: bool,
    pomo_accum: f32,

    // --- Section 4: File Shelf ---
    pub shelf_files: Vec<(String, String, String)>,

    // --- Maskot ---
    pub devil_eyes: DevilUnionEyes,
    pub chibi_liebe: ChibiLiebe,
}

impl NotchController {
    pub fn new() -> Self {
        let (bat_pct, charging) = Self::read_windows_power();
        let live_media_arc: Arc<Mutex<Option<LiveMediaSnapshot>>> = Arc::new(Mutex::new(None));
        Self::spawn_gsmtc_media_poller(live_media_arc.clone());

        let day_idx = (Local::now().ordinal() as usize) % MOTIVATIONAL_QUOTES.len();

        let mut ctrl = Self {
            state: NotchState::Pet,
            last_expanded_tab: NotchState::Hub,
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
            stagedock_flash_timer: 0.0,

            volume_level: 72,
            brightness_level: 80,

            battery_percent: bat_pct,
            is_charging: charging,
            power_poll_timer: 0.0,

            quote_offset: day_idx,

            media_title: String::from("Menunggu Musik / Video..."),
            media_artist: String::from("Spotify · YouTube · Browser"),
            media_playing: false,
            media_progress: 0.0,
            media_elapsed_exact: 0.0,
            last_raw_gsmtc_pos: -1.0,
            media_total_secs: 280.0,
            show_side_lyrics: true,
            timed_lyrics: vec![
                (0.0, String::from("♪ Putar lagu di Spotify / YouTube")),
                (6.0, String::from("Timeline & Lirik bergerak mulus real-time")),
                (12.0, String::from("Klik garis timeline untuk seek waktu")),
                (18.0, String::from("Klik ikon 💬 untuk tampil/sembunyikan lirik")),
            ],
            active_lyric_idx: 0,
            live_media_arc,

            pomo_remaining_secs: 25 * 60,
            pomo_total_secs: 25 * 60,
            pomo_running: false,
            pomo_is_break: false,
            pomo_accum: 0.0,

            shelf_files: Vec::new(),

            devil_eyes: DevilUnionEyes::new(),
            chibi_liebe: ChibiLiebe::new(),
        };
        ctrl.chibi_liebe.sync_with_state(ctrl.state);
        ctrl
    }

    pub fn current_quote(&self) -> &'static DailyQuote {
        &MOTIVATIONAL_QUOTES[self.quote_offset % MOTIVATIONAL_QUOTES.len()]
    }

    /// Ganti mode Pomodoro secara langsung antara Focus 25 Menit (`false`) atau Break 5 Menit (`true`)
    pub fn set_pomo_mode(&mut self, is_break: bool) {
        self.pomo_is_break = is_break;
        self.pomo_running = false;
        self.pomo_accum = 0.0;
        self.pomo_total_secs = if is_break { 5 * 60 } else { 25 * 60 };
        self.pomo_remaining_secs = self.pomo_total_secs;
    }

    pub fn trigger_stagedock(&mut self) {
        self.stagedock_flash_timer = 0.45;
        std::thread::spawn(|| {
            let candidates = ["StageDock\0", "StageDockV2\0", "Stage Manager\0"];
            for title in candidates {
                let wide: Vec<u16> = title.encode_utf16().collect();
                unsafe {
                    if let Ok(hwnd) = FindWindowW(PCWSTR::null(), PCWSTR(wide.as_ptr())) {
                        if !hwnd.0.is_null() {
                            let _ = ShowWindow(hwnd, SW_RESTORE);
                            let _ = SetForegroundWindow(hwnd);
                            return;
                        }
                    }
                }
            }

            let exe_paths = [
                r"D:\StageDockV2\StageDock.exe",
                r"D:\StageDockV2\stagedock.exe",
                r"D:\StageDockV2\target\release\stagedock.exe",
                r"D:\StageDockV2\target\release\StageDock.exe",
            ];
            for path in exe_paths {
                if std::path::Path::new(path).exists() {
                    let _ = std::process::Command::new(path)
                        .creation_flags(CREATE_NO_WINDOW)
                        .spawn();
                    return;
                }
            }

            unsafe {
                let inputs = [
                    INPUT {
                        r#type: INPUT_KEYBOARD,
                        Anonymous: INPUT_0 {
                            ki: KEYBDINPUT {
                                wVk: VK_LWIN,
                                ..Default::default()
                            },
                        },
                    },
                    INPUT {
                        r#type: INPUT_KEYBOARD,
                        Anonymous: INPUT_0 {
                            ki: KEYBDINPUT {
                                wVk: VK_TAB,
                                ..Default::default()
                            },
                        },
                    },
                    INPUT {
                        r#type: INPUT_KEYBOARD,
                        Anonymous: INPUT_0 {
                            ki: KEYBDINPUT {
                                wVk: VK_TAB,
                                dwFlags: KEYEVENTF_KEYUP,
                                ..Default::default()
                            },
                        },
                    },
                    INPUT {
                        r#type: INPUT_KEYBOARD,
                        Anonymous: INPUT_0 {
                            ki: KEYBDINPUT {
                                wVk: VK_LWIN,
                                dwFlags: KEYEVENTF_KEYUP,
                                ..Default::default()
                            },
                        },
                    },
                ];
                let _ = SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
            }
        });
    }

    fn spawn_gsmtc_media_poller(shared: Arc<Mutex<Option<LiveMediaSnapshot>>>) {
        std::thread::spawn(move || {
            let mut last_fetched_title = String::new();
            let mut cached_lyrics: Vec<(f32, String)> = Vec::new();

            loop {
                std::thread::sleep(std::time::Duration::from_millis(900));

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

                let (mut raw_gsmtc_pos, mut total_secs) = (0.0_f32, 240.0_f32);
                if let Ok(timeline) = session.GetTimelineProperties() {
                    if let (Ok(pos), Ok(end)) = (timeline.Position(), timeline.EndTime()) {
                        raw_gsmtc_pos = (pos.Duration as f32 / 10_000_000.0).max(0.0);
                        let end_s = (end.Duration as f32 / 10_000_000.0).max(1.0);
                        if end_s > 5.0 {
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
                            cached_lyrics = Self::fetch_timed_lrclib(&title, &artist, total_secs);
                        }

                        if !title.is_empty() {
                            if let Ok(mut guard) = shared.lock() {
                                *guard = Some(LiveMediaSnapshot {
                                    title,
                                    artist: if artist.is_empty() {
                                        String::from("YouTube / Media")
                                    } else {
                                        artist
                                    },
                                    playing,
                                    raw_gsmtc_pos,
                                    total_secs,
                                    timed_lyrics: cached_lyrics.clone(),
                                });
                            }
                        }
                    }
                }
            }
        });
    }

    fn fetch_timed_lrclib(title: &str, artist: &str, total_secs: f32) -> Vec<(f32, String)> {
        let query = format!(
            "https://lrclib.net/api/search?q={}%20{}",
            title.replace(' ', "%20"),
            artist.replace(' ', "%20")
        );
        if let Ok(resp) = reqwest::blocking::get(&query) {
            if let Ok(json) = resp.json::<serde_json::Value>() {
                if let Some(arr) = json.as_array() {
                    if let Some(first) = arr.first() {
                        if let Some(synced) = first.get("syncedLyrics").and_then(|v| v.as_str()) {
                            let mut timed = Vec::new();
                            for line in synced.lines() {
                                if let (Some(open), Some(close)) = (line.find('['), line.find(']')) {
                                    if close > open + 1 {
                                        let tag = &line[open + 1..close];
                                        let text = line[close + 1..].trim().to_string();
                                        if !text.is_empty() {
                                            if let Some(colon) = tag.find(':') {
                                                let mins: f32 =
                                                    tag[..colon].parse().unwrap_or(0.0);
                                                let secs: f32 =
                                                    tag[colon + 1..].parse().unwrap_or(0.0);
                                                timed.push((mins * 60.0 + secs, text));
                                            }
                                        }
                                    }
                                }
                            }
                            if !timed.is_empty() {
                                return timed;
                            }
                        }

                        if let Some(plain) = first.get("plainLyrics").and_then(|v| v.as_str()) {
                            let raw_lines: Vec<String> = plain
                                .lines()
                                .map(|l| l.trim().to_string())
                                .filter(|l| !l.is_empty())
                                .collect();
                            if !raw_lines.is_empty() {
                                let step = (total_secs.max(60.0)) / (raw_lines.len() as f32);
                                return raw_lines
                                    .into_iter()
                                    .enumerate()
                                    .map(|(i, l)| (i as f32 * step, l))
                                    .collect();
                            }
                        }
                    }
                }
            }
        }
        vec![
            (0.0, format!("♪ {}", title)),
            (4.0, format!("Artist: {}", if artist.is_empty() { "Unknown" } else { artist })),
            (8.0, String::from("Streaming aktif di Nusa Notch")),
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
        self.overlay_timer = 3.2;
        if !self.is_hovered {
            self.width.set_target(280.0);
            self.height.set_target(44.0);
            self.corner_radius.set_target(18.0);
        }
    }

    pub fn ordered_tabs() -> [NotchState; 5] {
        [
            NotchState::Hub,
            NotchState::Music,
            NotchState::Control,
            NotchState::Pomo,
            NotchState::Shelf,
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
            NotchState::Idle => (210.0, 36.0, 16.0),
            NotchState::Hub => (446.0, 146.0, 22.0),
            NotchState::Music => (462.0, 148.0, 22.0),
            NotchState::Control => (446.0, 142.0, 22.0),
            NotchState::Pomo => (446.0, 146.0, 22.0),
            NotchState::Shelf => (446.0, 146.0, 22.0),
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

    pub fn on_mouse_wheel(&mut self, delta: i16, _lx: f32, ly: f32, _pill_w: f32) {
        let step: i16 = if delta > 0 { 5 } else { -5 };

        if !self.is_hovered {
            if let Some(ref mut ov) = self.active_overlay {
                match ov {
                    OverlayKind::Volume(v) => {
                        let next = (*v as i16 + step).clamp(0, 100) as u8;
                        *v = next;
                        self.volume_level = next;
                        Self::apply_system_volume(next);
                        self.overlay_timer = 3.0;
                        return;
                    }
                    OverlayKind::Brightness(b) => {
                        let next = (*b as i16 + step).clamp(0, 100) as u8;
                        *b = next;
                        self.brightness_level = next;
                        Self::apply_system_brightness(next);
                        self.overlay_timer = 3.0;
                        return;
                    }
                    _ => {}
                }
            }
        }

        if self.state == NotchState::Control {
            let cy = ly - 28.0;
            let body_h = (self.height.value - 36.0).max(60.0);
            if cy < body_h * 0.5 {
                let next = (self.volume_level as i16 + step).clamp(0, 100) as u8;
                self.volume_level = next;
                Self::apply_system_volume(next);
            } else {
                let next = (self.brightness_level as i16 + step).clamp(0, 100) as u8;
                self.brightness_level = next;
                Self::apply_system_brightness(next);
            }
            return;
        }

        if self.state == NotchState::Hub {
            if delta < 0 {
                self.quote_offset = (self.quote_offset + 1) % MOTIVATIONAL_QUOTES.len();
            } else if delta > 0 {
                self.quote_offset =
                    (self.quote_offset + MOTIVATIONAL_QUOTES.len() - 1) % MOTIVATIONAL_QUOTES.len();
            }
        } else if self.state == NotchState::Music && !self.timed_lyrics.is_empty() {
            if delta < 0 {
                self.active_lyric_idx = (self.active_lyric_idx + 1) % self.timed_lyrics.len();
            } else if delta > 0 {
                self.active_lyric_idx = if self.active_lyric_idx == 0 {
                    self.timed_lyrics.len() - 1
                } else {
                    self.active_lyric_idx - 1
                };
            }
        }
    }

    pub fn update(&mut self, dt: f32, cursor_in_pill: bool, cursor_offset: Option<(f32, f32)>) {
        let dt = dt.clamp(0.001, 0.05);
        self.anim_time += dt;
        if self.stagedock_flash_timer > 0.0 {
            self.stagedock_flash_timer = (self.stagedock_flash_timer - dt).max(0.0);
        }

        // PERBAIKAN TIMELINE MUSIK LOOPING DI DETIK KE-2:
        // Hanya sinkronkan `media_elapsed_exact` ketika judul lagu berganti ATAU ketika nilai mentah
        // `raw_gsmtc_pos` dari Windows benar-benar berubah (misalnya karena user melakukan seek)!
        if let Ok(mut guard) = self.live_media_arc.try_lock() {
            if let Some(snap) = guard.take() {
                let title_changed = self.media_title != snap.title;
                let gsmtc_pos_changed = (snap.raw_gsmtc_pos - self.last_raw_gsmtc_pos).abs() > 0.5;

                self.media_title = snap.title;
                self.media_artist = snap.artist;
                self.media_playing = snap.playing;

                if snap.total_secs > 5.0 {
                    self.media_total_secs = snap.total_secs;
                }

                if title_changed {
                    self.last_raw_gsmtc_pos = snap.raw_gsmtc_pos;
                    self.media_elapsed_exact = snap.raw_gsmtc_pos;
                } else if gsmtc_pos_changed && snap.raw_gsmtc_pos > 0.1 {
                    // Hanya update jika aplikasi pemutar musik benar-benar mengirim posisi baru (seek)
                    self.last_raw_gsmtc_pos = snap.raw_gsmtc_pos;
                    self.media_elapsed_exact = snap.raw_gsmtc_pos;
                }

                if !snap.timed_lyrics.is_empty() && (title_changed || self.timed_lyrics.len() <= 4)
                {
                    self.timed_lyrics = snap.timed_lyrics;
                }
            }
        }

        // Majukan timeline secara mulus setiap frame selama musik sedang diputar
        if self.media_playing && self.media_total_secs > 1.0 {
            self.media_elapsed_exact += dt;
            if self.media_elapsed_exact > self.media_total_secs {
                self.media_elapsed_exact = 0.0;
            }
            self.media_progress =
                (self.media_elapsed_exact / self.media_total_secs).clamp(0.0, 1.0);

            if !self.timed_lyrics.is_empty() {
                let mut found_idx = 0;
                for (i, (ts, _)) in self.timed_lyrics.iter().enumerate() {
                    if *ts <= self.media_elapsed_exact + 0.30 {
                        found_idx = i;
                    } else {
                        break;
                    }
                }
                self.active_lyric_idx = found_idx;
            }
        }

        let now = Local::now();
        let hm = (now.hour(), now.minute());
        if (hm.1 == 0 || hm.1 == 30) && self.last_half_hour_marker != hm {
            self.last_half_hour_marker = hm;
            if !self.is_hovered && self.state == NotchState::Pet {
                self.closed_quiet_timer = -6.5;
                self.transition_to(NotchState::Idle);
            }
        }

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

        if self.pomo_running {
            self.pomo_accum += dt;
            if self.pomo_accum >= 1.0 {
                self.pomo_accum -= 1.0;
                if self.pomo_remaining_secs > 0 {
                    self.pomo_remaining_secs -= 1;
                } else {
                    self.set_pomo_mode(!self.pomo_is_break);
                    self.pomo_running = true;
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

    pub fn apply_system_volume(level: u8) {
        let scalar = (level.min(100) as f32 / 100.0).clamp(0.0, 1.0);
        std::thread::spawn(move || unsafe {
            let _ = CoInitializeEx(None, COINIT_MULTITHREADED);
            if let Ok(enumerator) =
                CoCreateInstance::<_, IMMDeviceEnumerator>(&MMDeviceEnumerator, None, CLSCTX_ALL)
            {
                if let Ok(device) = enumerator.GetDefaultAudioEndpoint(eRender, eConsole) {
                    if let Ok(endpoint) = device.Activate::<IAudioEndpointVolume>(CLSCTX_ALL, None)
                    {
                        let _ = endpoint.SetMasterVolumeLevelScalar(scalar, std::ptr::null());
                    }
                }
            }
        });
    }

    pub fn apply_system_brightness(level: u8) {
        let lvl = level.min(100);
        std::thread::spawn(move || {
            let script = format!(
                "(Get-WmiObject -Namespace root/WMI -Class WmiMonitorBrightnessMethods).WmiSetBrightness(1,{})",
                lvl
            );
            let _ = std::process::Command::new("powershell")
                .args(["-NoProfile", "-NonInteractive", "-Command", &script])
                .creation_flags(CREATE_NO_WINDOW)
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

    pub fn handle_overlay_click(&mut self, lx: f32, pill_w: f32) -> bool {
        if let Some(ref mut ov) = self.active_overlay {
            let bar_l = 58.0;
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

    /// Menangani klik pada konten section (koordinat relatif terhadap area konten `left: 34.0, top: 26.0`)
    pub fn handle_panel_click(&mut self, lx: f32, ly: f32, pill_w: f32) -> bool {
        let content_l = 32.0;
        let content_t = 26.0;
        let content_w = pill_w - 64.0;
        let content_h = (self.height.value - 34.0).max(60.0);

        let cx = lx - content_l;
        let cy = ly - content_t;
        if cx < 0.0 || cy < 0.0 || cx > content_w || cy > content_h {
            return false;
        }

        match self.state {
            NotchState::Hub => {
                if cx >= 145.0 {
                    self.quote_offset = (self.quote_offset + 1) % MOTIVATIONAL_QUOTES.len();
                    return true;
                }
            }
            NotchState::Music => {
                let right_limit = if self.show_side_lyrics {
                    content_w * 0.56
                } else {
                    content_w - 8.0
                };

                // 1. Klik pada garis timeline musik (cy: 54..74) untuk seek waktu & lirik
                let bar_l = 38.0;
                let bar_r = (right_limit - 42.0).max(bar_l + 40.0);
                if (52.0..=74.0).contains(&cy) && cx >= bar_l && cx <= bar_r {
                    let ratio = ((cx - bar_l) / (bar_r - bar_l)).clamp(0.0, 1.0);
                    self.media_progress = ratio;
                    self.media_elapsed_exact = ratio * self.media_total_secs;
                    return true;
                }

                // 2. Klik pada baris kontrol bawah (cy: 75..116): [💬 Lyrics] [⏮] [▶/⏸] [⏭]
                if cy >= 75.0 && cx <= right_limit {
                    let mid = right_limit * 0.52;
                    if cx < mid - 52.0 {
                        self.show_side_lyrics = !self.show_side_lyrics;
                        return true;
                    } else if cx >= mid - 48.0 && cx < mid - 16.0 {
                        Self::send_system_media_command("prev");
                        self.media_elapsed_exact = 0.0;
                        self.media_progress = 0.0;
                        return true;
                    } else if cx >= mid - 16.0 && cx <= mid + 18.0 {
                        Self::send_system_media_command("toggle");
                        self.media_playing = !self.media_playing;
                        return true;
                    } else if cx > mid + 18.0 && cx <= mid + 54.0 {
                        Self::send_system_media_command("next");
                        self.media_elapsed_exact = 0.0;
                        self.media_progress = 0.0;
                        return true;
                    }
                }

                // 3. Klik area lirik kanan untuk lompat baris
                if self.show_side_lyrics && cx > right_limit && !self.timed_lyrics.is_empty() {
                    self.active_lyric_idx = (self.active_lyric_idx + 1) % self.timed_lyrics.len();
                    return true;
                }
            }
            NotchState::Control => {
                // 2 Baris Slider Bersih: Baris Atas = Volume, Baris Bawah = Brightness
                let track_l = 104.0;
                let track_r = (content_w - 52.0).max(track_l + 20.0);
                if cx >= track_l - 8.0 && cx <= track_r + 8.0 {
                    let pct = (((cx - track_l) / (track_r - track_l)).clamp(0.0, 1.0) * 100.0)
                        .round() as u8;
                    if cy < content_h * 0.5 {
                        self.volume_level = pct;
                        Self::apply_system_volume(pct);
                    } else {
                        self.brightness_level = pct;
                        Self::apply_system_brightness(pct);
                    }
                    return true;
                }
            }
            NotchState::Pomo => {
                // Baris Atas (cy: 12..44): Pemilih Mode [Focus 25m] (cx: 118..214) dan [Break 5m] (cx: 222..314)
                if (10.0..=46.0).contains(&cy) {
                    if (116.0..=216.0).contains(&cx) {
                        self.set_pomo_mode(false); // Focus 25 menit
                        return true;
                    } else if (220.0..=320.0).contains(&cx) {
                        self.set_pomo_mode(true); // Break 5 menit
                        return true;
                    }
                }
                // Baris Bawah (cy: 56..98): Tombol [Start / Pause] (cx: 116..216) dan [Reset] (cx: 220..300)
                if (54.0..=102.0).contains(&cy) {
                    if (116.0..=216.0).contains(&cx) {
                        self.pomo_running = !self.pomo_running;
                        return true;
                    } else if (220.0..=305.0).contains(&cx) {
                        self.pomo_running = false;
                        self.pomo_remaining_secs = self.pomo_total_secs;
                        return true;
                    }
                }
            }
            NotchState::Shelf => {
                if !self.shelf_files.is_empty() {
                    let slot_w = (content_w - 24.0) * 0.5;
                    let idx = if cx < 8.0 + slot_w { 0 } else { 1 };
                    if idx < self.shelf_files.len() {
                        let item_r = 8.0 + (idx as f32 + 1.0) * slot_w + idx as f32 * 8.0;
                        if cx >= item_r - 24.0 && cy <= 46.0 {
                            self.shelf_files.remove(idx);
                        } else {
                            let path = self.shelf_files[idx].2.clone();
                            Self::open_target_url_or_path(&path);
                        }
                        return true;
                    }
                }
            }
            _ => {}
        }
        false
    }
}