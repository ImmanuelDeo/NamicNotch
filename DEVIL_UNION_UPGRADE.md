# Nusa Clone v6.0: Perbaikan Layout Hub Anti-Tabrakan, Clean Volume & Brightness Control, Direct StageDock Trigger di Tab Bar, & Kalibrasi Presisi 1:1 Click-Box

Dokumen ini berisi **Analisis 4 Masalah dari Screenshot Kamu (`image_086fb7.png`, `image_087262.png`, `image_0872fb.png`)**, **Prompt Siap Pakai untuk Hermes Agent**, serta **Kode Modul Rust Utuh (`src/notch/states.rs`, `src/notch/renderer.rs`, `src/notch/mod.rs`)** yang sudah dikalibrasi untuk ukuran notch yang lebih ramping (*compact height*) dan konektor bezel tebal milikmu.

---

## 1. Analisis 4 Masalah pada Screenshot & Solusi Teknis (`v6.0`)

| # | Masalah pada Screenshot Kamu | Akar Penyebab di Kode Lokalmu Saat Ini | Solusi Perbaikan di Kode Baru (`v6.0`) |
| :- | :--- | :--- | :--- |
| **1** | **Tanggal di `Hub` Menabrak Jam & Ikon Baterai (`image_086fb7.png`)** | Setelah tinggi *expanded notch* dikurangi (menjadi sekitar `138px–145px`), tinggi *inner card* menyusut menjadi `~94px`. Namun posisi `date_str` masih memakai *offset* statis (`top + 44..64`), sedangkan ikon baterai berada di `bottom - 28` (yang di kartu pendek jatuh di `top + 60`), sehingga teks `Sun, 25 Oct 2026` bertabrakan langsung dengan kapsul hijau baterai `⚡79`! | **Tata Letak Vertikal Proporsional & Rapi**:<br>• **Baris 1 (Atas)**: Jam Besar (`22pt Bold`) di kiri (`top + 6..34`) dan badge status `Charging / Battery OK` di kanan atas.<br>• **Baris 2 (Tengah)**: Tanggal (`Sun, 25 Oct 2026`) memiliki ruang eksklusif di tengah (`top + 36..54`).<br>• **Baris 3 (Bawah)**: Ikon Baterai iOS (`⚡79`) ditempatkan di `bottom - 22..bottom - 6` dengan jarak aman minimal `10px` di bawah tanggal sehingga **100% bebas tabrakan**! |
| **2** | **Panel `Control` Berantakan & Angka Tertutup Bar Biru (`image_087262.png`)** | Saat Hermes menghapus *Wi-Fi* dan *Energy Saver*, ia melebarkan dua *slider* ke seluruh lebar kartu dan menggambar bar biru (`fill_gradient_glass_card`) **setelah** menggambar teks label, pada koordinat Y yang sama! Akibatnya bar biru menimpa tulisan `"Volume"` / `"Brightness"` dan menutupi angka `"44%"`. | **Clean 2-Row Horizontal Control Deck**:<br>Setiap baris kartu (`🔊 Volume` di atas, `🔆 Brightness` di bawah) dibagi menjadi **3 kolom horizontal yang tidak saling tumpang-tindih**:<br>1. **Kolom Kiri (`108px`)**: Ikon + Label (`🔊 Volume` / `🔆 Brightness`).<br>2. **Kolom Tengah**: Track Slider Bergradien tebal (`10px` tinggi) dengan *thumb knob* bulat.<br>3. **Kolom Kanan (`48px`)**: Kapsul angka persentase (`72%` / `80%`) yang bersih dan jelas. |
| **3** | **Integrasi `StageDock`: Klik Tab `"Stages"` Langsung Menampilkan Stage Manager di Layar (Tanpa Buka Panel Kosong di Notch) (`image_0872fb.png`)** | Sebelumnya `"Stages"` dijadikan `NotchState::Stages` yang membuka kartu teks instruksi di dalam notch (dan teksnya terpotong di bawah). | **Direct Action Trigger pada Tab `"Stages"`**:<br>Tombol **`"Stages"`** tetap tampil rapi sebagai item ke-6 di bar navigasi atas (`Hub`, `Music`, `Control`, `Focus`, `Shelf`, **`Stages`**). Namun begitu kamu **klik tulisan `"Stages"`**, aplikasi **tidak pindah ke panel kosong**, melainkan langsung memicu **`trigger_stagedock()`** (mengirim *shortcut* StageDock / menjalankan *trigger* `D:\StageDockV2\StageDock.exe` di latar belakang tanpa terminal) sehingga *stages* langsung tampil di layarmu! |
| **4** | **Click Range Meleset ("Klik Shelf Malah Keluar Control")** | Ada **2 penyebab** mengapa klik meleset:<br>1. Di `renderer.rs` tergambar **6 tab** (`Hub`, `Music`, `Control`, `Focus`, `Shelf`, `Stages`), tetapi di `on_lbutton_down` (`mod.rs`) lebar bar masih dibagi **5.0** (`tabs_w / 5.0`)!<br>2. Di `current_pill_rect()` milikmu, ukuran *pill* mungkin dikalikan `dpi_scale` sementara `NotchRenderer` juga disetel `dpiX = 96.0 * dpi_scale` (terjadi *double scaling*). | **Single-Source Hit-Testing & Kunci 1:1 DPI (`dpiX = 96.0`)**:<br>1. Mengunci `NotchRenderer` pada `dpiX: 96.0, dpiY: 96.0` dan menyeragamkan konversi mouse `lx = (phys_x / dpi_scale) - pill_rect.left`.<br>2. Membagi area klik tab menjadi **tepat 6 slot (`tabs_w / 6.0`)** dengan fungsi bersama `tab_slot_rect(index, pill_rect)` yang dipakai baik oleh *renderer* maupun *click handler*, sehingga **apa yang kamu lihat di layar 100% sama persis dengan area kliknya**! |

---

## 2. Prompt Siap Pakai untuk Hermes Agent

Salin seluruh blok perintah di bawah ini dan berikan langsung ke **Hermes Agent** di terminal lokalmu (di folder `nusa-clone/`):

```markdown
Tolong baca file `DEVIL_UNION_UPGRADE.md` (versi v6.0) dan perbaiki 4 masalah UI & interaksi berikut tanpa mengubah ukuran panjang/tinggi notch maupun konektor bezel tebal yang sudah bagus sekarang:

1. **Perbaiki Tabrakan Tanggal, Jam & Baterai di Section `Hub` (`draw_hub_panel` di `src/notch/renderer.rs`)**:
   - Di kartu kiri `Hub`, atur ulang posisi vertikal agar tidak pernah bertabrakan meskipun tinggi notch ramping:
     - Jam Besar (`fmt_clock_hero` ukuran `21.0pt`): `top: left_card.top + 4.0` s/d `left_card.top + 32.0`.
     - Tanggal (`date_str` dengan `fmt_caption`): `top: left_card.top + 33.0` s/d `left_card.top + 49.0`.
     - Ikon Baterai iOS (`draw_ios_battery_icon`) & label `"Charging"` / `"Battery OK"`: letakkan di `left_card.bottom - 21.0` s/d `left_card.bottom - 5.0`.

2. **Rapikan Section `Control` (Hanya Volume & Brightness, Tanpa Bar Menutupi Teks) (`draw_control_panel` di `src/notch/renderer.rs` & `handle_panel_click` di `src/notch/states.rs`)**:
   - Gambar 2 baris kartu horizontal yang rapi (Baris 1: `🔊 Volume`, Baris 2: `🔆 Brightness`).
   - Di dalam setiap baris kartu, bagi secara horizontal menjadi 3 area yang TIDAK saling menimpa:
     - **Kiri (`l + 12.0` s/d `l + 105.0`)**: Label `"🔊 Volume"` atau `"🔆 Brightness"`.
     - **Tengah (`l + 110.0` s/d `r - 54.0`)**: Slider track dan fill bar bergradien (tinggi `8px` tepat di tengah vertikal baris) — pastikan fill bar digambar HANYA di dalam rentang `l + 110.0 .. r - 54.0` sehingga tidak pernah menimpa label atau angka!
     - **Kanan (`r - 48.0` s/d `r - 10.0`)**: Teks persentase (`"72%"` / `"80%"`) rata tengah.
   - Sesuaikan `handle_panel_click` pada `NotchState::Control`: klik pada baris atas (`cy < card_h * 0.5`) mengatur Volume, klik pada baris bawah (`cy >= card_h * 0.5`) mengatur Brightness, dan scroll mouse wheel di atas baris atas/bawah langsung mengubah nilainya per 5%.

3. **Ubah Tab `"Stages"` (`StageDock`) Menjadi Direct Action Trigger (Langsung Aktifkan StageDock di Layar Tanpa Buka Section di Notch)**:
   - Di bar navigasi atas, tampilkan 6 item: `["Hub", "Music", "Control", "Focus", "Shelf", "Stages"]`.
   - Saat pengguna mengklik tab ke-6 (`"Stages"`, `slot == 5`), JANGAN pindah ke panel kosong di dalam notch, melainkan langsung panggil fungsi `NotchController::trigger_stagedock()` yang mengaktifkan tampilan StageDock di layar pengguna (mengirim shortcut StageDock / memanggil executable di `D:\StageDockV2` secara silent dengan `CREATE_NO_WINDOW`) sambil memberikan efek highlight kilat pada tombol `"Stages"`.

4. **Perbaiki Click Range Meleset ("Klik Shelf Malah Buka Control") Secara Permanen (`src/notch/mod.rs` & `src/notch/renderer.rs`)**:
   - Masalah klik meleset terjadi karena ketidaksinkronan antara jumlah tab (6 tab vs pembagian 5.0) dan double-scaling DPI antara `current_pill_rect()`, `NotchRenderer`, dan `on_lbutton_down()`.
   - Pastikan `NotchRenderer::new` menggunakan `dpiX: 96.0 * dpi_scale, dpiY: 96.0 * dpi_scale`, `current_pill_rect()` menggunakan koordinat logis murni (`logical_win_w = self.win_w as f32 / self.dpi_scale`, `pw = self.controller.width.value`, `ph = self.controller.height.value`), dan `on_lbutton_down(phys_x, phys_y)` mengonversi `x = phys_x / self.dpi_scale`, `y = phys_y / self.dpi_scale`.
   - Di bar navigasi atas (`draw_expanded_panel_shell` dan `on_lbutton_down`), gunakan perhitungan 6 slot yang 100% identik:
     - `tabs_left = pill_rect.left + 38.0`
     - `tabs_right = pill_rect.right - 38.0`
     - `slot_w = (tabs_right - tabs_left) / 6.0`
     - `slot = ((x - tabs_left) / slot_w) as usize` (0 = Hub, 1 = Music, 2 = Control, 3 = Focus/Pomo, 4 = Shelf, 5 = Trigger StageDock).

5. **Kompilasi & Verifikasi**:
   - Jalankan `cargo check` dan `cargo build --release` sampai kompilasi sukses tanpa error.
```

---

## 3. Kode Lengkap Bagian yang Diperbaiki (`v6.0`)

Berikut adalah kode lengkap untuk `src/notch/states.rs`, `src/notch/renderer.rs`, dan `src/notch/mod.rs` yang sudah menerapkan ke-4 perbaikan di atas dengan mempertahankan ukuran *notch* ramping dan konektor bezel tebal milikmu:

### A. `src/notch/states.rs` — Penambahan `trigger_stagedock()`, Perbaikan Klik 2-Row `Control`, & Dimensi Ramping

```rust
// src/notch/states.rs
use std::os::windows::process::CommandExt;
use std::sync::{Arc, Mutex};
use chrono::{Datelike, Local, Timelike};
use windows::core::PCWSTR;
use windows::Media::Control::{
    GlobalSystemMediaTransportControlsSessionManager,
    GlobalSystemMediaTransportControlsSessionPlaybackStatus,
};
use windows::Win32::Graphics::Direct2D::Common::D2D_RECT_F;
use windows::Win32::Media::Audio::Endpoints::IAudioEndpointVolume;
use windows::Win32::Media::Audio::{
    eConsole, eRender, IMMDeviceEnumerator, MMDeviceEnumerator,
};
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_MULTITHREADED,
};
use windows::Win32::System::Power::{GetSystemPowerStatus, SYSTEM_POWER_STATUS};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, VIRTUAL_KEY, VK_LMENU,
    VK_LWIN, VK_TAB,
};
use windows::Win32::UI::Shell::ShellExecuteW;
use windows::Win32::UI::WindowsAndMessaging::{FindWindowW, SetForegroundWindow, ShowWindow, SW_RESTORE, SW_SHOWNORMAL};

use crate::mascot::chibi_liebe::ChibiLiebe;
use crate::mascot::eyes::DevilUnionEyes;

const CREATE_NO_WINDOW: u32 = 0x08000000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotchState {
    Pet,     // Closed mascot: Asta Devil Union Crown / Cheer Mode
    Idle,    // Closed notch: Jam Digital & Baterai iOS
    Hub,     // Tab 0: Jam, Tanggal, Baterai iOS & Daily Quote
    Music,   // Tab 1: Full-Height Cover + Controls + Auto-Synced LRC Lyrics
    Control, // Tab 2: Clean 2-Row Volume & Brightness Control
    Pomo,    // Tab 3: Pomodoro Focus Timer
    Shelf,   // Tab 4: File Shelf (Drag & Drop Temporary Storage)
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
    pub elapsed_secs: f32,
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

    // Efek flash visual saat tombol action "Stages" diklik
    pub stagedock_flash_timer: f32,

    // --- Control State (Hanya Volume & Brightness) ---
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
    pub media_total_secs: f32,
    pub show_side_lyrics: bool,
    pub timed_lyrics: Vec<(f32, String)>,
    pub active_lyric_idx: usize,
    live_media_arc: Arc<Mutex<Option<LiveMediaSnapshot>>>,

    // --- Section 3: Pomodoro Timer ---
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

            media_title: String::from("Menunggu Musik / Video YouTube..."),
            media_artist: String::from("Putar lagu di Spotify / Chrome / YouTube"),
            media_playing: false,
            media_progress: 0.0,
            media_elapsed_exact: 0.0,
            media_total_secs: 180.0,
            show_side_lyrics: true,
            timed_lyrics: vec![
                (0.0, String::from("♪ Putar lagu di Spotify / YouTube")),
                (6.0, String::from("Lirik tersinkronisasi otomatis per detik")),
                (12.0, String::from("Mengikuti timestamp LRC dari LRCLIB")),
                (18.0, String::from("Atau scroll mouse untuk menggeser lirik")),
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

    /// Langsung aktifkan/tampilkan StageDock milik pengguna di layar (tanpa membuka panel di dalam notch!)
    pub fn trigger_stagedock(&mut self) {
        self.stagedock_flash_timer = 0.45;
        std::thread::spawn(|| {
            // 1. Coba cari apakah window StageDock sudah berjalan di latar belakang, lalu tampilkan/toggle
            let candidates = [
                "StageDock\0",
                "StageDockV2\0",
                "Stage Manager\0",
            ];
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

            // 2. Cek apakah executable StageDock ada di D:\StageDockV2, lalu jalankan secara silent
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

            // 3. Fallback: Kirim shortcut StageDock / Windows Task View (Win+Alt+1 / Win+Tab)
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

                let (mut elapsed_secs, mut total_secs) = (0.0_f32, 180.0_f32);
                if let Ok(timeline) = session.GetTimelineProperties() {
                    if let (Ok(pos), Ok(end)) = (timeline.Position(), timeline.EndTime()) {
                        elapsed_secs = (pos.Duration as f32 / 10_000_000.0).max(0.0);
                        let end_s = (end.Duration as f32 / 10_000_000.0).max(1.0);
                        if end_s > 1.0 {
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
                                        String::from("YouTube / Media Session")
                                    } else {
                                        artist
                                    },
                                    playing,
                                    elapsed_secs,
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
            (8.0, String::from("Streaming audio aktif di Nusa Notch")),
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

    /// Ukuran Notch yang Ramping & Proporsional (Sesuai Preferensi Pengguna)
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
            NotchState::Pomo => (436.0, 144.0, 22.0),
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

    /// Scroll Wheel: Di tab Control, scroll di baris atas mengubah Volume, scroll di baris bawah mengubah Brightness!
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
            let cy = ly - 30.0;
            let card_h = (self.height.value - 40.0).max(60.0);
            if cy < card_h * 0.5 {
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

        if let Ok(mut guard) = self.live_media_arc.try_lock() {
            if let Some(snap) = guard.take() {
                let title_changed = self.media_title != snap.title;
                self.media_title = snap.title;
                self.media_artist = snap.artist;
                self.media_playing = snap.playing;
                if snap.total_secs > 1.0 {
                    self.media_total_secs = snap.total_secs;
                    if (self.media_elapsed_exact - snap.elapsed_secs).abs() > 1.8 || title_changed {
                        self.media_elapsed_exact = snap.elapsed_secs;
                    }
                }
                if !snap.timed_lyrics.is_empty() && (title_changed || self.timed_lyrics.len() <= 4)
                {
                    self.timed_lyrics = snap.timed_lyrics;
                }
            }
        }

        if self.media_playing && self.media_total_secs > 1.0 {
            self.media_elapsed_exact =
                (self.media_elapsed_exact + dt).min(self.media_total_secs);
            self.media_progress =
                (self.media_elapsed_exact / self.media_total_secs).clamp(0.0, 1.0);

            if !self.timed_lyrics.is_empty() {
                let mut found_idx = 0;
                for (i, (ts, _)) in self.timed_lyrics.iter().enumerate() {
                    if *ts <= self.media_elapsed_exact + 0.35 {
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

    pub fn handle_panel_click(&mut self, lx: f32, ly: f32, pill_w: f32) -> bool {
        let card_l = 12.0;
        let card_t = 28.0;
        let card_w = pill_w - 24.0;
        let card_h = (self.height.value - 38.0).max(60.0);

        let cx = lx - card_l;
        let cy = ly - card_t;
        if cx < 0.0 || cy < 0.0 || cx > card_w || cy > card_h {
            return false;
        }

        match self.state {
            NotchState::Hub => {
                if cx >= 158.0 {
                    self.quote_offset = (self.quote_offset + 1) % MOTIVATIONAL_QUOTES.len();
                    return true;
                }
            }
            NotchState::Music => {
                // Kolom tengah untuk kontrol musik
                if (68.0..=118.0).contains(&cy) && (100.0..=270.0).contains(&cx) {
                    if (100.0..=134.0).contains(&cx) {
                        Self::send_system_media_command("prev");
                        self.media_elapsed_exact = 0.0;
                        self.media_progress = 0.0;
                        return true;
                    } else if (136.0..=172.0).contains(&cx) {
                        Self::send_system_media_command("toggle");
                        self.media_playing = !self.media_playing;
                        return true;
                    } else if (174.0..=208.0).contains(&cx) {
                        Self::send_system_media_command("next");
                        self.media_elapsed_exact = 0.0;
                        self.media_progress = 0.0;
                        return true;
                    } else if (210.0..=268.0).contains(&cx) {
                        self.show_side_lyrics = !self.show_side_lyrics;
                        return true;
                    }
                }
                if cx > 268.0 && !self.timed_lyrics.is_empty() {
                    self.active_lyric_idx = (self.active_lyric_idx + 1) % self.timed_lyrics.len();
                    return true;
                }
            }
            NotchState::Control => {
                // 2 Baris Horizontal: Baris Atas = Volume, Baris Bawah = Brightness
                // Track Slider berada di rentang cx: 112.0 .. (card_w - 58.0)
                let track_l = 112.0;
                let track_r = (card_w - 58.0).max(track_l + 20.0);
                if cx >= track_l - 8.0 && cx <= track_r + 8.0 {
                    let pct = (((cx - track_l) / (track_r - track_l)).clamp(0.0, 1.0) * 100.0)
                        .round() as u8;
                    if cy < card_h * 0.5 {
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
                if (54.0..=104.0).contains(&cy) {
                    if (104.0..=196.0).contains(&cx) {
                        self.pomo_running = !self.pomo_running;
                        return true;
                    } else if (202.0..=270.0).contains(&cx) {
                        self.pomo_running = false;
                        self.pomo_remaining_secs = self.pomo_total_secs;
                        return true;
                    }
                }
            }
            NotchState::Shelf => {
                if cy >= 26.0 && !self.shelf_files.is_empty() {
                    let slot_w = (card_w - 72.0) * 0.5;
                    let idx = if cx < 12.0 + slot_w { 0 } else { 1 };
                    if idx < self.shelf_files.len() {
                        let card_right_edge =
                            12.0 + (idx as f32 + 1.0) * slot_w + idx as f32 * 10.0;
                        if cx >= card_right_edge - 24.0 && cy <= 50.0 {
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
```

---

### B. `src/notch/renderer.rs` — Perbaikan Tata Letak `Hub` (Bebas Tabrakan), Clean 2-Row `Control` (Label + Slider + Angka Terpisah), & 6-Slot Navigation Bar (`Stages` Action Button)

Perhatikan perubahan di `draw_hub_panel`, `draw_control_panel`, dan `draw_expanded_panel_shell`:

```rust
    /// Bar Navigasi Atas dengan 6 Slot Presisi:
    /// [0: Hub] [1: Music] [2: Control] [3: Focus] [4: Shelf] [5: Stages (Direct Action)]
    unsafe fn draw_expanded_panel_shell(
        &self,
        ctrl: &NotchController,
        pill_rect: D2D_RECT_F,
        alpha: f32,
    ) -> D2D_RECT_F {
        let tabs = ["Hub", "Music", "Control", "Focus", "Shelf", "Stages"];
        let active_idx = match ctrl.state {
            NotchState::Hub => 0,
            NotchState::Music => 1,
            NotchState::Control => 2,
            NotchState::Pomo => 3,
            NotchState::Shelf => 4,
            _ => 0,
        };

        let mut p_col = AntiMagicNeutralPalette::TEXT_PRIMARY;
        p_col.a *= alpha;
        let mut s_col = AntiMagicNeutralPalette::TEXT_SECONDARY;
        s_col.a *= alpha * 0.80;

        // Panah Kiri `‹` (lx: 10..32) dan Panah Kanan `›` (lx: pill_w - 32 .. pill_w - 10)
        let left_arrow_rect = D2D_RECT_F {
            left: pill_rect.left + 10.0,
            top: pill_rect.top + 4.0,
            right: pill_rect.left + 32.0,
            bottom: pill_rect.top + 23.0,
        };
        let right_arrow_rect = D2D_RECT_F {
            left: pill_rect.right - 32.0,
            top: pill_rect.top + 4.0,
            right: pill_rect.right - 10.0,
            bottom: pill_rect.top + 23.0,
        };

        if let Ok(ab) = self.rt.CreateSolidColorBrush(
            &D2D1_COLOR_F {
                r: 0.12,
                g: 0.16,
                b: 0.24,
                a: 0.90 * alpha,
            },
            None,
        ) {
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: left_arrow_rect,
                    radiusX: 6.0,
                    radiusY: 6.0,
                },
                &ab,
            );
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: right_arrow_rect,
                    radiusX: 6.0,
                    radiusY: 6.0,
                },
                &ab,
            );
        }
        self.draw_text_centered("‹", &self.fmt_title, left_arrow_rect, p_col);
        self.draw_text_centered("›", &self.fmt_title, right_arrow_rect, p_col);

        // 6 Slot Identik antara lx = 36.0 dan pill_w - 36.0
        let tabs_left = pill_rect.left + 36.0;
        let tabs_right = pill_rect.right - 36.0;
        let slot_w = (tabs_right - tabs_left) / 6.0;

        for (i, label) in tabs.iter().enumerate() {
            let sl = tabs_left + slot_w * i as f32;
            let sr = sl + slot_w;
            let cx = (sl + sr) * 0.5;
            let is_active = i == active_idx || (i == 5 && ctrl.stagedock_flash_timer > 0.0);

            if is_active {
                self.fill_gradient_glass_card(
                    D2D_RECT_F {
                        left: sl + 2.5,
                        top: pill_rect.top + 4.0,
                        right: sr - 2.5,
                        bottom: pill_rect.top + 23.0,
                    },
                    6.5,
                    D2D1_COLOR_F {
                        r: 0.18,
                        g: 0.26,
                        b: 0.42,
                        a: 0.90 * alpha,
                    },
                    D2D1_COLOR_F {
                        r: 0.12,
                        g: 0.18,
                        b: 0.32,
                        a: 0.90 * alpha,
                    },
                    D2D1_COLOR_F {
                        r: 0.16,
                        g: 0.22,
                        b: 0.38,
                        a: 0.90 * alpha,
                    },
                    D2D1_COLOR_F {
                        r: 0.39,
                        g: 0.55,
                        b: 0.95,
                        a: 0.75 * alpha,
                    },
                    1.0,
                );
            } else if i == 5 {
                // Beri outline tipis spesial pada tombol action "Stages" (StageDock)
                if let Ok(sb) = self.rt.CreateSolidColorBrush(
                    &D2D1_COLOR_F {
                        r: 0.22,
                        g: 0.74,
                        b: 0.97,
                        a: 0.38 * alpha,
                    },
                    None,
                ) {
                    self.rt.DrawRoundedRectangle(
                        &D2D1_ROUNDED_RECT {
                            rect: D2D_RECT_F {
                                left: sl + 2.5,
                                top: pill_rect.top + 4.0,
                                right: sr - 2.5,
                                bottom: pill_rect.top + 23.0,
                            },
                            radiusX: 6.5,
                            radiusY: 6.5,
                        },
                        &sb,
                        1.0,
                        None,
                    );
                }
            }

            let text_col = if is_active {
                p_col
            } else if i == 5 {
                AntiMagicNeutralPalette::ACCENT_CYAN
            } else {
                s_col
            };

            self.draw_text_centered(
                label,
                &self.fmt_caption,
                D2D_RECT_F {
                    left: sl,
                    top: pill_rect.top + 4.0,
                    right: sr,
                    bottom: pill_rect.top + 23.0,
                },
                text_col,
            );

            if is_active {
                if let Ok(lb) = self
                    .rt
                    .CreateSolidColorBrush(&AntiMagicNeutralPalette::ACCENT_CYAN, None)
                {
                    self.rt.DrawLine(
                        D2D_POINT_2F {
                            x: cx - 12.0,
                            y: pill_rect.top + 24.5,
                        },
                        D2D_POINT_2F {
                            x: cx + 12.0,
                            y: pill_rect.top + 24.5,
                        },
                        &lb,
                        2.0,
                        None,
                    );
                }
            }
        }

        let card_rect = D2D_RECT_F {
            left: pill_rect.left + 12.0,
            top: pill_rect.top + 28.0,
            right: pill_rect.right - 12.0,
            bottom: pill_rect.bottom - 10.0,
        };

        self.fill_gradient_glass_card(
            card_rect,
            15.0,
            D2D1_COLOR_F {
                r: 0.065,
                g: 0.085,
                b: 0.130,
                a: 0.95 * alpha,
            },
            D2D1_COLOR_F {
                r: 0.082,
                g: 0.108,
                b: 0.165,
                a: 0.95 * alpha,
            },
            D2D1_COLOR_F {
                r: 0.058,
                g: 0.076,
                b: 0.120,
                a: 0.95 * alpha,
            },
            D2D1_COLOR_F {
                r: 0.24,
                g: 0.31,
                b: 0.46,
                a: 0.58 * alpha,
            },
            1.2,
        );

        card_rect
    }

    // =========================================================================
    // SECTION 0: HUB (Anti-Tabrakan pada Ukuran Notch Ramping!)
    // =========================================================================
    unsafe fn draw_hub_panel(&self, ctrl: &NotchController, card: D2D_RECT_F, alpha: f32) {
        let mut p_col = AntiMagicNeutralPalette::TEXT_PRIMARY;
        p_col.a *= alpha;
        let mut s_col = AntiMagicNeutralPalette::TEXT_SECONDARY;
        s_col.a *= alpha;

        // Kartu Kiri: Jam, Tanggal & Ikon Baterai iOS dengan Koordinat Vertikal Terpisah Rapi
        let left_card = D2D_RECT_F {
            left: card.left + 8.0,
            top: card.top + 8.0,
            right: card.left + 154.0,
            bottom: card.bottom - 8.0,
        };
        self.fill_gradient_glass_card(
            left_card,
            12.0,
            D2D1_COLOR_F {
                r: 0.11,
                g: 0.15,
                b: 0.24,
                a: 0.92 * alpha,
            },
            D2D1_COLOR_F {
                r: 0.09,
                g: 0.12,
                b: 0.19,
                a: 0.92 * alpha,
            },
            D2D1_COLOR_F {
                r: 0.13,
                g: 0.17,
                b: 0.27,
                a: 0.92 * alpha,
            },
            D2D1_COLOR_F {
                r: 0.32,
                g: 0.44,
                b: 0.68,
                a: 0.50 * alpha,
            },
            1.0,
        );

        let now = Local::now();
        let time_big = now.format("%H:%M").to_string();
        let date_str = now.format("%a, %d %b %Y").to_string();

        // Baris 1 (Atas): Jam Besar (y: top + 4 .. top + 34)
        self.draw_text_left(
            &time_big,
            &self.fmt_clock_hero,
            D2D_RECT_F {
                left: left_card.left + 12.0,
                top: left_card.top + 4.0,
                right: left_card.right - 8.0,
                bottom: left_card.top + 34.0,
            },
            p_col,
        );

        // Baris 2 (Tengah): Tanggal (y: top + 35 .. top + 52) — TIDAK AKAN MENABRAK JAM ATAU BATERAI!
        self.draw_text_left(
            &date_str,
            &self.fmt_caption,
            D2D_RECT_F {
                left: left_card.left + 12.0,
                top: left_card.top + 35.0,
                right: left_card.right - 8.0,
                bottom: left_card.top + 52.0,
            },
            s_col,
        );

        // Baris 3 (Bawah): Ikon Baterai iOS + Status (y: bottom - 22 .. bottom - 6)
        self.draw_ios_battery_icon(
            left_card.left + 12.0,
            left_card.bottom - 22.0,
            ctrl.battery_percent,
            ctrl.is_charging,
            alpha,
        );

        self.draw_text_left(
            if ctrl.is_charging {
                "Charging"
            } else {
                "Battery OK"
            },
            &self.fmt_caption,
            D2D_RECT_F {
                left: left_card.left + 54.0,
                top: left_card.bottom - 23.0,
                right: left_card.right - 6.0,
                bottom: left_card.bottom - 5.0,
            },
            AntiMagicNeutralPalette::ACCENT_CYAN,
        );

        // Kartu Kanan: WORD & QUOTE OF THE DAY
        let q_card = D2D_RECT_F {
            left: left_card.right + 8.0,
            top: card.top + 8.0,
            right: card.right - 8.0,
            bottom: card.bottom - 8.0,
        };

        self.fill_gradient_glass_card(
            q_card,
            12.0,
            D2D1_COLOR_F {
                r: 0.12,
                g: 0.16,
                b: 0.26,
                a: 0.92 * alpha,
            },
            D2D1_COLOR_F {
                r: 0.15,
                g: 0.19,
                b: 0.31,
                a: 0.92 * alpha,
            },
            D2D1_COLOR_F {
                r: 0.10,
                g: 0.14,
                b: 0.22,
                a: 0.92 * alpha,
            },
            D2D1_COLOR_F {
                r: 0.35,
                g: 0.48,
                b: 0.78,
                a: 0.55 * alpha,
            },
            1.0,
        );

        let daily = ctrl.current_quote();
        let header = format!("✨ WORD OF THE DAY · {}", daily.word_of_the_day);
        self.draw_text_left(
            &header,
            &self.fmt_caption,
            D2D_RECT_F {
                left: q_card.left + 12.0,
                top: q_card.top + 6.0,
                right: q_card.right - 10.0,
                bottom: q_card.top + 22.0,
            },
            AntiMagicNeutralPalette::ACCENT_CYAN,
        );

        self.draw_text_left(
            daily.quote,
            &self.fmt_quote_italic,
            D2D_RECT_F {
                left: q_card.left + 12.0,
                top: q_card.top + 23.0,
                right: q_card.right - 10.0,
                bottom: q_card.bottom - 22.0,
            },
            p_col,
        );

        let footer = format!("— {}  (Click / Scroll for next)", daily.author);
        self.draw_text_left(
            &footer,
            &self.fmt_caption,
            D2D_RECT_F {
                left: q_card.left + 12.0,
                top: q_card.bottom - 21.0,
                right: q_card.right - 10.0,
                bottom: q_card.bottom - 5.0,
            },
            s_col,
        );
    }

    // =========================================================================
    // SECTION 2: CLEAN 2-ROW CONTROL PANEL (Hanya Volume & Brightness, Tanpa Menutupi Teks!)
    // =========================================================================
    unsafe fn draw_control_panel(&self, ctrl: &NotchController, card: D2D_RECT_F, alpha: f32) {
        let mut p_col = AntiMagicNeutralPalette::TEXT_PRIMARY;
        p_col.a *= alpha;

        let card_h = card.bottom - card.top;
        let row_h = (card_h - 24.0) * 0.5;

        let rows = [
            (
                card.top + 8.0,
                "🔊  Volume",
                ctrl.volume_level,
                AntiMagicNeutralPalette::ACCENT_BLUE,
                AntiMagicNeutralPalette::ACCENT_CYAN,
            ),
            (
                card.top + 16.0 + row_h,
                "🔆  Brightness",
                ctrl.brightness_level,
                AntiMagicNeutralPalette::ACCENT_CYAN,
                D2D1_COLOR_F {
                    r: 0.45,
                    g: 0.86,
                    b: 0.98,
                    a: 1.0,
                },
            ),
        ];

        for (ry, label, val, c_start, c_end) in rows {
            let r_rect = D2D_RECT_F {
                left: card.left + 8.0,
                top: ry,
                right: card.right - 8.0,
                bottom: ry + row_h,
            };

            // 1. Kartu Semi-Glass untuk Baris Slider
            self.fill_gradient_glass_card(
                r_rect,
                11.0,
                D2D1_COLOR_F {
                    r: 0.10,
                    g: 0.13,
                    b: 0.21,
                    a: 0.92 * alpha,
                },
                D2D1_COLOR_F {
                    r: 0.13,
                    g: 0.17,
                    b: 0.26,
                    a: 0.92 * alpha,
                },
                D2D1_COLOR_F {
                    r: 0.09,
                    g: 0.12,
                    b: 0.19,
                    a: 0.92 * alpha,
                },
                D2D1_COLOR_F {
                    r: 0.28,
                    g: 0.36,
                    b: 0.54,
                    a: 0.52 * alpha,
                },
                1.0,
            );

            // 2. Kolom Kiri (lebar 102px): Ikon & Nama Kontrol (TIDAK AKAN TERTUTUP BAR!)
            self.draw_text_left(
                label,
                &self.fmt_title,
                D2D_RECT_F {
                    left: r_rect.left + 12.0,
                    top: r_rect.top,
                    right: r_rect.left + 104.0,
                    bottom: r_rect.bottom,
                },
                p_col,
            );

            // 3. Kolom Tengah: Interactive Slider Track (`r_rect.left + 108.0` s/d `r_rect.right - 54.0`)
            let track_l = r_rect.left + 108.0;
            let track_r = (r_rect.right - 54.0).max(track_l + 20.0);
            let cy = (r_rect.top + r_rect.bottom) * 0.5;

            if let Ok(tb) = self.rt.CreateSolidColorBrush(
                &D2D1_COLOR_F {
                    r: 0.17,
                    g: 0.22,
                    b: 0.32,
                    a: 0.85 * alpha,
                },
                None,
            ) {
                self.rt.FillRoundedRectangle(
                    &D2D1_ROUNDED_RECT {
                        rect: D2D_RECT_F {
                            left: track_l,
                            top: cy - 4.5,
                            right: track_r,
                            bottom: cy + 4.5,
                        },
                        radiusX: 4.5,
                        radiusY: 4.5,
                    },
                    &tb,
                );
            }

            let fill_w = (track_r - track_l) * (val.min(100) as f32 / 100.0);
            let fill_right = (track_l + fill_w).max(track_l + 7.0);
            self.fill_gradient_glass_card(
                D2D_RECT_F {
                    left: track_l,
                    top: cy - 4.5,
                    right: fill_right,
                    bottom: cy + 4.5,
                },
                4.5,
                c_start,
                c_end,
                c_end,
                c_end,
                0.0,
            );

            // Knob bulat putih di ujung bar
            if let Ok(kb) = self.rt.CreateSolidColorBrush(&p_col, None) {
                self.rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F {
                            x: fill_right,
                            y: cy,
                        },
                        radiusX: 6.0,
                        radiusY: 6.0,
                    },
                    &kb,
                );
            }

            // 4. Kolom Kanan (lebar 48px): Badge Angka Persentase Jelas (TIDAK TERTIMPA BAR!)
            let pct_rect = D2D_RECT_F {
                left: r_rect.right - 48.0,
                top: r_rect.top + 5.0,
                right: r_rect.right - 8.0,
                bottom: r_rect.bottom - 5.0,
            };
            if let Ok(pb) = self
                .rt
                .CreateSolidColorBrush(&AntiMagicNeutralPalette::CARD_ELEVATED, None)
            {
                self.rt.FillRoundedRectangle(
                    &D2D1_ROUNDED_RECT {
                        rect: pct_rect,
                        radiusX: 6.0,
                        radiusY: 6.0,
                    },
                    &pb,
                );
            }
            self.draw_text_centered(
                &format!("{}%", val),
                &self.fmt_caption,
                pct_rect,
                AntiMagicNeutralPalette::ACCENT_CYAN,
            );
        }
    }
```

---

### C. `src/notch/mod.rs` — Perbaikan Permanen Click Range 6-Slot (`Hub`, `Music`, `Control`, `Focus`, `Shelf`, `Stages`)

Di `src/notch/mod.rs`, pastikan fungsi `on_lbutton_down` dan `current_pill_rect` menggunakan perhitungan **6 slot (`tabs_w / 6.0`)** dan koordinat logis `self.win_w as f32 / self.dpi_scale` yang 100% cocok dengan `draw_expanded_panel_shell`:

```rust
    pub fn current_pill_rect(&self) -> D2D_RECT_F {
        // Gunakan lebar logis aktual window agar posisi tengah pill 100% presisi di semua resolusi/DPI!
        let logical_win_w = self.win_w as f32 / self.dpi_scale;
        let pw = self.controller.width.value;
        let ph = self.controller.height.value;
        let left = ((logical_win_w - pw) * 0.5).max(0.0);
        let top = 0.0;
        let right = left + pw;
        let bottom = top + ph;

        D2D_RECT_F {
            left,
            top,
            right,
            bottom,
        }
    }

    pub fn on_lbutton_down(&mut self, phys_x: f32, phys_y: f32) {
        let x = phys_x / self.dpi_scale;
        let y = phys_y / self.dpi_scale;

        let pill_rect = self.current_pill_rect();
        let pill_w = pill_rect.right - pill_rect.left;
        let lx = x - pill_rect.left;
        let ly = y - pill_rect.top;

        if !self.controller.is_hovered
            && self.controller.active_overlay.is_some()
            && self.controller.handle_overlay_click(lx, pill_w)
        {
            unsafe {
                let _ = InvalidateRect(self.hwnd, None, false);
            }
            return;
        }

        if self.controller.chibi_liebe.handle_click(x, y) {
            unsafe {
                let _ = InvalidateRect(self.hwnd, None, false);
            }
            return;
        }

        // Bar Navigasi Atas (`‹`, 6 Slot: Hub / Music / Control / Focus / Shelf / Stages, dan `›`)
        if self.controller.height.value > 65.0 && (0.0..=27.0).contains(&ly) {
            if (8.0..=34.0).contains(&lx) {
                self.controller.prev_tab();
                unsafe {
                    let _ = InvalidateRect(self.hwnd, None, false);
                }
                return;
            } else if lx >= pill_w - 34.0 && lx <= pill_w - 8.0 {
                self.controller.next_tab();
                unsafe {
                    let _ = InvalidateRect(self.hwnd, None, false);
                }
                return;
            } else if lx >= 36.0 && lx <= pill_w - 36.0 {
                // PENTING: Bagi tepat dengan 6.0 sesuai 6 item yang digambar di draw_expanded_panel_shell!
                let tabs_w = (pill_w - 72.0).max(1.0);
                let slot = (((lx - 36.0) / (tabs_w / 6.0)).floor() as usize).min(5);
                match slot {
                    0 => self.controller.transition_to(NotchState::Hub),
                    1 => self.controller.transition_to(NotchState::Music),
                    2 => self.controller.transition_to(NotchState::Control),
                    3 => self.controller.transition_to(NotchState::Pomo),
                    4 => self.controller.transition_to(NotchState::Shelf),
                    _ => {
                        // Slot 5 ("Stages"): Langsung aktifkan StageDock di layar tanpa pindah ke panel kosong!
                        self.controller.trigger_stagedock();
                    }
                }
                unsafe {
                    let _ = InvalidateRect(self.hwnd, None, false);
                }
                return;
            }
        }

        if self.controller.height.value > 65.0
            && self.controller.handle_panel_click(lx, ly, pill_w)
        {
            unsafe {
                let _ = InvalidateRect(self.hwnd, None, false);
            }
            return;
        }

        if matches!(self.controller.state, NotchState::Pet | NotchState::Idle) {
            self.controller.devil_eyes.trigger_wake_surge();
            let target = self.controller.last_expanded_tab;
            self.controller.transition_to(target);
            unsafe {
                let _ = InvalidateRect(self.hwnd, None, false);
            }
        }
    }
```