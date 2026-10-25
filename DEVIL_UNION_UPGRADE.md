# Nusa Clone: Perbaikan Interaksi Notch, Smooth Spring Animation, Neutral Anti-Magic Theme, Mata Asta (Devil Union) & Chibi Liebe

Dokumen ini berisi **Diagnosis & Solusi Arsitektur**, **Spesifikasi Palet Neutral Slate-Blue Anti-Magic**, **Prompt Siap Pakai untuk Hermes Agent**, serta **Kode Modul Rust Lengkap (`windows` v0.58 + Direct2D)** untuk memperbaiki *notch* yang tidak merespons dan animasi yang kaku, mengganti *idle mascot* menjadi mata **Asta (Black Clover) — Devil Union Mode**, serta menambahkan **Chibi Liebe** yang berinteraksi secara selektif pada fitur terpilih (`Music`, `Pomodoro`, dan `Shelf`).

---

## 1. Diagnosis Masalah pada `ARCHITECTURE.md` & Solusi Teknis

Berdasarkan `ARCHITECTURE.md` proyekmu, ada **3 penyebab utama** mengapa *notch* saat ini tidak berfungsi (tidak bisa diklik/di-*hover*) dan animasinya sangat kaku:

| Masalah Saat Ini | Penyebab Teknis di Win32 / Direct2D | Solusi Perbaikan |
| :--- | :--- | :--- |
| **Notch tidak merespons hover atau klik sama sekali** | Penggunaan flag `WS_EX_TRANSPARENT` secara permanen pada saat pembuatan window (`CreateWindowExW`). Di Windows API, kombinasi `WS_EX_LAYERED \| WS_EX_TRANSPARENT` membuat window menjadi **100% click-through** — semua *event* mouse (`WM_MOUSEMOVE`, `WM_LBUTTONDOWN`) langsung tembus ke aplikasi di belakangnya. | **Dynamic Click-Through Toggle & Cursor Polling**: Poll posisi kursor dengan `GetCursorPos` di dalam *timer* (`WM_TIMER`). Saat kursor masuk ke *bounding box* notch yang sedang aktif, hapus bit `WS_EX_TRANSPARENT` via `SetWindowLongPtrW(hwnd, GWL_EXSTYLE, ...)`. Saat kursor keluar dari area notch, pasang kembali `WS_EX_TRANSPARENT` agar area transparan di luar *pill* tidak menghalangi klik desktop. |
| **Buka-tutup notch sangat kaku (patah-patah / instan)** | Ukuran *pill* (`width`, `height`, `corner_radius`) diubah langsung secara diskrit saat berganti *state*, dan window hanya digambar ulang saat ada *event* sistem tanpa *frame loop* fisika. | **Critically-Damped Spring Physics Engine (60–120 FPS On-Demand)**: Gunakan interpolasi pegas (*spring-damper* `stiffness = 310.0`, `damping = 26.0`) untuk `width`, `height`, `corner_radius`, dan `content_alpha`. Nyalakan `SetTimer` berkecepatan tinggi (`8ms` / ~120 FPS) ketika animasi berjalan atau saat mode `pet` aktif, dan hentikan *rendering* saat posisi pegas sudah konvergen (*settled*) agar tetap **0% CPU saat idle statis**. |
| **Transparansi sudut pill bocor atau kotak hitam terlihat** | Window berukuran tetap atau *render target* Direct2D tidak mendukung *per-pixel alpha composition* di atas desktop. | Buat ukuran jendela (`HWND`) sebesar ukuran maksimum ekspansi (`520x220`), bersihkan *canvas* dengan `D2D1_COLOR_F { r: 0.0, g: 0.0, b: 0.0, a: 0.0 }` menggunakan `D2D1_ALPHA_MODE_PREMULTIPLIED` + `DwmExtendFrameIntoClientArea` (`MARGINS: -1`), lalu gambar *rounded rect* sebesar ukuran pegas saat ini (`current_w`, `current_h`). |

---

## 2. Palet Warna "Neutral Slate-Blue Anti-Magic" & Selektivitas Chibi Liebe

Agar menyatu dengan tema utama laptopmu (**biru, abu-abu, dan warna netral**) tanpa warna merah yang terlalu mencolok, gaya visual *Anti-Magic* dikalibrasi menjadi **Obsidian Slate, Cool Steel Grey, dan Muted Wine-Rose tipis**:

| Elemen UI / Visual | Kode Warna (Hex) | Nilai `D2D1_COLOR_F` (`r, g, b, a`) | Fungsi Visual |
| :--- | :--- | :--- | :--- |
| **Notch Pill Background** | `#0B0E14` | `(0.043, 0.055, 0.078, 0.97)` | Hitam-biru *obsidian* pekat yang menyatu elegan dengan tema desktop gelap/abu-abu/biru. |
| **Elevated Card / Surface** | `#131822` | `(0.075, 0.094, 0.133, 0.92)` | Abu-abu kebiruan (*dark slate*) untuk kartu album musik, *drop zone* Shelf, dan bar navigasi. |
| **Neutral Steel Border** | `#263042` | `(0.149, 0.188, 0.259, 0.55)` | Garis tepi *pill* abu-abu kebiruan dingin yang bersih dan netral. |
| **Primary Accent (Steel Blue)** | `#6488EA` | `(0.392, 0.533, 0.918, 1.00)` | Aksen utama untuk *progress bar* musik, cincin Pomodoro, dan indikator tab aktif. |
| **Anti-Magic Smoke / Sparks** | `#1E2533` & `#8A99B5` | `(0.118, 0.145, 0.200, 0.85)` | Partikel *Anti-Magic* monokrom abu-abu arang (*charcoal*) dan *silver-slate* (bukan merah neon). |
| **Subtle Crimson Eye Core** | `#A83244` | `(0.659, 0.196, 0.267, 0.90)` | Merah *muted crimson / wine* yang kalem khusus pada inti iris Asta & mata Chibi Liebe agar tetap ikonik tanpa menabrak warna biru/abu-abu laptop. |

### Di Mana Saja Chibi Liebe Muncul? (Selektif 3 Fitur Saja)
Sesuai permintaanmu agar **tidak semua fitur** dipenuhi maskot, **Chibi Liebe** (wujud mungil Liebe dari bahu Asta dengan rambut abu-abu *ash-silver*, tanduk melengkung kecil, sayap mungil, dan ekspresi *deadpan/smug*) hanya muncul pada **3 state terpilih**:
1. **`NotchState::Music` (`Headbop`)**: Chibi Liebe duduk santai di pojok kanan atas *album art* / kapsul musik sambil mengangguk-angguk (*headbop*) mengikuti irama lagu dan mengepakkan sayap kecilnya.
2. **`NotchState::Pomo` (`FocusGuard`)**: Chibi Liebe berdiri melipat tangan di samping *ring timer* Pomodoro dengan tatapan mengawasi fokusmu (dan menguap santai saat fase *Short Break*).
3. **`NotchState::Shelf` (`CatchingFiles`)**: Chibi Liebe mengintip dari tepi bawah *drop zone* sambil merentangkan kedua tangan kecilnya seolah siap menangkap file yang kamu *drag-and-drop*.
- *Catatan*: Pada mode **`Idle`**, **`Lyrics`**, **`Water`**, dan **`Todo`**, Chibi Liebe **tidak ditampilkan** agar tampilan tetap bersih dan minimalis.

---

## 3. Prompt Siap Pakai untuk Hermes Agent

Salin seluruh blok di bawah ini dan berikan langsung ke **Hermes Agent** di terminal lokalmu (jalankan di *root directory* `nusa-clone/`):

```markdown
Tolong perbaiki dan upgrade repositori Rust `nusa-clone` ini (menggunakan `windows = "0.58"` dan Direct2D) berdasarkan panduan di `DEVIL_UNION_UPGRADE.md` dengan spesifikasi tugas berikut:

### Tujuan Utama
1. **Perbaiki Interaksi Notch (Hover & Click) yang Mati (`src/notch/mod.rs`)**:
   - Jangan kunci window dengan `WS_EX_TRANSPARENT` secara statis.
   - Buat sistem *dynamic hit-testing*: gunakan timer polling (`GetCursorPos`) setiap tick untuk mengecek apakah kursor berada di dalam *bounding box* pill saat ini (`current_pill_rect()`).
   - Jika kursor berada di dalam pill (`cursor_in_pill == true`), cabut flag `WS_EX_TRANSPARENT` menggunakan `SetWindowLongPtrW(hwnd, GWL_EXSTYLE, style & !(WS_EX_TRANSPARENT.0 as isize))` sehingga window menerima `WM_LBUTTONDOWN`, `WM_MOUSEWHEEL`, dan drag-and-drop.
   - Jika kursor berada di luar pill (`cursor_in_pill == false`), aktifkan kembali `WS_EX_TRANSPARENT` agar area transparan di sekeliling pill tidak memblokir klik di bagian atas layar Windows.
   - Gunakan `DwmExtendFrameIntoClientArea` dengan `MARGINS { cxLeftWidth: -1, cxRightWidth: -1, cyTopWidth: -1, cyBottomWidth: -1 }` dan `D2D1_ALPHA_MODE_PREMULTIPLIED` agar latar belakang di luar pill benar-benar transparan.

2. **Implementasikan Smooth Spring-Damper Animation (`src/notch/states.rs`)**:
   - Tambahkan struct `SpringValue` (critically-damped spring physics dengan `stiffness: 310.0`, `damping: 26.0`) untuk menginterpolasi `width`, `height`, `corner_radius`, dan `content_alpha` secara mulus setiap frame (`dt`).
   - Gunakan `SetTimer` dengan interval `8ms` (~120 FPS) untuk mengupdate fisika pegas dan memanggil `InvalidateRect`.

3. **Terapkan Tema Visual "Neutral Slate-Blue Anti-Magic" (`src/notch/renderer.rs`)**:
   - Gunakan palet warna netral yang serasi dengan tema laptop biru/abu-abu:
     - Background Pill: Obsidian Slate (`#0B0E14`)
     - Surface / Card: Dark Slate (`#131822`)
     - Border: Cool Steel Grey (`#263042`) dengan aksen Slate-Blue (`#6488EA`)
     - Anti-Magic Sparks: Monokrom Charcoal (`#1E2533`) & Silver-Slate (`#8A99B5`) dengan sentuhan Muted Wine-Rose tipis (`#A83244`) yang tidak mencolok.

4. **Ganti Mode Idle Mascot (`src/mascot/eyes.rs`) menjadi Mata Asta (Devil Union Mode) Versi Neutral-Toned**:
   - Saat state berada di `NotchState::Pet`, render sepasang mata **Asta Devil Union Mode** menggunakan geometri vektor Direct2D (`ID2D1PathGeometry` & `ID2D1RadialGradientBrush`):
     - Aura luar menggunakan *Smoky Slate-Charcoal* (`#1E2533` & `#6488EA` tipis) sehingga tetap netral di desktop biru/abu-abu.
     - Iris menggunakan *Muted Crimson / Wine-Rose* (`#A83244` ke `#591622`) dengan potongan kelopak atas miring tajam (*sharp anime upper eyelid*).
     - Pupil di tengah iris berbentuk **tanda plus (`+`) meruncing (tapered cross)** berwarna hitam obsidian (`#07090D`) dengan cincin tipis *silver-rose* yang mengikuti kursor (*cursor tracking* dalam radius 360px).
     - Mendukung siklus `Awakening`, `Active` (cursor tracking + organic glance + sharp blink), `Dozing` (setelah 45 detik idle), dan `WakeSurge`.

5. **Tambahkan Interaksi Selektif Chibi Liebe (`src/mascot/chibi_liebe.rs` & `src/mascot/mod.rs`)**:
   - Buat modul `src/mascot/chibi_liebe.rs` yang menggambar karakter **Chibi Liebe** secara prosedural dengan Direct2D (rambut runcing warna *ash-silver* `#9AA5B8`, tanduk hitam melengkung dengan aksen garis abu-abu, sayap kelelawar mungil di punggung, dan mata kecil ekspresi *deadpan/smug*).
   - Tampilkan Chibi Liebe **HANYA pada 3 state berikut** (jangan tampilkan di state lain agar tetap minimalis):
     a. `NotchState::Music`: Pose `Headbop` — duduk di sudut kanan atas panel musik sambil mengangguk-angguk mengikuti irama dan mengepakkan sayap kecilnya.
     b. `NotchState::Pomo`: Pose `FocusGuard` — berdiri melipat tangan di dekat *timer* Pomodoro untuk menemani sesi fokus.
     c. `NotchState::Shelf`: Pose `CatchingFiles` — mengintip dari bawah area *drop zone* sambil mengangkat kedua tangan kecilnya saat pengguna membuka Shelf.
   - Berikan reaksi *click/poke*: jika pengguna mengklik area Chibi Liebe, picu ekspresi kaget/kesal lucu (`Poked` selama 0.8 detik dengan loncatan kecil dan simbol perempatan siku di kepala).

6. **Verifikasi Kompilasi**:
   - Hubungkan semua modul di `src/mascot/mod.rs`, `src/notch/states.rs`, `src/notch/renderer.rs`, dan `src/notch/mod.rs`, lalu jalankan `cargo check` dan `cargo build --release` hingga sukses tanpa error.
```

---

## 4. Konfigurasi `Cargo.toml` yang Direkomendasikan

Pastikan `Cargo.toml` kamu menyertakan *feature flags* `windows` v0.58.0 yang dibutuhkan untuk Direct2D, DWM (transparansi penuh), serta Win32 UI:

```toml
[package]
name = "nusa-clone"
version = "0.3.0"
edition = "2021"

[dependencies]
windows = { version = "0.58.0", features = [
    "Win32_Foundation",
    "Win32_Graphics_Direct2D",
    "Win32_Graphics_Direct2D_Common",
    "Win32_Graphics_DirectWrite",
    "Win32_Graphics_Dxgi_Common",
    "Win32_Graphics_Dwm",
    "Win32_Graphics_Gdi",
    "Win32_System_Com",
    "Win32_System_LibraryLoader",
    "Win32_System_Performance",
    "Win32_UI_Controls",
    "Win32_UI_HiDpi",
    "Win32_UI_Input_KeyboardAndMouse",
    "Win32_UI_Shell",
    "Win32_UI_WindowsAndMessaging",
] }
```

---

## 5. Kode Modul Lengkap Siap Integrasi

Berikut adalah **5 file modul inti** yang sudah disesuaikan dengan palet **Neutral Slate-Blue Anti-Magic** serta tambahan **Chibi Liebe**:

### A. `src/mascot/mod.rs` — Pendaftaran Modul Maskot

```rust
// src/mascot/mod.rs
pub mod chibi_liebe;
pub mod eyes;
```

---

### B. `src/mascot/eyes.rs` — Mata Asta Devil Union Mode (Neutral Slate-Blue & Muted Crimson Edition)

Aura dan partikel *Anti-Magic* kini menggunakan kombinasi **Obsidian Charcoal (`#141923`)** dan **Cool Steel Slate (`#7A8CA8`)**, sedangkan iris mata menggunakan **Muted Wine-Crimson (`#A83244`)** yang elegan dan tidak mencolok di desktop bernuansa biru/abu-abu.

```rust
// src/mascot/eyes.rs
use windows::Win32::Graphics::Direct2D::Common::{
    D2D1_COLOR_F, D2D1_FIGURE_BEGIN_FILLED, D2D1_FIGURE_END_CLOSED, D2D1_GRADIENT_STOP,
    D2D_POINT_2F, D2D_RECT_F,
};
use windows::Win32::Graphics::Direct2D::{
    ID2D1Factory, ID2D1HwndRenderTarget, ID2D1PathGeometry, D2D1_ELLIPSE, D2D1_EXTEND_MODE_CLAMP,
    D2D1_GAMMA_2_2, D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES, D2D1_ROUNDED_RECT,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DevilEyePhase {
    Awakening { timer: f32 },
    Active { idle_seconds: f32 },
    Dozing { timer: f32 },
    WakeSurge { timer: f32 },
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
    pub kind: u8, // 0 = Obsidian Charcoal, 1 = Cool Slate-Blue, 2 = Muted Rose Ember
}

pub struct DevilUnionEyes {
    pub phase: DevilEyePhase,
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
            phase: DevilEyePhase::Awakening { timer: 0.0 },
            time: 0.0,
            look_x: 0.0,
            look_y: 0.0,
            target_look_x: 0.0,
            target_look_y: 0.0,
            openness: 0.05,
            blink_timer: 0.0,
            next_blink_in: 3.2,
            glance_timer: 2.0,
            aura_intensity: 1.1,
            pupil_scale: 1.25,
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
            rng_seed: 0xA57A_BC02,
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
        let rx = (self.next_f32() - 0.5) * 18.0 + side * 15.0;
        let ry = (self.next_f32() - 0.5) * 7.0;
        let vx = (self.next_f32() - 0.5) * 12.0 + side * 3.5;
        let vy = -9.0 - self.next_f32() * 15.0;
        let max_life = 0.50 + self.next_f32() * 0.65;
        let life = if random_life {
            self.next_f32() * max_life
        } else {
            0.0
        };
        let size = 1.3 + self.next_f32() * 2.0;
        // Mayoritas partikel berwarna Slate/Charcoal netral, hanya 1 dari 5 yang memiliki aksen Muted Rose
        let kind = if idx % 5 == 0 {
            2
        } else if idx % 2 == 0 {
            1
        } else {
            0
        };

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
        self.phase = DevilEyePhase::Awakening { timer: 0.0 };
        self.openness = 0.05;
        self.pupil_scale = 1.30;
        self.aura_intensity = 1.25;
    }

    pub fn trigger_wake_surge(&mut self) {
        if matches!(self.phase, DevilEyePhase::Dozing { .. }) {
            self.phase = DevilEyePhase::WakeSurge { timer: 0.0 };
            self.aura_intensity = 1.35;
        }
    }

    pub fn update(&mut self, dt: f32, cursor_offset: Option<(f32, f32)>) {
        let dt = dt.clamp(0.001, 0.05);
        self.time += dt;

        let mut cursor_near = false;
        if let Some((dx, dy)) = cursor_offset {
            let dist = (dx * dx + dy * dy).sqrt();
            if dist < 360.0 {
                cursor_near = true;
                self.target_look_x = (dx / 180.0).clamp(-1.0, 1.0);
                self.target_look_y = (dy / 110.0).clamp(-0.75, 0.85);
                if matches!(self.phase, DevilEyePhase::Dozing { .. }) {
                    self.trigger_wake_surge();
                }
            }
        }

        let target_openness: f32;
        let target_aura: f32;
        let target_pupil: f32;

        match &mut self.phase {
            DevilEyePhase::Awakening { timer } => {
                *timer += dt;
                let p = (*timer / 0.45).clamp(0.0, 1.0);
                target_openness = if p < 0.6 { p / 0.6 * 1.10 } else { 1.0 }.min(1.04);
                target_pupil = 1.30 - 0.30 * p;
                target_aura = 1.25 - 0.35 * p;
                if *timer >= 0.45 {
                    self.phase = DevilEyePhase::Active { idle_seconds: 0.0 };
                }
            }
            DevilEyePhase::Active { idle_seconds } => {
                if cursor_near {
                    *idle_seconds = 0.0;
                    target_aura = 1.05;
                    target_pupil = 0.88;
                } else {
                    *idle_seconds += dt;
                    target_aura = 0.72 + 0.12 * (self.time * 2.8).sin();
                    target_pupil = 1.0 + 0.05 * (self.time * 2.0).cos();

                    self.glance_timer -= dt;
                    if self.glance_timer <= 0.0 {
                        let r = self.next_f32();
                        if r < 0.38 {
                            self.target_look_x = 0.0;
                            self.target_look_y = 0.0;
                        } else {
                            self.target_look_x = (self.next_f32() - 0.5) * 1.45;
                            self.target_look_y = (self.next_f32() - 0.5) * 0.55;
                        }
                        self.glance_timer = 2.0 + self.next_f32() * 3.0;
                    }

                    if *idle_seconds > 45.0 {
                        self.phase = DevilEyePhase::Dozing { timer: 0.0 };
                    }
                }
                target_openness = 1.0;
            }
            DevilEyePhase::Dozing { timer } => {
                *timer += dt;
                self.target_look_x = 0.0;
                self.target_look_y = 0.22;
                target_openness = 0.32 + 0.04 * (self.time * 1.4).sin();
                target_aura = 0.40 + 0.10 * (self.time * 1.4).sin();
                target_pupil = 1.04;
            }
            DevilEyePhase::WakeSurge { timer } => {
                *timer += dt;
                target_openness = 1.06;
                target_aura = 1.35;
                target_pupil = 0.80;
                if *timer >= 0.30 {
                    self.phase = DevilEyePhase::Active { idle_seconds: 0.0 };
                }
            }
        }

        self.next_blink_in -= dt;
        if self.next_blink_in <= 0.0 {
            self.blink_timer = 0.14;
            self.next_blink_in = 2.8 + self.next_f32() * 3.8;
        }

        let blink_factor = if self.blink_timer > 0.0 {
            self.blink_timer -= dt;
            let norm = (self.blink_timer / 0.14).clamp(0.0, 1.0);
            (norm - 0.5).abs() * 2.0
        } else {
            1.0
        };

        let effective_openness = target_openness * blink_factor;

        let smooth = (14.0 * dt).min(1.0);
        self.look_x += (self.target_look_x - self.look_x) * smooth;
        self.look_y += (self.target_look_y - self.look_y) * smooth;
        self.openness += (effective_openness - self.openness) * (22.0 * dt).min(1.0);
        self.aura_intensity += (target_aura - self.aura_intensity) * (10.0 * dt).min(1.0);
        self.pupil_scale += (target_pupil - self.pupil_scale) * (12.0 * dt).min(1.0);

        let aura_speed_mult = self.aura_intensity.max(0.45);
        for i in 0..self.sparks.len() {
            self.sparks[i].life += dt * aura_speed_mult;
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
        let center_y = (pill_rect.top + pill_rect.bottom) * 0.5;
        let eye_spacing = 17.5;

        unsafe {
            self.draw_anti_magic_sparks(rt, center_x, center_y, alpha_mult);

            for side in [-1.0_f32, 1.0_f32] {
                let base_x = center_x + side * eye_spacing;
                let base_y = center_y + 0.5;
                self.draw_single_devil_eye(factory, rt, base_x, base_y, side, alpha_mult);
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

            // Warna partikel dominan Charcoal & Cool Steel-Slate agar netral dengan tema laptop biru/abu-abu
            let color = match spark.kind {
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

    unsafe fn draw_single_devil_eye(
        &self,
        factory: &ID2D1Factory,
        rt: &ID2D1HwndRenderTarget,
        base_x: f32,
        base_y: f32,
        side: f32,
        alpha_mult: f32,
    ) {
        let shift_x = self.look_x * 4.2;
        let shift_y = self.look_y * 2.2;
        let ex = base_x + shift_x;
        let ey = base_y + shift_y;

        let eye_rx = 8.2;
        let eye_ry = (6.2 * self.openness).max(0.6);

        // --- LAYER 1: Neutral Smoky Slate-Wine Outer Glow ---
        let glow_stops = [
            D2D1_GRADIENT_STOP {
                position: 0.0,
                color: D2D1_COLOR_F {
                    r: 0.52,
                    g: 0.22,
                    b: 0.30,
                    a: 0.38 * self.aura_intensity * alpha_mult,
                },
            },
            D2D1_GRADIENT_STOP {
                position: 0.55,
                color: D2D1_COLOR_F {
                    r: 0.22,
                    g: 0.28,
                    b: 0.42,
                    a: 0.20 * self.aura_intensity * alpha_mult,
                },
            },
            D2D1_GRADIENT_STOP {
                position: 1.0,
                color: D2D1_COLOR_F {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.0,
                },
            },
        ];

        if let Ok(stop_collection) =
            rt.CreateGradientStopCollection(&glow_stops, D2D1_GAMMA_2_2, D2D1_EXTEND_MODE_CLAMP)
        {
            let props = D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES {
                center: D2D_POINT_2F { x: ex, y: ey },
                gradientOriginOffset: D2D_POINT_2F { x: 0.0, y: 0.0 },
                radiusX: eye_rx * 1.75,
                radiusY: (eye_ry * 1.85).max(3.0),
            };
            if let Ok(glow_brush) = rt.CreateRadialGradientBrush(&props, None, &stop_collection) {
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F { x: ex, y: ey },
                        radiusX: eye_rx * 1.75,
                        radiusY: (eye_ry * 1.85).max(3.0),
                    },
                    &glow_brush,
                );
            }
        }

        // Saat berkedip, tampilkan garis tipis Silver-Rose yang elegan
        if self.openness < 0.16 {
            let slit_color = D2D1_COLOR_F {
                r: 0.68,
                g: 0.32,
                b: 0.40,
                a: 0.85 * alpha_mult,
            };
            if let Ok(brush) = rt.CreateSolidColorBrush(&slit_color, None) {
                let rect = D2D1_ROUNDED_RECT {
                    rect: D2D_RECT_F {
                        left: ex - eye_rx,
                        top: ey - 0.8,
                        right: ex + eye_rx,
                        bottom: ey + 0.8,
                    },
                    radiusX: 0.8,
                    radiusY: 0.8,
                };
                rt.FillRoundedRectangle(&rect, &brush);
            }
            return;
        }

        // --- LAYER 2: Muted Wine-Crimson Iris (Kalem & Tidak Mencolok) ---
        let iris_rx = 6.4;
        let iris_ry = (5.6 * self.openness).max(1.2);

        let iris_stops = [
            D2D1_GRADIENT_STOP {
                position: 0.0,
                color: D2D1_COLOR_F {
                    r: 0.82,
                    g: 0.32,
                    b: 0.38,
                    a: alpha_mult,
                },
            },
            D2D1_GRADIENT_STOP {
                position: 0.68,
                color: D2D1_COLOR_F {
                    r: 0.62,
                    g: 0.16,
                    b: 0.24,
                    a: alpha_mult,
                },
            },
            D2D1_GRADIENT_STOP {
                position: 1.0,
                color: D2D1_COLOR_F {
                    r: 0.24,
                    g: 0.08,
                    b: 0.14,
                    a: alpha_mult,
                },
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

        // --- LAYER 3: Pupil Tanda Plus ('+') Khas Asta Devil Union Mode ---
        let px = base_x + shift_x * 1.18;
        let py = base_y + shift_y * 1.18;
        let arm_h = 4.6 * self.pupil_scale;
        let arm_v = (4.4 * self.pupil_scale * self.openness).max(1.0);
        let half_thick = 1.15 * self.pupil_scale;

        // Cincin tepi pupil '+' berwarna Cool Silver-Rose tipis agar bentuk '+' terbaca jelas
        if let Ok(inner_ring_geom) = self.create_tapered_plus_geometry(
            factory,
            px,
            py,
            arm_h * 1.14,
            arm_v * 1.14,
            half_thick * 1.22,
        ) {
            let ring_color = D2D1_COLOR_F {
                r: 0.85,
                g: 0.65,
                b: 0.72,
                a: 0.55 * alpha_mult,
            };
            if let Ok(ring_brush) = rt.CreateSolidColorBrush(&ring_color, None) {
                rt.FillGeometry(&inner_ring_geom, &ring_brush, None);
            }
        }

        // Inti pupil '+' Obsidian Black
        if let Ok(plus_geom) =
            self.create_tapered_plus_geometry(factory, px, py, arm_h, arm_v, half_thick)
        {
            let pupil_color = D2D1_COLOR_F {
                r: 0.04,
                g: 0.05,
                b: 0.08,
                a: alpha_mult,
            };
            if let Ok(pupil_brush) = rt.CreateSolidColorBrush(&pupil_color, None) {
                rt.FillGeometry(&plus_geom, &pupil_brush, None);
            }
        }

        // --- LAYER 4: Upper Eyelid Cut (Menyatu dengan warna Obsidian Pill #0B0E14) ---
        self.draw_sharp_upper_eyelid_mask(
            factory,
            rt,
            ex,
            ey,
            iris_rx + 1.5,
            iris_ry,
            side,
            alpha_mult,
        );

        // --- LAYER 5: Cool Ice-Blue / Silver Specular Glint ---
        let glint_color = D2D1_COLOR_F {
            r: 0.88,
            g: 0.92,
            b: 0.98,
            a: 0.72 * alpha_mult,
        };
        if let Ok(glint_brush) = rt.CreateSolidColorBrush(&glint_color, None) {
            rt.FillEllipse(
                &D2D1_ELLIPSE {
                    point: D2D_POINT_2F {
                        x: px - side * 2.1,
                        y: py - iris_ry * 0.35,
                    },
                    radiusX: 1.05,
                    radiusY: 0.75 * self.openness,
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

                // Warna sama persis dengan Obsidian Slate Pill (#0B0E14)
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

### C. `src/mascot/chibi_liebe.rs` — Komponen Interaktif Chibi Liebe (Selektif pada Music, Pomodoro & Shelf)

Modul baru ini menggambar karakter **Chibi Liebe** (wujud iblis mungil Asta) menggunakan vektor Direct2D dengan warna dominan **Ash-Silver (`#9AA5B8`)** dan **Obsidian Slate (`#161C28`)** yang sangat menyatu dengan tema biru/abu-abu laptopmu.

```rust
// src/mascot/chibi_liebe.rs
use windows::Win32::Graphics::Direct2D::Common::{
    D2D1_COLOR_F, D2D1_FIGURE_BEGIN_FILLED, D2D1_FIGURE_END_CLOSED, D2D_POINT_2F, D2D_RECT_F,
};
use windows::Win32::Graphics::Direct2D::{
    ID2D1Factory, ID2D1HwndRenderTarget, D2D1_ELLIPSE, D2D1_ROUNDED_RECT,
};

use crate::notch::states::NotchState;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LiebeAction {
    /// Tidak tampil (untuk state Idle, Pet, Lyrics, Water, Todo)
    Hidden,
    /// State Music: Duduk santai sambil headbop mengikuti irama musik
    MusicHeadbop,
    /// State Pomodoro: Berdiri melipat tangan mengawasi fokus timer
    PomoFocusGuard,
    /// State Shelf: Mengintip dari bawah drop-zone siap menangkap file
    ShelfCatchingFiles,
}

pub struct ChibiLiebe {
    pub action: LiebeAction,
    pub time: f32,
    /// Timer saat Chibi Liebe diklik/di-poke oleh pengguna (efek loncat & kesal lucu)
    pub poked_timer: f32,
    /// Posisi pusat Chibi Liebe terakhir untuk deteksi klik
    pub last_center: (f32, f32),
}

impl Default for ChibiLiebe {
    fn default() -> Self {
        Self::new()
    }
}

impl ChibiLiebe {
    pub fn new() -> Self {
        Self {
            action: LiebeAction::Hidden,
            time: 0.0,
            poked_timer: 0.0,
            last_center: (0.0, 0.0),
        }
    }

    /// Tentukan apakah Chibi Liebe aktif berdasarkan NotchState saat ini (HANYA 3 state)
    pub fn sync_with_state(&mut self, state: NotchState) {
        self.action = match state {
            NotchState::Music => LiebeAction::MusicHeadbop,
            NotchState::Pomo => LiebeAction::PomoFocusGuard,
            NotchState::Shelf => LiebeAction::ShelfCatchingFiles,
            _ => LiebeAction::Hidden,
        };
    }

    pub fn is_visible(&self) -> bool {
        self.action != LiebeAction::Hidden
    }

    pub fn update(&mut self, dt: f32) {
        self.time += dt.clamp(0.001, 0.05);
        if self.poked_timer > 0.0 {
            self.poked_timer = (self.poked_timer - dt).max(0.0);
        }
    }

    /// Cek apakah klik mouse mengenai tubuh Chibi Liebe (radius 22px)
    pub fn handle_click(&mut self, click_x: f32, click_y: f32) -> bool {
        if !self.is_visible() {
            return false;
        }
        let dx = click_x - self.last_center.0;
        let dy = click_y - self.last_center.1;
        if (dx * dx + dy * dy).sqrt() <= 22.0 {
            self.poked_timer = 0.85; // Animasi kesal/kaget selama 0.85 detik
            return true;
        }
        false
    }

    pub fn render(
        &mut self,
        factory: &ID2D1Factory,
        rt: &ID2D1HwndRenderTarget,
        pill_rect: D2D_RECT_F,
        alpha: f32,
    ) {
        if !self.is_visible() || alpha <= 0.05 {
            return;
        }

        // Tentukan koordinat anchor Chibi Liebe berdasarkan fitur aktif
        let (base_x, base_y) = match self.action {
            LiebeAction::MusicHeadbop => (pill_rect.right - 38.0, pill_rect.bottom - 34.0),
            LiebeAction::PomoFocusGuard => (pill_rect.right - 42.0, pill_rect.bottom - 36.0),
            LiebeAction::ShelfCatchingFiles => (
                (pill_rect.left + pill_rect.right) * 0.5 + 125.0,
                pill_rect.bottom - 30.0,
            ),
            LiebeAction::Hidden => return,
        };

        // Offset animasi per pose
        let (bob_x, bob_y, wing_flap) = if self.poked_timer > 0.0 {
            let jump = (self.poked_timer * 18.0).sin().abs() * -6.5;
            (0.0, jump, (self.time * 28.0).sin() * 5.0)
        } else {
            match self.action {
                LiebeAction::MusicHeadbop => {
                    // Mengangguk ritmis (~120 BPM)
                    let beat = (self.time * 6.2).sin();
                    (beat * 1.5, beat.abs() * -3.2, (self.time * 12.4).cos() * 2.5)
                }
                LiebeAction::PomoFocusGuard => {
                    // Bernapas tenang & mengawasi
                    let breath = (self.time * 2.4).sin();
                    (0.0, breath * -1.2, breath * 1.0)
                }
                LiebeAction::ShelfCatchingFiles => {
                    // Bergoyang antusias menunggu drop file
                    let sway = (self.time * 4.5).sin();
                    (sway * 2.2, (self.time * 9.0).cos() * -1.5, sway * 3.0)
                }
                LiebeAction::Hidden => (0.0, 0.0, 0.0),
            }
        };

        let cx = base_x + bob_x;
        let cy = base_y + bob_y;
        self.last_center = (cx, cy);

        unsafe {
            // Palet Chibi Liebe (Neutral Cool Grey, Ash Silver, Obsidian & Muted Wine)
            let obsidian_body = D2D1_COLOR_F { r: 0.09, g: 0.11, b: 0.16, a: alpha };
            let slate_outline = D2D1_COLOR_F { r: 0.32, g: 0.39, b: 0.52, a: alpha * 0.85 };
            let ash_silver_hair = D2D1_COLOR_F { r: 0.62, g: 0.68, b: 0.78, a: alpha };
            let skin_grey = D2D1_COLOR_F { r: 0.78, g: 0.82, b: 0.88, a: alpha };
            let horn_dark = D2D1_COLOR_F { r: 0.06, g: 0.07, b: 0.10, a: alpha };
            let muted_crimson = D2D1_COLOR_F { r: 0.72, g: 0.22, b: 0.30, a: alpha };

            let Ok(body_brush) = rt.CreateSolidColorBrush(&obsidian_body, None) else { return };
            let Ok(outline_brush) = rt.CreateSolidColorBrush(&slate_outline, None) else { return };
            let Ok(hair_brush) = rt.CreateSolidColorBrush(&ash_silver_hair, None) else { return };
            let Ok(skin_brush) = rt.CreateSolidColorBrush(&skin_grey, None) else { return };
            let Ok(horn_brush) = rt.CreateSolidColorBrush(&horn_dark, None) else { return };
            let Ok(eye_brush) = rt.CreateSolidColorBrush(&muted_crimson, None) else { return };

            // 1. Sayap Mungil & Ekor Panah di Belakang Tubuh
            for side in [-1.0_f32, 1.0_f32] {
                if let Ok(wing_geom) = factory.CreatePathGeometry() {
                    if let Ok(sink) = wing_geom.Open() {
                        sink.BeginFigure(
                            D2D_POINT_2F { x: cx + side * 4.0, y: cy + 5.0 },
                            D2D1_FIGURE_BEGIN_FILLED,
                        );
                        sink.AddLines(&[
                            D2D_POINT_2F {
                                x: cx + side * 13.5,
                                y: cy + 1.0 - wing_flap,
                            },
                            D2D_POINT_2F {
                                x: cx + side * 11.0,
                                y: cy + 8.5 - wing_flap * 0.5,
                            },
                            D2D_POINT_2F { x: cx + side * 4.5, y: cy + 9.0 },
                        ]);
                        sink.EndFigure(D2D1_FIGURE_END_CLOSED);
                        let _ = sink.Close();
                        rt.FillGeometry(&wing_geom, &body_brush, None);
                        rt.DrawGeometry(&wing_geom, &outline_brush, 0.9, None);
                    }
                }
            }

            // 2. Torso Mungil Chibi
            let torso_rect = D2D1_ROUNDED_RECT {
                rect: D2D_RECT_F {
                    left: cx - 5.5,
                    top: cy + 3.5,
                    right: cx + 5.5,
                    bottom: cy + 14.5,
                },
                radiusX: 3.5,
                radiusY: 3.5,
            };
            rt.FillRoundedRectangle(&torso_rect, &body_brush);
            rt.DrawRoundedRectangle(&torso_rect, &outline_brush, 0.9, None);

            // Pose tangan sesuai state
            match self.action {
                LiebeAction::ShelfCatchingFiles => {
                    // Kedua tangan diangkat ke atas siap menangkap file
                    for side in [-1.0_f32, 1.0_f32] {
                        rt.DrawLine(
                            D2D_POINT_2F { x: cx + side * 5.0, y: cy + 6.0 },
                            D2D_POINT_2F { x: cx + side * 10.5, y: cy - 1.0 },
                            &outline_brush,
                            2.2,
                            None,
                        );
                    }
                }
                LiebeAction::PomoFocusGuard => {
                    // Tangan dilipat di dada (crossed arms)
                    rt.DrawLine(
                        D2D_POINT_2F { x: cx - 4.5, y: cy + 7.5 },
                        D2D_POINT_2F { x: cx + 4.5, y: cy + 7.5 },
                        &outline_brush,
                        2.4,
                        None,
                    );
                }
                _ => {}
            }

            // 3. Kepala Chibi Bulat
            let head_ellipse = D2D1_ELLIPSE {
                point: D2D_POINT_2F { x: cx, y: cy - 2.5 },
                radiusX: 8.5,
                radiusY: 7.5,
            };
            rt.FillEllipse(&head_ellipse, &skin_brush);

            // 4. Rambut Runcing Ash-Silver Khas Liebe
            if let Ok(hair_geom) = factory.CreatePathGeometry() {
                if let Ok(sink) = hair_geom.Open() {
                    sink.BeginFigure(
                        D2D_POINT_2F { x: cx - 9.5, y: cy - 3.0 },
                        D2D1_FIGURE_BEGIN_FILLED,
                    );
                    sink.AddLines(&[
                        D2D_POINT_2F { x: cx - 11.5, y: cy - 7.5 },
                        D2D_POINT_2F { x: cx - 6.5, y: cy - 8.5 },
                        D2D_POINT_2F { x: cx - 4.0, y: cy - 12.0 },
                        D2D_POINT_2F { x: cx, y: cy - 9.5 },
                        D2D_POINT_2F { x: cx + 4.0, y: cy - 12.0 },
                        D2D_POINT_2F { x: cx + 6.5, y: cy - 8.5 },
                        D2D_POINT_2F { x: cx + 11.5, y: cy - 7.5 },
                        D2D_POINT_2F { x: cx + 9.5, y: cy - 3.0 },
                        D2D_POINT_2F { x: cx + 4.5, y: cy - 5.5 },
                        D2D_POINT_2F { x: cx, y: cy - 3.5 },
                        D2D_POINT_2F { x: cx - 4.5, y: cy - 5.5 },
                    ]);
                    sink.EndFigure(D2D1_FIGURE_END_CLOSED);
                    let _ = sink.Close();
                    rt.FillGeometry(&hair_geom, &hair_brush, None);
                }
            }

            // 5. Sepasang Tanduk Melengkung di Kepala
            for side in [-1.0_f32, 1.0_f32] {
                if let Ok(horn_geom) = factory.CreatePathGeometry() {
                    if let Ok(sink) = horn_geom.Open() {
                        sink.BeginFigure(
                            D2D_POINT_2F { x: cx + side * 4.5, y: cy - 8.5 },
                            D2D1_FIGURE_BEGIN_FILLED,
                        );
                        sink.AddLines(&[
                            D2D_POINT_2F { x: cx + side * 9.5, y: cy - 15.5 },
                            D2D_POINT_2F { x: cx + side * 7.5, y: cy - 8.0 },
                        ]);
                        sink.EndFigure(D2D1_FIGURE_END_CLOSED);
                        let _ = sink.Close();
                        rt.FillGeometry(&horn_geom, &horn_brush, None);
                        rt.DrawGeometry(&horn_geom, &outline_brush, 0.8, None);
                    }
                }
            }

            // 6. Mata Deadpan/Smug & Garis Pipi Khas Liebe
            for side in [-1.0_f32, 1.0_f32] {
                let ex = cx + side * 3.4;
                let ey = cy - 1.5;
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F { x: ex, y: ey },
                        radiusX: 1.5,
                        radiusY: if self.poked_timer > 0.0 { 1.8 } else { 1.1 },
                    },
                    &eye_brush,
                );
                // Garis hitam vertikal tipis di bawah mata khas Liebe
                rt.DrawLine(
                    D2D_POINT_2F { x: ex, y: ey + 1.2 },
                    D2D_POINT_2F { x: ex, y: ey + 3.4 },
                    &horn_brush,
                    0.9,
                    None,
                );
            }

            // 7. Jika baru diklik (Poked): gambar simbol kesal lucu di atas kepala
            if self.poked_timer > 0.0 {
                let ax = cx + 9.0;
                let ay = cy - 13.0;
                rt.DrawLine(
                    D2D_POINT_2F { x: ax - 2.5, y: ay },
                    D2D_POINT_2F { x: ax + 2.5, y: ay },
                    &eye_brush,
                    1.3,
                    None,
                );
                rt.DrawLine(
                    D2D_POINT_2F { x: ax, y: ay - 2.5 },
                    D2D_POINT_2F { x: ax, y: ay + 2.5 },
                    &eye_brush,
                    1.3,
                    None,
                );
            }
        }
    }
}
```

---

### D. `src/notch/states.rs` — Mesin Fisika Pegas (*Spring-Damper*) + Sinkronisasi Mata Asta & Chibi Liebe

```rust
// src/notch/states.rs
use crate::mascot::chibi_liebe::ChibiLiebe;
use crate::mascot::eyes::DevilUnionEyes;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotchState {
    Pet,    // Idle mascot: Mata Asta Devil Union Mode
    Idle,   // Closed notch dengan jam/baterai
    Music,  // Expanded music player (+ Chibi Liebe Headbop)
    Lyrics, // Split / wide synced lyrics (Tanpa Chibi Liebe - bersih)
    Pomo,   // Pomodoro focus timer (+ Chibi Liebe FocusGuard)
    Water,  // Water reminder tracker (Tanpa Chibi Liebe)
    Shelf,  // Drag & drop file shelf (+ Chibi Liebe CatchingFiles)
    Todo,   // Task list (Tanpa Chibi Liebe)
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

    pub width: SpringValue,
    pub height: SpringValue,
    pub corner_radius: SpringValue,
    pub content_alpha: SpringValue,

    pub closed_quiet_timer: f32,
    pub hover_grace_timer: f32,

    pub devil_eyes: DevilUnionEyes,
    pub chibi_liebe: ChibiLiebe,
}

impl NotchController {
    pub fn new() -> Self {
        Self {
            state: NotchState::Pet,
            last_expanded_tab: NotchState::Music,
            is_hovered: false,
            minimal_mode: false,
            mascot_enabled: true,
            width: SpringValue::new(168.0, 310.0, 26.0),
            height: SpringValue::new(34.0, 310.0, 26.0),
            corner_radius: SpringValue::new(17.0, 310.0, 26.0),
            content_alpha: SpringValue::new(1.0, 240.0, 28.0),
            closed_quiet_timer: 0.0,
            hover_grace_timer: 0.0,
            devil_eyes: DevilUnionEyes::new(),
            chibi_liebe: ChibiLiebe::new(),
        }
    }

    pub fn target_dimensions_for(state: NotchState, minimal_mode: bool) -> (f32, f32, f32) {
        if minimal_mode && matches!(state, NotchState::Pet | NotchState::Idle) {
            return (120.0, 6.0, 3.0);
        }

        match state {
            NotchState::Pet => (164.0, 34.0, 17.0),
            NotchState::Idle => (186.0, 34.0, 17.0),
            NotchState::Music => (390.0, 148.0, 26.0),
            NotchState::Lyrics => (470.0, 168.0, 28.0),
            NotchState::Pomo => (370.0, 144.0, 26.0),
            NotchState::Water => (380.0, 156.0, 26.0),
            NotchState::Shelf => (420.0, 172.0, 28.0),
            NotchState::Todo => (390.0, 184.0, 28.0),
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
        if cursor_in_pill {
            self.hover_grace_timer = 0.12;
            if !self.is_hovered {
                self.is_hovered = true;
                self.devil_eyes.trigger_wake_surge();
                self.transition_to(self.last_expanded_tab);
            }
        } else if self.is_hovered {
            self.hover_grace_timer -= dt;
            if self.hover_grace_timer <= 0.0 {
                self.is_hovered = false;
                self.closed_quiet_timer = 0.0;
                if self.mascot_enabled {
                    self.transition_to(NotchState::Pet);
                } else {
                    self.transition_to(NotchState::Idle);
                }
            }
        } else if self.state == NotchState::Idle && self.mascot_enabled {
            self.closed_quiet_timer += dt;
            if self.closed_quiet_timer >= 5.0 {
                self.transition_to(NotchState::Pet);
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

    pub fn is_animating(&self) -> bool {
        !self.width.is_settled()
            || !self.height.is_settled()
            || !self.corner_radius.is_settled()
            || !self.content_alpha.is_settled()
            || self.state == NotchState::Pet
            || self.chibi_liebe.is_visible()
    }
}
```

---

### E. `src/notch/renderer.rs` — Direct2D Renderer dengan Tema Neutral Slate-Blue Anti-Magic & Chibi Liebe

```rust
// src/notch/renderer.rs
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Direct2D::Common::{
    D2D1_ALPHA_MODE_PREMULTIPLIED, D2D1_COLOR_F, D2D1_PIXEL_FORMAT, D2D_POINT_2F, D2D_RECT_F,
    D2D_SIZE_U,
};
use windows::Win32::Graphics::Direct2D::*;
use windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_B8G8R8A8_UNORM;
use windows::Win32::Graphics::Gdi::{BeginPaint, EndPaint, PAINTSTRUCT};

use super::states::{NotchController, NotchState};

/// Konstanta Palet Tema Neutral Slate-Blue Anti-Magic
pub struct AntiMagicNeutralPalette;

impl AntiMagicNeutralPalette {
    /// Obsidian Slate Background (#0B0E14)
    pub const PILL_BG: D2D1_COLOR_F = D2D1_COLOR_F {
        r: 0.043,
        g: 0.055,
        b: 0.078,
        a: 0.97,
    };
    /// Dark Slate Card / Inner Surface (#131822)
    pub const CARD_SURFACE: D2D1_COLOR_F = D2D1_COLOR_F {
        r: 0.075,
        g: 0.094,
        b: 0.133,
        a: 0.92,
    };
    /// Cool Steel Border (#263042)
    pub const STEEL_BORDER: D2D1_COLOR_F = D2D1_COLOR_F {
        r: 0.149,
        g: 0.188,
        b: 0.259,
        a: 0.55,
    };
    /// Steel Blue Accent (#6488EA)
    pub const ACCENT_BLUE: D2D1_COLOR_F = D2D1_COLOR_F {
        r: 0.392,
        g: 0.533,
        b: 0.918,
        a: 0.95,
    };
}

pub struct NotchRenderer {
    pub factory: ID2D1Factory,
    pub rt: ID2D1HwndRenderTarget,
}

impl NotchRenderer {
    pub fn new(hwnd: HWND, width: u32, height: u32) -> windows::core::Result<Self> {
        unsafe {
            let factory: ID2D1Factory =
                D2D1CreateFactory(D2D1_FACTORY_TYPE_SINGLE_THREADED, None)?;

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
                pixelSize: D2D_SIZE_U { width, height },
                presentOptions: D2D1_PRESENT_OPTIONS_IMMEDIATELY,
            };

            let rt = factory.CreateHwndRenderTarget(&render_props, &hwnd_props)?;
            rt.SetAntialiasMode(D2D1_ANTIALIAS_MODE_PER_PRIMITIVE);

            Ok(Self { factory, rt })
        }
    }

    pub fn render_frame(&self, controller: &mut NotchController, pill_rect: D2D_RECT_F) {
        unsafe {
            let mut ps = PAINTSTRUCT::default();
            let hwnd = self.rt.GetHwnd();
            let _ = BeginPaint(hwnd, &mut ps);

            self.rt.BeginDraw();

            // 1. Bersihkan canvas transparan penuh
            self.rt.Clear(Some(&D2D1_COLOR_F {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            }));

            let rounded_pill = D2D1_ROUNDED_RECT {
                rect: pill_rect,
                radiusX: controller.corner_radius.value,
                radiusY: controller.corner_radius.value,
            };

            // 2. Gambar latar belakang pill Obsidian Slate (#0B0E14)
            if let Ok(bg_brush) = self
                .rt
                .CreateSolidColorBrush(&AntiMagicNeutralPalette::PILL_BG, None)
            {
                self.rt.FillRoundedRectangle(&rounded_pill, &bg_brush);
            }

            // 3. Gambar tepi luar (Cool Steel Grey dengan sedikit denyut Slate-Wine saat mode Pet)
            let border_color = if controller.state == NotchState::Pet {
                let pulse = controller.devil_eyes.aura_intensity.clamp(0.3, 1.2);
                D2D1_COLOR_F {
                    r: 0.24 * pulse,
                    g: 0.22,
                    b: 0.34,
                    a: 0.45,
                }
            } else {
                AntiMagicNeutralPalette::STEEL_BORDER
            };

            if let Ok(border_brush) = self.rt.CreateSolidColorBrush(&border_color, None) {
                self.rt
                    .DrawRoundedRectangle(&rounded_pill, &border_brush, 1.15, None);
            }

            // 4. Render konten berdasarkan state aktif
            if controller.state == NotchState::Pet && !controller.minimal_mode {
                let collapse_factor = ((60.0 - controller.height.value) / 26.0).clamp(0.0, 1.0);
                controller.devil_eyes.render(
                    &self.factory,
                    &self.rt,
                    pill_rect,
                    collapse_factor,
                );
            } else if controller.height.value > 65.0 {
                let content_alpha = controller.content_alpha.value.clamp(0.0, 1.0);

                // Gambar indikator bar navigasi atas bertema Slate-Blue
                self.draw_top_tab_indicator(controller.state, pill_rect, content_alpha);

                // Panggil fungsi render konten panel (Music, Lyrics, Pomo, Water, Shelf, Todo) milikmu di sini

                // Render Chibi Liebe secara selektif (HANYA tampil jika state adalah Music, Pomo, atau Shelf)
                if controller.mascot_enabled {
                    controller.chibi_liebe.render(
                        &self.factory,
                        &self.rt,
                        pill_rect,
                        content_alpha,
                    );
                }
            }

            let _ = self.rt.EndDraw(None, None);
            let _ = EndPaint(hwnd, &ps);
        }
    }

    unsafe fn draw_top_tab_indicator(
        &self,
        state: NotchState,
        pill_rect: D2D_RECT_F,
        alpha: f32,
    ) {
        let idx = match state {
            NotchState::Music => 0,
            NotchState::Lyrics => 1,
            NotchState::Pomo => 2,
            NotchState::Water => 3,
            NotchState::Shelf | NotchState::Todo => 4,
            _ => return,
        };

        let total_w = pill_rect.right - pill_rect.left;
        let slot_w = total_w / 5.0;
        let indicator_x = pill_rect.left + slot_w * (idx as f32 + 0.5);
        let indicator_y = pill_rect.top + 10.0;

        let mut accent = AntiMagicNeutralPalette::ACCENT_BLUE;
        accent.a *= alpha;

        if let Ok(brush) = self.rt.CreateSolidColorBrush(&accent, None) {
            self.rt.DrawLine(
                D2D_POINT_2F {
                    x: indicator_x - 14.0,
                    y: indicator_y,
                },
                D2D_POINT_2F {
                    x: indicator_x + 14.0,
                    y: indicator_y,
                },
                &brush,
                2.2,
                None,
            );
        }
    }
}
```

*(Catatan: Pada `src/notch/mod.rs` dari bagian sebelumnya, pastikan di dalam `on_lbutton_down` kamu juga memanggil `if self.controller.chibi_liebe.handle_click(x, y) { let _ = InvalidateRect(self.hwnd, None, false); return; }` sebelum mengecek klik tab atas, sehingga Chibi Liebe bisa di-klik/poke!)*

---

## 6. Apa yang Perlu Dilakukan oleh Hermes Agent di Lokalmu

1. **Simpan File Panduan Ini** di *root folder* proyek `nusa-clone/` milikmu sebagai `DEVIL_UNION_UPGRADE.md`.
2. **Jalankan Hermes Agent** di dalam direktori `nusa-clone/` dan berikan perintah berikut:
   > *"Baca file `DEVIL_UNION_UPGRADE.md`, lalu terapkan seluruh perbaikan interaksi window (`src/notch/mod.rs`), animasi pegas (`src/notch/states.rs`), tema Neutral Slate-Blue Anti-Magic (`src/notch/renderer.rs`), mata Asta Devil Union (`src/mascot/eyes.rs`), serta komponen selektif Chibi Liebe (`src/mascot/chibi_liebe.rs`). Pertahankan integrasi media/LRCLIB/shelf yang sudah ada, lalu jalankan `cargo build --release`."*
3. **Hasil Akhir yang Akan Kamu Dapatkan**:
   - **Notch Responsif & Animasi Selembut Sutra**: Begitu kursor mendekati bagian atas layar, mata Asta melirik mengikuti kursor; saat kursor menyentuh *pill*, *notch* membesar dengan *spring physics* mulus dan langsung menerima klik.
   - **Tema Netral menyatu dengan Laptop Biru/Abu-abu**: Warna dasar *Obsidian Slate* (`#0B0E14`), *Steel Grey* (`#263042`), dan aksen *Steel Blue* (`#6488EA`) menjaga tampilan tetap kalem, sementara iris mata Asta menggunakan *Muted Wine-Crimson* (`#A83244`) yang elegan.
   - **Chibi Liebe Selektif**: Chibi Liebe hanya menemanimu di **Music** (*headbop*), **Pomodoro** (*focus guard*), dan **Shelf** (*siap menangkap file*), serta bereaksi lucu saat kamu klik langsung tubuhnya.