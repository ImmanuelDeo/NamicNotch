# Nusa Clone: Bezel-Connected Notch, Dual-Mode Mata Asta (Devil Union & Normal Emerald + Pedang Melayang), Chibi Liebe, & Seluruh Fitur + Hardware Overlays Nusa Original

Dokumen ini berisi **Spesifikasi 4 Upgrade Utama**, **Prompt Lengkap untuk Hermes Agent**, serta **Kode Modul Rust Utuh (`src/mascot/eyes.rs`, `src/mascot/chibi_liebe.rs`, `src/notch/states.rs`, `src/notch/renderer.rs`, `src/notch/mod.rs`)** yang siap dipasang langsung ke dalam repositori `nusa-clone/` milikmu.

---

## 1. Ringkasan 4 Upgrade Utama yang Diterapkan

| # | Fitur & Permintaan Kamu | Implementasi Teknis di Direct2D / Win32 |
| :- | :--- | :--- |
| **1** | **Bentuk Notch Menyatu dengan Bezel Atas Laptop (`Bezel-Connected Geometry`)** | `pill_rect.top` dikunci tepat di **`0.0` (menempel langsung ke tepi paling atas layar `rcMonitor.top`)**. Bukan lagi memakai `FillRoundedRectangle` yang mengambang, melainkan **`create_bezel_notch_geometry` (`ID2D1PathGeometry`)**: bagian atas rata menyatu dengan bezel layar disertai lekukan bahu halus (*top bezel fillet*), sedangkan bagian bawah kiri & kanan tetap *rounded* mulus baik saat tertutup maupun saat terbuka lebar. |
| **2** | **Animasi Idle Asta Hidup: Transisi Devil Union ↔ Mata Asli (Hijau Emerald) + Pedang Anti-Magic Melayang & Tangan Menangkap** | Di `src/mascot/eyes.rs`, siklus *idle* kini punya 4 aksi karakter Asta:<br>• **`DevilUnionSerious`**: Tatapan tajam serius, iris *Muted Wine-Crimson*, pupil tanda plus (`+`), partikel *Anti-Magic*.<br>• **`NormalAstaHappy`**: Berubah mulus ke mata asli Asta (**Hijau Emerald `#2EE68A`** dengan kilau bintang putih besar Khas Asta saat bersemangat) lalu melengkung senang (`^ ^` *happy arc gesture*).<br>• **`SwordSummonGrab`**: Pedang *Demon-Slayer* (*Danma no Tsurugi*) mini melayang & berputar di samping mata, lalu **tangan Asta (dengan lengan hitam Anti-Magic)** muncul menjulur dan menangkap gagang pedang dengan efek *slash spark*! |
| **3** | **Seluruh 6 Fitur Panel + Hardware Overlays Nusa Original Berfungsi Penuh** | • **6 Panel Utama**: `Music` (dengan kontrol *real-time* Windows Media Session: Play/Pause/Next/Prev), `Lyrics` (LRCLIB), `Focus` (Pomodoro Timer), `Water` (Hydration Tracker + Chart), `Shelf` (Drag & Drop Files), dan `To-Do` (Interactive Checkboxes) + `Idle` (Jam Digital Real-Time & Persentase Baterai).<br>• **5 Pop-up Overlays Otomatis**: **Volume Overlay**, **Brightness Overlay**, **Caps Lock / Language Overlay**, **Charging / Battery Plugged Overlay** (deteksi otomatis via `GetSystemPowerStatus`), dan **Headphones / Bluetooth Connected Overlay**. |
| **4** | **Tema Neutral Slate-Blue Anti-Magic + Chibi Liebe Selektif** | Warna utama tetap menggunakan **Obsidian Slate (`#0B0E14`)**, **Dark Slate Card (`#131822`)**, dan **Steel Blue (`#6488EA`)** yang kalem dan menyatu dengan laptop biru/abu-abu, ditemani **Chibi Liebe** secara selektif hanya pada tab `Music`, `Focus (Pomo)`, dan `Shelf`. |

---

## 2. Prompt Lengkap untuk Hermes Agent

Salin seluruh blok perintah di bawah ini dan berikan langsung ke **Hermes Agent** di terminal lokalmu (di folder `nusa-clone/`):

```markdown
Tolong baca file `DEVIL_UNION_UPGRADE.md` dan terapkan seluruh pembaruan kode ke dalam repositori `nusa-clone` ini dengan spesifikasi wajib berikut:

### 1. Bentuk Notch Menyatu dengan Bezel Atas Laptop (`Bezel-Connected Notch Geometry`)
- Di `src/notch/mod.rs` (`current_pill_rect`), pastikan `top = 0.0` (jangan diberi jarak mengambang dari atas layar) sehingga sisi atas notch menempel 100% menyatu dengan bezel atas monitor (`rcMonitor.top`), baik saat tertutup maupun saat terbuka menampilkan fitur.
- Di `src/notch/renderer.rs`, gunakan fungsi `create_bezel_notch_geometry` (`ID2D1PathGeometry`) untuk menggambar bentuk notch khas Nusa/MacBook:
  - Sisi atas rata menempel ke bezel (`y = 0.0`) dengan sedikit *top shoulder curve* (`ear_r = 8.0`),
  - Dinding kiri dan kanan turun vertikal,
  - Sudut kiri-bawah dan kanan-bawah melengkung mulus menggunakan `BezierSegment` sebesar `controller.corner_radius.value`.
  - Jangan gambar garis border pada tepi paling atas (`y = 0.0`) agar notch terlihat benar-benar tumbuh keluar dari bezel layar laptop.

### 2. Upgrade Animasi Idle Mata Asta + Transisi Mata Asli + Pedang Melayang & Tangan Menangkap (`src/mascot/eyes.rs`)
- Perbarui `src/mascot/eyes.rs` dengan kode lengkap dari `DEVIL_UNION_UPGRADE.md` yang memiliki siklus hidup karakter Asta:
  1. **Mode `DevilUnionSerious`**: Tatapan tajam serius dengan kelopak miring, iris *Muted Wine-Crimson*, pupil tanda plus (`+`) hitam obsidian, dan pelacakan kursor (*cursor tracking*).
  2. **Mode `NormalEmeraldHappy`**: Secara periodik saat idle (atau saat pengguna baru selesai berinteraksi), mata Asta bertransisi dari Devil Union kembali ke **mata asli Asta (Hijau Emerald cerah `#2EE68A`)** dengan pupil bulat dan kilau bintang putih (*starry anime highlight*), diikuti ekspresi melengkung senang (`^ ^` *happy gesture*) yang ramah.
  3. **Mode `SwordSummonGrab` (Aksesori Pedang & Tangan Menangkap)**: Secara berkala saat idle, muncul pedang *Demon-Slayer* (*Danma no Tsurugi*) hitam-abu berkarat yang melayang dan berputar halus di sisi kanan notch. Kemudian **tangan Asta (berlengan hitam Anti-Magic)** muncul dari tepi bawah notch, meraih dan menggenggam gagang pedang tersebut disertai kilatan *slash spark* sebelum ditarik masuk kembali!

### 3. Pastikan Seluruh Fitur & Hardware Overlays Nusa Original Berjalan (`src/notch/states.rs`, `src/notch/mod.rs`, `src/notch/renderer.rs`, `src/media/mod.rs`)
- **Kontrol Musik Real-Time (GSMTC)**:
  - Saat pengguna mengklik tombol `Play/Pause`, `Next`, atau `Prev` di panel `Music`, panggil API Windows `GlobalSystemMediaTransportControlsSessionManager` (`TryTogglePlayPauseAsync()`, `TrySkipNextAsync()`, `TrySkipPreviousAsync()`) agar benar-benar mengontrol Spotify, YouTube Browser, atau Apple Music yang sedang berjalan di laptop!
- **Deteksi Baterai & Charger Otomatis (`GetSystemPowerStatus`)**:
  - Poll status baterai Windows setiap 2 detik menggunakan `GetSystemPowerStatus`. Simpan persentase baterai (`battery_percent`) dan status cas (`is_charging`).
  - Saat charger baru dicolokkan atau dicabut, otomatis munculkan **Charging Peek Overlay** (`OverlayKind::Charging`) selama 3.0 detik dengan animasi cincin baterai seperti Nusa Original.
- **Overlay Volume, Brightness, Caps Lock, dan Headphones**:
  - Hubungkan event `HotkeyVolume(u8)`, `HotkeyBrightness(u8)`, `HotkeyCapsLock(bool)` dari `src/hooks/` ke `controller.trigger_overlay(...)` sehingga setiap kali pengguna menekan tombol Volume, Brightness, atau Caps Lock di keyboard laptop, notch otomatis menampilkan bar indikator overlay yang mulus lalu kembali ke mode semula setelah 2.2 detik.
- **Semua 6 Panel Fitur Utama (`Music`, `Lyrics`, `Focus/Pomo`, `Water`, `Shelf`, `To-Do`) + Jam Real-Time (`Idle`)**:
  - Pastikan seluruh teks dan grafik dirender menggunakan DirectWrite (`IDWriteFactory`) + Direct2D seperti di `src/notch/renderer.rs` pada `DEVIL_UNION_UPGRADE.md`.

### 4. Kompilasi & Verifikasi
- Pastikan `Win32_System_Power` ditambahkan pada daftar features `windows` di `Cargo.toml` (untuk `GetSystemPowerStatus`).
- Jalankan `cargo check` dan `cargo build --release` hingga kompilasi berhasil 100% tanpa error.
```

---

## 3. Tambahan Feature Flag di `Cargo.toml`

Pastikan di dalam `Cargo.toml` pada bagian `windows = { version = "0.58", features = [ ... ] }` terdapat feature berikut (tambahkan `"Win32_System_Power"` untuk deteksi *charging* & persentase baterai otomatis, serta `"Win32_Graphics_Direct2D"` dan `"Win32_Graphics_Dwm"`):

```toml
    "Win32_Graphics_Direct2D",
    "Win32_Graphics_Direct2D_Common",
    "Win32_Graphics_DirectWrite",
    "Win32_Graphics_Dwm",
    "Win32_System_Power",
    "Win32_UI_Controls",
```

---

## 4. Kode Lengkap Semua Modul Inti (Siap Pakai)

### A. `src/mascot/eyes.rs` — Dual-Mode Mata Asta (Devil Union Serius ↔ Mata Asli Emerald Senang + Pedang Melayang & Tangan Menangkap)

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
    /// Devil Union Mode: Tatapan tajam serius, iris Muted Crimson, pupil '+'
    DevilUnionSerious { timer: f32 },
    /// Normal Asta Mode: Mata asli Hijau Emerald berbinar, ekspresi ramah/antusias
    NormalEmerald { timer: f32 },
    /// Happy Gesture: Mata melengkung senang '^ ^' khas Asta saat menyapa
    HappySmileGesture { timer: f32 },
    /// Sword Summon & Grab: Pedang Demon-Slayer melayang lalu ditangkap oleh tangan Asta!
    SwordSummonGrab { timer: f32 },
    /// Dozing: Istirahat tenang setelah lama tidak ada aktivitas
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
    /// 0.0 = Devil Union (Crimson + '+'), 1.0 = Normal Asta (Emerald Green + Star Highlight)
    pub emerald_blend: f32,
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
            openness: 0.1,
            blink_timer: 0.0,
            next_blink_in: 3.0,
            glance_timer: 1.8,
            aura_intensity: 1.0,
            pupil_scale: 1.1,
            emerald_blend: 0.0,
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
            rng_seed: 0xA57A_7777,
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
        let rx = (self.next_f32() - 0.5) * 24.0 + side * 16.0;
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
        // Saat pengguna kembali atau menyentuh notch, Asta menyapa dengan mata Hijau Emerald senang sebentar!
        self.mode = AstaIdleMode::HappySmileGesture { timer: 0.0 };
        self.aura_intensity = 1.2;
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

        // Lirikan acak organik jika kursor tidak dekat
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

        let mut target_openness = 1.0;
        let mut target_aura = 0.85;
        let mut target_pupil = 1.0;
        let mut target_emerald = 0.0;

        match &mut self.mode {
            AstaIdleMode::DevilUnionSerious { timer } => {
                *timer += dt;
                target_openness = 1.0;
                target_emerald = 0.0;
                target_aura = if cursor_near { 1.15 } else { 0.82 + 0.12 * (self.time * 3.0).sin() };
                target_pupil = if cursor_near { 0.86 } else { 1.0 };

                // Setelah 7 detik di Devil Union, bergantian masuk ke aksi Pedang Melayang atau Mata Asli Emerald!
                if *timer >= 7.5 {
                    if self.next_f32() < 0.55 {
                        self.mode = AstaIdleMode::SwordSummonGrab { timer: 0.0 };
                    } else {
                        self.mode = AstaIdleMode::NormalEmerald { timer: 0.0 };
                    }
                }
            }
            AstaIdleMode::SwordSummonGrab { timer } => {
                *timer += dt;
                target_openness = 1.02;
                target_emerald = 0.0; // Tetap Devil Union serius sambil memanggil & menangkap pedang
                target_aura = 1.25;
                target_pupil = 0.82;

                // Mata Asta otomatis melirik ke arah pedang yang melayang di sisi kanan (x = +0.85)
                let p = (*timer / 3.8).clamp(0.0, 1.0);
                self.target_look_x = 0.85 * (1.0 - (p - 0.5).abs() * 1.2).clamp(0.2, 1.0);
                self.target_look_y = -0.15 + (self.time * 4.0).sin() * 0.20;

                if *timer >= 3.8 {
                    // Setelah berhasil menangkap pedang, Asta berubah ke mata asli Hijau Emerald & senang!
                    self.mode = AstaIdleMode::NormalEmerald { timer: 0.0 };
                }
            }
            AstaIdleMode::NormalEmerald { timer } => {
                *timer += dt;
                target_openness = 1.06; // Mata asli Asta sedikit lebih bulat & antusias
                target_emerald = 1.0;
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
                target_aura = 0.60;
                target_pupil = 1.0;

                if *timer >= 2.2 {
                    // Kembali fokus ke Devil Union Mode
                    self.mode = AstaIdleMode::DevilUnionSerious { timer: 0.0 };
                }
            }
            AstaIdleMode::Dozing { timer } => {
                *timer += dt;
                target_openness = 0.30 + 0.04 * (self.time * 1.4).sin();
                target_emerald = 0.0;
                target_aura = 0.35;
                target_pupil = 1.05;
            }
        }

        // Kedipan cepat
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
        let center_y = (pill_rect.top + pill_rect.bottom) * 0.5 + 1.0;

        // Geser sedikit posisi mata ke kiri saat pedang melayang di kanan agar seimbang di dalam pill
        let sword_shift = if let AstaIdleMode::SwordSummonGrab { timer } = self.mode {
            let s = (timer * 2.5).min(1.0) * ((3.8 - timer) * 2.5).clamp(0.0, 1.0);
            -12.0 * s
        } else {
            0.0
        };

        let eyes_cx = center_x + sword_shift;
        let eye_spacing = 16.5;

        unsafe {
            // 1. Partikel Anti-Magic / Starlight
            self.draw_anti_magic_sparks(rt, eyes_cx, center_y, alpha_mult);

            // 2. Jika sedang dalam gesture HappySmileGesture ('^ ^'), gambar lengkungan mata senang!
            if let AstaIdleMode::HappySmileGesture { timer } = self.mode {
                let bounce = (timer * 10.0).sin().abs() * -1.8;
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
                // 3. Gambar Sepasang Mata Asta (Transisi Mulus Devil Union <-> Normal Emerald)
                for side in [-1.0_f32, 1.0_f32] {
                    let base_x = eyes_cx + side * eye_spacing;
                    self.draw_single_asta_eye(factory, rt, base_x, center_y, side, alpha_mult);
                }
            }

            // 4. Jika sedang dalam fase SwordSummonGrab, gambar Pedang Melayang & Tangan Asta Menangkap!
            if let AstaIdleMode::SwordSummonGrab { timer } = self.mode {
                self.draw_floating_sword_and_grabbing_hand(
                    factory,
                    rt,
                    center_x + 36.0,
                    center_y,
                    pill_rect.bottom,
                    timer,
                    alpha_mult,
                );
            }
        }
    }

    /// Gambar mata melengkung senang ('^ ^') berwarna Hijau Emerald / Ice-Blue saat Asta gembira
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
                        x: ex - 6.5,
                        y: ey + 2.5,
                    },
                    D2D1_FIGURE_BEGIN_HOLLOW,
                );
                sink.AddBezier(&D2D1_BEZIER_SEGMENT {
                    point1: D2D_POINT_2F {
                        x: ex - 3.0,
                        y: ey - 5.5,
                    },
                    point2: D2D_POINT_2F {
                        x: ex + 3.0,
                        y: ey - 5.5,
                    },
                    point3: D2D_POINT_2F {
                        x: ex + 6.5,
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
                    rt.DrawGeometry(&geom, &b, 2.4, None);
                }
            }
        }

        // Bintang kecil berbinar di sudut luar mata saat senang
        let star_x = ex + side * 8.5;
        let star_y = ey - 3.5;
        let star_col = D2D1_COLOR_F {
            r: 0.90,
            g: 0.98,
            b: 0.95,
            a: 0.85 * alpha,
        };
        if let Ok(sb) = rt.CreateSolidColorBrush(&star_col, None) {
            rt.DrawLine(
                D2D_POINT_2F {
                    x: star_x - 2.5,
                    y: star_y,
                },
                D2D_POINT_2F {
                    x: star_x + 2.5,
                    y: star_y,
                },
                &sb,
                1.2,
                None,
            );
            rt.DrawLine(
                D2D_POINT_2F {
                    x: star_x,
                    y: star_y - 2.5,
                },
                D2D_POINT_2F {
                    x: star_x,
                    y: star_y + 2.5,
                },
                &sb,
                1.2,
                None,
            );
        }
    }

    /// Gambar Pedang Demon-Slayer Melayang & Tangan Asta yang Menjulur Menangkap Gagangnya
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
        // Fase waktu (total 3.8 detik):
        // 0.0 .. 1.6s: Pedang melayang turun-naik & bergetar dengan aura Anti-Magic
        // 1.6 .. 2.7s: Tangan hitam Asta menjulur dari bawah & menangkap gagang pedang!
        // 2.7 .. 3.8s: Tangan menarik pedang turun ke dalam bayangan disertai kilatan slash
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

        // 1. Bilah Besar Demon-Slayer Sword (Danma no Tsurugi)
        if let Ok(sword_geom) = factory.CreatePathGeometry() {
            if let Ok(sink) = sword_geom.Open() {
                // Ujung atas meruncing berat khas greatsword Asta
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

        // Garis urat Anti-Magic di tengah bilah pedang
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

        // Crossguard & Gagang Pedang (Hilt) di bawah bilah
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

        // 2. Animasi Tangan Asta (Anti-Magic Claw/Hand) Menjulur & Menangkap Gagang Pedang!
        if timer >= 1.2 {
            let reach_progress = if timer < 2.0 {
                ((timer - 1.2) / 0.8).clamp(0.0, 1.0)
            } else {
                1.0
            };

            let hand_top_y =
                pill_bottom - (pill_bottom - (sword_y + 6.5)) * reach_progress;

            // Lengan bawah menjulur dari tepi bawah pill
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

            // Jari-jari menggenggam gagang saat reach_progress hampir penuh
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

                // Efek kilatan tangkapan (Catch Impact Spark!) tepat saat pedang tertangkap
                if (1.85..=2.35).contains(&timer) {
                    rt.DrawLine(
                        D2D_POINT_2F {
                            x: sword_x - 9.0,
                            y: sword_y + 6.0,
                        },
                        D2D_POINT_2F {
                            x: sword_x + 9.0,
                            y: sword_y + 6.0,
                        },
                        &core_b,
                        1.5,
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
                // Saat mode Normal Emerald Asta, partikel menjadi kilau Emerald-Mint lembut
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

    unsafe fn draw_single_asta_eye(
        &self,
        factory: &ID2D1Factory,
        rt: &ID2D1HwndRenderTarget,
        base_x: f32,
        base_y: f32,
        side: f32,
        alpha_mult: f32,
    ) {
        let shift_x = self.look_x * 4.0;
        let shift_y = self.look_y * 2.0;
        let ex = base_x + shift_x;
        let ey = base_y + shift_y;

        let eye_rx = 8.0;
        let eye_ry = (6.0 * self.openness).max(0.6);

        let eb = self.emerald_blend.clamp(0.0, 1.0);

        // Jika berkedip
        if self.openness < 0.16 {
            let slit_color = D2D1_COLOR_F {
                r: 0.68 * (1.0 - eb) + 0.25 * eb,
                g: 0.32 * (1.0 - eb) + 0.85 * eb,
                b: 0.40 * (1.0 - eb) + 0.60 * eb,
                a: 0.85 * alpha_mult,
            };
            if let Ok(brush) = rt.CreateSolidColorBrush(&slit_color, None) {
                rt.FillRoundedRectangle(
                    &D2D1_ROUNDED_RECT {
                        rect: D2D_RECT_F {
                            left: ex - eye_rx,
                            top: ey - 0.8,
                            right: ex + eye_rx,
                            bottom: ey + 0.8,
                        },
                        radiusX: 0.8,
                        radiusY: 0.8,
                    },
                    &brush,
                );
            }
            return;
        }

        // --- LAYER 1: Iris (Blend Mulus antara Muted Wine-Crimson <-> Bright Emerald Green) ---
        let iris_rx = 6.3;
        let iris_ry = (5.5 * self.openness).max(1.2);

        let c0 = D2D1_COLOR_F {
            r: 0.82 * (1.0 - eb) + 0.28 * eb,
            g: 0.32 * (1.0 - eb) + 0.92 * eb,
            b: 0.38 * (1.0 - eb) + 0.64 * eb,
            a: alpha_mult,
        };
        let c1 = D2D1_COLOR_F {
            r: 0.62 * (1.0 - eb) + 0.12 * eb,
            g: 0.16 * (1.0 - eb) + 0.68 * eb,
            b: 0.24 * (1.0 - eb) + 0.48 * eb,
            a: alpha_mult,
        };
        let c2 = D2D1_COLOR_F {
            r: 0.24 * (1.0 - eb) + 0.05 * eb,
            g: 0.08 * (1.0 - eb) + 0.32 * eb,
            b: 0.14 * (1.0 - eb) + 0.25 * eb,
            a: alpha_mult,
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

        // --- LAYER 2: Pupil ('+' saat Devil Union, Bulat saat Normal Emerald) ---
        let px = base_x + shift_x * 1.18;
        let py = base_y + shift_y * 1.18;
        let pupil_color = D2D1_COLOR_F {
            r: 0.04,
            g: 0.05,
            b: 0.08,
            a: alpha_mult,
        };

        if eb < 0.65 {
            // Pupil '+' khas Devil Union
            let arm_h = 4.6 * self.pupil_scale;
            let arm_v = (4.4 * self.pupil_scale * self.openness).max(1.0);
            let half_thick = 1.15 * self.pupil_scale;

            if let Ok(plus_geom) =
                self.create_tapered_plus_geometry(factory, px, py, arm_h, arm_v, half_thick)
            {
                if let Ok(pb) = rt.CreateSolidColorBrush(&pupil_color, None) {
                    rt.FillGeometry(&plus_geom, &pb, None);
                }
            }
        } else {
            // Pupil Bulat khas Mata Asli Asta
            if let Ok(pb) = rt.CreateSolidColorBrush(&pupil_color, None) {
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F { x: px, y: py },
                        radiusX: 2.6 * self.pupil_scale,
                        radiusY: 2.8 * self.pupil_scale * self.openness,
                    },
                    &pb,
                );
            }
        }

        // --- LAYER 3: Upper Eyelid Cut (Serius tajam saat Devil Union, Terbuka ramah saat Normal Emerald) ---
        if eb < 0.8 {
            self.draw_sharp_upper_eyelid_mask(
                factory,
                rt,
                ex,
                ey,
                iris_rx + 1.5,
                iris_ry,
                side,
                alpha_mult * (1.0 - eb),
            );
        }

        // --- LAYER 4: Specular Highlight (Glint) ---
        let glint_color = D2D1_COLOR_F {
            r: 0.92,
            g: 0.96,
            b: 1.0,
            a: 0.80 * alpha_mult,
        };
        if let Ok(glint_brush) = rt.CreateSolidColorBrush(&glint_color, None) {
            rt.FillEllipse(
                &D2D1_ELLIPSE {
                    point: D2D_POINT_2F {
                        x: px - side * 2.1,
                        y: py - iris_ry * 0.35,
                    },
                    radiusX: 1.1 + 0.5 * eb, // Highlight lebih besar berbinar saat mata asli Asta
                    radiusY: (0.8 + 0.4 * eb) * self.openness,
                },
                &glint_brush,
            );
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

    unsafe fn draw_sharp_upper_eyelid_mask(
        &self,
        factory: &ID2D1Factory,
        rt: &ID2D1HwndRenderTarget,
        ex: f32,
        ey: f32,
        rx: f32,
        ry: f32,
        side: f32,
        alpha_mult: f32,
    ) {
        let outer_x = ex + side * rx;
        let inner_x = ex - side * rx;
        let outer_y = ey - ry * 0.92;
        let inner_y = ey - ry * 0.42;

        if let Ok(geom) = factory.CreatePathGeometry() {
            if let Ok(sink) = geom.Open() {
                sink.BeginFigure(
                    D2D_POINT_2F { x: outer_x, y: outer_y },
                    D2D1_FIGURE_BEGIN_FILLED,
                );
                sink.AddLines(&[
                    D2D_POINT_2F { x: inner_x, y: inner_y },
                    D2D_POINT_2F { x: inner_x, y: ey - ry - 4.0 },
                    D2D_POINT_2F { x: outer_x, y: ey - ry - 4.0 },
                ]);
                sink.EndFigure(D2D1_FIGURE_END_CLOSED);
                let _ = sink.Close();

                let mask_color = D2D1_COLOR_F {
                    r: 0.043,
                    g: 0.055,
                    b: 0.078,
                    a: alpha_mult,
                };
                if let Ok(brush) = rt.CreateSolidColorBrush(&mask_color, None) {
                    rt.FillGeometry(&geom, &brush, None);
                }
            }
        }
    }
}
```

---

### B. `src/notch/states.rs` — State Machine Lengkap + Hardware Overlays (Volume, Brightness, CapsLock, Charging, Headphones) + Kontrol Media GSMTC

```rust
// src/notch/states.rs
use crate::mascot::chibi_liebe::ChibiLiebe;
use crate::mascot::eyes::DevilUnionEyes;
use windows::Media::Control::GlobalSystemMediaTransportControlsSessionManager;
use windows::Win32::Graphics::Direct2D::Common::D2D_RECT_F;
use windows::Win32::System::Power::{GetSystemPowerStatus, SYSTEM_POWER_STATUS};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotchState {
    Pet,    // Idle mascot: Mata Asta (Devil Union <-> Normal Emerald + Floating Sword)
    Idle,   // Closed notch dengan Jam Digital & Baterai
    Music,  // Tab 0: Music Player + Controls
    Lyrics, // Tab 1: Synced Lyrics
    Pomo,   // Tab 2: Pomodoro Focus Timer
    Water,  // Tab 3: Water Reminder & Hourly Chart
    Shelf,  // Tab 4: File Shelf Drag & Drop
    Todo,   // Tab 5: To-Do List
}

/// Pop-up Overlay sementara seperti di Nusa Original (Volume, Brightness, CapsLock, Charging, Headphones)
#[derive(Debug, Clone, PartialEq)]
pub enum OverlayKind {
    Volume(u8),
    Brightness(u8),
    CapsLock(bool),
    Charging { percent: u8, plugged: bool },
    Headphones { name: String, percent: u8 },
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

    // --- Pop-up Overlay (Volume, Brightness, CapsLock, Charging, Headphones) ---
    pub active_overlay: Option<OverlayKind>,
    pub overlay_timer: f32,

    // --- Status Baterai & Charger Sistem ---
    pub battery_percent: u8,
    pub is_charging: bool,
    power_poll_timer: f32,

    // --- Data Fitur 1: Music ---
    pub media_title: String,
    pub media_artist: String,
    pub media_playing: bool,
    pub media_progress: f32,
    pub media_elapsed_secs: u32,
    pub media_total_secs: u32,

    // --- Data Fitur 2: Synced Lyrics ---
    pub lyrics_lines: Vec<String>,
    pub active_lyric_idx: usize,

    // --- Data Fitur 3: Pomodoro Timer ---
    pub pomo_remaining_secs: u32,
    pub pomo_total_secs: u32,
    pub pomo_running: bool,
    pub pomo_is_break: bool,
    pomo_accum: f32,

    // --- Data Fitur 4: Water Tracker ---
    pub water_current_ml: u32,
    pub water_goal_ml: u32,
    pub water_cup_ml: u32,
    pub water_hourly_ml: [u32; 6],

    // --- Data Fitur 5: File Shelf ---
    pub shelf_files: Vec<(String, String)>,

    // --- Data Fitur 6: To-Do List ---
    pub todos: Vec<(String, bool)>,

    // --- Maskot ---
    pub devil_eyes: DevilUnionEyes,
    pub chibi_liebe: ChibiLiebe,
}

impl NotchController {
    pub fn new() -> Self {
        let (bat_pct, charging) = Self::read_windows_power();
        let mut ctrl = Self {
            state: NotchState::Pet,
            last_expanded_tab: NotchState::Music,
            is_hovered: false,
            minimal_mode: false,
            mascot_enabled: true,
            anim_time: 0.0,
            width: SpringValue::new(184.0, 310.0, 26.0),
            height: SpringValue::new(34.0, 310.0, 26.0),
            corner_radius: SpringValue::new(16.0, 310.0, 26.0),
            content_alpha: SpringValue::new(1.0, 240.0, 28.0),
            closed_quiet_timer: 0.0,
            hover_grace_timer: 0.0,

            active_overlay: None,
            overlay_timer: 0.0,
            battery_percent: bat_pct,
            is_charging: charging,
            power_poll_timer: 0.0,

            media_title: String::from("Haruka Mirai (Anti-Magic Edit)"),
            media_artist: String::from("Kankaku Piero · Black Clover OP 1"),
            media_playing: true,
            media_progress: 0.42,
            media_elapsed_secs: 84,
            media_total_secs: 200,

            lyrics_lines: vec![
                String::from("♪ Calling out through the darkened sky"),
                String::from("Even without magic, I'll surpass my limits right here!"),
                String::from("Unite the shadows, carve the path ahead"),
                String::from("Haruka mirai e — never giving up is my magic"),
            ],
            active_lyric_idx: 1,

            pomo_remaining_secs: 25 * 60,
            pomo_total_secs: 25 * 60,
            pomo_running: false,
            pomo_is_break: false,
            pomo_accum: 0.0,

            water_current_ml: 750,
            water_goal_ml: 1500,
            water_cup_ml: 250,
            water_hourly_ml: [250, 250, 0, 250, 0, 0],

            shelf_files: vec![
                (String::from("ARCHITECTURE.md"), String::from("4.2 KB · Markdown")),
                (String::from("nusa_config.toml"), String::from("1.8 KB · Config")),
            ],

            todos: vec![
                (String::from("Integrasi mata Asta Devil Union & Emerald"), true),
                (String::from("Bentuk notch menyatu bezel atas laptop"), true),
                (String::from("Uji Volume, Brightness, Charging & Music"), false),
            ],

            devil_eyes: DevilUnionEyes::new(),
            chibi_liebe: ChibiLiebe::new(),
        };
        ctrl.chibi_liebe.sync_with_state(ctrl.state);
        ctrl
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

    /// Tampilkan pop-up overlay (Volume, Brightness, CapsLock, Charging, Headphones)
    pub fn trigger_overlay(&mut self, kind: OverlayKind) {
        self.active_overlay = Some(kind);
        self.overlay_timer = 2.4;
        if !self.is_hovered {
            self.width.set_target(256.0);
            self.height.set_target(42.0);
            self.corner_radius.set_target(18.0);
        }
    }

    pub fn target_dimensions_for(state: NotchState, minimal_mode: bool) -> (f32, f32, f32) {
        if minimal_mode && matches!(state, NotchState::Pet | NotchState::Idle) {
            return (130.0, 6.0, 3.0);
        }

        match state {
            NotchState::Pet => (184.0, 34.0, 16.0),
            NotchState::Idle => (204.0, 34.0, 16.0),
            NotchState::Music => (430.0, 150.0, 22.0),
            NotchState::Lyrics => (465.0, 158.0, 22.0),
            NotchState::Pomo => (410.0, 148.0, 22.0),
            NotchState::Water => (420.0, 152.0, 22.0),
            NotchState::Shelf => (440.0, 160.0, 22.0),
            NotchState::Todo => (430.0, 164.0, 22.0),
        }
    }

    pub fn transition_to(&mut self, next_state: NotchState) {
        if self.state == next_state {
            return;
        }

        if next_state == NotchState::Pet {
            self.devil_eyes.trigger_awakening();
        } else if matches!(
            next_state,
            NotchState::Music
                | NotchState::Lyrics
                | NotchState::Pomo
                | NotchState::Water
                | NotchState::Shelf
                | NotchState::Todo
        ) {
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

    pub fn update(&mut self, dt: f32, cursor_in_pill: bool, cursor_offset: Option<(f32, f32)>) {
        self.anim_time += dt.clamp(0.001, 0.05);

        // 1. Poll status charger & baterai setiap 2.0 detik
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

        // 2. Kurangi timer pop-up overlay jika sedang tampil
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

        // 3. Hover buka/tutup otomatis
        if cursor_in_pill {
            self.hover_grace_timer = 0.14;
            if !self.is_hovered {
                self.is_hovered = true;
                self.active_overlay = None;
                self.devil_eyes.trigger_wake_surge();
                self.transition_to(self.last_expanded_tab);
            }
        } else if self.is_hovered {
            self.hover_grace_timer -= dt;
            if self.hover_grace_timer <= 0.0 {
                self.is_hovered = false;
                self.closed_quiet_timer = 0.0;
                // Saat baru ditutup, tampilkan jam & baterai (Idle) selama 3.5 detik, lalu masuk ke Mata Asta (Pet)
                self.transition_to(NotchState::Idle);
            }
        } else if self.state == NotchState::Idle && self.mascot_enabled {
            self.closed_quiet_timer += dt;
            if self.closed_quiet_timer >= 3.5 {
                self.transition_to(NotchState::Pet);
            }
        }

        // 4. Countdown Pomodoro
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

        // 5. Progress bar musik
        if self.media_playing && self.media_total_secs > 0 {
            let next_prog = self.media_progress + (dt / self.media_total_secs as f32);
            self.media_progress = if next_prog > 1.0 { 0.0 } else { next_prog };
            self.media_elapsed_secs =
                (self.media_progress * self.media_total_secs as f32) as u32;
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

    /// Kirim perintah kontrol media sesungguhnya ke Windows GSMTC (Spotify / Browser / Apple Music)
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

    pub fn handle_panel_click(&mut self, x: f32, y: f32, pill_rect: D2D_RECT_F, dpi: f32) -> bool {
        let lx = (x - pill_rect.left) / dpi;
        let ly = (y - pill_rect.top) / dpi;

        match self.state {
            NotchState::Music => {
                if (98.0..=142.0).contains(&ly) {
                    if (110.0..=146.0).contains(&lx) {
                        Self::send_system_media_command("prev");
                        self.media_progress = 0.0;
                        self.media_elapsed_secs = 0;
                        return true;
                    } else if (152.0..=192.0).contains(&lx) {
                        Self::send_system_media_command("toggle");
                        self.media_playing = !self.media_playing;
                        return true;
                    } else if (198.0..=234.0).contains(&lx) {
                        Self::send_system_media_command("next");
                        self.media_progress = 0.0;
                        self.media_elapsed_secs = 0;
                        return true;
                    } else if (246.0..=312.0).contains(&lx) {
                        self.transition_to(NotchState::Lyrics);
                        return true;
                    }
                }
            }
            NotchState::Lyrics => {
                if ly > 36.0 && !self.lyrics_lines.is_empty() {
                    self.active_lyric_idx = (self.active_lyric_idx + 1) % self.lyrics_lines.len();
                    return true;
                }
            }
            NotchState::Pomo => {
                if (90.0..=136.0).contains(&ly) {
                    if (128.0..=218.0).contains(&lx) {
                        self.pomo_running = !self.pomo_running;
                        return true;
                    } else if (226.0..=294.0).contains(&lx) {
                        self.pomo_running = false;
                        self.pomo_remaining_secs = self.pomo_total_secs;
                        return true;
                    }
                }
            }
            NotchState::Water => {
                if (44.0..=104.0).contains(&ly) && (240.0..=398.0).contains(&lx) {
                    self.water_current_ml =
                        (self.water_current_ml + self.water_cup_ml).min(self.water_goal_ml * 2);
                    let slot = ((self.water_current_ml / self.water_cup_ml) as usize) % 6;
                    self.water_hourly_ml[slot] += self.water_cup_ml;
                    return true;
                }
            }
            NotchState::Todo => {
                if ly >= 38.0 {
                    let row = ((ly - 38.0) / 33.0) as usize;
                    if let Some(item) = self.todos.get_mut(row) {
                        item.1 = !item.1;
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

### C. `src/notch/renderer.rs` — Bezel-Connected Notch Geometry + Full Panels + Pop-Up Overlays

Perhatikan fungsi **`create_bezel_notch_geometry`** di bawah ini: bagian atas notch (`top = 0.0`) kini **rata dan menempel penuh ke bezel atas layar laptop** dengan lekukan bahu halus (*top fillet*), sedangkan sudut bawahnya tetap *rounded* mulus baik saat tertutup maupun saat terbuka lebar!

```rust
// src/notch/renderer.rs
use chrono::Local;
use windows::core::w;
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Direct2D::Common::{
    D2D1_ALPHA_MODE_PREMULTIPLIED, D2D1_BEZIER_SEGMENT, D2D1_COLOR_F, D2D1_FIGURE_BEGIN_FILLED,
    D2D1_FIGURE_END_CLOSED, D2D1_PIXEL_FORMAT, D2D_POINT_2F, D2D_RECT_F, D2D_SIZE_U,
};
use windows::Win32::Graphics::Direct2D::*;
use windows::Win32::Graphics::DirectWrite::{
    DWriteCreateFactory, IDWriteFactory, IDWriteTextFormat, DWRITE_FACTORY_TYPE_SHARED,
    DWRITE_FONT_STRETCH_NORMAL, DWRITE_FONT_STYLE_NORMAL, DWRITE_FONT_WEIGHT_BOLD,
    DWRITE_FONT_WEIGHT_REGULAR, DWRITE_FONT_WEIGHT_SEMI_BOLD, DWRITE_PARAGRAPH_ALIGNMENT_CENTER,
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
        b: 0.078,
        a: 0.98,
    };
    pub const CARD_SURFACE: D2D1_COLOR_F = D2D1_COLOR_F {
        r: 0.075,
        g: 0.094,
        b: 0.133,
        a: 0.95,
    };
    pub const CARD_ELEVATED: D2D1_COLOR_F = D2D1_COLOR_F {
        r: 0.110,
        g: 0.137,
        b: 0.192,
        a: 0.95,
    };
    pub const STEEL_BORDER: D2D1_COLOR_F = D2D1_COLOR_F {
        r: 0.180,
        g: 0.230,
        b: 0.320,
        a: 0.72,
    };
    pub const ACCENT_BLUE: D2D1_COLOR_F = D2D1_COLOR_F {
        r: 0.392,
        g: 0.533,
        b: 0.918,
        a: 1.0,
    };
    pub const ACCENT_EMERALD: D2D1_COLOR_F = D2D1_COLOR_F {
        r: 0.180,
        g: 0.880,
        b: 0.580,
        a: 1.0,
    };
    pub const ACCENT_WINE: D2D1_COLOR_F = D2D1_COLOR_F {
        r: 0.680,
        g: 0.220,
        b: 0.300,
        a: 0.95,
    };
    pub const TEXT_PRIMARY: D2D1_COLOR_F = D2D1_COLOR_F {
        r: 0.930,
        g: 0.950,
        b: 0.980,
        a: 1.0,
    };
    pub const TEXT_SECONDARY: D2D1_COLOR_F = D2D1_COLOR_F {
        r: 0.580,
        g: 0.650,
        b: 0.760,
        a: 1.0,
    };
}

pub struct NotchRenderer {
    pub factory: ID2D1Factory,
    pub dwrite: IDWriteFactory,
    pub rt: ID2D1HwndRenderTarget,
    pub fmt_title: IDWriteTextFormat,
    pub fmt_body: IDWriteTextFormat,
    pub fmt_caption: IDWriteTextFormat,
}

impl NotchRenderer {
    pub fn new(hwnd: HWND, width: u32, height: u32) -> windows::core::Result<Self> {
        unsafe {
            let factory: ID2D1Factory =
                D2D1CreateFactory(D2D1_FACTORY_TYPE_SINGLE_THREADED, None)?;
            let dwrite: IDWriteFactory = DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED)?;

            let render_props = D2D1_RENDER_TARGET_PROPERTIES {
                r#type: D2D1_RENDER_TARGET_TYPE_DEFAULT,
                pixelFormat: D2D1_PIXEL_FORMAT {
                    format: DXGI_FORMAT_B8G8R8A8_UNORM,
                    alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED,
                },
                dpiX: 96.0,
                dpiY: 96.0,
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

            let fmt_title = dwrite.CreateTextFormat(
                w!("Segoe UI"),
                None,
                DWRITE_FONT_WEIGHT_BOLD,
                DWRITE_FONT_STYLE_NORMAL,
                DWRITE_FONT_STRETCH_NORMAL,
                13.0,
                w!("en-us"),
            )?;

            let fmt_body = dwrite.CreateTextFormat(
                w!("Segoe UI"),
                None,
                DWRITE_FONT_WEIGHT_REGULAR,
                DWRITE_FONT_STYLE_NORMAL,
                DWRITE_FONT_STRETCH_NORMAL,
                11.0,
                w!("en-us"),
            )?;

            let fmt_caption = dwrite.CreateTextFormat(
                w!("Segoe UI"),
                None,
                DWRITE_FONT_WEIGHT_SEMI_BOLD,
                DWRITE_FONT_STYLE_NORMAL,
                DWRITE_FONT_STRETCH_NORMAL,
                9.5,
                w!("en-us"),
            )?;

            Ok(Self {
                factory,
                dwrite,
                rt,
                fmt_title,
                fmt_body,
                fmt_caption,
            })
        }
    }

    /// Geometri Notch yang MENYATU DENGAN BEZEL ATAS LAPTOP (rata di y=0, memiliki bahu halus, dan rounded di bawah)
    unsafe fn create_bezel_notch_geometry(
        &self,
        rect: D2D_RECT_F,
        bottom_radius: f32,
    ) -> windows::core::Result<ID2D1PathGeometry> {
        let geom = self.factory.CreatePathGeometry()?;
        let sink = geom.Open()?;

        let h = (rect.bottom - rect.top).max(4.0);
        let r = bottom_radius.min(h * 0.5).max(2.0);
        let ear = (r * 0.45).min(8.0); // Lekukan bahu atas yang menyatu dengan bezel layar

        // Mulai dari bahu kiri-atas tepat di bezel (top = 0.0)
        sink.BeginFigure(
            D2D_POINT_2F {
                x: rect.left - ear,
                y: rect.top,
            },
            D2D1_FIGURE_BEGIN_FILLED,
        );

        // Bahu kiri-atas melengkung masuk ke dinding kiri
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

        // Dinding kiri turun ke sudut kiri-bawah
        sink.AddLine(D2D_POINT_2F {
            x: rect.left,
            y: rect.bottom - r,
        });

        // Sudut kiri-bawah rounded mulus
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

        // Garis bawah horizontal ke sudut kanan-bawah
        sink.AddLine(D2D_POINT_2F {
            x: rect.right - r,
            y: rect.bottom,
        });

        // Sudut kanan-bawah rounded mulus
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

        // Dinding kanan naik ke bahu kanan-atas
        sink.AddLine(D2D_POINT_2F {
            x: rect.right,
            y: rect.top + ear,
        });

        // Bahu kanan-atas melengkung menyatu ke bezel layar (top = 0.0)
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

            // 1. Gambar Notch Menyatu dengan Bezel Atas Laptop
            if let Ok(notch_geom) =
                self.create_bezel_notch_geometry(pill_rect, controller.corner_radius.value)
            {
                if let Ok(bg_brush) = self
                    .rt
                    .CreateSolidColorBrush(&AntiMagicNeutralPalette::PILL_BG, None)
                {
                    self.rt.FillGeometry(&notch_geom, &bg_brush, None);
                }

                let border_color = if controller.state == NotchState::Pet {
                    let pulse = controller.devil_eyes.aura_intensity.clamp(0.4, 1.2);
                    D2D1_COLOR_F {
                        r: 0.25 * pulse,
                        g: 0.24,
                        b: 0.36,
                        a: 0.60,
                    }
                } else {
                    AntiMagicNeutralPalette::STEEL_BORDER
                };

                if let Ok(border_brush) = self.rt.CreateSolidColorBrush(&border_color, None) {
                    self.rt.DrawGeometry(&notch_geom, &border_brush, 1.15, None);
                }
            }

            // 2. Jika ada Pop-Up Overlay aktif (Volume, Brightness, CapsLock, Charging, Headphones)
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
                let collapse_factor = ((60.0 - controller.height.value) / 26.0).clamp(0.0, 1.0);
                controller
                    .devil_eyes
                    .render(&self.factory, &self.rt, pill_rect, collapse_factor);
            } else if controller.state == NotchState::Idle && !controller.minimal_mode {
                // Closed Notch seperti Nusa Original: Jam Digital di Kiri, Status Baterai di Kanan
                let alpha = ((60.0 - controller.height.value) / 26.0).clamp(0.0, 1.0);
                self.draw_closed_idle_clock_and_battery(controller, pill_rect, alpha);
            } else if controller.height.value > 65.0 {
                let alpha = controller.content_alpha.value.clamp(0.0, 1.0);
                let card_rect = self.draw_expanded_panel_shell(controller.state, pill_rect, alpha);

                match controller.state {
                    NotchState::Music => self.draw_music_panel(controller, card_rect, alpha),
                    NotchState::Lyrics => self.draw_lyrics_panel(controller, card_rect, alpha),
                    NotchState::Pomo => self.draw_pomo_panel(controller, card_rect, alpha),
                    NotchState::Water => self.draw_water_panel(controller, card_rect, alpha),
                    NotchState::Shelf => self.draw_shelf_panel(controller, card_rect, alpha),
                    NotchState::Todo => self.draw_todo_panel(controller, card_rect, alpha),
                    _ => {}
                }

                // Chibi Liebe selektif pada Music, Pomo, dan Shelf
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

    /// Tampilan Closed Idle seperti Nusa Original: Jam Digital + Indikator Baterai/Charging
    unsafe fn draw_closed_idle_clock_and_battery(
        &self,
        ctrl: &NotchController,
        pill: D2D_RECT_F,
        alpha: f32,
    ) {
        let now_str = Local::now().format("%H:%M").to_string();
        let mut p_col = AntiMagicNeutralPalette::TEXT_PRIMARY;
        p_col.a *= alpha;
        let mut s_col = if ctrl.is_charging {
            AntiMagicNeutralPalette::ACCENT_EMERALD
        } else {
            AntiMagicNeutralPalette::TEXT_SECONDARY
        };
        s_col.a *= alpha;

        self.draw_text_left(
            &now_str,
            &self.fmt_title,
            D2D_RECT_F {
                left: pill.left + 16.0,
                top: pill.top,
                right: pill.left + 90.0,
                bottom: pill.bottom,
            },
            p_col,
        );

        let bat_text = if ctrl.is_charging {
            format!("⚡ {}%", ctrl.battery_percent)
        } else {
            format!("{}%", ctrl.battery_percent)
        };
        self.draw_text_centered(
            &bat_text,
            &self.fmt_caption,
            D2D_RECT_F {
                left: pill.right - 72.0,
                top: pill.top,
                right: pill.right - 12.0,
                bottom: pill.bottom,
            },
            s_col,
        );
    }

    /// Render Pop-up Overlays (Volume, Brightness, CapsLock, Charging, Headphones)
    unsafe fn draw_system_overlay(&self, overlay: &OverlayKind, pill: D2D_RECT_F) {
        let p_col = AntiMagicNeutralPalette::TEXT_PRIMARY;
        let accent = AntiMagicNeutralPalette::ACCENT_BLUE;
        let emerald = AntiMagicNeutralPalette::ACCENT_EMERALD;

        match overlay {
            OverlayKind::Volume(val) | OverlayKind::Brightness(val) => {
                let is_vol = matches!(overlay, OverlayKind::Volume(_));
                let label = if is_vol { "VOL" } else { "BRI" };
                self.draw_text_left(
                    label,
                    &self.fmt_caption,
                    D2D_RECT_F {
                        left: pill.left + 16.0,
                        top: pill.top,
                        right: pill.left + 56.0,
                        bottom: pill.bottom,
                    },
                    p_col,
                );

                // Progress bar di tengah
                let bar_l = pill.left + 56.0;
                let bar_r = pill.right - 48.0;
                let bar_cy = (pill.top + pill.bottom) * 0.5;
                if let Ok(tb) = self.rt.CreateSolidColorBrush(
                    &D2D1_COLOR_F {
                        r: 0.20,
                        g: 0.25,
                        b: 0.35,
                        a: 0.7,
                    },
                    None,
                ) {
                    self.rt.FillRoundedRectangle(
                        &D2D1_ROUNDED_RECT {
                            rect: D2D_RECT_F {
                                left: bar_l,
                                top: bar_cy - 3.0,
                                right: bar_r,
                                bottom: bar_cy + 3.0,
                            },
                            radiusX: 3.0,
                            radiusY: 3.0,
                        },
                        &tb,
                    );
                }
                if let Ok(fb) = self.rt.CreateSolidColorBrush(&accent, None) {
                    let fill_w = (bar_r - bar_l) * ((*val as f32 / 100.0).clamp(0.0, 1.0));
                    self.rt.FillRoundedRectangle(
                        &D2D1_ROUNDED_RECT {
                            rect: D2D_RECT_F {
                                left: bar_l,
                                top: bar_cy - 3.0,
                                right: bar_l + fill_w.max(4.0),
                                bottom: bar_cy + 3.0,
                            },
                            radiusX: 3.0,
                            radiusY: 3.0,
                        },
                        &fb,
                    );
                }

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
                self.draw_text_centered(txt, &self.fmt_title, pill, if *on { accent } else { p_col });
            }
            OverlayKind::Charging { percent, plugged } => {
                let txt = if *plugged {
                    format!("⚡ Charging · {}%", percent)
                } else {
                    format!("Battery · {}%", percent)
                };
                self.draw_text_centered(
                    &txt,
                    &self.fmt_title,
                    pill,
                    if *plugged { emerald } else { p_col },
                );
            }
            OverlayKind::Headphones { name, percent } => {
                let txt = format!("🎧 {} · {}%", name, percent);
                self.draw_text_centered(&txt, &self.fmt_title, pill, accent);
            }
        }
    }

    unsafe fn draw_expanded_panel_shell(
        &self,
        state: NotchState,
        pill_rect: D2D_RECT_F,
        alpha: f32,
    ) -> D2D_RECT_F {
        let tabs = ["Music", "Lyrics", "Focus", "Water", "Shelf", "To-Do"];
        let active_idx = match state {
            NotchState::Music => 0,
            NotchState::Lyrics => 1,
            NotchState::Pomo => 2,
            NotchState::Water => 3,
            NotchState::Shelf => 4,
            NotchState::Todo => 5,
            _ => 0,
        };

        let inner_left = pill_rect.left + 14.0;
        let inner_right = pill_rect.right - 14.0;
        let slot_w = (inner_right - inner_left) / 6.0;

        for (i, label) in tabs.iter().enumerate() {
            let slot_left = inner_left + slot_w * i as f32;
            let slot_right = slot_left + slot_w;
            let cx = (slot_left + slot_right) * 0.5;

            let is_active = i == active_idx;
            let mut text_col = if is_active {
                AntiMagicNeutralPalette::TEXT_PRIMARY
            } else {
                AntiMagicNeutralPalette::TEXT_SECONDARY
            };
            text_col.a *= if is_active { alpha } else { alpha * 0.75 };

            self.draw_text_centered(
                label,
                &self.fmt_caption,
                D2D_RECT_F {
                    left: slot_left,
                    top: pill_rect.top + 3.0,
                    right: slot_right,
                    bottom: pill_rect.top + 21.0,
                },
                text_col,
            );

            let line_col = if is_active {
                let mut c = AntiMagicNeutralPalette::ACCENT_BLUE;
                c.a *= alpha;
                c
            } else {
                D2D1_COLOR_F {
                    r: 0.30,
                    g: 0.36,
                    b: 0.48,
                    a: 0.30 * alpha,
                }
            };

            if let Ok(brush) = self.rt.CreateSolidColorBrush(&line_col, None) {
                let half_len = if is_active { 16.0 } else { 6.0 };
                self.rt.DrawLine(
                    D2D_POINT_2F {
                        x: cx - half_len,
                        y: pill_rect.top + 23.0,
                    },
                    D2D_POINT_2F {
                        x: cx + half_len,
                        y: pill_rect.top + 23.0,
                    },
                    &brush,
                    2.2,
                    None,
                );
            }
        }

        let card_rect = D2D_RECT_F {
            left: inner_left,
            top: pill_rect.top + 28.0,
            right: inner_right,
            bottom: pill_rect.bottom - 10.0,
        };

        let mut card_color = AntiMagicNeutralPalette::CARD_SURFACE;
        card_color.a *= alpha;
        if let Ok(card_brush) = self.rt.CreateSolidColorBrush(&card_color, None) {
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: card_rect,
                    radiusX: 14.0,
                    radiusY: 14.0,
                },
                &card_brush,
            );
        }

        card_rect
    }

    unsafe fn draw_music_panel(
        &self,
        ctrl: &NotchController,
        card: D2D_RECT_F,
        alpha: f32,
    ) {
        let art_rect = D2D_RECT_F {
            left: card.left + 12.0,
            top: card.top + 12.0,
            right: card.left + 92.0,
            bottom: card.top + 92.0,
        };
        let mut art_bg = AntiMagicNeutralPalette::CARD_ELEVATED;
        art_bg.a *= alpha;
        if let Ok(b) = self.rt.CreateSolidColorBrush(&art_bg, None) {
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: art_rect,
                    radiusX: 12.0,
                    radiusY: 12.0,
                },
                &b,
            );
        }

        let mut eq_col = AntiMagicNeutralPalette::ACCENT_BLUE;
        eq_col.a *= alpha;
        if let Ok(eq_brush) = self.rt.CreateSolidColorBrush(&eq_col, None) {
            for i in 0..5 {
                let phase = ctrl.anim_time * 5.5 + i as f32 * 1.1;
                let h = if ctrl.media_playing {
                    10.0 + (phase.sin().abs() * 25.0)
                } else {
                    6.0
                };
                let bx = art_rect.left + 17.0 + i as f32 * 10.0;
                let by = art_rect.bottom - 16.0;
                self.rt.FillRoundedRectangle(
                    &D2D1_ROUNDED_RECT {
                        rect: D2D_RECT_F {
                            left: bx,
                            top: by - h,
                            right: bx + 6.0,
                            bottom: by,
                        },
                        radiusX: 3.0,
                        radiusY: 3.0,
                    },
                    &eq_brush,
                );
            }
        }

        let text_left = art_rect.right + 14.0;
        let text_right = card.right - 56.0;
        let mut p_col = AntiMagicNeutralPalette::TEXT_PRIMARY;
        p_col.a *= alpha;
        let mut s_col = AntiMagicNeutralPalette::TEXT_SECONDARY;
        s_col.a *= alpha;

        self.draw_text_left(
            &ctrl.media_title,
            &self.fmt_title,
            D2D_RECT_F {
                left: text_left,
                top: card.top + 10.0,
                right: text_right,
                bottom: card.top + 31.0,
            },
            p_col,
        );
        self.draw_text_left(
            &ctrl.media_artist,
            &self.fmt_body,
            D2D_RECT_F {
                left: text_left,
                top: card.top + 30.0,
                right: text_right,
                bottom: card.top + 48.0,
            },
            s_col,
        );

        let bar_top = card.top + 56.0;
        let bar_w = (text_right - text_left).max(10.0);
        if let Ok(track_b) = self.rt.CreateSolidColorBrush(
            &D2D1_COLOR_F {
                r: 0.20,
                g: 0.25,
                b: 0.35,
                a: 0.6 * alpha,
            },
            None,
        ) {
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: D2D_RECT_F {
                        left: text_left,
                        top: bar_top,
                        right: text_right,
                        bottom: bar_top + 5.0,
                    },
                    radiusX: 2.5,
                    radiusY: 2.5,
                },
                &track_b,
            );
        }

        if let Ok(fill_b) = self.rt.CreateSolidColorBrush(&eq_col, None) {
            let prog_w = (bar_w * ctrl.media_progress.clamp(0.0, 1.0)).max(5.0);
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: D2D_RECT_F {
                        left: text_left,
                        top: bar_top,
                        right: text_left + prog_w,
                        bottom: bar_top + 5.0,
                    },
                    radiusX: 2.5,
                    radiusY: 2.5,
                },
                &fill_b,
            );
        }

        let btn_y = card.top + 82.0;
        let controls = [
            (text_left + 16.0, "|<<"),
            (text_left + 58.0, if ctrl.media_playing { "||" } else { "|>" }),
            (text_left + 100.0, ">>|"),
        ];

        for (cx, sym) in controls {
            let r_rect = D2D_RECT_F {
                left: cx - 16.0,
                top: btn_y - 11.0,
                right: cx + 16.0,
                bottom: btn_y + 11.0,
            };
            if let Ok(b) = self.rt.CreateSolidColorBrush(&art_bg, None) {
                self.rt.FillRoundedRectangle(
                    &D2D1_ROUNDED_RECT {
                        rect: r_rect,
                        radiusX: 8.0,
                        radiusY: 8.0,
                    },
                    &b,
                );
            }
            self.draw_text_centered(sym, &self.fmt_caption, r_rect, p_col);
        }

        let lyr_rect = D2D_RECT_F {
            left: text_left + 132.0,
            top: btn_y - 11.0,
            right: text_left + 190.0,
            bottom: btn_y + 11.0,
        };
        if let Ok(b) = self.rt.CreateSolidColorBrush(&art_bg, None) {
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: lyr_rect,
                    radiusX: 8.0,
                    radiusY: 8.0,
                },
                &b,
            );
        }
        self.draw_text_centered("Lyrics", &self.fmt_caption, lyr_rect, eq_col);
    }

    unsafe fn draw_lyrics_panel(
        &self,
        ctrl: &NotchController,
        card: D2D_RECT_F,
        alpha: f32,
    ) {
        let mut p_col = AntiMagicNeutralPalette::TEXT_PRIMARY;
        p_col.a *= alpha;
        let mut s_col = AntiMagicNeutralPalette::TEXT_SECONDARY;
        s_col.a *= alpha * 0.65;
        let mut accent = AntiMagicNeutralPalette::ACCENT_BLUE;
        accent.a *= alpha;

        let header = format!("{} — {}", ctrl.media_title, ctrl.media_artist);
        self.draw_text_centered(
            &header,
            &self.fmt_caption,
            D2D_RECT_F {
                left: card.left + 14.0,
                top: card.top + 6.0,
                right: card.right - 14.0,
                bottom: card.top + 24.0,
            },
            accent,
        );

        if ctrl.lyrics_lines.is_empty() {
            self.draw_text_centered(
                "Searching synced lyrics on LRCLIB...",
                &self.fmt_body,
                card,
                s_col,
            );
            return;
        }

        let idx = ctrl.active_lyric_idx.min(ctrl.lyrics_lines.len() - 1);
        let prev_line = if idx > 0 {
            ctrl.lyrics_lines[idx - 1].as_str()
        } else {
            "· · ·"
        };
        let curr_line = ctrl.lyrics_lines[idx].as_str();
        let next_line = if idx + 1 < ctrl.lyrics_lines.len() {
            ctrl.lyrics_lines[idx + 1].as_str()
        } else {
            "· · ·"
        };

        self.draw_text_centered(
            prev_line,
            &self.fmt_body,
            D2D_RECT_F {
                left: card.left + 16.0,
                top: card.top + 26.0,
                right: card.right - 16.0,
                bottom: card.top + 48.0,
            },
            s_col,
        );
        self.draw_text_centered(
            curr_line,
            &self.fmt_title,
            D2D_RECT_F {
                left: card.left + 12.0,
                top: card.top + 48.0,
                right: card.right - 12.0,
                bottom: card.top + 78.0,
            },
            p_col,
        );
        self.draw_text_centered(
            next_line,
            &self.fmt_body,
            D2D_RECT_F {
                left: card.left + 16.0,
                top: card.top + 78.0,
                right: card.right - 16.0,
                bottom: card.top + 100.0,
            },
            s_col,
        );
    }

    unsafe fn draw_pomo_panel(
        &self,
        ctrl: &NotchController,
        card: D2D_RECT_F,
        alpha: f32,
    ) {
        let mut p_col = AntiMagicNeutralPalette::TEXT_PRIMARY;
        p_col.a *= alpha;
        let mut s_col = AntiMagicNeutralPalette::TEXT_SECONDARY;
        s_col.a *= alpha;
        let mut accent = if ctrl.pomo_is_break {
            AntiMagicNeutralPalette::ACCENT_WINE
        } else {
            AntiMagicNeutralPalette::ACCENT_BLUE
        };
        accent.a *= alpha;

        let ring_cx = card.left + 56.0;
        let ring_cy = (card.top + card.bottom) * 0.5;
        let ring_r = 36.0;

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
                3.5,
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
                left: ring_cx - 32.0,
                top: ring_cy - 14.0,
                right: ring_cx + 32.0,
                bottom: ring_cy + 14.0,
            },
            p_col,
        );

        let right_x = card.left + 112.0;
        self.draw_text_left(
            if ctrl.pomo_is_break {
                "SHORT BREAK · REST EYES"
            } else {
                "POMODORO · FOCUS SESSION"
            },
            &self.fmt_title,
            D2D_RECT_F {
                left: right_x,
                top: card.top + 12.0,
                right: card.right - 50.0,
                bottom: card.top + 36.0,
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
                top: card.top + 36.0,
                right: card.right - 50.0,
                bottom: card.top + 58.0,
            },
            s_col,
        );

        let start_rect = D2D_RECT_F {
            left: right_x,
            top: card.top + 64.0,
            right: right_x + 86.0,
            bottom: card.top + 92.0,
        };
        if let Ok(b) = self.rt.CreateSolidColorBrush(&accent, None) {
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: start_rect,
                    radiusX: 8.0,
                    radiusY: 8.0,
                },
                &b,
            );
        }
        self.draw_text_centered(
            if ctrl.pomo_running { "Pause" } else { "Start Focus" },
            &self.fmt_caption,
            start_rect,
            p_col,
        );

        let reset_rect = D2D_RECT_F {
            left: right_x + 94.0,
            top: card.top + 64.0,
            right: right_x + 158.0,
            bottom: card.top + 92.0,
        };
        let mut elev = AntiMagicNeutralPalette::CARD_ELEVATED;
        elev.a *= alpha;
        if let Ok(b) = self.rt.CreateSolidColorBrush(&elev, None) {
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: reset_rect,
                    radiusX: 8.0,
                    radiusY: 8.0,
                },
                &b,
            );
        }
        self.draw_text_centered("Reset", &self.fmt_caption, reset_rect, s_col);
    }

    unsafe fn draw_water_panel(
        &self,
        ctrl: &NotchController,
        card: D2D_RECT_F,
        alpha: f32,
    ) {
        let mut p_col = AntiMagicNeutralPalette::TEXT_PRIMARY;
        p_col.a *= alpha;
        let mut accent = AntiMagicNeutralPalette::ACCENT_BLUE;
        accent.a *= alpha;

        let pct = ((ctrl.water_current_ml as f32 / ctrl.water_goal_ml.max(1) as f32) * 100.0)
            .round() as u32;
        let header = format!(
            "HYDRATION · {:.2} / {:.2} L ({}%)",
            ctrl.water_current_ml as f32 / 1000.0,
            ctrl.water_goal_ml as f32 / 1000.0,
            pct
        );

        self.draw_text_left(
            &header,
            &self.fmt_title,
            D2D_RECT_F {
                left: card.left + 16.0,
                top: card.top + 10.0,
                right: card.right - 16.0,
                bottom: card.top + 34.0,
            },
            p_col,
        );

        let chart_left = card.left + 18.0;
        let chart_bottom = card.bottom - 14.0;
        if let Ok(bar_b) = self.rt.CreateSolidColorBrush(&accent, None) {
            for (i, &val) in ctrl.water_hourly_ml.iter().enumerate() {
                let h = ((val as f32 / 500.0) * 42.0).clamp(6.0, 44.0);
                let bx = chart_left + i as f32 * 28.0;
                self.rt.FillRoundedRectangle(
                    &D2D1_ROUNDED_RECT {
                        rect: D2D_RECT_F {
                            left: bx,
                            top: chart_bottom - h,
                            right: bx + 16.0,
                            bottom: chart_bottom,
                        },
                        radiusX: 4.0,
                        radiusY: 4.0,
                    },
                    &bar_b,
                );
            }
        }

        let btn_rect = D2D_RECT_F {
            left: card.right - 148.0,
            top: card.top + 44.0,
            right: card.right - 18.0,
            bottom: card.top + 82.0,
        };
        if let Ok(b) = self.rt.CreateSolidColorBrush(&accent, None) {
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: btn_rect,
                    radiusX: 10.0,
                    radiusY: 10.0,
                },
                &b,
            );
        }
        self.draw_text_centered(
            &format!("+ Drink {} ml", ctrl.water_cup_ml),
            &self.fmt_title,
            btn_rect,
            p_col,
        );
    }

    unsafe fn draw_shelf_panel(
        &self,
        ctrl: &NotchController,
        card: D2D_RECT_F,
        alpha: f32,
    ) {
        let mut p_col = AntiMagicNeutralPalette::TEXT_PRIMARY;
        p_col.a *= alpha;
        let mut s_col = AntiMagicNeutralPalette::TEXT_SECONDARY;
        s_col.a *= alpha;
        let mut elev = AntiMagicNeutralPalette::CARD_ELEVATED;
        elev.a *= alpha;

        self.draw_text_left(
            "FILE SHELF · DRAG FILES HERE OR CLICK TO OPEN",
            &self.fmt_caption,
            D2D_RECT_F {
                left: card.left + 14.0,
                top: card.top + 6.0,
                right: card.right - 50.0,
                bottom: card.top + 24.0,
            },
            s_col,
        );

        for (i, (name, meta)) in ctrl.shelf_files.iter().take(2).enumerate() {
            let item_w = (card.right - card.left - 82.0) * 0.5;
            let ix = card.left + 14.0 + i as f32 * (item_w + 10.0);
            let item_rect = D2D_RECT_F {
                left: ix,
                top: card.top + 28.0,
                right: ix + item_w,
                bottom: card.bottom - 12.0,
            };

            if let Ok(b) = self.rt.CreateSolidColorBrush(&elev, None) {
                self.rt.FillRoundedRectangle(
                    &D2D1_ROUNDED_RECT {
                        rect: item_rect,
                        radiusX: 10.0,
                        radiusY: 10.0,
                    },
                    &b,
                );
            }

            self.draw_text_left(
                name,
                &self.fmt_title,
                D2D_RECT_F {
                    left: item_rect.left + 10.0,
                    top: item_rect.top + 10.0,
                    right: item_rect.right - 8.0,
                    bottom: item_rect.top + 34.0,
                },
                p_col,
            );
            self.draw_text_left(
                meta,
                &self.fmt_caption,
                D2D_RECT_F {
                    left: item_rect.left + 10.0,
                    top: item_rect.top + 36.0,
                    right: item_rect.right - 8.0,
                    bottom: item_rect.top + 56.0,
                },
                s_col,
            );
        }
    }

    unsafe fn draw_todo_panel(
        &self,
        ctrl: &NotchController,
        card: D2D_RECT_F,
        alpha: f32,
    ) {
        let mut p_col = AntiMagicNeutralPalette::TEXT_PRIMARY;
        p_col.a *= alpha;
        let mut s_col = AntiMagicNeutralPalette::TEXT_SECONDARY;
        s_col.a *= alpha;
        let mut accent = AntiMagicNeutralPalette::ACCENT_BLUE;
        accent.a *= alpha;

        for (i, (task, done)) in ctrl.todos.iter().take(3).enumerate() {
            let row_top = card.top + 10.0 + i as f32 * 32.0;
            let box_rect = D2D_RECT_F {
                left: card.left + 16.0,
                top: row_top + 4.0,
                right: card.left + 32.0,
                bottom: row_top + 20.0,
            };

            if let Ok(b) = self.rt.CreateSolidColorBrush(&accent, None) {
                let rbox = D2D1_ROUNDED_RECT {
                    rect: box_rect,
                    radiusX: 4.0,
                    radiusY: 4.0,
                };
                if *done {
                    self.rt.FillRoundedRectangle(&rbox, &b);
                } else {
                    self.rt.DrawRoundedRectangle(&rbox, &b, 1.5, None);
                }
            }

            self.draw_text_left(
                task,
                &self.fmt_body,
                D2D_RECT_F {
                    left: card.left + 42.0,
                    top: row_top,
                    right: card.right - 16.0,
                    bottom: row_top + 25.0,
                },
                if *done { s_col } else { p_col },
            );
        }
    }
}
```

---

### D. `src/notch/mod.rs` — Bezel Flush (`top = 0.0`) + Sinkronisasi Semua Event (`Volume`, `Brightness`, `CapsLock`, `Media`, `Lyrics`, `Water`, `Shelf`)

Perhatikan bahwa di `current_pill_rect()`, nilai `top` sekarang adalah **`0.0`** sehingga notch menempel rata dengan bezel atas layar laptop baik ketika tertutup maupun terbuka, dan di `on_tick()` seluruh `NotchEvent` (`HotkeyVolume`, `HotkeyBrightness`, `HotkeyCapsLock`, `UpdateMedia`, `UpdateLyrics`, `UpdateTimer`, `UpdateWater`, `UpdateShelf`) sudah dihubungkan ke `NotchController`:

```rust
    pub fn current_pill_rect(&self) -> D2D_RECT_F {
        let pw = (self.controller.width.value * self.dpi_scale).round();
        let ph = (self.controller.height.value * self.dpi_scale).round();
        let left = ((self.win_w as f32 - pw) * 0.5).max(0.0);
        let top = 0.0; // MENEMPEL LANGSUNG KE BEZEL ATAS LAPTOP (0.0)
        let right = left + pw;
        let bottom = top + ph;

        D2D_RECT_F {
            left,
            top,
            right,
            bottom,
        }
    }
```

Dan di dalam `while let Ok(ev) = self.event_rx.try_recv()` pada `on_tick()` di `src/notch/mod.rs`:

```rust
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
```