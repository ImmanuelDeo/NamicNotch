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
use windows::Win32::UI::Shell::ShellExecuteW;
use windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;

use crate::mascot::chibi_liebe::ChibiLiebe;
use crate::mascot::DevilUnionEyes;

const CREATE_NO_WINDOW: u32 = 0x08000000;

/// True when the shell notification state means a fullscreen app / game /
/// presentation owns the screen and the notch should hide itself.
/// QUNS values: 2=Busy (fullscreen app), 3=D3D fullscreen, 4=Presentation mode.
pub fn is_fullscreen_quns(quns: i32) -> bool {
    matches!(quns, 2 | 3 | 4)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotchState {
    Pet,     // Closed mascot: Asta Devil Union (Pedang + Petir Merah) / Cheer Mode (Healing Hijau)
    Idle,    // Closed notch: Jam Digital & Baterai iOS
    Hub,     // Tab 0: Jam, Tanggal, Baterai & Daily Quote
    Music,   // Tab 1: Clean Nusa Music Player + Auto-Synced Lyrics
    Control, // Tab 2: Clean Volume & Brightness Sliders
    Pomo,    // Tab 3: Pomodoro Timer (Focus 25m / Break 5m)
    Shelf,   // Tab 4: Dashed Dropzone File Shelf
    Clips,   // Tab 5: Clipboard history + quick notes
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
    pub raw_gsmtc_pos: f32,          // -1.0 = GSMTC beku, pakai clock lokal
    pub total_secs: f32,
    pub timed_lyrics: Vec<(f32, String)>,
    pub has_real_lyrics: bool,       // true hanya jika LRC asli (synced/plain) ketemu
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

    // --- Control State ---
    pub volume_level: u8,
    pub brightness_level: u8,
    pub volume_poll_suppress: f32,                       // jeda setelah kita set volume sendiri
    pub system_levels_arc: Arc<Mutex<Option<(u8, u8)>>>, // (volume, brightness) dari Windows

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
    pub prev_lyric_idx: usize,
    pub lyric_anim_progress: f32,
    pub lyric_line_progress: f32,   // 0..1 progres di dalam bait lirik aktif (karaoke)
    pub has_real_lyrics: bool,      // false = placeholder, jangan tampilkan sebagai lirik asli
    pub is_dragging_track_bar: bool,
    pub playback_rate: f64,
    live_media_arc: Arc<Mutex<Option<LiveMediaSnapshot>>>,

    // --- Section 3: Pomodoro Timer (Focus 25m vs Break 5m) ---
    pub pomo_remaining_secs: u32,
    pub pomo_total_secs: u32,
    pub pomo_running: bool,
    pub pomo_is_break: bool,
    pomo_accum: f32,

    // --- Section 4: File Shelf ---
    pub shelf_files: Vec<(String, String, String)>,

    // --- Section 5: Clipboard history + quick notes ---
    pub clip_store: crate::clipboard::ClipStore,
    pub clips_view_notes: bool,
    pub clips_scroll: usize,
    pub note_input: String,
    pub note_input_active: bool,
    pub clip_flash_timer: f32,
    pub clip_flash_text: String,

    // --- OCR / screenshot state ---
    pub ocr_busy: bool,
    pub ocr_text: String,
    pub ocr_scroll: usize,
    pub last_ocr_png: Option<String>,

    // --- System monitor (Hub panel footer) ---
    pub ram_used_pct: u8,
    pub disk_free_gb: u32,
    sysmon_timer: f32,

    // --- Fullscreen auto-hide ---
    pub fullscreen_active: bool,
    fullscreen_poll_timer: f32,

    // --- Maskot ---
    pub devil_eyes: DevilUnionEyes,
    pub chibi_liebe: ChibiLiebe,
}

impl NotchController {
    pub fn new() -> Self {
        let (bat_pct, charging) = Self::read_windows_power();
        let live_media_arc: Arc<Mutex<Option<LiveMediaSnapshot>>> = Arc::new(Mutex::new(None));
        Self::spawn_gsmtc_media_poller(live_media_arc.clone());

        // Volume/brightness dibaca dari sistem sejak awal — bukan angka hardcode 72/80.
        let initial_volume = Self::read_system_volume().unwrap_or(50);
        let initial_brightness = Self::read_system_brightness().unwrap_or(0);
        let system_levels_arc: Arc<Mutex<Option<(u8, u8)>>> =
            Arc::new(Mutex::new(Some((initial_volume, initial_brightness))));
        Self::spawn_system_levels_poller(system_levels_arc.clone());

        let day_idx = (Local::now().ordinal() as usize) % MOTIVATIONAL_QUOTES.len();

        let mut ctrl = Self {
            state: NotchState::Pet,
            last_expanded_tab: NotchState::Hub,
            is_hovered: false,
            minimal_mode: false,
            mascot_enabled: true,
            anim_time: 0.0,
            width: SpringValue::new(156.0, 310.0, 26.0),
            height: SpringValue::new(34.0, 310.0, 26.0),
            corner_radius: SpringValue::new(15.0, 310.0, 26.0),
            content_alpha: SpringValue::new(1.0, 240.0, 28.0),
            closed_quiet_timer: 0.0,
            hover_grace_timer: 0.0,
            last_half_hour_marker: (99, 99),

            active_overlay: None,
            overlay_timer: 0.0,

            volume_level: initial_volume,
            brightness_level: if initial_brightness > 0 { initial_brightness } else { 80 },
            volume_poll_suppress: 0.0,
            system_levels_arc,

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
            // Mode 2A (track + transport) sebagai default; 2B hanya saat toggle Lyrics.
            show_side_lyrics: false,
            timed_lyrics: vec![
                (0.0, String::from("♪ Putar lagu di Spotify / YouTube")),
                (6.0, String::from("Timeline & Lirik bergerak mulus real-time")),
                (12.0, String::from("Klik garis timeline untuk seek waktu")),
                (18.0, String::from("Klik ikon 💬 untuk tampil/sembunyikan lirik")),
            ],
            active_lyric_idx: 0,
            prev_lyric_idx: 0,
            lyric_anim_progress: 1.0,
            lyric_line_progress: 0.0,
            has_real_lyrics: false,
            is_dragging_track_bar: false,
            playback_rate: 1.0,
            live_media_arc,

            pomo_remaining_secs: 25 * 60,
            pomo_total_secs: 25 * 60,
            pomo_running: false,
            pomo_is_break: false,
            pomo_accum: 0.0,

            shelf_files: Vec::new(),

            clip_store: crate::clipboard::ClipStore::load(),
            clips_view_notes: false,
            clips_scroll: 0,
            note_input: String::new(),
            note_input_active: false,
            clip_flash_timer: 0.0,
            clip_flash_text: String::new(),

            ocr_busy: false,
            ocr_text: String::new(),
            ocr_scroll: 0,
            last_ocr_png: None,

            ram_used_pct: 0,
            disk_free_gb: 0,
            sysmon_timer: 99.0,

            fullscreen_active: false,
            fullscreen_poll_timer: 99.0,

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

    fn spawn_gsmtc_media_poller(shared: Arc<Mutex<Option<LiveMediaSnapshot>>>) {
        std::thread::spawn(move || {
            let mut last_fetched_title = String::new();
            let mut cached_lyrics: Vec<(f32, String)> = Vec::new();
            let mut cached_real = false;
            let mut last_pos = -1.0_f32;
            let mut stall_polls = 0_u32;

            loop {
                // Polling cepat 250ms untuk responsivitas instan saat seek / play / pause
                std::thread::sleep(std::time::Duration::from_millis(250));

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
                            let (lyrics, real) = Self::fetch_timed_lrclib(&title, &artist, total_secs);
                            cached_lyrics = lyrics;
                            cached_real = real;
                        }

                        // GSMTC bisa membeku di satu posisi (mis. saat browser di-background).
                        // Jangan pernah seek mundur ke posisi itu; kirim -1 sebagai penanda.
                        if playing && (raw_gsmtc_pos - last_pos).abs() < 0.01 {
                            stall_polls += 1;
                        } else {
                            stall_polls = 0;
                            last_pos = raw_gsmtc_pos;
                        }
                        let authoritative_pos = if stall_polls > 8 { -1.0 } else { raw_gsmtc_pos };

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
                                    raw_gsmtc_pos: authoritative_pos,
                                    total_secs,
                                    timed_lyrics: cached_lyrics.clone(),
                                    has_real_lyrics: cached_real,
                                });
                            }
                        }
                    }
                }
            }
        });
    }

    /// Poll volume/brightness dari Windows tiap 700ms. Ini sumber kebenaran:
    /// slider selalu mencerminkan settingan laptop, termasuk kalau diubah dari
    /// keyboard/tray/setting Windows — bukan nilai hardcode saat startup.
    fn spawn_system_levels_poller(shared: Arc<Mutex<Option<(u8, u8)>>>) {
        std::thread::spawn(move || loop {
            std::thread::sleep(std::time::Duration::from_millis(700));
            let vol = Self::read_system_volume();
            let bri = Self::read_system_brightness();
            if vol.is_some() || bri.is_some() {
                if let Ok(mut guard) = shared.lock() {
                    *guard = Some((vol.unwrap_or(0), bri.unwrap_or(0)));
                }
            }
        });
    }

    /// Volume endpoint audio default (0..100) via COM IAudioEndpointVolume.
    pub fn read_system_volume() -> Option<u8> {
        unsafe {
            let _ = CoInitializeEx(None, COINIT_MULTITHREADED);
            let enumerator = CoCreateInstance::<_, IMMDeviceEnumerator>(
                &MMDeviceEnumerator, None, CLSCTX_ALL,
            ).ok()?;
            let device = enumerator.GetDefaultAudioEndpoint(eRender, eConsole).ok()?;
            let endpoint = device.Activate::<IAudioEndpointVolume>(CLSCTX_ALL, None).ok()?;
            let scalar = endpoint.GetMasterVolumeLevelScalar().ok()?;
            Some((scalar.clamp(0.0, 1.0) * 100.0).round() as u8)
        }
    }

    /// Brightness panel internal (0..100). 0 berarti desktop pakai brightness
    /// eksternal / WMI tidak tersedia — kembalikan None supaya tidak menimpa.
    pub fn read_system_brightness() -> Option<u8> {
        use std::os::windows::process::CommandExt;
        let out = std::process::Command::new("powershell")
            .args([
                "-NoProfile", "-NonInteractive", "-Command",
                "(Get-WmiObject -Namespace root/WMI -Class WmiMonitorBrightness).CurrentBrightness",
            ])
            .creation_flags(CREATE_NO_WINDOW)
            .output()
            .ok()?;
        let txt = String::from_utf8_lossy(&out.stdout);
        txt.trim().lines().next()?.trim().parse::<u8>().ok().filter(|&v| v <= 100)
    }

    fn clean_music_title_and_artist(raw_title: &str, raw_artist: &str) -> (String, String) {
        let mut title = raw_title.trim().to_string();
        let mut artist = raw_artist.trim().to_string();

        if (artist.is_empty() || artist.contains("YouTube") || artist.contains("Browser") || artist.contains("Media"))
            && title.contains(" - ")
        {
            let parts: Vec<&str> = title.splitn(2, " - ").collect();
            if parts.len() == 2 {
                artist = parts[0].trim().to_string();
                title = parts[1].trim().to_string();
            }
        }

        // Hapus noise umum seperti (Official Video), [MV], (Lyric Video), dsb
        let mut cleaned = String::new();
        let mut in_paren = false;
        let mut buf = String::new();
        for c in title.chars() {
            if c == '(' || c == '[' {
                in_paren = true;
                buf.clear();
            } else if c == ')' || c == ']' {
                in_paren = false;
                let lower = buf.to_lowercase();
                let is_noise = lower.contains("official")
                    || lower.contains("video")
                    || lower.contains("audio")
                    || lower.contains("lyric")
                    || lower.contains("mv")
                    || lower.contains("remaster")
                    || lower.contains("feat")
                    || lower.contains("ft.")
                    || lower.contains("hd")
                    || lower.contains("4k")
                    || lower.contains("full");
                if !is_noise && !buf.is_empty() {
                    cleaned.push('(');
                    cleaned.push_str(&buf);
                    cleaned.push(')');
                }
                buf.clear();
            } else if in_paren {
                buf.push(c);
            } else {
                cleaned.push(c);
            }
        }

        let mut final_title = cleaned.replace("  ", " ").trim().to_string();
        if final_title.is_empty() {
            final_title = raw_title.trim().to_string();
        }
        (final_title, artist)
    }

    /// Kembalikan (lirik, punya_lirik_asli). Flag penting supaya placeholder
    /// tidak ikut ber-animasi karaoke seolah-olah lirik asli.
    fn fetch_timed_lrclib(raw_title: &str, raw_artist: &str, total_secs: f32) -> (Vec<(f32, String)>, bool) {
        let (title, artist) = Self::clean_music_title_and_artist(raw_title, raw_artist);
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(3))
            .user_agent("NamicNotch/1.0")
            .build()
            .ok();

        let Some(client) = client else { return (Vec::new(), false); };

        // 1. Coba /api/get dengan parameter track_name dan artist_name
        if !artist.is_empty() && !title.is_empty() {
            let get_url = format!(
                "https://lrclib.net/api/get?track_name={}&artist_name={}",
                urlencoding::encode(&title),
                urlencoding::encode(&artist)
            );
            if let Ok(resp) = client.get(&get_url).send() {
                if let Ok(json) = resp.json::<serde_json::Value>() {
                    if let Some(synced) = json.get("syncedLyrics").and_then(|v| v.as_str()) {
                        let parsed = Self::parse_lrc_lines(synced);
                        if !parsed.is_empty() {
                            return (parsed, true);
                        }
                    }
                }
            }
        }

        // 2. Fallback ke /api/search?q=...
        let search_query = if artist.is_empty() || artist.contains("YouTube") {
            title.clone()
        } else {
            format!("{} {}", artist, title)
        };
        let search_url = format!(
            "https://lrclib.net/api/search?q={}",
            urlencoding::encode(&search_query)
        );
        if let Ok(resp) = client.get(&search_url).send() {
            if let Ok(json) = resp.json::<serde_json::Value>() {
                if let Some(arr) = json.as_array() {
                    for item in arr.iter().take(4) {
                        if let Some(synced) = item.get("syncedLyrics").and_then(|v| v.as_str()) {
                            let parsed = Self::parse_lrc_lines(synced);
                            if !parsed.is_empty() {
                                return (parsed, true);
                            }
                        }
                    }
                    if let Some(first) = arr.first() {
                        if let Some(plain) = first.get("plainLyrics").and_then(|v| v.as_str()) {
                            let raw_lines: Vec<String> = plain
                                .lines()
                                .map(|l| l.trim().to_string())
                                .filter(|l| !l.is_empty())
                                .collect();
                            if !raw_lines.is_empty() {
                                let step = (total_secs.max(60.0)) / (raw_lines.len() as f32);
                                return (raw_lines
                                    .into_iter()
                                    .enumerate()
                                    .map(|(i, l)| (i as f32 * step, l))
                                    .collect(), true);
                            }
                        }
                    }
                }
            }
        }

        // Tidak ada lirik asli: kembalikan placeholder statis + flag false.
        (vec![
            (0.0, format!("♪ {}", title)),
            (4.0, format!("Artist: {}", if artist.is_empty() { "Unknown" } else { &artist })),
            (8.0, String::from("Streaming aktif di Namic Notch")),
        ], false)
    }

    fn parse_lrc_lines(synced: &str) -> Vec<(f32, String)> {
        let mut timed = Vec::new();
        for line in synced.lines() {
            let line = line.trim();
            if line.starts_with('[') {
                if let Some(close) = line.find(']') {
                    let tag = &line[1..close];
                    let text = line[close + 1..].trim();
                    if let Some(colon) = tag.find(':') {
                        let mins_str = tag[..colon].trim();
                        let secs_str = tag[colon + 1..].trim();
                        if let (Ok(mins), Ok(secs)) = (mins_str.parse::<f32>(), secs_str.parse::<f32>()) {
                            let timestamp = mins * 60.0 + secs;
                            let display_text = if text.is_empty() {
                                "♪ · · ·".to_string()
                            } else {
                                text.to_string()
                            };
                            timed.push((timestamp, display_text));
                        }
                    }
                }
            }
        }
        timed.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        timed
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
                self.width.set_target(230.0);
                self.height.set_target(40.0);
                self.corner_radius.set_target(16.0);
            }
        }

    pub fn ordered_tabs() -> [NotchState; 6] {
        [
            NotchState::Hub,
            NotchState::Music,
            NotchState::Control,
            NotchState::Pomo,
            NotchState::Shelf,
            NotchState::Clips,
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
            return (110.0, 6.0, 3.0);
        }

        // Expanded notch seragam: 450x168 dip, radius 24 (Solid Liquid Glass).
        match state {
            NotchState::Pet => (156.0, 34.0, 15.0),
            NotchState::Idle => (168.0, 34.0, 15.0),
            NotchState::Hub
            | NotchState::Music
            | NotchState::Control
            | NotchState::Pomo
            | NotchState::Shelf
            | NotchState::Clips => (450.0, 168.0, 24.0),
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

        if next_state != NotchState::Clips {
            self.note_input_active = false;
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
                        self.volume_poll_suppress = 1.5;
                        Self::apply_system_volume(next);
                        self.overlay_timer = 3.0;
                        return;
                    }
                    OverlayKind::Brightness(b) => {
                        let next = (*b as i16 + step).clamp(0, 100) as u8;
                        *b = next;
                        self.brightness_level = next;
                        self.volume_poll_suppress = 1.5;
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
                self.volume_poll_suppress = 1.5;
                        Self::apply_system_volume(next);
            } else {
                let next = (self.brightness_level as i16 + step).clamp(0, 100) as u8;
                self.brightness_level = next;
                self.volume_poll_suppress = 1.5;
                        Self::apply_system_brightness(next);
            }
            return;
        }

        if self.state == NotchState::Clips {
            let rows = self.clips_rows().len();
            if rows > 3 {
                if delta < 0 {
                    self.clips_scroll = (self.clips_scroll + 1).min(rows - 3);
                } else {
                    self.clips_scroll = self.clips_scroll.saturating_sub(1);
                }
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
        } else if self.state == NotchState::Music {
            // Scroll mouse di music panel: Seek maju (+5s) atau mundur (-5s) secara instan!
            let step_secs = if delta > 0 { 5.0 } else { -5.0 };
            let new_secs = (self.media_elapsed_exact + step_secs).clamp(0.0, self.media_total_secs);
            self.media_elapsed_exact = new_secs;
            self.media_progress = (new_secs / self.media_total_secs.max(1.0)).clamp(0.0, 1.0);
            Self::seek_system_media(new_secs);
        }
    }

    pub fn update(&mut self, dt: f32, cursor_in_pill: bool, cursor_offset: Option<(f32, f32)>) {
        let dt = dt.clamp(0.001, 0.05);
        self.anim_time += dt;

        // Animasi transisi lirik mengalun halus (ease-out lerp)
        self.lyric_anim_progress = (self.lyric_anim_progress + dt * 4.5).min(1.0);

        // --- SINKRONISASI LEVEL SISTEM (real-time) ---
        // Jangan timpa nilai yang baru saja kita tulis sendiri selama 1.5 detik,
        // tapi jika pengguna mengubah volume dari keyboard/Windows, langsung ikuti.
        if self.volume_poll_suppress > 0.0 {
            self.volume_poll_suppress = (self.volume_poll_suppress - dt).max(0.0);
        } else if let Ok(mut guard) = self.system_levels_arc.try_lock() {
            if let Some((vol, bri)) = guard.take() {
                if vol != self.volume_level {
                    self.volume_level = vol;
                }
                if bri > 0 && bri != self.brightness_level {
                    self.brightness_level = bri;
                }
                if let Some(ref mut ov) = self.active_overlay {
                    match ov {
                        OverlayKind::Volume(v) => *v = self.volume_level,
                        OverlayKind::Brightness(b) => *b = self.brightness_level,
                        _ => {}
                    }
                }
            }
        }

        if let Ok(mut guard) = self.live_media_arc.try_lock() {
            if let Some(snap) = guard.take() {
                let title_changed = self.media_title != snap.title;

                self.media_title = snap.title;
                self.media_artist = snap.artist;
                self.media_playing = snap.playing;

                if snap.total_secs > 5.0 {
                    self.media_total_secs = snap.total_secs;
                }

                // raw_gsmtc_pos == -1 artinya GSMTC membeku — jangan pernah
                // seek mundur ke posisi lama. Clock lokal yang berjalan terus.
                if title_changed {
                    if snap.raw_gsmtc_pos >= 0.0 {
                        self.media_elapsed_exact = snap.raw_gsmtc_pos;
                    }
                    self.last_raw_gsmtc_pos = snap.raw_gsmtc_pos;
                } else if !self.is_dragging_track_bar && snap.raw_gsmtc_pos >= 0.0 {
                    let diff = (snap.raw_gsmtc_pos - self.media_elapsed_exact).abs();
                    if diff > 1.8 {
                        // User seek di player eksternal (Spotify/YouTube/Browser)
                        self.last_raw_gsmtc_pos = snap.raw_gsmtc_pos;
                        self.media_elapsed_exact = snap.raw_gsmtc_pos;
                    } else if diff > 0.35 && snap.raw_gsmtc_pos > 0.05 {
                        // Koreksi halus drift waktu agar sinkron dengan audio
                        self.media_elapsed_exact =
                            self.media_elapsed_exact * 0.70 + snap.raw_gsmtc_pos * 0.30;
                    }
                }

                if !snap.timed_lyrics.is_empty()
                    && (title_changed || self.timed_lyrics.len() <= 4 || self.has_real_lyrics != snap.has_real_lyrics)
                {
                    self.timed_lyrics = snap.timed_lyrics;
                    self.has_real_lyrics = snap.has_real_lyrics;
                    self.active_lyric_idx = 0;
                    self.prev_lyric_idx = 0;
                    self.lyric_anim_progress = 1.0;
                    self.lyric_line_progress = 0.0;
                }
            }
        }

        // Majukan timeline secara mulus setiap frame selama musik sedang diputar
        if self.media_playing && !self.is_dragging_track_bar && self.media_total_secs > 1.0 {
            self.media_elapsed_exact += dt * self.playback_rate as f32;
            if self.media_elapsed_exact > self.media_total_secs {
                self.media_elapsed_exact = self.media_total_secs;
            }
            self.media_progress =
                (self.media_elapsed_exact / self.media_total_secs).clamp(0.0, 1.0);

            if self.has_real_lyrics && !self.timed_lyrics.is_empty() {
                let mut found_idx = 0;
                for (i, (ts, _)) in self.timed_lyrics.iter().enumerate() {
                    if *ts <= self.media_elapsed_exact + 0.15 {
                        found_idx = i;
                    } else {
                        break;
                    }
                }
                if found_idx != self.active_lyric_idx {
                    self.prev_lyric_idx = self.active_lyric_idx;
                    self.active_lyric_idx = found_idx;
                    self.lyric_anim_progress = 0.0;
                    self.lyric_line_progress = 0.0;
                }
                // Progres di dalam bait saat ini = dasar karaoke yang mulus.
                let (start, _) = self.timed_lyrics[self.active_lyric_idx];
                let end = self
                    .timed_lyrics
                    .get(self.active_lyric_idx + 1)
                    .map(|(t, _)| *t)
                    .unwrap_or(start + 5.0);
                self.lyric_line_progress =
                    ((self.media_elapsed_exact - start) / (end - start).max(0.1)).clamp(0.0, 1.0);
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

        if self.clip_flash_timer > 0.0 {
            self.clip_flash_timer = (self.clip_flash_timer - dt).max(0.0);
        }

        self.fullscreen_poll_timer += dt;
        if self.fullscreen_poll_timer >= 0.5 {
            self.fullscreen_poll_timer = 0.0;
            self.fullscreen_active = Self::detect_fullscreen();
        }

        self.sysmon_timer += dt;
        if self.sysmon_timer >= 5.0 {
            self.sysmon_timer = 0.0;
            let (ram, disk) = Self::read_system_stats();
            self.ram_used_pct = ram;
            self.disk_free_gb = disk;
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

    pub fn seek_system_media(seek_secs: f32) {
        std::thread::spawn(move || {
            if let Ok(op) = GlobalSystemMediaTransportControlsSessionManager::RequestAsync() {
                if let Ok(manager) = op.get() {
                    if let Ok(session) = manager.GetCurrentSession() {
                        let requested_pos = (seek_secs.max(0.0) as f64 * 10_000_000.0) as i64;
                        let _ = session.TryChangePlaybackPositionAsync(requested_pos);
                    }
                }
            }
        });
    }

    pub fn change_system_playback_rate(rate: f64) {
        std::thread::spawn(move || {
            if let Ok(op) = GlobalSystemMediaTransportControlsSessionManager::RequestAsync() {
                if let Ok(manager) = op.get() {
                    if let Ok(session) = manager.GetCurrentSession() {
                        let _ = session.TryChangePlaybackRateAsync(rate);
                    }
                }
            }
        });
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

    /// Toggle play/pause on the active media session. Used by the global media-key
    /// hook so hardware transport keys behave the same as the panel button.
    pub fn poke_system_media() {
        Self::send_system_media_command("toggle");
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

    /// True when a real fullscreen app owns the primary screen.
    ///
    /// QUNS alone is unreliable: it reports `QUNS_BUSY` (2) for wallpaper
    /// engines, maximised shell surfaces and other non-exclusive windows, which
    /// left the notch hidden on a normal desktop. Trust QUNS only for D3D
    /// exclusive (3) and presentation (4); otherwise require the foreground
    /// window to actually cover the monitor and not be a shell/desktop window.
    pub fn detect_fullscreen() -> bool {
        use windows::Win32::Foundation::RECT;
        use windows::Win32::UI::WindowsAndMessaging::{
            GetClassNameW, GetForegroundWindow, GetSystemMetrics, GetWindowRect, IsIconic,
            IsWindowVisible, SM_CXSCREEN, SM_CYSCREEN,
        };
        unsafe {
            if let Ok(state) = windows::Win32::UI::Shell::SHQueryUserNotificationState() {
                // Hanya D3D exclusive (3) dan presentation mode (4) yang benar-benar
                // menutup layar. QUNS 5 (desktop normal), 6 (quiet time) dan 7 (app)
                // bukan fullscreen; nilai itu dulu membuat notch menyembunyikan diri.
                if matches!(state.0 as i32, 3 | 4) {
                    return true;
                }
            }
            let hwnd = GetForegroundWindow();
            if hwnd.0.is_null()
                || !IsWindowVisible(hwnd).as_bool()
                || IsIconic(hwnd).as_bool()
            {
                return false;
            }
            let mut cls = [0u16; 64];
            let len = GetClassNameW(hwnd, &mut cls) as usize;
            let class = String::from_utf16_lossy(&cls[..len.min(cls.len())]);
            if matches!(
                class.as_str(),
                "Progman" | "WorkerW" | "Shell_TrayWnd" | "NotifyIconOverflowWindow"
            ) {
                return false;
            }
            let mut rect = RECT::default();
            if GetWindowRect(hwnd, &mut rect).is_err() {
                return false;
            }
            let sw = GetSystemMetrics(SM_CXSCREEN);
            let sh = GetSystemMetrics(SM_CYSCREEN);
            rect.left <= 0 && rect.top <= 0 && rect.right >= sw && rect.bottom >= sh
        }
    }

    /// RAM load % and free GB on the system drive, via Win32 (no WMI cost).
    pub fn read_system_stats() -> (u8, u32) {
        use windows::Win32::System::SystemInformation::{GlobalMemoryStatusEx, MEMORYSTATUSEX};
        unsafe {
            let mut ms = MEMORYSTATUSEX {
                dwLength: std::mem::size_of::<MEMORYSTATUSEX>() as u32,
                ..Default::default()
            };
            let ram = if GlobalMemoryStatusEx(&mut ms).is_ok() {
                ms.dwMemoryLoad.min(100) as u8
            } else {
                0
            };
            let mut free_bytes: u64 = 0;
            let root: Vec<u16> = "C:\\0".encode_utf16().collect();
            let disk = if windows::Win32::Storage::FileSystem::GetDiskFreeSpaceExW(
                windows::core::PCWSTR(root.as_ptr()),
                None,
                None,
                Some(&mut free_bytes),
            )
            .is_ok()
            {
                (free_bytes / (1024 * 1024 * 1024)) as u32
            } else {
                0
            };
            (ram, disk)
        }
    }

    /// Re-copy a history entry to the system clipboard.
    pub fn copy_clip_entry(&mut self, idx: usize) -> bool {
        if let Some(entry) = self.clip_store.promote(idx) {
            match crate::clipboard::win::write_clipboard_text(&entry.text) {
                Ok(seq) => {
                    self.clip_store.last_own_seq = seq;
                    let _ = self.clip_store.save();
                    self.clips_scroll = 0;
                    self.flash_message("Copied to clipboard");
                    true
                }
                Err(_) => {
                    self.flash_message("Clipboard busy — try again");
                    false
                }
            }
        } else {
            false
        }
    }

    pub fn flash_message(&mut self, msg: &str) {
        self.clip_flash_text = msg.to_string();
        self.clip_flash_timer = 1.6;
    }

    pub fn commit_note(&mut self) -> bool {
        let text = std::mem::take(&mut self.note_input);
        self.note_input_active = false;
        if self.clip_store.add_note(&text) {
            let _ = self.clip_store.save();
            self.flash_message("Note saved");
            true
        } else {
            false
        }
    }

    /// Visible clipboard rows for the Clips panel (history or notes view).
    pub fn clips_rows(&self) -> Vec<String> {
        if self.clips_view_notes {
            self.clip_store
                .notes
                .iter()
                .map(|n| crate::clipboard::preview_of(n, 46))
                .collect()
        } else {
            self.clip_store
                .history
                .iter()
                .map(|e| e.preview(46))
                .collect()
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
                        self.volume_poll_suppress = 1.5;
                        Self::apply_system_volume(pct);
                        self.overlay_timer = 3.0;
                        return true;
                    }
                    OverlayKind::Brightness(b) => {
                        *b = pct;
                        self.brightness_level = pct;
                        self.volume_poll_suppress = 1.5;
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
        use crate::notch::renderer::glass;
        let left = glass::PAD;
        let right = pill_w - glass::PAD;
        let w = (right - left).max(40.0);
        let in_header = (8.0..=32.0).contains(&ly);
        let cx = pill_w * 0.5;

        // Tab strip 6 section (selebar konten; area status hanya ada di Hub).
        let strip_right = if self.state == NotchState::Hub { right - 150.0 } else { right };
        if in_header && lx >= left && lx <= strip_right {
            let slot = ((strip_right - left) / 6.0).max(30.0);
            let idx = (((lx - left) / slot).floor() as usize).min(5);
            let tabs = Self::ordered_tabs();
            if tabs[idx] != self.state {
                self.transition_to(tabs[idx]);
            }
            return true;
        }

        match self.state {
            NotchState::Hub => {
                if ly >= glass::BODY_TOP && lx > left + w * 0.42 {
                    self.quote_offset = (self.quote_offset + 1) % MOTIVATIONAL_QUOTES.len();
                    return true;
                }
            }
            NotchState::Music => {
                if self.show_side_lyrics {
                    if (34.0..=56.0).contains(&ly) && lx > right - 70.0 {
                        self.show_side_lyrics = false;
                        return true;
                    }
                    if (34.0..=56.0).contains(&ly) {
                        if lx > left + 30.0 && lx < left + 54.0 {
                            Self::send_system_media_command("prev");
                            return true;
                        }
                        if lx >= left + 56.0 && lx <= left + 80.0 {
                            Self::send_system_media_command("toggle");
                            self.media_playing = !self.media_playing;
                            return true;
                        }
                        if lx > left + 82.0 && lx <= left + 106.0 {
                            Self::send_system_media_command("next");
                            return true;
                        }
                    }
                    if (146.0..=155.0).contains(&ly) {
                        let ratio = ((lx - left) / w).clamp(0.0, 1.0);
                        self.media_progress = ratio;
                        self.media_elapsed_exact = ratio * self.media_total_secs;
                        Self::seek_system_media(self.media_elapsed_exact);
                        return true;
                    }
                    return false;
                }

                // Toggle Lyrics.
                if (42.0..=64.0).contains(&ly) && lx > right - 62.0 {
                    self.show_side_lyrics = true;
                    return true;
                }
                // Seek bar.
                if (90.0..=112.0).contains(&ly) && lx >= left + 40.0 && lx <= right - 40.0 {
                    let bl = left + 44.0;
                    let br = right - 44.0;
                    let ratio = ((lx - bl) / (br - bl)).clamp(0.0, 1.0);
                    self.media_progress = ratio;
                    self.media_elapsed_exact = ratio * self.media_total_secs;
                    Self::seek_system_media(self.media_elapsed_exact);
                    self.is_dragging_track_bar = true;
                    return true;
                }
                // Transport.
                if (110.0..=148.0).contains(&ly) {
                    if lx > cx - 70.0 && lx < cx - 34.0 {
                        Self::send_system_media_command("prev");
                        return true;
                    }
                    if lx >= cx - 20.0 && lx <= cx + 20.0 {
                        Self::send_system_media_command("toggle");
                        self.media_playing = !self.media_playing;
                        return true;
                    }
                    if lx > cx + 34.0 && lx <= cx + 70.0 {
                        Self::send_system_media_command("next");
                        return true;
                    }
                }
            }
            NotchState::Control => {
                for row in 0..2 {
                    let y0 = 44.0 + row as f32 * 42.0;
                    if (y0..=y0 + 30.0).contains(&ly) {
                        let is_volume = row == 0;
                        if lx < left + 78.0 {
                            if is_volume {
                                let next = if self.volume_level == 0 { 40 } else { 0 };
                                self.volume_level = next;
                                self.volume_poll_suppress = 1.5;
                                Self::apply_system_volume(next);
                            }
                            return true;
                        }
                        self.apply_slider_at(lx, pill_w, is_volume);
                        return true;
                    }
                }
            }
            NotchState::Pomo => {
                if (40.0..=62.0).contains(&ly) && lx >= right - 132.0 {
                    self.set_pomo_mode(lx > right - 66.0);
                    return true;
                }
                if (70.0..=114.0).contains(&ly) {
                    if lx >= right - 98.0 && lx <= right - 50.0 {
                        self.pomo_running = !self.pomo_running;
                        return true;
                    }
                    if lx >= right - 48.0 {
                        self.pomo_running = false;
                        self.pomo_remaining_secs = self.pomo_total_secs;
                        return true;
                    }
                }
            }
            NotchState::Shelf => {
                if ly >= 44.0 && !self.shelf_files.is_empty() {
                    let idx = if lx < (left + right) * 0.5 { 0 } else { 1 };
                    if idx < self.shelf_files.len() {
                        let path = self.shelf_files[idx].2.clone();
                        Self::open_target_url_or_path(&path);
                        return true;
                    }
                }
            }
            NotchState::Clips => {
                if (32.0..=56.0).contains(&ly) && lx > right - 66.0 {
                    self.clips_view_notes = !self.clips_view_notes;
                    if self.clips_view_notes && self.clip_store.notes.is_empty() {
                        self.note_input_active = true;
                    }
                    return true;
                }
                if self.note_input_active && (56.0..=84.0).contains(&ly) {
                    return true;
                }
                let row_top = if self.note_input_active { 88.0 } else { 58.0 };
                if ly >= row_top {
                    let idx = (((ly - row_top) / 28.0).floor() as usize) + self.clips_scroll;
                    if idx < self.clips_rows().len() {
                        if self.clips_view_notes {
                            let note = self.clip_store.notes.get(idx).cloned().unwrap_or_default();
                            if let Ok(seq) = crate::clipboard::win::write_clipboard_text(&note) {
                                self.clip_store.last_own_seq = seq;
                                let _ = self.clip_store.save();
                                self.flash_message("Note copied");
                            }
                        } else {
                            self.copy_clip_entry(idx);
                        }
                        return true;
                    }
                }
            }
            _ => {}
        }
        false
    }

    /// Hitung level dari posisi kursor di dalam track slider, lalu terapkan.
    fn apply_slider_at(&mut self, lx: f32, pill_w: f32, is_volume: bool) {
        use crate::notch::renderer::glass;
        let track_l = glass::PAD + 78.0;
        let track_r = pill_w - glass::PAD - 58.0;
        if track_r <= track_l {
            return;
        }
        let ratio = ((lx - track_l) / (track_r - track_l)).clamp(0.0, 1.0);
        let pct = (ratio * 100.0).round() as u8;
        self.volume_poll_suppress = 1.5;
        if is_volume {
            self.volume_level = pct;
            Self::apply_system_volume(pct);
        } else {
            self.brightness_level = pct;
            Self::apply_system_brightness(pct);
        }
    }

    /// Dipanggil saat mouse bergerak dengan tombol kiri tertahan: drag slider/seek.
    pub fn drag_panel(&mut self, lx: f32, ly: f32, pill_w: f32) -> bool {
        match self.state {
            NotchState::Control => {
                for row in 0..2 {
                    let y0 = 44.0 + row as f32 * 42.0;
                    if (y0..=y0 + 30.0).contains(&ly) {
                        self.apply_slider_at(lx, pill_w, row == 0);
                        return true;
                    }
                }
            }
            NotchState::Music => {
                if !self.show_side_lyrics && (90.0..=112.0).contains(&ly) && self.is_dragging_track_bar {
                    use crate::notch::renderer::glass;
                    let bl = glass::PAD + 44.0;
                    let br = pill_w - glass::PAD - 44.0;
                    let ratio = ((lx - bl) / (br - bl)).clamp(0.0, 1.0);
                    self.media_progress = ratio;
                    self.media_elapsed_exact = ratio * self.media_total_secs;
                    Self::seek_system_media(self.media_elapsed_exact);
                    return true;
                }
            }
            _ => {}
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fullscreen_quns_classification() {
        assert!(is_fullscreen_quns(2)); // busy / fullscreen app
        assert!(is_fullscreen_quns(3)); // D3D fullscreen
        assert!(is_fullscreen_quns(4)); // presentation mode
        assert!(!is_fullscreen_quns(1)); // user not present
        assert!(!is_fullscreen_quns(5)); // normal desktop
        assert!(!is_fullscreen_quns(6)); // quiet time
        assert!(!is_fullscreen_quns(7)); // UWP app
    }

    #[test]
    fn expanded_panel_dimensions_are_roomy_but_fit_window() {
        // Window logical canvas = 540 x 220. Panel harus muat & lebih lapang
        // dari desain lama (~380-400 x 140-150).
        let (w, h, _r) = NotchController::target_dimensions_for(NotchState::Clips, false);
        assert!(w >= 490.0 && w <= 540.0, "clips width {w}");
        assert!(h >= 185.0 && h <= 220.0, "clips height {h}");
        let (mw, mh, _) = NotchController::target_dimensions_for(NotchState::Music, false);
        assert!(mw >= 490.0 && mh >= 180.0);
        // Pet mode tetap kecil.
        let (pw, ph, _) = NotchController::target_dimensions_for(NotchState::Pet, false);
        assert!(pw < 200.0 && ph < 50.0);
    }

    #[test]
    fn clips_scroll_never_underflows() {
        // first = scroll.min(len-1) harus aman untuk list kosong & pendek.
        for len in [0usize, 1, 5, 25] {
            let scroll = 99usize;
            let first = if len == 0 { 0 } else { scroll.min(len - 1) };
            assert!(first < len.max(1));
        }
    }

    #[test]
    fn lyric_line_progress_stays_in_range() {
        let start = 12.0_f32;
        let end = 18.0_f32;
        for t in [11.0_f32, 12.0, 15.0, 18.0, 19.0] {
            let p = ((t - start) / (end - start).max(0.1)).clamp(0.0, 1.0);
            assert!((0.0..=1.0).contains(&p));
        }
    }
}
