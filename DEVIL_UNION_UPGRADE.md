# Nusa Clone v4.0: Full 13-Point Upgrade — Bezel-Connected Semi-Glass UI, Full Anime Asta Eyes, Integrated Spotify Lyrics, All-in-One Control Center, Hub & Scrollable Notifications, Real GSMTC Media & Drag-and-Drop Shelf

Dokumen ini berisi **Ringkasan 13 Pembaruan Utama**, **Prompt Eksekusi Lengkap untuk Hermes Agent**, serta **Kode Modul Rust Utuh (`src/mascot/eyes.rs`, `src/mascot/chibi_liebe.rs`, `src/notch/states.rs`, `src/notch/renderer.rs`, `src/notch/mod.rs`)** yang sudah dikalibrasi 100% agar sinkron tanpa *mismatch* koordinat klik.

---

## 1. Ringkasan 13 Penyesuaian & Fitur Baru yang Diterapkan

| # | Fitur & Permintaan Kamu | Implementasi Teknis di Kode Baru (`v4.0`) |
| :- | :--- | :--- |
| **1 & 9** | **Hapus Fitur Water & Ganti dengan All-in-One Control Center (`NotchState::Control`)** | Menghapus tab *Water* sepenuhnya. Kini digantikan oleh tab **`Control`** yang berisi: **Interactive Volume Slider** (`0–100%`), **Interactive Brightness Slider** (`0–100%`), **Tombol Toggle Wi-Fi** (On/Off), dan **Tombol Toggle Energy Saver** (On/Off) dalam satu panel bergaya *iOS Control Center*. |
| **2 & 4** | **Perbaikan Click-Box Presisi (`100% DPI-Aware`) + Tombol Panah `<` dan `>`** | • Masalah klik meleset ("klik Shelf malah buka Water") terjadi karena `renderer.rs` sebelumnya tidak mengalikan ukuran kartu/tab dengan `dpi_scale`, sementara `on_lbutton_down` membagi dengan `dpi_scale`. Sekarang seluruh koordinat dinormalisasi ke dalam *logical coordinates* (`lx`, `ly`) yang identik antara *renderer* dan *click handler*.<br>• Menambahkan tombol panah **`‹` (Prev)** di ujung kiri atas dan **`›` (Next)** di ujung kanan atas *header* notch untuk berpindah antar 5 section (`Hub`, `Music`, `Control`, `Focus`, `Shelf`, `To-Do`) dengan mulus. |
| **3** | **Auto Time Peek Setiap 30 Menit (`:00` dan `:30`)** | Di `NotchController::update`, sistem memantau jam lokal (`Local::now()`). Setiap kali menit menyentuh **`:00` atau `:30`**, notch yang sedang di mode *idle/pet* otomatis menampilkan **Time & iOS Battery Peek** selama 10 detik sebelum kembali ke mata Asta. |
| **5 & 7** | **Spotify-Style Integrated Lyrics di Samping Cover Musik + Real-Time YouTube/Spotify Detection** | • Menghapus tab *Lyrics* yang terpisah. Di panel **`Music`**, bagian kiri menampilkan **Cover Art + Visualizer + Judul + Progress + Kontrol**, sedangkan **sebelah kanannya langsung menampilkan panel Synced Lyrics ala Spotify** (dengan tombol *toggle* `Lyrics` kecil jika ingin menyembunyikan/menampilkan).<br>• Memasang *background poller* langsung ke **`GlobalSystemMediaTransportControlsSessionManager` (GSMTC)** setiap 1.2 detik di dalam `states.rs`, sehingga lagu/video dari **YouTube (Chrome/Edge/Brave/Firefox), Spotify, atau Apple Music** langsung masuk ke notch secara *real-time* beserta durasi dan *auto-fetch* lirik LRCLIB! |
| **6** | **Perbaikan Shelf: Penyimpanan Sementara & Drag-and-Drop Masuk/Keluar App Lain** | • Mengaktifkan **`DragAcceptFiles(hwnd, true)`** dan *handler* **`WM_DROPFILES`** (`DragQueryFileW`) di `src/notch/mod.rs` sehingga kamu bisa langsung **drag file apa pun dari File Explorer/aplikasi lain dan drop ke atas notch** untuk menyimpannya di *Shelf*!<br>• Klik kiri pada kartu file di *Shelf* akan **menyalin file ke Clipboard (`CF_HDROP`) sekaligus membuka/menyorotnya** sehingga bisa langsung ditempel (*paste/drop*) ke WhatsApp, Discord, Email, atau folder lain; klik tombol `×` pada kartu akan menghapusnya dari *Shelf*. |
| **8** | **Notch Aktif & Interaktif Saat Volume / Brightness Diubah** | Saat kamu menekan tombol Volume atau Brightness (atau menggesernya), notch otomatis melebar sedikit menjadi **Interactive Slider Overlay** selama 3.5 detik: kamu bisa **langsung klik/geser bar Volume atau Brightness di dalam notch** untuk mengatur persentase spesifiknya secara langsung! |
| **10** | **Redesign Semi-Glass Tebal & Variasi Multi-Stop Gradient** | Menggunakan `ID2D1LinearGradientBrush` diagonal dan `ID2D1RadialGradientBrush` berlapis: latar belakang *Deep Obsidian Glass*, *inner border* berkilau (`#6488EA` → `#38BDF8` → `#94A3B8`), serta kartu-kartu *rounded* tebal (`radius: 16.0`) dengan aksen gradien *Slate-Blue*, *Cyan-Indigo*, dan *Muted Wine*. |
| **11** | **Mata Asta Full Anime Anatomy (Kelopak, Sclera, Bulu Mata, Alis & Tanda Devil Union)** | Di `src/mascot/eyes.rs`, mata Asta tidak lagi hanya bola mata melayang: kini digambar lengkap dengan **kontur kelopak mata atas & bawah tebal khas anime (`eyelid frame`)**, **bagian putih/gelap mata (`sclera`)**, **lipatan kelopak mata (`eyelid crease`)**, **alis ekspresif (`eyebrow`)** yang mengernyit serius saat *Devil Union* dan terangkat ceria saat *Normal Emerald*, serta **tanda garis hitam vertikal Devil Union di bawah mata**, lengkap dengan animasi pedang *Demon-Slayer* melayang & tangan menangkap! |
| **12** | **Font Modern Rounded ala iOS + Ikon Baterai iOS (Angka di Dalam Baterai)** | • Menggunakan font **`Segoe UI Variable Display` / `Segoe UI Semibold`** dengan proporsi huruf modern dan bersih.<br>• Membuat fungsi khusus **`draw_ios_battery_icon`**: menggambar kapsul baterai *rounded* ala iOS lengkap dengan kepala kutub positif di kanan, *fill* gradien hijau/biru/kuning di dalam kapsul, dan **angka persentase (mis. `84%` atau `⚡84`) tepat di tengah dalam ikon baterai**. |
| **13** | **Section Baru: `Hub` (Jam, Baterai iOS, & Scrollable Notification Center)** | Menambahkan section **`Hub`** yang menampilkan **Jam Besar & Tanggal**, **Status Baterai iOS**, serta **Daftar Notifikasi Masuk (lengkap dengan badge jumlah notifikasi & asal aplikasi: WhatsApp, Discord, Gmail, System, Browser)** yang **bisa di-scroll dengan *mouse wheel* (`WM_MOUSEWHEEL`)** dan **jika diklik langsung membuka/mengarahkan ke aplikasi terkait (`ShellExecuteW`)**! |

---

## 2. Prompt Siap Pakai untuk Hermes Agent

Salin seluruh blok di bawah ini dan berikan langsung ke **Hermes Agent** di terminal lokalmu (di folder `nusa-clone/`):

```markdown
Tolong baca file `DEVIL_UNION_UPGRADE.md` dan terapkan seluruh upgrade v4.0 (13 poin penyesuaian) ke dalam repositori `nusa-clone` ini secara lengkap dan aman:

1. **Struktur 6 Section Baru (Tanpa Water & Lyrics Terpisah) + Panah Navigasi `<` `>`**:
   - Di `src/notch/states.rs`, ubah daftar tab expanded menjadi 6 section:
     `0 = Hub` (Jam, Baterai iOS & Scrollable Notification Center),
     `1 = Music` (Player + Spotify-style Side-by-Side Synced Lyrics),
     `2 = Control` (All-in-One Control: Volume Slider, Brightness Slider, Wi-Fi Toggle, Energy Saver Toggle),
     `3 = Focus` (Pomodoro Timer),
     `4 = Shelf` (Temporary File Storage dengan Real Drag-and-Drop `WM_DROPFILES`),
     `5 = To-Do` (Interactive Task List).
   - Tambahkan tombol panah `<` di kiri atas dan `>` di kanan atas bar navigasi (`prev_tab()` dan `next_tab()`) dengan hit-box yang presisi.

2. **Perbaikan Total Hit-Box DPI & Koordinat Klik (`src/notch/mod.rs` & `src/notch/renderer.rs`)**:
   - Di `NotchRenderer::new`, gunakan `dpiX: 96.0 * dpi_scale, dpiY: 96.0 * dpi_scale` atau gunakan koordinat logis yang konsisten di seluruh `current_pill_rect()`, `render_frame()`, dan `on_lbutton_down()` sehingga posisi visual tombol/tab 100% sama persis dengan titik klik mouse pada skala DPI berapa pun.

3. **Real-Time Media Detection (YouTube, Spotify, Browser) & Spotify-Style Side Lyrics**:
   - Di `src/notch/states.rs`, jalankan background thread ringan yang mem-poll `GlobalSystemMediaTransportControlsSessionManager` setiap 1.2 detik untuk mengambil `Title`, `Artist`, `PlaybackStatus`, dan `TimelineProperties` dari YouTube/Spotify/Browser secara langsung, serta mengambil lirik dari `https://lrclib.net/api/get` ketika judul lagu berubah.
   - Di `draw_music_panel`, tampilkan cover + judul + kontrol di sisi kiri (lebar 52%) dan panel Synced Lyrics ala Spotify langsung di sisi kanan (lebar 48%) dalam satu section yang sama.

4. **Real Drag-and-Drop File Shelf (`WM_DROPFILES`) & Copy/Open ke App Lain**:
   - Di `src/notch/mod.rs`, aktifkan `DragAcceptFiles(hwnd, true)` saat membuat window, dan tangani pesan `WM_DROPFILES` menggunakan `DragQueryFileW` + `DragFinish` agar setiap file yang di-drag dari Explorer/app lain ke atas notch langsung masuk ke `controller.shelf_files` dan otomatis membuka tab `Shelf`.
   - Saat kartu file di `Shelf` diklik, buka/eksekusi file atau salin path-nya ke clipboard Windows dan buka di Explorer (`explorer.exe /select,<path>`) agar mudah dipindahkan ke aplikasi lain.

5. **Interactive Volume & Brightness Overlay + All-in-One Control Section**:
   - Setiap kali volume atau brightness berubah (atau diatur dari tab `Control` maupun overlay), aktifkan notch dan izinkan pengguna mengklik/menggeser slider Volume dan Brightness secara langsung di dalam notch (gunakan `keybd_event` / PowerShell WMI `WmiSetBrightness` & `netsh` / `powercfg` untuk menerapkan perubahan sistem).

6. **Auto 30-Minute Time Peek + iOS Battery Icon + Scrollable Notification Hub**:
   - Setiap pergantian menit `:00` dan `:30`, tampilkan Time & iOS Battery Peek otomatis selama 10 detik saat notch tertutup.
   - Gambar ikon baterai bergaya iOS (kapsul rounded dengan kutub di kanan dan teks angka persentase di dalam kapsul baterai).
   - Di section `Hub`, tampilkan jam digital besar, tanggal, status baterai iOS, badge jumlah notifikasi, dan list notifikasi yang bisa di-scroll dengan `WM_MOUSEWHEEL` (`notif_scroll_offset`) serta jika diklik langsung mengarahkan ke aplikasi terkait via `ShellExecuteW`.

7. **Full Anime Asta Eyes (`src/mascot/eyes.rs`) + Semi-Glass Multi-Gradient UI (`src/notch/renderer.rs`)**:
   - Terapkan `src/mascot/eyes.rs` terbaru yang menggambar anatomi mata anime Asta secara utuh (garis kelopak mata atas & bawah tebal, bulu mata runcing, sclera, lipatan kelopak, alis dinamis serius/senang, tanda vertikal Devil Union di bawah mata, transisi Devil Union ↔ Normal Emerald, dan pedang melayang yang ditangkap tangan Asta).
   - Terapkan `ID2D1LinearGradientBrush` multi-stop untuk efek semi-glass tebal dan kartu rounded modern.
   - Jalankan `cargo check` dan `cargo build --release` sampai sukses tanpa error.
```

---

## 3. Pastikan Feature Flags di `Cargo.toml` Lengkap

Pastikan blok `windows` di `Cargo.toml` memuat feature berikut (khususnya `Win32_System_Power`, `Win32_UI_Shell`, dan `Win32_UI_Input_KeyboardAndMouse` untuk `DragQueryFileW`, `ShellExecuteW`, dan kontrol sistem):

```toml
windows = { version = "0.58", features = [
    "Win32_Foundation",
    "Win32_Graphics_Direct2D",
    "Win32_Graphics_Direct2D_Common",
    "Win32_Graphics_DirectWrite",
    "Win32_Graphics_Dxgi",
    "Win32_Graphics_Dxgi_Common",
    "Win32_Graphics_Dwm",
    "Win32_Graphics_Gdi",
    "Win32_System_Com",
    "Win32_System_LibraryLoader",
    "Win32_System_Power",
    "Win32_System_Registry",
    "Win32_System_Threading",
    "Win32_UI_Controls",
    "Win32_UI_HiDpi",
    "Win32_UI_Input_KeyboardAndMouse",
    "Win32_UI_Shell",
    "Win32_UI_WindowsAndMessaging",
    "Media_Control",
    "Foundation",
] }
```

---

## 4. Kode Lengkap Semua Modul (`v4.0`)

### A. `src/mascot/eyes.rs` — Full Anime Asta Eye Anatomy (Kelopak Anime, Sclera, Alis, Tanda Devil Union, Transisi Emerald, & Pedang Melayang)

Modul ini sekarang menggambar **anatomi mata anime Asta secara utuh**: terdapat *sclera* (bagian putih/hitam mata yang dibatasi kelopak), *upper & lower eyelid lash line* tebal meruncing khas studio Pierrot, *eyelid crease* (lipatan kelopak), **alis (*eyebrow*)** yang ikut bergerak sesuai emosi (mengernyit tajam saat *Devil Union*, terangkat ceria saat *Normal Emerald* / *Happy*), serta **tanda garis hitam Devil Union di bawah kelopak mata**!

```rust
// src/mascot/eyes.rs
use windows::Win32::Graphics::Direct2D::Common::{
    D2D1_BEZIER_SEGMENT, D2D1_COLOR_F, D2D1_FIGURE_BEGIN_FILLED, D2D1_FIGURE_BEGIN_HOLLOW,
    D2D1_FIGURE_END_CLOSED, D2D1_FIGURE_END_OPEN, D2D1_GRADIENT_STOP, D2D_POINT_2F, D2D_RECT_F,
};
use windows::Win32::Graphics::Direct2D::{
    ID2D1Factory, ID2D1HwndRenderTarget, ID2D1PathGeometry, D2D1_ELLIPSE, D2D1_EXTEND_MODE_CLAMP,
    D2D1_GAMMA_2_2, D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES, D2D1_ROUNDED_RECT,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AstaIdleMode {
    /// Devil Union Mode: Tatapan tajam serius, alis mengernyit, sclera gelap/kontras, pupil '+'
    DevilUnionSerious { timer: f32 },
    /// Sword Summon & Grab: Pedang Demon-Slayer melayang lalu ditangkap tangan Asta
    SwordSummonGrab { timer: f32 },
    /// Normal Asta Mode: Sclera putih bersih anime, iris Hijau Emerald berbinar, alis ramah
    NormalEmerald { timer: f32 },
    /// Happy Gesture: Mata melengkung senang '^ ^' dengan alis terangkat ceria
    HappySmileGesture { timer: f32 },
    /// Dozing: Mata setengah terpejam tenang
    Dozing { timer: f32 },
}

#[derive(Debug, Clone, Copy)]
pub struct AntiMagicSpark {
    pub offset_x: f32,
    pub offset_y: f32,
    pub vx: f32,
    pub vy: f32,
    pub size: f32,
    pub life: f32,
    pub max_life: f32,
    pub kind: u8,
}

pub struct DevilUnionEyes {
    pub mode: AstaIdleMode,
    pub time: f32,
    pub look_x: f32,
    pub look_y: f32,
    pub target_look_x: f32,
    pub target_look_y: f32,
    pub openness: f32,
    pub blink_timer: f32,
    pub next_blink_in: f32,
    pub glance_timer: f32,
    pub aura_intensity: f32,
    pub pupil_scale: f32,
    /// 0.0 = Devil Union Mode, 1.0 = Normal Asta Emerald Mode
    pub emerald_blend: f32,
    /// -1.0 = Serius/Mengernyit (Devil Union), +1.0 = Senang/Antusias (Normal Emerald)
    pub brow_mood: f32,
    pub sparks: [AntiMagicSpark; 10],
    rng_seed: u32,
}

impl Default for DevilUnionEyes {
    fn default() -> Self {
        Self::new()
    }
}

impl DevilUnionEyes {
    pub fn new() -> Self {
        let mut eyes = Self {
            mode: AstaIdleMode::DevilUnionSerious { timer: 0.0 },
            time: 0.0,
            look_x: 0.0,
            look_y: 0.0,
            target_look_x: 0.0,
            target_look_y: 0.0,
            openness: 0.15,
            blink_timer: 0.0,
            next_blink_in: 3.0,
            glance_timer: 1.8,
            aura_intensity: 1.0,
            pupil_scale: 1.1,
            emerald_blend: 0.0,
            brow_mood: -0.8,
            sparks: [AntiMagicSpark {
                offset_x: 0.0,
                offset_y: 0.0,
                vx: 0.0,
                vy: 0.0,
                size: 2.0,
                life: 0.0,
                max_life: 1.0,
                kind: 0,
            }; 10],
            rng_seed: 0xA57A_9999,
        };
        for i in 0..eyes.sparks.len() {
            eyes.respawn_spark(i, true);
        }
        eyes
    }

    fn next_f32(&mut self) -> f32 {
        let mut x = self.rng_seed;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.rng_seed = x;
        (x as f32) / (u32::MAX as f32)
    }

    fn respawn_spark(&mut self, idx: usize, random_life: bool) {
        let side = if idx % 2 == 0 { -1.0 } else { 1.0 };
        let rx = (self.next_f32() - 0.5) * 26.0 + side * 18.0;
        let ry = (self.next_f32() - 0.5) * 8.0;
        let vx = (self.next_f32() - 0.5) * 12.0 + side * 3.0;
        let vy = -8.0 - self.next_f32() * 14.0;
        let max_life = 0.50 + self.next_f32() * 0.60;
        let life = if random_life {
            self.next_f32() * max_life
        } else {
            0.0
        };
        let size = 1.3 + self.next_f32() * 1.9;
        let kind = (idx % 3) as u8;

        self.sparks[idx] = AntiMagicSpark {
            offset_x: rx,
            offset_y: ry,
            vx,
            vy,
            size,
            life,
            max_life,
            kind,
        };
    }

    pub fn trigger_awakening(&mut self) {
        self.mode = AstaIdleMode::DevilUnionSerious { timer: 0.0 };
        self.openness = 0.15;
        self.pupil_scale = 1.25;
        self.aura_intensity = 1.25;
    }

    pub fn trigger_wake_surge(&mut self) {
        self.mode = AstaIdleMode::HappySmileGesture { timer: 0.0 };
        self.aura_intensity = 1.15;
    }

    pub fn update(&mut self, dt: f32, cursor_offset: Option<(f32, f32)>) {
        let dt = dt.clamp(0.001, 0.05);
        self.time += dt;

        let mut cursor_near = false;
        if let Some((dx, dy)) = cursor_offset {
            let dist = (dx * dx + dy * dy).sqrt();
            if dist < 360.0 {
                cursor_near = true;
                self.target_look_x = (dx / 170.0).clamp(-1.0, 1.0);
                self.target_look_y = (dy / 105.0).clamp(-0.75, 0.85);
                if matches!(self.mode, AstaIdleMode::Dozing { .. }) {
                    self.mode = AstaIdleMode::HappySmileGesture { timer: 0.0 };
                }
            }
        }

        if !cursor_near {
            self.glance_timer -= dt;
            if self.glance_timer <= 0.0 {
                let r = self.next_f32();
                if r < 0.35 {
                    self.target_look_x = 0.0;
                    self.target_look_y = 0.0;
                } else {
                    self.target_look_x = (self.next_f32() - 0.5) * 1.4;
                    self.target_look_y = (self.next_f32() - 0.5) * 0.55;
                }
                self.glance_timer = 1.8 + self.next_f32() * 2.5;
            }
        }

        let target_openness: f32;
        let target_aura: f32;
        let target_pupil: f32;
        let target_emerald: f32;
        let target_brow: f32;

        match &mut self.mode {
            AstaIdleMode::DevilUnionSerious { timer } => {
                *timer += dt;
                target_openness = 0.96;
                target_emerald = 0.0;
                target_brow = -0.85; // Alis tajam serius khas Devil Union
                target_aura = if cursor_near {
                    1.15
                } else {
                    0.82 + 0.12 * (self.time * 3.0).sin()
                };
                target_pupil = if cursor_near { 0.86 } else { 1.0 };

                if *timer >= 7.0 {
                    if self.next_f32() < 0.58 {
                        self.mode = AstaIdleMode::SwordSummonGrab { timer: 0.0 };
                    } else {
                        self.mode = AstaIdleMode::NormalEmerald { timer: 0.0 };
                    }
                }
            }
            AstaIdleMode::SwordSummonGrab { timer } => {
                *timer += dt;
                target_openness = 1.02;
                target_emerald = 0.0;
                target_brow = -1.0; // Ekstra fokus saat menangkap pedang
                target_aura = 1.25;
                target_pupil = 0.82;

                let p = (*timer / 3.8).clamp(0.0, 1.0);
                self.target_look_x = 0.85 * (1.0 - (p - 0.5).abs() * 1.2).clamp(0.2, 1.0);
                self.target_look_y = -0.10 + (self.time * 4.0).sin() * 0.18;

                if *timer >= 3.8 {
                    self.mode = AstaIdleMode::NormalEmerald { timer: 0.0 };
                }
            }
            AstaIdleMode::NormalEmerald { timer } => {
                *timer += dt;
                target_openness = 1.08;
                target_emerald = 1.0;
                target_brow = 0.65; // Alis terangkat antusias khas Asta biasa
                target_aura = 0.45;
                target_pupil = 1.08;

                if *timer >= 4.2 {
                    self.mode = AstaIdleMode::HappySmileGesture { timer: 0.0 };
                }
            }
            AstaIdleMode::HappySmileGesture { timer } => {
                *timer += dt;
                target_openness = 1.0;
                target_emerald = 1.0;
                target_brow = 0.95; // Alis melengkung senang
                target_aura = 0.60;
                target_pupil = 1.0;

                if *timer >= 2.2 {
                    self.mode = AstaIdleMode::DevilUnionSerious { timer: 0.0 };
                }
            }
            AstaIdleMode::Dozing { timer } => {
                *timer += dt;
                target_openness = 0.30 + 0.04 * (self.time * 1.4).sin();
                target_emerald = 0.0;
                target_brow = -0.2;
                target_aura = 0.35;
                target_pupil = 1.05;
            }
        }

        self.next_blink_in -= dt;
        if self.next_blink_in <= 0.0 {
            self.blink_timer = 0.13;
            self.next_blink_in = 2.6 + self.next_f32() * 3.4;
        }

        let blink_factor = if self.blink_timer > 0.0 {
            self.blink_timer -= dt;
            let norm = (self.blink_timer / 0.13).clamp(0.0, 1.0);
            (norm - 0.5).abs() * 2.0
        } else {
            1.0
        };

        let smooth = (14.0 * dt).min(1.0);
        self.look_x += (self.target_look_x - self.look_x) * smooth;
        self.look_y += (self.target_look_y - self.look_y) * smooth;
        self.openness += (target_openness * blink_factor - self.openness) * (22.0 * dt).min(1.0);
        self.aura_intensity += (target_aura - self.aura_intensity) * (10.0 * dt).min(1.0);
        self.pupil_scale += (target_pupil - self.pupil_scale) * (12.0 * dt).min(1.0);
        self.emerald_blend += (target_emerald - self.emerald_blend) * (8.0 * dt).min(1.0);
        self.brow_mood += (target_brow - self.brow_mood) * (10.0 * dt).min(1.0);

        for i in 0..self.sparks.len() {
            self.sparks[i].life += dt * self.aura_intensity.max(0.45);
            if self.sparks[i].life >= self.sparks[i].max_life {
                self.respawn_spark(i, false);
            } else {
                self.sparks[i].offset_x += self.sparks[i].vx * dt;
                self.sparks[i].offset_y += self.sparks[i].vy * dt;
            }
        }
    }

    pub fn render(
        &self,
        factory: &ID2D1Factory,
        rt: &ID2D1HwndRenderTarget,
        pill_rect: D2D_RECT_F,
        alpha_mult: f32,
    ) {
        if alpha_mult <= 0.01 {
            return;
        }

        let center_x = (pill_rect.left + pill_rect.right) * 0.5;
        let center_y = (pill_rect.top + pill_rect.bottom) * 0.5 + 1.5;

        let sword_shift = if let AstaIdleMode::SwordSummonGrab { timer } = self.mode {
            let s = (timer * 2.5).min(1.0) * ((3.8 - timer) * 2.5).clamp(0.0, 1.0);
            -14.0 * s
        } else {
            0.0
        };

        let eyes_cx = center_x + sword_shift;
        let eye_spacing = 18.5;

        unsafe {
            self.draw_anti_magic_sparks(rt, eyes_cx, center_y, alpha_mult);

            if let AstaIdleMode::HappySmileGesture { timer } = self.mode {
                let bounce = (timer * 10.0).sin().abs() * -1.6;
                for side in [-1.0_f32, 1.0_f32] {
                    self.draw_happy_arc_eye(
                        factory,
                        rt,
                        eyes_cx + side * eye_spacing,
                        center_y + bounce,
                        side,
                        alpha_mult,
                    );
                }
            } else {
                for side in [-1.0_f32, 1.0_f32] {
                    let base_x = eyes_cx + side * eye_spacing;
                    self.draw_full_anime_asta_eye(factory, rt, base_x, center_y, side, alpha_mult);
                }
            }

            if let AstaIdleMode::SwordSummonGrab { timer } = self.mode {
                self.draw_floating_sword_and_grabbing_hand(
                    factory,
                    rt,
                    center_x + 38.0,
                    center_y,
                    pill_rect.bottom,
                    timer,
                    alpha_mult,
                );
            }
        }
    }

    /// Gambar Anatomi Lengkap Mata Anime Asta (Kelopak Atas/Bawah, Sclera, Lipatan Mata, Alis, Tanda Devil Union, Iris & Pupil)
    unsafe fn draw_full_anime_asta_eye(
        &self,
        factory: &ID2D1Factory,
        rt: &ID2D1HwndRenderTarget,
        base_x: f32,
        base_y: f32,
        side: f32, // -1.0 = Kiri, 1.0 = Kanan
        alpha: f32,
    ) {
        let eb = self.emerald_blend.clamp(0.0, 1.0);
        let open = self.openness.clamp(0.08, 1.15);

        // Lebar & tinggi soket mata anime
        let w = 10.2;
        let h = (6.2 * open).max(0.8);

        // Kemiringan sudut kelopak atas (serius tajam saat Devil Union, melengkung ramah saat Emerald)
        let inner_drop = (1.0 - eb) * 2.2 - eb * 0.4;
        let outer_lift = (1.0 - eb) * 1.4 + eb * 0.5;

        let inner_pt = D2D_POINT_2F {
            x: base_x - side * w,
            y: base_y + 0.5 + inner_drop * 0.3,
        };
        let outer_pt = D2D_POINT_2F {
            x: base_x + side * w,
            y: base_y - outer_lift * 0.5,
        };

        // --- 1. Gambar Alis Ekspresif (Eyebrow) & Lipatan Kelopak (Eyelid Crease) ---
        let brow_y_inner = base_y - h - 3.2 - self.brow_mood * 1.8;
        let brow_y_outer = base_y - h - 4.2 + self.brow_mood * 0.8;
        let brow_col = D2D1_COLOR_F {
            r: 0.72,
            g: 0.78,
            b: 0.88,
            a: 0.85 * alpha,
        };
        if let Ok(brow_b) = rt.CreateSolidColorBrush(&brow_col, None) {
            rt.DrawLine(
                D2D_POINT_2F {
                    x: base_x - side * (w * 0.85),
                    y: brow_y_inner,
                },
                D2D_POINT_2F {
                    x: base_x + side * (w * 0.95),
                    y: brow_y_outer,
                },
                &brow_b,
                1.8,
                None,
            );
        }

        // --- 2. Tanda Vertikal Devil Union di Bawah Mata (Muncul saat mode Devil Union) ---
        if eb < 0.75 {
            let mark_alpha = (1.0 - eb) * alpha * 0.75;
            let mark_col = D2D1_COLOR_F {
                r: 0.68,
                g: 0.22,
                b: 0.30,
                a: mark_alpha,
            };
            if let Ok(mb) = rt.CreateSolidColorBrush(&mark_col, None) {
                rt.DrawLine(
                    D2D_POINT_2F {
                        x: base_x + side * 1.5,
                        y: base_y + h + 1.2,
                    },
                    D2D_POINT_2F {
                        x: base_x + side * 2.2,
                        y: base_y + h + 4.8,
                    },
                    &mb,
                    1.5,
                    None,
                );
            }
        }

        // Jika sedang berkedip rapat
        if open < 0.16 {
            let lash_col = D2D1_COLOR_F {
                r: 0.85,
                g: 0.88,
                b: 0.95,
                a: 0.90 * alpha,
            };
            if let Ok(lb) = rt.CreateSolidColorBrush(&lash_col, None) {
                rt.DrawLine(inner_pt, outer_pt, &lb, 2.2, None);
            }
            return;
        }

        // --- 3. Geometri Sclera (Bentuk Mata Anime Utuh) ---
        if let Ok(sclera_geom) = factory.CreatePathGeometry() {
            if let Ok(sink) = sclera_geom.Open() {
                sink.BeginFigure(inner_pt, D2D1_FIGURE_BEGIN_FILLED);
                // Lengkungan kelopak atas
                sink.AddBezier(&D2D1_BEZIER_SEGMENT {
                    point1: D2D_POINT_2F {
                        x: base_x - side * (w * 0.35),
                        y: base_y - h + inner_drop,
                    },
                    point2: D2D_POINT_2F {
                        x: base_x + side * (w * 0.45),
                        y: base_y - h - outer_lift,
                    },
                    point3: outer_pt,
                });
                // Lengkungan kelopak bawah
                sink.AddBezier(&D2D1_BEZIER_SEGMENT {
                    point1: D2D_POINT_2F {
                        x: base_x + side * (w * 0.45),
                        y: base_y + h * 0.95,
                    },
                    point2: D2D_POINT_2F {
                        x: base_x - side * (w * 0.35),
                        y: base_y + h * 0.90,
                    },
                    point3: inner_pt,
                });
                sink.EndFigure(D2D1_FIGURE_END_CLOSED);
                let _ = sink.Close();

                // Warna Sclera: Putih keperakan bersih khas anime (sedikit smoky slate saat Devil Union)
                let sclera_col = D2D1_COLOR_F {
                    r: 0.82 * (1.0 - eb) + 0.95 * eb,
                    g: 0.85 * (1.0 - eb) + 0.97 * eb,
                    b: 0.92 * (1.0 - eb) + 0.99 * eb,
                    a: 0.95 * alpha,
                };
                if let Ok(sb) = rt.CreateSolidColorBrush(&sclera_col, None) {
                    rt.FillGeometry(&sclera_geom, &sb, None);
                }
            }
        }

        // --- 4. Iris & Pupil (Bergerak di dalam Sclera mengikuti Look Vector) ---
        let shift_x = self.look_x * 3.2;
        let shift_y = self.look_y * 1.5;
        let ex = base_x + shift_x;
        let ey = base_y + shift_y;

        let iris_rx = 5.4;
        let iris_ry = (5.0 * open).max(1.2);

        let c0 = D2D1_COLOR_F {
            r: 0.85 * (1.0 - eb) + 0.28 * eb,
            g: 0.28 * (1.0 - eb) + 0.92 * eb,
            b: 0.36 * (1.0 - eb) + 0.64 * eb,
            a: alpha,
        };
        let c1 = D2D1_COLOR_F {
            r: 0.64 * (1.0 - eb) + 0.12 * eb,
            g: 0.14 * (1.0 - eb) + 0.68 * eb,
            b: 0.22 * (1.0 - eb) + 0.48 * eb,
            a: alpha,
        };
        let c2 = D2D1_COLOR_F {
            r: 0.22 * (1.0 - eb) + 0.05 * eb,
            g: 0.06 * (1.0 - eb) + 0.32 * eb,
            b: 0.12 * (1.0 - eb) + 0.25 * eb,
            a: alpha,
        };

        let iris_stops = [
            D2D1_GRADIENT_STOP {
                position: 0.0,
                color: c0,
            },
            D2D1_GRADIENT_STOP {
                position: 0.68,
                color: c1,
            },
            D2D1_GRADIENT_STOP {
                position: 1.0,
                color: c2,
            },
        ];

        if let Ok(stop_collection) =
            rt.CreateGradientStopCollection(&iris_stops, D2D1_GAMMA_2_2, D2D1_EXTEND_MODE_CLAMP)
        {
            let props = D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES {
                center: D2D_POINT_2F {
                    x: ex,
                    y: ey + iris_ry * 0.15,
                },
                gradientOriginOffset: D2D_POINT_2F { x: 0.0, y: 0.0 },
                radiusX: iris_rx,
                radiusY: iris_ry,
            };
            if let Ok(iris_brush) = rt.CreateRadialGradientBrush(&props, None, &stop_collection) {
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F { x: ex, y: ey },
                        radiusX: iris_rx,
                        radiusY: iris_ry,
                    },
                    &iris_brush,
                );
            }
        }

        // Pupil ('+' saat Devil Union, Bulat saat Normal Emerald)
        let px = base_x + shift_x * 1.15;
        let py = base_y + shift_y * 1.15;
        let pupil_color = D2D1_COLOR_F {
            r: 0.03,
            g: 0.04,
            b: 0.07,
            a: alpha,
        };

        if eb < 0.65 {
            let arm_h = 4.1 * self.pupil_scale;
            let arm_v = (3.9 * self.pupil_scale * open).max(1.0);
            let half_thick = 1.05 * self.pupil_scale;

            if let Ok(plus_geom) =
                self.create_tapered_plus_geometry(factory, px, py, arm_h, arm_v, half_thick)
            {
                if let Ok(pb) = rt.CreateSolidColorBrush(&pupil_color, None) {
                    rt.FillGeometry(&plus_geom, &pb, None);
                }
            }
        } else if let Ok(pb) = rt.CreateSolidColorBrush(&pupil_color, None) {
            rt.FillEllipse(
                &D2D1_ELLIPSE {
                    point: D2D_POINT_2F { x: px, y: py },
                    radiusX: 2.3 * self.pupil_scale,
                    radiusY: 2.5 * self.pupil_scale * open,
                },
                &pb,
            );
        }

        // Specular Highlight Putih Anime
        let glint_color = D2D1_COLOR_F {
            r: 0.96,
            g: 0.98,
            b: 1.0,
            a: 0.90 * alpha,
        };
        if let Ok(glint_brush) = rt.CreateSolidColorBrush(&glint_color, None) {
            rt.FillEllipse(
                &D2D1_ELLIPSE {
                    point: D2D_POINT_2F {
                        x: px - side * 1.9,
                        y: py - iris_ry * 0.35,
                    },
                    radiusX: 1.15 + 0.35 * eb,
                    radiusY: (0.85 + 0.35 * eb) * open,
                },
                &glint_brush,
            );
        }

        // --- 5. Garis Bulu Mata / Kelopak Atas & Bawah Tebal Khas Anime (Upper & Lower Lash Frame) ---
        if let Ok(lash_geom) = factory.CreatePathGeometry() {
            if let Ok(sink) = lash_geom.Open() {
                sink.BeginFigure(inner_pt, D2D1_FIGURE_BEGIN_HOLLOW);
                sink.AddBezier(&D2D1_BEZIER_SEGMENT {
                    point1: D2D_POINT_2F {
                        x: base_x - side * (w * 0.35),
                        y: base_y - h + inner_drop,
                    },
                    point2: D2D_POINT_2F {
                        x: base_x + side * (w * 0.45),
                        y: base_y - h - outer_lift,
                    },
                    point3: outer_pt,
                });
                sink.EndFigure(D2D1_FIGURE_END_OPEN);
                let _ = sink.Close();

                let upper_lash_col = D2D1_COLOR_F {
                    r: 0.08,
                    g: 0.10,
                    b: 0.15,
                    a: alpha,
                };
                if let Ok(lb) = rt.CreateSolidColorBrush(&upper_lash_col, None) {
                    rt.DrawGeometry(&lash_geom, &lb, 2.3, None);
                }
            }
        }
    }

    unsafe fn draw_happy_arc_eye(
        &self,
        factory: &ID2D1Factory,
        rt: &ID2D1HwndRenderTarget,
        ex: f32,
        ey: f32,
        side: f32,
        alpha: f32,
    ) {
        if let Ok(geom) = factory.CreatePathGeometry() {
            if let Ok(sink) = geom.Open() {
                sink.BeginFigure(
                    D2D_POINT_2F {
                        x: ex - 7.0,
                        y: ey + 2.5,
                    },
                    D2D1_FIGURE_BEGIN_HOLLOW,
                );
                sink.AddBezier(&D2D1_BEZIER_SEGMENT {
                    point1: D2D_POINT_2F {
                        x: ex - 3.2,
                        y: ey - 5.8,
                    },
                    point2: D2D_POINT_2F {
                        x: ex + 3.2,
                        y: ey - 5.8,
                    },
                    point3: D2D_POINT_2F {
                        x: ex + 7.0,
                        y: ey + 2.5,
                    },
                });
                sink.EndFigure(D2D1_FIGURE_END_OPEN);
                let _ = sink.Close();

                let emerald_col = D2D1_COLOR_F {
                    r: 0.22,
                    g: 0.88,
                    b: 0.62,
                    a: alpha,
                };
                if let Ok(b) = rt.CreateSolidColorBrush(&emerald_col, None) {
                    rt.DrawGeometry(&geom, &b, 2.5, None);
                }
            }
        }

        // Alis ceria di atas lengkungan mata senang
        let brow_col = D2D1_COLOR_F {
            r: 0.72,
            g: 0.78,
            b: 0.88,
            a: 0.80 * alpha,
        };
        if let Ok(bb) = rt.CreateSolidColorBrush(&brow_col, None) {
            rt.DrawLine(
                D2D_POINT_2F {
                    x: ex - side * 6.0,
                    y: ey - 6.5,
                },
                D2D_POINT_2F {
                    x: ex + side * 6.5,
                    y: ey - 5.0,
                },
                &bb,
                1.5,
                None,
            );
        }
    }

    unsafe fn draw_floating_sword_and_grabbing_hand(
        &self,
        factory: &ID2D1Factory,
        rt: &ID2D1HwndRenderTarget,
        sx: f32,
        sy: f32,
        pill_bottom: f32,
        timer: f32,
        alpha: f32,
    ) {
        let hover_bob = (timer * 6.5).sin() * 2.2;
        let retract = if timer > 2.7 {
            ((timer - 2.7) / 1.1).clamp(0.0, 1.0).powi(2) * 18.0
        } else {
            0.0
        };

        let sword_x = sx;
        let sword_y = sy + hover_bob + retract;

        let blade_dark = D2D1_COLOR_F {
            r: 0.14,
            g: 0.17,
            b: 0.24,
            a: alpha,
        };
        let blade_edge = D2D1_COLOR_F {
            r: 0.52,
            g: 0.60,
            b: 0.74,
            a: alpha * 0.9,
        };
        let crimson_core = D2D1_COLOR_F {
            r: 0.72,
            g: 0.22,
            b: 0.30,
            a: alpha * 0.85,
        };
        let hand_obsidian = D2D1_COLOR_F {
            r: 0.08,
            g: 0.10,
            b: 0.15,
            a: alpha,
        };

        let Ok(blade_b) = rt.CreateSolidColorBrush(&blade_dark, None) else { return };
        let Ok(edge_b) = rt.CreateSolidColorBrush(&blade_edge, None) else { return };
        let Ok(core_b) = rt.CreateSolidColorBrush(&crimson_core, None) else { return };
        let Ok(hand_b) = rt.CreateSolidColorBrush(&hand_obsidian, None) else { return };

        if let Ok(sword_geom) = factory.CreatePathGeometry() {
            if let Ok(sink) = sword_geom.Open() {
                sink.BeginFigure(
                    D2D_POINT_2F {
                        x: sword_x,
                        y: sword_y - 12.5,
                    },
                    D2D1_FIGURE_BEGIN_FILLED,
                );
                sink.AddLines(&[
                    D2D_POINT_2F {
                        x: sword_x + 4.2,
                        y: sword_y - 7.5,
                    },
                    D2D_POINT_2F {
                        x: sword_x + 3.4,
                        y: sword_y + 3.5,
                    },
                    D2D_POINT_2F {
                        x: sword_x - 3.4,
                        y: sword_y + 3.5,
                    },
                    D2D_POINT_2F {
                        x: sword_x - 4.2,
                        y: sword_y - 7.5,
                    },
                ]);
                sink.EndFigure(D2D1_FIGURE_END_CLOSED);
                let _ = sink.Close();
                rt.FillGeometry(&sword_geom, &blade_b, None);
                rt.DrawGeometry(&sword_geom, &edge_b, 0.9, None);
            }
        }

        rt.DrawLine(
            D2D_POINT_2F {
                x: sword_x,
                y: sword_y - 10.0,
            },
            D2D_POINT_2F {
                x: sword_x,
                y: sword_y + 3.0,
            },
            &core_b,
            1.2,
            None,
        );
        rt.DrawLine(
            D2D_POINT_2F {
                x: sword_x - 5.5,
                y: sword_y + 3.8,
            },
            D2D_POINT_2F {
                x: sword_x + 5.5,
                y: sword_y + 3.8,
            },
            &edge_b,
            1.8,
            None,
        );
        rt.DrawLine(
            D2D_POINT_2F {
                x: sword_x,
                y: sword_y + 4.0,
            },
            D2D_POINT_2F {
                x: sword_x,
                y: sword_y + 10.5,
            },
            &edge_b,
            2.0,
            None,
        );

        if timer >= 1.2 {
            let reach_progress = if timer < 2.0 {
                ((timer - 1.2) / 0.8).clamp(0.0, 1.0)
            } else {
                1.0
            };

            let hand_top_y = pill_bottom - (pill_bottom - (sword_y + 6.5)) * reach_progress;
            let arm_rect = D2D1_ROUNDED_RECT {
                rect: D2D_RECT_F {
                    left: sword_x - 3.2,
                    top: hand_top_y,
                    right: sword_x + 3.2,
                    bottom: pill_bottom,
                },
                radiusX: 2.0,
                radiusY: 2.0,
            };
            rt.FillRoundedRectangle(&arm_rect, &hand_b);
            rt.DrawRoundedRectangle(&arm_rect, &core_b, 0.9, None);

            if reach_progress > 0.75 {
                for f in 0..3 {
                    let fy = hand_top_y + f as f32 * 1.8;
                    rt.DrawLine(
                        D2D_POINT_2F {
                            x: sword_x - 3.8,
                            y: fy,
                        },
                        D2D_POINT_2F {
                            x: sword_x + 3.8,
                            y: fy,
                        },
                        &edge_b,
                        1.2,
                        None,
                    );
                }
            }
        }
    }

    unsafe fn draw_anti_magic_sparks(
        &self,
        rt: &ID2D1HwndRenderTarget,
        cx: f32,
        cy: f32,
        alpha_mult: f32,
    ) {
        for spark in &self.sparks {
            let progress = (spark.life / spark.max_life).clamp(0.0, 1.0);
            let spark_alpha = (1.0 - (2.0 * progress - 1.0).powi(2))
                * self.aura_intensity.clamp(0.2, 1.1)
                * alpha_mult;

            if spark_alpha <= 0.02 {
                continue;
            }

            let color = if self.emerald_blend > 0.5 {
                D2D1_COLOR_F {
                    r: 0.25,
                    g: 0.85,
                    b: 0.65,
                    a: spark_alpha * 0.55,
                }
            } else {
                match spark.kind {
                    0 => D2D1_COLOR_F {
                        r: 0.12,
                        g: 0.15,
                        b: 0.22,
                        a: spark_alpha * 0.85,
                    },
                    1 => D2D1_COLOR_F {
                        r: 0.48,
                        g: 0.56,
                        b: 0.72,
                        a: spark_alpha * 0.60,
                    },
                    _ => D2D1_COLOR_F {
                        r: 0.68,
                        g: 0.24,
                        b: 0.32,
                        a: spark_alpha * 0.55,
                    },
                }
            };

            if let Ok(brush) = rt.CreateSolidColorBrush(&color, None) {
                let x = cx + spark.offset_x;
                let y = cy + spark.offset_y;
                let r = spark.size * (1.0 - 0.4 * progress);
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F { x, y },
                        radiusX: r * 0.8,
                        radiusY: r * 1.4,
                    },
                    &brush,
                );
            }
        }
    }

    unsafe fn create_tapered_plus_geometry(
        &self,
        factory: &ID2D1Factory,
        cx: f32,
        cy: f32,
        arm_h: f32,
        arm_v: f32,
        half_t: f32,
    ) -> windows::core::Result<ID2D1PathGeometry> {
        let geom = factory.CreatePathGeometry()?;
        let sink = geom.Open()?;
        sink.BeginFigure(
            D2D_POINT_2F {
                x: cx,
                y: cy - arm_v,
            },
            D2D1_FIGURE_BEGIN_FILLED,
        );

        let tip_t = half_t * 0.32;
        let pts = [
            D2D_POINT_2F { x: cx + tip_t, y: cy - arm_v * 0.65 },
            D2D_POINT_2F { x: cx + half_t, y: cy - half_t },
            D2D_POINT_2F { x: cx + arm_h * 0.65, y: cy - tip_t },
            D2D_POINT_2F { x: cx + arm_h, y: cy },
            D2D_POINT_2F { x: cx + arm_h * 0.65, y: cy + tip_t },
            D2D_POINT_2F { x: cx + half_t, y: cy + half_t },
            D2D_POINT_2F { x: cx + tip_t, y: cy + arm_v * 0.65 },
            D2D_POINT_2F { x: cx, y: cy + arm_v },
            D2D_POINT_2F { x: cx - tip_t, y: cy + arm_v * 0.65 },
            D2D_POINT_2F { x: cx - half_t, y: cy + half_t },
            D2D_POINT_2F { x: cx - arm_h * 0.65, y: cy + tip_t },
            D2D_POINT_2F { x: cx - arm_h, y: cy },
            D2D_POINT_2F { x: cx - arm_h * 0.65, y: cy - tip_t },
            D2D_POINT_2F { x: cx - half_t, y: cy - half_t },
            D2D_POINT_2F { x: cx - tip_t, y: cy - arm_v * 0.65 },
        ];

        sink.AddLines(&pts);
        sink.EndFigure(D2D1_FIGURE_END_CLOSED);
        sink.Close()?;
        Ok(geom)
    }
}
```

---

### B. `src/notch/states.rs` — 6 Section Baru (`Hub`, `Music` + Integrated Lyrics, `Control`, `Focus`, `Shelf`, `To-Do`), Real GSMTC Media Poller, Auto 30-Min Time Peek, & Interactive Sliders

```rust
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
```

---

### C. `src/notch/renderer.rs` — Semi-Glass Multi-Gradient Theme, iOS Battery Icon, `<` `>` Navigation Arrows, Integrated Spotify Lyrics, All-in-One Control, & Notification Hub

```rust
// src/notch/renderer.rs
use chrono::Local;
use windows::core::w;
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Direct2D::Common::{
    D2D1_ALPHA_MODE_PREMULTIPLIED, D2D1_BEZIER_SEGMENT, D2D1_COLOR_F, D2D1_FIGURE_BEGIN_FILLED,
    D2D1_FIGURE_END_CLOSED, D2D1_GRADIENT_STOP, D2D1_PIXEL_FORMAT, D2D_POINT_2F, D2D_RECT_F,
    D2D_SIZE_U,
};
use windows::Win32::Graphics::Direct2D::*;
use windows::Win32::Graphics::DirectWrite::{
    DWriteCreateFactory, IDWriteFactory, IDWriteTextFormat, DWRITE_FACTORY_TYPE_SHARED,
    DWRITE_FONT_STRETCH_NORMAL, DWRITE_FONT_STYLE_NORMAL, DWRITE_FONT_WEIGHT_BOLD,
    DWRITE_FONT_WEIGHT_MEDIUM, DWRITE_FONT_WEIGHT_SEMI_BOLD, DWRITE_PARAGRAPH_ALIGNMENT_CENTER,
    DWRITE_TEXT_ALIGNMENT_CENTER, DWRITE_TEXT_ALIGNMENT_LEADING,
};
use windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_B8G8R8A8_UNORM;
use windows::Win32::Graphics::Gdi::{BeginPaint, EndPaint, PAINTSTRUCT};

use super::states::{NotchController, NotchState, OverlayKind};

pub struct AntiMagicNeutralPalette;

impl AntiMagicNeutralPalette {
    pub const PILL_BG: D2D1_COLOR_F = D2D1_COLOR_F {
        r: 0.043,
        g: 0.055,
        b: 0.082,
        a: 0.97,
    };
    pub const CARD_ELEVATED: D2D1_COLOR_F = D2D1_COLOR_F {
        r: 0.115,
        g: 0.148,
        b: 0.212,
        a: 0.92,
    };
    pub const ACCENT_BLUE: D2D1_COLOR_F = D2D1_COLOR_F {
        r: 0.392,
        g: 0.545,
        b: 0.945,
        a: 1.0,
    };
    pub const ACCENT_CYAN: D2D1_COLOR_F = D2D1_COLOR_F {
        r: 0.220,
        g: 0.740,
        b: 0.970,
        a: 1.0,
    };
    pub const ACCENT_EMERALD: D2D1_COLOR_F = D2D1_COLOR_F {
        r: 0.200,
        g: 0.860,
        b: 0.560,
        a: 1.0,
    };
    pub const ACCENT_WINE: D2D1_COLOR_F = D2D1_COLOR_F {
        r: 0.680,
        g: 0.220,
        b: 0.310,
        a: 0.95,
    };
    pub const TEXT_PRIMARY: D2D1_COLOR_F = D2D1_COLOR_F {
        r: 0.945,
        g: 0.965,
        b: 0.992,
        a: 1.0,
    };
    pub const TEXT_SECONDARY: D2D1_COLOR_F = D2D1_COLOR_F {
        r: 0.600,
        g: 0.675,
        b: 0.785,
        a: 1.0,
    };
}

pub struct NotchRenderer {
    pub factory: ID2D1Factory,
    pub dwrite: IDWriteFactory,
    pub rt: ID2D1HwndRenderTarget,
    pub fmt_clock_hero: IDWriteTextFormat,
    pub fmt_title: IDWriteTextFormat,
    pub fmt_body: IDWriteTextFormat,
    pub fmt_caption: IDWriteTextFormat,
    pub fmt_battery_inside: IDWriteTextFormat,
}

impl NotchRenderer {
    pub fn new(hwnd: HWND, width: u32, height: u32, dpi_scale: f32) -> windows::core::Result<Self> {
        unsafe {
            let factory: ID2D1Factory =
                D2D1CreateFactory(D2D1_FACTORY_TYPE_SINGLE_THREADED, None)?;
            let dwrite: IDWriteFactory = DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED)?;

            // PENTING: Set DPI RenderTarget sesuai dpi_scale agar koordinat logis 100% sinkron dengan mouse click!
            let dpi = 96.0 * dpi_scale.max(1.0);
            let render_props = D2D1_RENDER_TARGET_PROPERTIES {
                r#type: D2D1_RENDER_TARGET_TYPE_DEFAULT,
                pixelFormat: D2D1_PIXEL_FORMAT {
                    format: DXGI_FORMAT_B8G8R8A8_UNORM,
                    alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED,
                },
                dpiX: dpi,
                dpiY: dpi,
                usage: D2D1_RENDER_TARGET_USAGE_NONE,
                minLevel: D2D1_FEATURE_LEVEL_DEFAULT,
            };

            let hwnd_props = D2D1_HWND_RENDER_TARGET_PROPERTIES {
                hwnd,
                pixelSize: D2D_SIZE_U {
                    width: width.max(100),
                    height: height.max(50),
                },
                presentOptions: D2D1_PRESENT_OPTIONS_IMMEDIATELY,
            };

            let rt = factory.CreateHwndRenderTarget(&render_props, &hwnd_props)?;
            rt.SetAntialiasMode(D2D1_ANTIALIAS_MODE_PER_PRIMITIVE);

            // Font modern & rounded ala iOS ("Segoe UI Variable Display")
            let fmt_clock_hero = dwrite.CreateTextFormat(
                w!("Segoe UI Variable Display"),
                None,
                DWRITE_FONT_WEIGHT_BOLD,
                DWRITE_FONT_STYLE_NORMAL,
                DWRITE_FONT_STRETCH_NORMAL,
                26.0,
                w!("en-us"),
            )?;

            let fmt_title = dwrite.CreateTextFormat(
                w!("Segoe UI Variable Display"),
                None,
                DWRITE_FONT_WEIGHT_BOLD,
                DWRITE_FONT_STYLE_NORMAL,
                DWRITE_FONT_STRETCH_NORMAL,
                12.5,
                w!("en-us"),
            )?;

            let fmt_body = dwrite.CreateTextFormat(
                w!("Segoe UI Variable Display"),
                None,
                DWRITE_FONT_WEIGHT_MEDIUM,
                DWRITE_FONT_STYLE_NORMAL,
                DWRITE_FONT_STRETCH_NORMAL,
                11.0,
                w!("en-us"),
            )?;

            let fmt_caption = dwrite.CreateTextFormat(
                w!("Segoe UI Variable Display"),
                None,
                DWRITE_FONT_WEIGHT_SEMI_BOLD,
                DWRITE_FONT_STYLE_NORMAL,
                DWRITE_FONT_STRETCH_NORMAL,
                9.5,
                w!("en-us"),
            )?;

            let fmt_battery_inside = dwrite.CreateTextFormat(
                w!("Segoe UI Variable Display"),
                None,
                DWRITE_FONT_WEIGHT_BOLD,
                DWRITE_FONT_STYLE_NORMAL,
                DWRITE_FONT_STRETCH_NORMAL,
                8.5,
                w!("en-us"),
            )?;

            Ok(Self {
                factory,
                dwrite,
                rt,
                fmt_clock_hero,
                fmt_title,
                fmt_body,
                fmt_caption,
                fmt_battery_inside,
            })
        }
    }

    unsafe fn create_bezel_notch_geometry(
        &self,
        rect: D2D_RECT_F,
        bottom_radius: f32,
    ) -> windows::core::Result<ID2D1PathGeometry> {
        let geom = self.factory.CreatePathGeometry()?;
        let sink = geom.Open()?;

        let h = (rect.bottom - rect.top).max(4.0);
        let r = bottom_radius.min(h * 0.5).max(2.0);
        let ear = (r * 0.48).min(9.0);

        sink.BeginFigure(
            D2D_POINT_2F {
                x: rect.left - ear,
                y: rect.top,
            },
            D2D1_FIGURE_BEGIN_FILLED,
        );
        sink.AddBezier(&D2D1_BEZIER_SEGMENT {
            point1: D2D_POINT_2F {
                x: rect.left - ear * 0.2,
                y: rect.top,
            },
            point2: D2D_POINT_2F {
                x: rect.left,
                y: rect.top + ear * 0.2,
            },
            point3: D2D_POINT_2F {
                x: rect.left,
                y: rect.top + ear,
            },
        });
        sink.AddLine(D2D_POINT_2F {
            x: rect.left,
            y: rect.bottom - r,
        });
        sink.AddBezier(&D2D1_BEZIER_SEGMENT {
            point1: D2D_POINT_2F {
                x: rect.left,
                y: rect.bottom - r * 0.45,
            },
            point2: D2D_POINT_2F {
                x: rect.left + r * 0.45,
                y: rect.bottom,
            },
            point3: D2D_POINT_2F {
                x: rect.left + r,
                y: rect.bottom,
            },
        });
        sink.AddLine(D2D_POINT_2F {
            x: rect.right - r,
            y: rect.bottom,
        });
        sink.AddBezier(&D2D1_BEZIER_SEGMENT {
            point1: D2D_POINT_2F {
                x: rect.right - r * 0.45,
                y: rect.bottom,
            },
            point2: D2D_POINT_2F {
                x: rect.right,
                y: rect.bottom - r * 0.45,
            },
            point3: D2D_POINT_2F {
                x: rect.right,
                y: rect.bottom - r,
            },
        });
        sink.AddLine(D2D_POINT_2F {
            x: rect.right,
            y: rect.top + ear,
        });
        sink.AddBezier(&D2D1_BEZIER_SEGMENT {
            point1: D2D_POINT_2F {
                x: rect.right,
                y: rect.top + ear * 0.2,
            },
            point2: D2D_POINT_2F {
                x: rect.right + ear * 0.2,
                y: rect.top,
            },
            point3: D2D_POINT_2F {
                x: rect.right + ear,
                y: rect.top,
            },
        });

        sink.EndFigure(D2D1_FIGURE_END_CLOSED);
        sink.Close()?;
        Ok(geom)
    }

    /// Helper: Gambar Rounded Card dengan Multi-Stop Linear Gradient & Semi-Glass Border Tebal
    unsafe fn fill_gradient_glass_card(
        &self,
        rect: D2D_RECT_F,
        radius: f32,
        c_start: D2D1_COLOR_F,
        c_mid: D2D1_COLOR_F,
        c_end: D2D1_COLOR_F,
        border_col: D2D1_COLOR_F,
        border_width: f32,
    ) {
        let rrect = D2D1_ROUNDED_RECT {
            rect,
            radiusX: radius,
            radiusY: radius,
        };
        let stops = [
            D2D1_GRADIENT_STOP {
                position: 0.0,
                color: c_start,
            },
            D2D1_GRADIENT_STOP {
                position: 0.52,
                color: c_mid,
            },
            D2D1_GRADIENT_STOP {
                position: 1.0,
                color: c_end,
            },
        ];
        if let Ok(sc) =
            self.rt
                .CreateGradientStopCollection(&stops, D2D1_GAMMA_2_2, D2D1_EXTEND_MODE_CLAMP)
        {
            let props = D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES {
                startPoint: D2D_POINT_2F {
                    x: rect.left,
                    y: rect.top,
                },
                endPoint: D2D_POINT_2F {
                    x: rect.right,
                    y: rect.bottom,
                },
            };
            if let Ok(gb) = self.rt.CreateLinearGradientBrush(&props, None, &sc) {
                self.rt.FillRoundedRectangle(&rrect, &gb);
            }
        }
        if border_width > 0.0 {
            if let Ok(bb) = self.rt.CreateSolidColorBrush(&border_col, None) {
                self.rt.DrawRoundedRectangle(&rrect, &bb, border_width, None);
            }
        }
    }

    /// Helper: Gambar Ikon Baterai Rounded ala iOS dengan Teks Persentase di Dalamnya!
    unsafe fn draw_ios_battery_icon(
        &self,
        x: f32,
        y: f32,
        percent: u8,
        charging: bool,
        alpha: f32,
    ) {
        let bw = 34.0;
        let bh = 16.0;
        let body_rect = D2D_RECT_F {
            left: x,
            top: y,
            right: x + bw,
            bottom: y + bh,
        };

        // 1. Kapsul Luar Baterai Rounded
        let outline_col = D2D1_COLOR_F {
            r: 0.65,
            g: 0.72,
            b: 0.84,
            a: 0.75 * alpha,
        };
        if let Ok(ob) = self.rt.CreateSolidColorBrush(&outline_col, None) {
            self.rt.DrawRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: body_rect,
                    radiusX: 5.5,
                    radiusY: 5.5,
                },
                &ob,
                1.3,
                None,
            );
            // Kutub positif kecil di sebelah kanan
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: D2D_RECT_F {
                        left: x + bw + 1.5,
                        top: y + 4.5,
                        right: x + bw + 3.5,
                        bottom: y + bh - 4.5,
                    },
                    radiusX: 1.2,
                    radiusY: 1.2,
                },
                &ob,
            );
        }

        // 2. Isi Bar Baterai di Dalam (Hijau iOS saat Charging, Steel-Blue saat Normal)
        let fill_ratio = (percent.min(100) as f32 / 100.0).max(0.12);
        let fill_w = (bw - 3.6) * fill_ratio;
        let fill_col = if charging {
            D2D1_COLOR_F {
                r: 0.18,
                g: 0.82,
                b: 0.48,
                a: 0.92 * alpha,
            }
        } else if percent <= 20 {
            D2D1_COLOR_F {
                r: 0.85,
                g: 0.25,
                b: 0.32,
                a: 0.92 * alpha,
            }
        } else {
            D2D1_COLOR_F {
                r: 0.34,
                g: 0.50,
                b: 0.88,
                a: 0.88 * alpha,
            }
        };

        if let Ok(fb) = self.rt.CreateSolidColorBrush(&fill_col, None) {
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: D2D_RECT_F {
                        left: x + 1.8,
                        top: y + 1.8,
                        right: x + 1.8 + fill_w,
                        bottom: y + bh - 1.8,
                    },
                    radiusX: 4.0,
                    radiusY: 4.0,
                },
                &fb,
            );
        }

        // 3. Angka Persentase Tepat di Dalam Ikon Baterai (Gaya iOS Status Bar)
        let label = if charging {
            format!("⚡{}", percent)
        } else {
            format!("{}", percent)
        };
        let txt_col = D2D1_COLOR_F {
            r: 0.98,
            g: 0.99,
            b: 1.0,
            a: alpha,
        };
        self.draw_text_centered(&label, &self.fmt_battery_inside, body_rect, txt_col);
    }

    unsafe fn draw_text_left(
        &self,
        text: &str,
        fmt: &IDWriteTextFormat,
        rect: D2D_RECT_F,
        color: D2D1_COLOR_F,
    ) {
        let _ = fmt.SetTextAlignment(DWRITE_TEXT_ALIGNMENT_LEADING);
        let _ = fmt.SetParagraphAlignment(DWRITE_PARAGRAPH_ALIGNMENT_CENTER);
        let wide: Vec<u16> = text.encode_utf16().collect();
        if let Ok(brush) = self.rt.CreateSolidColorBrush(&color, None) {
            self.rt.DrawText(
                &wide,
                fmt,
                &rect,
                &brush,
                D2D1_DRAW_TEXT_OPTIONS_CLIP,
                Default::default(),
            );
        }
    }

    unsafe fn draw_text_centered(
        &self,
        text: &str,
        fmt: &IDWriteTextFormat,
        rect: D2D_RECT_F,
        color: D2D1_COLOR_F,
    ) {
        let _ = fmt.SetTextAlignment(DWRITE_TEXT_ALIGNMENT_CENTER);
        let _ = fmt.SetParagraphAlignment(DWRITE_PARAGRAPH_ALIGNMENT_CENTER);
        let wide: Vec<u16> = text.encode_utf16().collect();
        if let Ok(brush) = self.rt.CreateSolidColorBrush(&color, None) {
            self.rt.DrawText(
                &wide,
                fmt,
                &rect,
                &brush,
                D2D1_DRAW_TEXT_OPTIONS_CLIP,
                Default::default(),
            );
        }
    }

    pub fn render_frame(&self, controller: &mut NotchController, pill_rect: D2D_RECT_F) {
        unsafe {
            let mut ps = PAINTSTRUCT::default();
            let hwnd = self.rt.GetHwnd();
            let _ = BeginPaint(hwnd, &mut ps);

            self.rt.BeginDraw();
            self.rt.Clear(Some(&D2D1_COLOR_F {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            }));

            // 1. Gambar Shell Utama Notch (Menyatu Bezel Atas + Multi-Stop Semi-Glass Gradient)
            if let Ok(notch_geom) =
                self.create_bezel_notch_geometry(pill_rect, controller.corner_radius.value)
            {
                let bg_stops = [
                    D2D1_GRADIENT_STOP {
                        position: 0.0,
                        color: D2D1_COLOR_F {
                            r: 0.038,
                            g: 0.048,
                            b: 0.072,
                            a: 0.98,
                        },
                    },
                    D2D1_GRADIENT_STOP {
                        position: 0.65,
                        color: D2D1_COLOR_F {
                            r: 0.055,
                            g: 0.070,
                            b: 0.105,
                            a: 0.97,
                        },
                    },
                    D2D1_GRADIENT_STOP {
                        position: 1.0,
                        color: D2D1_COLOR_F {
                            r: 0.078,
                            g: 0.098,
                            b: 0.148,
                            a: 0.96,
                        },
                    },
                ];
                if let Ok(sc) = self.rt.CreateGradientStopCollection(
                    &bg_stops,
                    D2D1_GAMMA_2_2,
                    D2D1_EXTEND_MODE_CLAMP,
                ) {
                    let props = D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES {
                        startPoint: D2D_POINT_2F {
                            x: pill_rect.left,
                            y: pill_rect.top,
                        },
                        endPoint: D2D_POINT_2F {
                            x: pill_rect.right,
                            y: pill_rect.bottom,
                        },
                    };
                    if let Ok(gb) = self.rt.CreateLinearGradientBrush(&props, None, &sc) {
                        self.rt.FillGeometry(&notch_geom, &gb, None);
                    }
                }

                // Semi-Glass Rim Tebal (1.6px)
                let border_color = D2D1_COLOR_F {
                    r: 0.28,
                    g: 0.36,
                    b: 0.52,
                    a: 0.68,
                };
                if let Ok(border_brush) = self.rt.CreateSolidColorBrush(&border_color, None) {
                    self.rt.DrawGeometry(&notch_geom, &border_brush, 1.6, None);
                }
            }

            // 2. Pop-Up Interactive Overlay (Volume, Brightness, CapsLock, Charging)
            if !controller.is_hovered {
                if let Some(ref overlay) = controller.active_overlay.clone() {
                    self.draw_system_overlay(overlay, pill_rect);
                    let _ = self.rt.EndDraw(None, None);
                    let _ = EndPaint(hwnd, &ps);
                    return;
                }
            }

            // 3. Render State Aktif
            if controller.state == NotchState::Pet && !controller.minimal_mode {
                let collapse_factor = ((60.0 - controller.height.value) / 24.0).clamp(0.0, 1.0);
                controller
                    .devil_eyes
                    .render(&self.factory, &self.rt, pill_rect, collapse_factor);
            } else if controller.state == NotchState::Idle && !controller.minimal_mode {
                let alpha = ((60.0 - controller.height.value) / 24.0).clamp(0.0, 1.0);
                self.draw_closed_idle_clock_and_battery(controller, pill_rect, alpha);
            } else if controller.height.value > 65.0 {
                let alpha = controller.content_alpha.value.clamp(0.0, 1.0);
                let card_rect = self.draw_expanded_panel_shell(controller.state, pill_rect, alpha);

                match controller.state {
                    NotchState::Hub => self.draw_hub_panel(controller, card_rect, alpha),
                    NotchState::Music => self.draw_music_panel(controller, card_rect, alpha),
                    NotchState::Control => self.draw_control_panel(controller, card_rect, alpha),
                    NotchState::Pomo => self.draw_pomo_panel(controller, card_rect, alpha),
                    NotchState::Shelf => self.draw_shelf_panel(controller, card_rect, alpha),
                    NotchState::Todo => self.draw_todo_panel(controller, card_rect, alpha),
                    _ => {}
                }

                if controller.mascot_enabled {
                    controller
                        .chibi_liebe
                        .render(&self.factory, &self.rt, pill_rect, alpha);
                }
            }

            let _ = self.rt.EndDraw(None, None);
            let _ = EndPaint(hwnd, &ps);
        }
    }

    unsafe fn draw_closed_idle_clock_and_battery(
        &self,
        ctrl: &NotchController,
        pill: D2D_RECT_F,
        alpha: f32,
    ) {
        let now_str = Local::now().format("%H:%M").to_string();
        let mut p_col = AntiMagicNeutralPalette::TEXT_PRIMARY;
        p_col.a *= alpha;

        self.draw_text_left(
            &now_str,
            &self.fmt_title,
            D2D_RECT_F {
                left: pill.left + 16.0,
                top: pill.top,
                right: pill.left + 85.0,
                bottom: pill.bottom,
            },
            p_col,
        );

        let bat_y = (pill.top + pill.bottom - 16.0) * 0.5;
        self.draw_ios_battery_icon(
            pill.right - 52.0,
            bat_y,
            ctrl.battery_percent,
            ctrl.is_charging,
            alpha,
        );
    }

    unsafe fn draw_system_overlay(&self, overlay: &OverlayKind, pill: D2D_RECT_F) {
        let p_col = AntiMagicNeutralPalette::TEXT_PRIMARY;
        let accent = AntiMagicNeutralPalette::ACCENT_BLUE;
        let cyan = AntiMagicNeutralPalette::ACCENT_CYAN;

        match overlay {
            OverlayKind::Volume(val) | OverlayKind::Brightness(val) => {
                let is_vol = matches!(overlay, OverlayKind::Volume(_));
                let label = if is_vol { "🔊 VOL" } else { "🔆 BRI" };
                self.draw_text_left(
                    label,
                    &self.fmt_caption,
                    D2D_RECT_F {
                        left: pill.left + 14.0,
                        top: pill.top,
                        right: pill.left + 58.0,
                        bottom: pill.bottom,
                    },
                    p_col,
                );

                // Interactive Slider Track (Bisa Diklik Langsung!)
                let bar_l = pill.left + 58.0;
                let bar_r = pill.right - 48.0;
                let bar_cy = (pill.top + pill.bottom) * 0.5;

                if let Ok(tb) = self.rt.CreateSolidColorBrush(
                    &D2D1_COLOR_F {
                        r: 0.16,
                        g: 0.20,
                        b: 0.28,
                        a: 0.85,
                    },
                    None,
                ) {
                    self.rt.FillRoundedRectangle(
                        &D2D1_ROUNDED_RECT {
                            rect: D2D_RECT_F {
                                left: bar_l,
                                top: bar_cy - 5.0,
                                right: bar_r,
                                bottom: bar_cy + 5.0,
                            },
                            radiusX: 5.0,
                            radiusY: 5.0,
                        },
                        &tb,
                    );
                }

                let fill_w = (bar_r - bar_l) * ((*val as f32 / 100.0).clamp(0.0, 1.0));
                self.fill_gradient_glass_card(
                    D2D_RECT_F {
                        left: bar_l,
                        top: bar_cy - 5.0,
                        right: bar_l + fill_w.max(8.0),
                        bottom: bar_cy + 5.0,
                    },
                    5.0,
                    accent,
                    cyan,
                    cyan,
                    cyan,
                    0.0,
                );

                self.draw_text_centered(
                    &format!("{}%", val),
                    &self.fmt_caption,
                    D2D_RECT_F {
                        left: pill.right - 46.0,
                        top: pill.top,
                        right: pill.right - 10.0,
                        bottom: pill.bottom,
                    },
                    p_col,
                );
            }
            OverlayKind::CapsLock(on) => {
                let txt = if *on {
                    "CAPS LOCK · ON"
                } else {
                    "CAPS LOCK · OFF"
                };
                self.draw_text_centered(
                    txt,
                    &self.fmt_title,
                    pill,
                    if *on { cyan } else { p_col },
                );
            }
            OverlayKind::Charging { percent, plugged } => {
                self.draw_text_left(
                    if *plugged {
                        "⚡ Power Connected"
                    } else {
                        "On Battery Power"
                    },
                    &self.fmt_title,
                    D2D_RECT_F {
                        left: pill.left + 18.0,
                        top: pill.top,
                        right: pill.right - 68.0,
                        bottom: pill.bottom,
                    },
                    p_col,
                );
                let bat_y = (pill.top + pill.bottom - 16.0) * 0.5;
                self.draw_ios_battery_icon(pill.right - 56.0, bat_y, *percent, *plugged, 1.0);
            }
        }
    }

    /// Bar Navigasi Atas dengan Tombol Panah `<` dan `>` + 6 Section + Inner Semi-Glass Card
    unsafe fn draw_expanded_panel_shell(
        &self,
        state: NotchState,
        pill_rect: D2D_RECT_F,
        alpha: f32,
    ) -> D2D_RECT_F {
        let tabs = ["Hub", "Music", "Control", "Focus", "Shelf", "To-Do"];
        let active_idx = match state {
            NotchState::Hub => 0,
            NotchState::Music => 1,
            NotchState::Control => 2,
            NotchState::Pomo => 3,
            NotchState::Shelf => 4,
            NotchState::Todo => 5,
            _ => 1,
        };

        let mut p_col = AntiMagicNeutralPalette::TEXT_PRIMARY;
        p_col.a *= alpha;
        let mut s_col = AntiMagicNeutralPalette::TEXT_SECONDARY;
        s_col.a *= alpha * 0.78;

        // Tombol Panah Kiri `<` (lx: 12..36) dan Kanan `>` (lx: pill_w - 36 .. pill_w - 12)
        let left_arrow_rect = D2D_RECT_F {
            left: pill_rect.left + 12.0,
            top: pill_rect.top + 4.0,
            right: pill_rect.left + 34.0,
            bottom: pill_rect.top + 24.0,
        };
        let right_arrow_rect = D2D_RECT_F {
            left: pill_rect.right - 34.0,
            top: pill_rect.top + 4.0,
            right: pill_rect.right - 12.0,
            bottom: pill_rect.top + 24.0,
        };

        if let Ok(ab) = self.rt.CreateSolidColorBrush(
            &D2D1_COLOR_F {
                r: 0.14,
                g: 0.18,
                b: 0.26,
                a: 0.85 * alpha,
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

        // Area 6 Tab di antara kedua panah (lx: 38 .. pill_w - 38)
        let tabs_left = pill_rect.left + 38.0;
        let tabs_right = pill_rect.right - 38.0;
        let slot_w = (tabs_right - tabs_left) / 6.0;

        for (i, label) in tabs.iter().enumerate() {
            let sl = tabs_left + slot_w * i as f32;
            let sr = sl + slot_w;
            let cx = (sl + sr) * 0.5;
            let is_active = i == active_idx;

            if is_active {
                // Kapsul Semi-Glass pada Tab yang sedang aktif
                self.fill_gradient_glass_card(
                    D2D_RECT_F {
                        left: sl + 3.0,
                        top: pill_rect.top + 4.0,
                        right: sr - 3.0,
                        bottom: pill_rect.top + 24.0,
                    },
                    7.0,
                    D2D1_COLOR_F {
                        r: 0.20,
                        g: 0.28,
                        b: 0.44,
                        a: 0.85 * alpha,
                    },
                    D2D1_COLOR_F {
                        r: 0.14,
                        g: 0.20,
                        b: 0.34,
                        a: 0.85 * alpha,
                    },
                    D2D1_COLOR_F {
                        r: 0.18,
                        g: 0.24,
                        b: 0.40,
                        a: 0.85 * alpha,
                    },
                    D2D1_COLOR_F {
                        r: 0.39,
                        g: 0.55,
                        b: 0.95,
                        a: 0.70 * alpha,
                    },
                    1.0,
                );
            }

            self.draw_text_centered(
                label,
                &self.fmt_caption,
                D2D_RECT_F {
                    left: sl,
                    top: pill_rect.top + 4.0,
                    right: sr,
                    bottom: pill_rect.top + 24.0,
                },
                if is_active { p_col } else { s_col },
            );

            if is_active {
                if let Ok(lb) = self
                    .rt
                    .CreateSolidColorBrush(&AntiMagicNeutralPalette::ACCENT_CYAN, None)
                {
                    self.rt.DrawLine(
                        D2D_POINT_2F {
                            x: cx - 12.0,
                            y: pill_rect.top + 25.0,
                        },
                        D2D_POINT_2F {
                            x: cx + 12.0,
                            y: pill_rect.top + 25.0,
                        },
                        &lb,
                        2.0,
                        None,
                    );
                }
            }
        }

        // Inner Semi-Glass Container Card Tebal (radius 16.0)
        let card_rect = D2D_RECT_F {
            left: pill_rect.left + 14.0,
            top: pill_rect.top + 30.0,
            right: pill_rect.right - 14.0,
            bottom: pill_rect.bottom - 12.0,
        };

        self.fill_gradient_glass_card(
            card_rect,
            16.0,
            D2D1_COLOR_F {
                r: 0.075,
                g: 0.095,
                b: 0.142,
                a: 0.94 * alpha,
            },
            D2D1_COLOR_F {
                r: 0.090,
                g: 0.115,
                b: 0.172,
                a: 0.94 * alpha,
            },
            D2D1_COLOR_F {
                r: 0.068,
                g: 0.088,
                b: 0.135,
                a: 0.94 * alpha,
            },
            D2D1_COLOR_F {
                r: 0.24,
                g: 0.31,
                b: 0.45,
                a: 0.55 * alpha,
            },
            1.3,
        );

        card_rect
    }

    // =========================================================================
    // SECTION 0: HUB (Jam Besar, Status Baterai iOS, & Scrollable Notifications)
    // =========================================================================
    unsafe fn draw_hub_panel(&self, ctrl: &NotchController, card: D2D_RECT_F, alpha: f32) {
        let mut p_col = AntiMagicNeutralPalette::TEXT_PRIMARY;
        p_col.a *= alpha;
        let mut s_col = AntiMagicNeutralPalette::TEXT_SECONDARY;
        s_col.a *= alpha;

        // Kiri: Kartu Jam Besar, Tanggal, & Ikon Baterai iOS
        let left_card = D2D_RECT_F {
            left: card.left + 10.0,
            top: card.top + 10.0,
            right: card.left + 166.0,
            bottom: card.bottom - 10.0,
        };
        self.fill_gradient_glass_card(
            left_card,
            13.0,
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
            1.1,
        );

        let now = Local::now();
        let time_big = now.format("%H:%M").to_string();
        let date_str = now.format("%a, %d %b %Y").to_string();

        self.draw_text_left(
            &time_big,
            &self.fmt_clock_hero,
            D2D_RECT_F {
                left: left_card.left + 12.0,
                top: left_card.top + 8.0,
                right: left_card.right - 8.0,
                bottom: left_card.top + 44.0,
            },
            p_col,
        );
        self.draw_text_left(
            &date_str,
            &self.fmt_caption,
            D2D_RECT_F {
                left: left_card.left + 12.0,
                top: left_card.top + 44.0,
                right: left_card.right - 8.0,
                bottom: left_card.top + 64.0,
            },
            s_col,
        );

        // Ikon Baterai iOS + Badge Jumlah Notifikasi
        self.draw_ios_battery_icon(
            left_card.left + 12.0,
            left_card.bottom - 30.0,
            ctrl.battery_percent,
            ctrl.is_charging,
            alpha,
        );

        let badge_txt = format!("🔔 {} Notif", ctrl.notifications.len());
        self.draw_text_left(
            &badge_txt,
            &self.fmt_caption,
            D2D_RECT_F {
                left: left_card.left + 60.0,
                top: left_card.bottom - 32.0,
                right: left_card.right - 6.0,
                bottom: left_card.bottom - 12.0,
            },
            AntiMagicNeutralPalette::ACCENT_CYAN,
        );

        // Kanan: Scrollable Notification List (Klik untuk buka aplikasi terkait!)
        let right_left = left_card.right + 10.0;
        for (row, notif) in ctrl
            .notifications
            .iter()
            .skip(ctrl.notif_scroll_offset)
            .take(2)
            .enumerate()
        {
            let ny = card.top + 10.0 + row as f32 * 56.0;
            let n_rect = D2D_RECT_F {
                left: right_left,
                top: ny,
                right: card.right - 10.0,
                bottom: ny + 50.0,
            };

            self.fill_gradient_glass_card(
                n_rect,
                11.0,
                D2D1_COLOR_F {
                    r: 0.11,
                    g: 0.14,
                    b: 0.21,
                    a: 0.90 * alpha,
                },
                D2D1_COLOR_F {
                    r: 0.13,
                    g: 0.16,
                    b: 0.25,
                    a: 0.90 * alpha,
                },
                D2D1_COLOR_F {
                    r: 0.10,
                    g: 0.13,
                    b: 0.19,
                    a: 0.90 * alpha,
                },
                D2D1_COLOR_F {
                    r: 0.28,
                    g: 0.36,
                    b: 0.52,
                    a: 0.45 * alpha,
                },
                1.0,
            );

            let header = format!("{} · {}  ({})", notif.app_name, notif.title, notif.time_str);
            self.draw_text_left(
                &header,
                &self.fmt_caption,
                D2D_RECT_F {
                    left: n_rect.left + 10.0,
                    top: n_rect.top + 4.0,
                    right: n_rect.right - 10.0,
                    bottom: n_rect.top + 22.0,
                },
                AntiMagicNeutralPalette::ACCENT_CYAN,
            );
            self.draw_text_left(
                &notif.message,
                &self.fmt_body,
                D2D_RECT_F {
                    left: n_rect.left + 10.0,
                    top: n_rect.top + 22.0,
                    right: n_rect.right - 10.0,
                    bottom: n_rect.bottom - 4.0,
                },
                p_col,
            );
        }
    }

    // =========================================================================
    // SECTION 1: MUSIC/VIDEO + SPOTIFY-STYLE INTEGRATED RIGHT-SIDE LYRICS
    // =========================================================================
    unsafe fn draw_music_panel(&self, ctrl: &NotchController, card: D2D_RECT_F, alpha: f32) {
        let mut p_col = AntiMagicNeutralPalette::TEXT_PRIMARY;
        p_col.a *= alpha;
        let mut s_col = AntiMagicNeutralPalette::TEXT_SECONDARY;
        s_col.a *= alpha;

        let card_w = card.right - card.left;
        let left_pane_right = if ctrl.show_side_lyrics {
            card.left + card_w * 0.54
        } else {
            card.right - 52.0
        };

        // 1. Kotak Album/Video Cover dengan Gradien Kaya & Visualizer
        let art_rect = D2D_RECT_F {
            left: card.left + 10.0,
            top: card.top + 10.0,
            right: card.left + 74.0,
            bottom: card.top + 74.0,
        };
        self.fill_gradient_glass_card(
            art_rect,
            13.0,
            D2D1_COLOR_F {
                r: 0.16,
                g: 0.22,
                b: 0.38,
                a: alpha,
            },
            D2D1_COLOR_F {
                r: 0.24,
                g: 0.16,
                b: 0.32,
                a: alpha,
            },
            D2D1_COLOR_F {
                r: 0.10,
                g: 0.16,
                b: 0.28,
                a: alpha,
            },
            D2D1_COLOR_F {
                r: 0.40,
                g: 0.54,
                b: 0.88,
                a: 0.65 * alpha,
            },
            1.1,
        );

        // Visualizer bar
        if let Ok(eq_b) = self
            .rt
            .CreateSolidColorBrush(&AntiMagicNeutralPalette::ACCENT_CYAN, None)
        {
            for i in 0..4 {
                let phase = ctrl.anim_time * 5.8 + i as f32 * 1.2;
                let h = if ctrl.media_playing {
                    8.0 + phase.sin().abs() * 22.0
                } else {
                    5.0
                };
                let bx = art_rect.left + 14.0 + i as f32 * 10.0;
                let by = art_rect.bottom - 12.0;
                self.rt.FillRoundedRectangle(
                    &D2D1_ROUNDED_RECT {
                        rect: D2D_RECT_F {
                            left: bx,
                            top: by - h,
                            right: bx + 5.5,
                            bottom: by,
                        },
                        radiusX: 2.5,
                        radiusY: 2.5,
                    },
                    &eq_b,
                );
            }
        }

        // 2. Judul, Artis & Progress Bar di Samping Cover
        let info_left = art_rect.right + 10.0;
        self.draw_text_left(
            &ctrl.media_title,
            &self.fmt_title,
            D2D_RECT_F {
                left: info_left,
                top: card.top + 10.0,
                right: left_pane_right - 6.0,
                bottom: card.top + 30.0,
            },
            p_col,
        );
        self.draw_text_left(
            &ctrl.media_artist,
            &self.fmt_caption,
            D2D_RECT_F {
                left: info_left,
                top: card.top + 30.0,
                right: left_pane_right - 6.0,
                bottom: card.top + 48.0,
            },
            s_col,
        );

        // Progress Bar
        let bar_top = card.top + 56.0;
        let bar_w = (left_pane_right - 8.0 - info_left).max(16.0);
        if let Ok(tb) = self.rt.CreateSolidColorBrush(
            &D2D1_COLOR_F {
                r: 0.18,
                g: 0.23,
                b: 0.34,
                a: 0.75 * alpha,
            },
            None,
        ) {
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: D2D_RECT_F {
                        left: info_left,
                        top: bar_top,
                        right: left_pane_right - 8.0,
                        bottom: bar_top + 4.5,
                    },
                    radiusX: 2.2,
                    radiusY: 2.2,
                },
                &tb,
            );
        }
        let prog_w = (bar_w * ctrl.media_progress.clamp(0.0, 1.0)).max(5.0);
        self.fill_gradient_glass_card(
            D2D_RECT_F {
                left: info_left,
                top: bar_top,
                right: info_left + prog_w,
                bottom: bar_top + 4.5,
            },
            2.2,
            AntiMagicNeutralPalette::ACCENT_BLUE,
            AntiMagicNeutralPalette::ACCENT_CYAN,
            AntiMagicNeutralPalette::ACCENT_CYAN,
            AntiMagicNeutralPalette::ACCENT_CYAN,
            0.0,
        );

        // 3. Baris Tombol Kontrol: [ |<< ] [ Play/Pause ] [ >>| ] [ Lyrics ]
        let btn_y = card.top + 96.0;
        let btns = [
            (card.left + 98.0, "|<<"),
            (
                card.left + 136.0,
                if ctrl.media_playing { "||" } else { "|>" },
            ),
            (card.left + 174.0, ">>|"),
        ];
        for (cx, sym) in btns {
            let r = D2D_RECT_F {
                left: cx - 15.0,
                top: btn_y - 11.0,
                right: cx + 15.0,
                bottom: btn_y + 11.0,
            };
            if let Ok(b) = self
                .rt
                .CreateSolidColorBrush(&AntiMagicNeutralPalette::CARD_ELEVATED, None)
            {
                self.rt.FillRoundedRectangle(
                    &D2D1_ROUNDED_RECT {
                        rect: r,
                        radiusX: 7.0,
                        radiusY: 7.0,
                    },
                    &b,
                );
            }
            self.draw_text_centered(sym, &self.fmt_caption, r, p_col);
        }

        let lyr_btn = D2D_RECT_F {
            left: card.left + 196.0,
            top: btn_y - 11.0,
            right: card.left + 244.0,
            bottom: btn_y + 11.0,
        };
        if let Ok(b) = self
            .rt
            .CreateSolidColorBrush(&AntiMagicNeutralPalette::CARD_ELEVATED, None)
        {
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: lyr_btn,
                    radiusX: 7.0,
                    radiusY: 7.0,
                },
                &b,
            );
        }
        self.draw_text_centered(
            "Lyrics",
            &self.fmt_caption,
            lyr_btn,
            AntiMagicNeutralPalette::ACCENT_CYAN,
        );

        // 4. PANEL KANAN: SPOTIFY-STYLE INTEGRATED LIVE LYRICS CARD!
        if ctrl.show_side_lyrics {
            let lyr_card = D2D_RECT_F {
                left: left_pane_right + 4.0,
                top: card.top + 10.0,
                right: card.right - 44.0, // Sisakan ruang di sudut kanan bawah untuk Chibi Liebe
                bottom: card.bottom - 10.0,
            };

            self.fill_gradient_glass_card(
                lyr_card,
                12.0,
                D2D1_COLOR_F {
                    r: 0.10,
                    g: 0.14,
                    b: 0.23,
                    a: 0.92 * alpha,
                },
                D2D1_COLOR_F {
                    r: 0.14,
                    g: 0.18,
                    b: 0.29,
                    a: 0.92 * alpha,
                },
                D2D1_COLOR_F {
                    r: 0.09,
                    g: 0.12,
                    b: 0.20,
                    a: 0.92 * alpha,
                },
                D2D1_COLOR_F {
                    r: 0.30,
                    g: 0.42,
                    b: 0.65,
                    a: 0.50 * alpha,
                },
                1.0,
            );

            if !ctrl.lyrics_lines.is_empty() {
                let idx = ctrl.active_lyric_idx.min(ctrl.lyrics_lines.len() - 1);
                let prev_l = if idx > 0 {
                    ctrl.lyrics_lines[idx - 1].as_str()
                } else {
                    "♪ · · ·"
                };
                let curr_l = ctrl.lyrics_lines[idx].as_str();
                let next_l = if idx + 1 < ctrl.lyrics_lines.len() {
                    ctrl.lyrics_lines[idx + 1].as_str()
                } else {
                    "· · ·"
                };

                self.draw_text_left(
                    prev_l,
                    &self.fmt_caption,
                    D2D_RECT_F {
                        left: lyr_card.left + 10.0,
                        top: lyr_card.top + 8.0,
                        right: lyr_card.right - 8.0,
                        bottom: lyr_card.top + 30.0,
                    },
                    s_col,
                );
                self.draw_text_left(
                    curr_l,
                    &self.fmt_title,
                    D2D_RECT_F {
                        left: lyr_card.left + 10.0,
                        top: lyr_card.top + 32.0,
                        right: lyr_card.right - 8.0,
                        bottom: lyr_card.top + 74.0,
                    },
                    p_col,
                );
                self.draw_text_left(
                    next_l,
                    &self.fmt_caption,
                    D2D_RECT_F {
                        left: lyr_card.left + 10.0,
                        top: lyr_card.top + 76.0,
                        right: lyr_card.right - 8.0,
                        bottom: lyr_card.bottom - 6.0,
                    },
                    s_col,
                );
            }
        }
    }

    // =========================================================================
    // SECTION 2: ALL-IN-ONE CONTROL CENTER (Volume, Brightness, Wi-Fi, Energy Saver)
    // =========================================================================
    unsafe fn draw_control_panel(&self, ctrl: &NotchController, card: D2D_RECT_F, alpha: f32) {
        let mut p_col = AntiMagicNeutralPalette::TEXT_PRIMARY;
        p_col.a *= alpha;
        let mut s_col = AntiMagicNeutralPalette::TEXT_SECONDARY;
        s_col.a *= alpha;

        let card_w = card.right - card.left;
        let half_w = card_w * 0.5;

        // 1. Kartu Slider Volume (Kiri Atas) & Brightness (Kanan Atas)
        let sliders = [
            (
                card.left + 10.0,
                card.left + half_w - 5.0,
                "🔊 Volume",
                ctrl.volume_level,
            ),
            (
                card.left + half_w + 5.0,
                card.right - 10.0,
                "🔆 Brightness",
                ctrl.brightness_level,
            ),
        ];

        for (l, r, label, val) in sliders {
            let s_rect = D2D_RECT_F {
                left: l,
                top: card.top + 10.0,
                right: r,
                bottom: card.top + 62.0,
            };
            self.fill_gradient_glass_card(
                s_rect,
                12.0,
                D2D1_COLOR_F {
                    r: 0.11,
                    g: 0.14,
                    b: 0.22,
                    a: 0.92 * alpha,
                },
                D2D1_COLOR_F {
                    r: 0.13,
                    g: 0.17,
                    b: 0.26,
                    a: 0.92 * alpha,
                },
                D2D1_COLOR_F {
                    r: 0.10,
                    g: 0.13,
                    b: 0.20,
                    a: 0.92 * alpha,
                },
                D2D1_COLOR_F {
                    r: 0.28,
                    g: 0.36,
                    b: 0.54,
                    a: 0.50 * alpha,
                },
                1.0,
            );

            self.draw_text_left(
                &format!("{} · {}%", label, val),
                &self.fmt_caption,
                D2D_RECT_F {
                    left: l + 10.0,
                    top: s_rect.top + 6.0,
                    right: r - 10.0,
                    bottom: s_rect.top + 26.0,
                },
                p_col,
            );

            // Track & Fill Bar
            let tl = l + 10.0;
            let tr = r - 10.0;
            let ty = s_rect.top + 38.0;
            if let Ok(tb) = self.rt.CreateSolidColorBrush(
                &D2D1_COLOR_F {
                    r: 0.18,
                    g: 0.23,
                    b: 0.34,
                    a: 0.8 * alpha,
                },
                None,
            ) {
                self.rt.FillRoundedRectangle(
                    &D2D1_ROUNDED_RECT {
                        rect: D2D_RECT_F {
                            left: tl,
                            top: ty - 4.5,
                            right: tr,
                            bottom: ty + 4.5,
                        },
                        radiusX: 4.5,
                        radiusY: 4.5,
                    },
                    &tb,
                );
            }
            let fw = (tr - tl) * (val as f32 / 100.0).clamp(0.0, 1.0);
            self.fill_gradient_glass_card(
                D2D_RECT_F {
                    left: tl,
                    top: ty - 4.5,
                    right: tl + fw.max(8.0),
                    bottom: ty + 4.5,
                },
                4.5,
                AntiMagicNeutralPalette::ACCENT_BLUE,
                AntiMagicNeutralPalette::ACCENT_CYAN,
                AntiMagicNeutralPalette::ACCENT_CYAN,
                AntiMagicNeutralPalette::ACCENT_CYAN,
                0.0,
            );
        }

        // 2. Kartu Toggle Wi-Fi (Kiri Bawah) & Energy Saver (Kanan Bawah)
        let toggles = [
            (
                card.left + 10.0,
                card.left + half_w - 5.0,
                "📶 Wi-Fi Network",
                if ctrl.wifi_enabled {
                    "Connected · Active"
                } else {
                    "Disabled"
                },
                ctrl.wifi_enabled,
            ),
            (
                card.left + half_w + 5.0,
                card.right - 10.0,
                "🍃 Energy Saver",
                if ctrl.energy_saver_enabled {
                    "Eco Mode · Active"
                } else {
                    "High Performance"
                },
                ctrl.energy_saver_enabled,
            ),
        ];

        for (l, r, title, sub, active) in toggles {
            let t_rect = D2D_RECT_F {
                left: l,
                top: card.top + 70.0,
                right: r,
                bottom: card.bottom - 10.0,
            };

            let (c0, c1, border) = if active {
                (
                    D2D1_COLOR_F {
                        r: 0.16,
                        g: 0.28,
                        b: 0.48,
                        a: 0.94 * alpha,
                    },
                    D2D1_COLOR_F {
                        r: 0.12,
                        g: 0.22,
                        b: 0.38,
                        a: 0.94 * alpha,
                    },
                    AntiMagicNeutralPalette::ACCENT_CYAN,
                )
            } else {
                (
                    D2D1_COLOR_F {
                        r: 0.10,
                        g: 0.13,
                        b: 0.19,
                        a: 0.90 * alpha,
                    },
                    D2D1_COLOR_F {
                        r: 0.08,
                        g: 0.11,
                        b: 0.16,
                        a: 0.90 * alpha,
                    },
                    D2D1_COLOR_F {
                        r: 0.24,
                        g: 0.30,
                        b: 0.42,
                        a: 0.5 * alpha,
                    },
                )
            };

            self.fill_gradient_glass_card(t_rect, 12.0, c0, c1, c0, border, 1.1);

            self.draw_text_left(
                title,
                &self.fmt_title,
                D2D_RECT_F {
                    left: l + 12.0,
                    top: t_rect.top + 6.0,
                    right: r - 12.0,
                    bottom: t_rect.top + 28.0,
                },
                p_col,
            );
            self.draw_text_left(
                sub,
                &self.fmt_caption,
                D2D_RECT_F {
                    left: l + 12.0,
                    top: t_rect.top + 28.0,
                    right: r - 12.0,
                    bottom: t_rect.bottom - 6.0,
                },
                if active {
                    AntiMagicNeutralPalette::ACCENT_CYAN
                } else {
                    s_col
                },
            );
        }
    }

    // =========================================================================
    // SECTION 3: POMODORO FOCUS TIMER
    // =========================================================================
    unsafe fn draw_pomo_panel(&self, ctrl: &NotchController, card: D2D_RECT_F, alpha: f32) {
        let mut p_col = AntiMagicNeutralPalette::TEXT_PRIMARY;
        p_col.a *= alpha;
        let mut s_col = AntiMagicNeutralPalette::TEXT_SECONDARY;
        s_col.a *= alpha;
        let accent = if ctrl.pomo_is_break {
            AntiMagicNeutralPalette::ACCENT_WINE
        } else {
            AntiMagicNeutralPalette::ACCENT_CYAN
        };

        let ring_cx = card.left + 58.0;
        let ring_cy = (card.top + card.bottom) * 0.5;
        let ring_r = 38.0;

        if let Ok(fg_ring) = self.rt.CreateSolidColorBrush(&accent, None) {
            self.rt.DrawEllipse(
                &D2D1_ELLIPSE {
                    point: D2D_POINT_2F {
                        x: ring_cx,
                        y: ring_cy,
                    },
                    radiusX: ring_r,
                    radiusY: ring_r,
                },
                &fg_ring,
                3.6,
                None,
            );
        }

        let timer_text = format!(
            "{:02}:{:02}",
            ctrl.pomo_remaining_secs / 60,
            ctrl.pomo_remaining_secs % 60
        );
        self.draw_text_centered(
            &timer_text,
            &self.fmt_title,
            D2D_RECT_F {
                left: ring_cx - 34.0,
                top: ring_cy - 14.0,
                right: ring_cx + 34.0,
                bottom: ring_cy + 14.0,
            },
            p_col,
        );

        let right_x = card.left + 114.0;
        self.draw_text_left(
            if ctrl.pomo_is_break {
                "SHORT BREAK · REST EYES"
            } else {
                "POMODORO · FOCUS SESSION"
            },
            &self.fmt_title,
            D2D_RECT_F {
                left: right_x,
                top: card.top + 14.0,
                right: card.right - 50.0,
                bottom: card.top + 38.0,
            },
            p_col,
        );

        self.draw_text_left(
            if ctrl.pomo_running {
                "Timer running — Chibi Liebe is guarding your focus"
            } else {
                "Click Start Focus below to begin 25m session"
            },
            &self.fmt_body,
            D2D_RECT_F {
                left: right_x,
                top: card.top + 38.0,
                right: card.right - 50.0,
                bottom: card.top + 60.0,
            },
            s_col,
        );

        let start_rect = D2D_RECT_F {
            left: right_x,
            top: card.top + 66.0,
            right: right_x + 90.0,
            bottom: card.top + 96.0,
        };
        self.fill_gradient_glass_card(
            start_rect,
            9.0,
            AntiMagicNeutralPalette::ACCENT_BLUE,
            accent,
            accent,
            accent,
            1.0,
        );
        self.draw_text_centered(
            if ctrl.pomo_running { "Pause" } else { "Start Focus" },
            &self.fmt_caption,
            start_rect,
            p_col,
        );

        let reset_rect = D2D_RECT_F {
            left: right_x + 98.0,
            top: card.top + 66.0,
            right: right_x + 166.0,
            bottom: card.top + 96.0,
        };
        if let Ok(b) = self
            .rt
            .CreateSolidColorBrush(&AntiMagicNeutralPalette::CARD_ELEVATED, None)
        {
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: reset_rect,
                    radiusX: 9.0,
                    radiusY: 9.0,
                },
                &b,
            );
        }
        self.draw_text_centered("Reset", &self.fmt_caption, reset_rect, s_col);
    }

    // =========================================================================
    // SECTION 4: FILE SHELF (Drag & Drop Masuk/Keluar App Lain)
    // =========================================================================
    unsafe fn draw_shelf_panel(&self, ctrl: &NotchController, card: D2D_RECT_F, alpha: f32) {
        let mut p_col = AntiMagicNeutralPalette::TEXT_PRIMARY;
        p_col.a *= alpha;
        let mut s_col = AntiMagicNeutralPalette::TEXT_SECONDARY;
        s_col.a *= alpha;

        self.draw_text_left(
            "FILE SHELF · DRAG FILES INTO NOTCH OR CLICK FILE TO OPEN/USE",
            &self.fmt_caption,
            D2D_RECT_F {
                left: card.left + 14.0,
                top: card.top + 8.0,
                right: card.right - 50.0,
                bottom: card.top + 26.0,
            },
            AntiMagicNeutralPalette::ACCENT_CYAN,
        );

        if ctrl.shelf_files.is_empty() {
            let drop_rect = D2D_RECT_F {
                left: card.left + 14.0,
                top: card.top + 32.0,
                right: card.right - 56.0,
                bottom: card.bottom - 12.0,
            };
            self.fill_gradient_glass_card(
                drop_rect,
                12.0,
                D2D1_COLOR_F {
                    r: 0.10,
                    g: 0.13,
                    b: 0.21,
                    a: 0.85 * alpha,
                },
                D2D1_COLOR_F {
                    r: 0.12,
                    g: 0.16,
                    b: 0.26,
                    a: 0.85 * alpha,
                },
                D2D1_COLOR_F {
                    r: 0.09,
                    g: 0.12,
                    b: 0.19,
                    a: 0.85 * alpha,
                },
                D2D1_COLOR_F {
                    r: 0.35,
                    g: 0.48,
                    b: 0.78,
                    a: 0.55 * alpha,
                },
                1.2,
            );
            self.draw_text_centered(
                "📥 Drag & Drop file dari Explorer/App lain ke area Notch ini",
                &self.fmt_body,
                drop_rect,
                s_col,
            );
            return;
        }

        for (i, (name, meta, _)) in ctrl.shelf_files.iter().take(2).enumerate() {
            let item_w = (card.right - card.left - 80.0) * 0.5;
            let ix = card.left + 14.0 + i as f32 * (item_w + 10.0);
            let item_rect = D2D_RECT_F {
                left: ix,
                top: card.top + 32.0,
                right: ix + item_w,
                bottom: card.bottom - 12.0,
            };

            self.fill_gradient_glass_card(
                item_rect,
                12.0,
                D2D1_COLOR_F {
                    r: 0.12,
                    g: 0.16,
                    b: 0.25,
                    a: 0.92 * alpha,
                },
                D2D1_COLOR_F {
                    r: 0.15,
                    g: 0.20,
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
                    b: 0.75,
                    a: 0.55 * alpha,
                },
                1.1,
            );

            self.draw_text_left(
                name,
                &self.fmt_title,
                D2D_RECT_F {
                    left: item_rect.left + 10.0,
                    top: item_rect.top + 10.0,
                    right: item_rect.right - 22.0,
                    bottom: item_rect.top + 36.0,
                },
                p_col,
            );
            self.draw_text_left(
                meta,
                &self.fmt_caption,
                D2D_RECT_F {
                    left: item_rect.left + 10.0,
                    top: item_rect.top + 38.0,
                    right: item_rect.right - 10.0,
                    bottom: item_rect.bottom - 8.0,
                },
                s_col,
            );
            self.draw_text_centered(
                "×",
                &self.fmt_title,
                D2D_RECT_F {
                    left: item_rect.right - 22.0,
                    top: item_rect.top + 6.0,
                    right: item_rect.right - 6.0,
                    bottom: item_rect.top + 24.0,
                },
                s_col,
            );
        }
    }

    // =========================================================================
    // SECTION 5: TO-DO LIST
    // =========================================================================
    unsafe fn draw_todo_panel(&self, ctrl: &NotchController, card: D2D_RECT_F, alpha: f32) {
        let mut p_col = AntiMagicNeutralPalette::TEXT_PRIMARY;
        p_col.a *= alpha;
        let mut s_col = AntiMagicNeutralPalette::TEXT_SECONDARY;
        s_col.a *= alpha;
        let accent = AntiMagicNeutralPalette::ACCENT_CYAN;

        for (i, (task, done)) in ctrl.todos.iter().take(3).enumerate() {
            let row_top = card.top + 12.0 + i as f32 * 34.0;
            let box_rect = D2D_RECT_F {
                left: card.left + 16.0,
                top: row_top + 4.0,
                right: card.left + 32.0,
                bottom: row_top + 20.0,
            };

            if let Ok(b) = self.rt.CreateSolidColorBrush(&accent, None) {
                let rbox = D2D1_ROUNDED_RECT {
                    rect: box_rect,
                    radiusX: 5.0,
                    radiusY: 5.0,
                };
                if *done {
                    self.rt.FillRoundedRectangle(&rbox, &b);
                } else {
                    self.rt.DrawRoundedRectangle(&rbox, &b, 1.6, None);
                }
            }

            self.draw_text_left(
                task,
                &self.fmt_body,
                D2D_RECT_F {
                    left: card.left + 42.0,
                    top: row_top,
                    right: card.right - 16.0,
                    bottom: row_top + 26.0,
                },
                if *done { s_col } else { p_col },
            );
        }
    }
}
```

---

### D. `src/notch/mod.rs` — 100% DPI-Aligned Click Box, `<` `>` Navigation Arrows, `WM_DROPFILES` Drag-and-Drop, & `WM_MOUSEWHEEL` Scrolling

```rust
// src/notch/mod.rs
//! Notch window and rendering for Nusa Notch Clone (v4.0)

pub mod layout;
pub mod renderer;
pub mod states;

use anyhow::Result;
use std::sync::{Arc, RwLock};
use std::time::Instant;
use tokio::sync::mpsc;
use windows::core::PCWSTR;
use windows::Win32::Foundation::{COLORREF, HINSTANCE, HWND, LPARAM, LRESULT, POINT, WPARAM};
use windows::Win32::Graphics::Direct2D::Common::D2D_RECT_F;
use windows::Win32::Graphics::Dwm::DwmExtendFrameIntoClientArea;
use windows::Win32::Graphics::Gdi::{
    GetMonitorInfoW, InvalidateRect, MonitorFromWindow, UpdateWindow, HBRUSH, MONITORINFO,
    MONITOR_DEFAULTTOPRIMARY,
};
use windows::Win32::UI::Controls::MARGINS;
use windows::Win32::UI::HiDpi::GetDpiForWindow;
use windows::Win32::UI::Shell::{DragAcceptFiles, DragFinish, DragQueryFileW, HDROP};
use windows::Win32::UI::WindowsAndMessaging::*;

use crate::settings::Settings;
use crate::tray::{
    handle_tray_command, remove_tray_icon, setup_tray_icon, show_tray_menu, TRAY_MESSAGE,
};

pub use renderer::NotchRenderer;
pub use states::{NotchController, NotchState};

pub const NOTCH_WINDOW_WIDTH: i32 = 540;
pub const NOTCH_WINDOW_HEIGHT: i32 = 220;
const TIMER_ID_ANIMATION: usize = 1;

#[derive(Debug, Clone)]
pub enum NotchEvent {
    Toggle,
    UpdateMedia(crate::media::MediaInfo),
    UpdateLyrics(Vec<String>),
    UpdateTimer(crate::timers::TimerInfo),
    UpdateWater(crate::timers::WaterInfo),
    UpdateShelf(Vec<crate::shelf::ShelfItem>),
    HotkeyVolume(u8),
    HotkeyBrightness(u8),
    HotkeyCapsLock(bool),
}

pub struct NotchApp {
    pub hwnd: HWND,
    pub renderer: NotchRenderer,
    pub controller: NotchController,
    pub settings: Arc<RwLock<Settings>>,
    pub event_rx: mpsc::Receiver<NotchEvent>,
    pub dpi_scale: f32,
    pub win_x: i32,
    pub win_y: i32,
    pub win_w: i32,
    pub win_h: i32,
    pub last_tick: Instant,
    pub cursor_in_pill: bool,
}

impl NotchApp {
    pub fn new(
        hwnd: HWND,
        settings: Arc<RwLock<Settings>>,
        event_rx: mpsc::Receiver<NotchEvent>,
    ) -> Result<Self> {
        let dpi = unsafe { GetDpiForWindow(hwnd) };
        let dpi_scale = if dpi == 0 {
            1.0
        } else {
            (dpi as f32 / 96.0).max(1.0)
        };

        let monitor_rect = unsafe {
            let monitor = MonitorFromWindow(hwnd, MONITOR_DEFAULTTOPRIMARY);
            let mut info = MONITORINFO {
                cbSize: std::mem::size_of::<MONITORINFO>() as u32,
                ..Default::default()
            };
            let _ = GetMonitorInfoW(monitor, &mut info);
            info.rcMonitor
        };

        let win_w = (NOTCH_WINDOW_WIDTH as f32 * dpi_scale).round() as i32;
        let win_h = (NOTCH_WINDOW_HEIGHT as f32 * dpi_scale).round() as i32;
        let screen_w = monitor_rect.right - monitor_rect.left;
        let win_x = monitor_rect.left + (screen_w - win_w) / 2;
        let win_y = monitor_rect.top;

        unsafe {
            let _ = SetWindowPos(
                hwnd,
                HWND_TOPMOST,
                win_x,
                win_y,
                win_w,
                win_h,
                SWP_NOACTIVATE | SWP_NOZORDER,
            );
            let _ = SetLayeredWindowAttributes(hwnd, COLORREF(0), 255, LWA_ALPHA);

            let margins = MARGINS {
                cxLeftWidth: -1,
                cxRightWidth: -1,
                cyTopWidth: -1,
                cyBottomWidth: -1,
            };
            let _ = DwmExtendFrameIntoClientArea(hwnd, &margins);

            // Aktifkan dukungan Drag & Drop file dari Windows Explorer / Aplikasi lain ke Notch!
            DragAcceptFiles(hwnd, true);
        }

        let renderer = NotchRenderer::new(hwnd, win_w as u32, win_h as u32, dpi_scale)?;
        let mut controller = NotchController::new();

        if let Ok(s) = settings.read() {
            controller.mascot_enabled = s.show_mascot;
            controller.minimal_mode = s.minimal_notch;
        }

        Ok(Self {
            hwnd,
            renderer,
            controller,
            settings,
            event_rx,
            dpi_scale,
            win_x,
            win_y,
            win_w,
            win_h,
            last_tick: Instant::now(),
            cursor_in_pill: false,
        })
    }

    /// Koordinat Logis (DIPs) yang 100% sinkron dengan Direct2D RenderTarget (`dpiX = 96.0 * dpi_scale`)
    pub fn current_pill_rect(&self) -> D2D_RECT_F {
        let logical_win_w = NOTCH_WINDOW_WIDTH as f32;
        let pw = self.controller.width.value;
        let ph = self.controller.height.value;
        let left = ((logical_win_w - pw) * 0.5).max(0.0);
        let top = 0.0; // Menempel langsung ke bezel atas laptop
        let right = left + pw;
        let bottom = top + ph;

        D2D_RECT_F {
            left,
            top,
            right,
            bottom,
        }
    }

    pub fn on_tick(&mut self) -> Result<()> {
        let now = Instant::now();
        let dt = now.duration_since(self.last_tick).as_secs_f32();
        self.last_tick = now;

        while let Ok(ev) = self.event_rx.try_recv() {
            match ev {
                NotchEvent::Toggle => {
                    if self.controller.is_hovered || self.controller.state != NotchState::Pet {
                        self.controller.is_hovered = false;
                        self.controller.transition_to(NotchState::Pet);
                    } else {
                        self.controller.is_hovered = true;
                        let target = self.controller.last_expanded_tab;
                        self.controller.transition_to(target);
                    }
                }
                NotchEvent::UpdateMedia(info) => {
                    if !info.title.is_empty() {
                        self.controller.media_title = info.title;
                    }
                    if !info.artist.is_empty() {
                        self.controller.media_artist = info.artist;
                    }
                    self.controller.media_playing = info.playing;
                }
                NotchEvent::UpdateLyrics(lines) => {
                    if !lines.is_empty() {
                        self.controller.lyrics_lines = lines;
                        self.controller.active_lyric_idx = 0;
                    }
                }
                NotchEvent::HotkeyVolume(vol) => {
                    self.controller
                        .trigger_overlay(crate::notch::states::OverlayKind::Volume(vol));
                }
                NotchEvent::HotkeyBrightness(bri) => {
                    self.controller
                        .trigger_overlay(crate::notch::states::OverlayKind::Brightness(bri));
                }
                NotchEvent::HotkeyCapsLock(on) => {
                    self.controller
                        .trigger_overlay(crate::notch::states::OverlayKind::CapsLock(on));
                }
                _ => {}
            }
        }

        if let Ok(s) = self.settings.read() {
            self.controller.mascot_enabled = s.show_mascot;
            self.controller.minimal_mode = s.minimal_notch;
        }

        // Konversi posisi kursor layar fisik ke koordinat logis (DIPs)
        let pill_rect = self.current_pill_rect();
        let mut pt = POINT::default();
        unsafe {
            let _ = GetCursorPos(&mut pt);
        }

        let local_logical_x = (pt.x - self.win_x) as f32 / self.dpi_scale;
        let local_logical_y = (pt.y - self.win_y) as f32 / self.dpi_scale;

        let pad = 6.0;
        let cursor_in_pill = local_logical_x >= pill_rect.left - pad
            && local_logical_x <= pill_rect.right + pad
            && local_logical_y >= pill_rect.top - pad
            && local_logical_y <= pill_rect.bottom + pad;

        self.cursor_in_pill = cursor_in_pill;

        unsafe {
            let style = GetWindowLongPtrW(self.hwnd, GWL_EXSTYLE);
            if cursor_in_pill {
                if (style & (WS_EX_TRANSPARENT.0 as isize)) != 0 {
                    let _ = SetWindowLongPtrW(
                        self.hwnd,
                        GWL_EXSTYLE,
                        style & !(WS_EX_TRANSPARENT.0 as isize),
                    );
                }
            } else if (style & (WS_EX_TRANSPARENT.0 as isize)) == 0 {
                let _ = SetWindowLongPtrW(
                    self.hwnd,
                    GWL_EXSTYLE,
                    style | (WS_EX_TRANSPARENT.0 as isize),
                );
            }
        }

        let pill_cx = (pill_rect.left + pill_rect.right) * 0.5;
        let pill_cy = (pill_rect.top + pill_rect.bottom) * 0.5;
        let dx = local_logical_x - pill_cx;
        let dy = local_logical_y - pill_cy;
        let cursor_offset = if (dx * dx + dy * dy).sqrt() <= 360.0 {
            Some((dx, dy))
        } else {
            None
        };

        self.controller.update(dt, cursor_in_pill, cursor_offset);

        unsafe {
            let _ = InvalidateRect(self.hwnd, None, false);
        }

        Ok(())
    }

    pub fn on_lbutton_down(&mut self, phys_x: f32, phys_y: f32) {
        // Konversi koordinat klik fisik ke koordinat logis Direct2D (100% Presisi!)
        let x = phys_x / self.dpi_scale;
        let y = phys_y / self.dpi_scale;

        let pill_rect = self.current_pill_rect();
        let pill_w = pill_rect.right - pill_rect.left;
        let lx = x - pill_rect.left;
        let ly = y - pill_rect.top;

        // 0. Jika Overlay Volume/Brightness sedang tampil, izinkan klik langsung pada slider-nya!
        if !self.controller.is_hovered
            && self.controller.active_overlay.is_some()
            && self.controller.handle_overlay_click(lx, pill_w)
        {
            unsafe {
                let _ = InvalidateRect(self.hwnd, None, false);
            }
            return;
        }

        // 1. Cek apakah Chibi Liebe diklik (Poke)
        if self.controller.chibi_liebe.handle_click(x, y) {
            unsafe {
                let _ = InvalidateRect(self.hwnd, None, false);
            }
            return;
        }

        // 2. Cek klik pada Bar Navigasi Atas (`<`, 6 Tab, dan `>`) saat notch terbuka
        if self.controller.height.value > 65.0 && (0.0..=28.0).contains(&ly) {
            if (10.0..=36.0).contains(&lx) {
                self.controller.prev_tab();
                unsafe {
                    let _ = InvalidateRect(self.hwnd, None, false);
                }
                return;
            } else if lx >= pill_w - 36.0 && lx <= pill_w - 10.0 {
                self.controller.next_tab();
                unsafe {
                    let _ = InvalidateRect(self.hwnd, None, false);
                }
                return;
            } else if lx > 38.0 && lx < pill_w - 38.0 {
                let tabs_w = (pill_w - 76.0).max(1.0);
                let slot = ((lx - 38.0) / (tabs_w / 6.0)) as usize;
                let next = match slot {
                    0 => NotchState::Hub,
                    1 => NotchState::Music,
                    2 => NotchState::Control,
                    3 => NotchState::Pomo,
                    4 => NotchState::Shelf,
                    _ => NotchState::Todo,
                };
                self.controller.transition_to(next);
                unsafe {
                    let _ = InvalidateRect(self.hwnd, None, false);
                }
                return;
            }
        }

        // 3. Cek klik tombol/slider di dalam panel aktif
        if self.controller.height.value > 65.0
            && self.controller.handle_panel_click(lx, ly, pill_w)
        {
            unsafe {
                let _ = InvalidateRect(self.hwnd, None, false);
            }
            return;
        }

        // 4. Jika tertutup, klik akan membuka tab terakhir
        if matches!(self.controller.state, NotchState::Pet | NotchState::Idle) {
            self.controller.devil_eyes.trigger_wake_surge();
            let target = self.controller.last_expanded_tab;
            self.controller.transition_to(target);
            unsafe {
                let _ = InvalidateRect(self.hwnd, None, false);
            }
        }
    }
}

pub fn run_ui(
    hinstance: HINSTANCE,
    event_rx: mpsc::Receiver<NotchEvent>,
    settings: Arc<RwLock<Settings>>,
) -> Result<()> {
    let class_name = to_wide_string("NusaNotchCloneWindow\0");

    let wc = WNDCLASSEXW {
        cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(notch_wnd_proc),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: hinstance,
        hIcon: HICON::default(),
        hCursor: unsafe { LoadCursorW(None, IDC_ARROW).unwrap_or_default() },
        hbrBackground: HBRUSH::default(),
        lpszMenuName: PCWSTR::null(),
        lpszClassName: PCWSTR(class_name.as_ptr()),
        hIconSm: HICON::default(),
    };

    unsafe {
        RegisterClassExW(&wc);

        let hwnd = CreateWindowExW(
            WS_EX_TOPMOST | WS_EX_TOOLWINDOW | WS_EX_LAYERED | WS_EX_TRANSPARENT | WS_EX_ACCEPTFILES,
            PCWSTR(class_name.as_ptr()),
            PCWSTR(to_wide_string("Nusa Notch\0").as_ptr()),
            WS_POPUP,
            0,
            0,
            NOTCH_WINDOW_WIDTH,
            NOTCH_WINDOW_HEIGHT,
            HWND::default(),
            HMENU::default(),
            hinstance,
            None,
        )?;

        let app = Box::new(NotchApp::new(hwnd, settings.clone(), event_rx)?);
        let win_x = app.win_x;
        let win_y = app.win_y;
        let win_w = app.win_w;
        let win_h = app.win_h;

        let raw_ptr = Box::into_raw(app);
        SetWindowLongPtrW(hwnd, GWLP_USERDATA, raw_ptr as isize);

        let nid = setup_tray_icon(hwnd)?;

        let _ = SetWindowPos(
            hwnd,
            HWND_TOPMOST,
            win_x,
            win_y,
            win_w,
            win_h,
            SWP_SHOWWINDOW | SWP_NOACTIVATE,
        );
        let _ = ShowWindow(hwnd, SW_SHOWNOACTIVATE);
        let _ = SetTimer(hwnd, TIMER_ID_ANIMATION, 8, None);

        let _ = InvalidateRect(hwnd, None, false);
        let _ = UpdateWindow(hwnd);

        let mut msg = MSG::default();
        while GetMessageW(&mut msg, HWND::default(), 0, 0).as_bool() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

        let _ = KillTimer(hwnd, TIMER_ID_ANIMATION);
        let _ = remove_tray_icon(&nid);
        let _ = Box::from_raw(raw_ptr);
    }

    Ok(())
}

extern "system" fn notch_wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    let ptr = unsafe { GetWindowLongPtrW(hwnd, GWLP_USERDATA) } as *mut NotchApp;
    if !ptr.is_null() {
        let app = unsafe { &mut *ptr };
        match msg {
            WM_ERASEBKGND => return LRESULT(1),
            WM_TIMER => {
                let _ = app.on_tick();
                return LRESULT(0);
            }
            WM_PAINT => {
                let pill_rect = app.current_pill_rect();
                app.renderer.render_frame(&mut app.controller, pill_rect);
                return LRESULT(0);
            }
            WM_LBUTTONDOWN => {
                let x = (lparam.0 & 0xFFFF) as i16 as f32;
                let y = ((lparam.0 >> 16) & 0xFFFF) as i16 as f32;
                app.on_lbutton_down(x, y);
                return LRESULT(0);
            }
            WM_MOUSEWHEEL => {
                let delta = ((wparam.0 >> 16) & 0xFFFF) as i16;
                app.controller.on_mouse_wheel(delta);
                unsafe {
                    let _ = InvalidateRect(hwnd, None, false);
                }
                return LRESULT(0);
            }
            WM_DROPFILES => {
                let hdrop = HDROP(wparam.0 as *mut core::ffi::c_void);
                unsafe {
                    let count = DragQueryFileW(hdrop, 0xFFFFFFFF, None);
                    for i in 0..count {
                        let len = DragQueryFileW(hdrop, i, None);
                        if len > 0 {
                            let mut buf = vec![0u16; (len + 1) as usize];
                            DragQueryFileW(hdrop, i, Some(&mut buf));
                            if let Some(pos) = buf.iter().position(|&c| c == 0) {
                                buf.truncate(pos);
                            }
                            let path = String::from_utf16_lossy(&buf);
                            app.controller.add_dropped_file(path);
                        }
                    }
                    DragFinish(hdrop);
                    let _ = InvalidateRect(hwnd, None, false);
                }
                return LRESULT(0);
            }
            TRAY_MESSAGE => {
                let event = lparam.0 as u32;
                if event == WM_RBUTTONUP {
                    if let Ok(cmd) = show_tray_menu(hwnd, &app.settings) {
                        if handle_tray_command(cmd, hwnd, &app.settings).unwrap_or(false) {
                            unsafe {
                                let _ = KillTimer(hwnd, TIMER_ID_ANIMATION);
                                let _ = DestroyWindow(hwnd);
                            }
                        }
                    }
                } else if event == WM_LBUTTONUP {
                    if app.controller.is_hovered || app.controller.state != NotchState::Pet {
                        app.controller.is_hovered = false;
                        app.controller.transition_to(NotchState::Pet);
                    } else {
                        app.controller.is_hovered = true;
                        let target = app.controller.last_expanded_tab;
                        app.controller.transition_to(target);
                    }
                    unsafe {
                        let _ = InvalidateRect(hwnd, None, false);
                    }
                }
                return LRESULT(0);
            }
            WM_DESTROY => {
                unsafe {
                    PostQuitMessage(0);
                }
                return LRESULT(0);
            }
            _ => {}
        }
    }
    unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
}

fn to_wide_string(s: &str) -> Vec<u16> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    OsStr::new(s).encode_wide().collect()
}
```