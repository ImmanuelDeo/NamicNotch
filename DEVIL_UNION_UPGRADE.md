# Nusa Clone v5.0: Pure Black Base Notch, Zero-Flash Native Volume/Brightness (No App Minimize), Auto-Synced LRC Lyrics, Full-Height Cover, Daily Quote Hub, & Anime-Accurate Asta Mascot (Cheer & Devil Union Reference Edition)

Dokumen ini berisi **Diagnosis & Solusi 6 Perbaikan Utama (v5.0)**, **Prompt Eksekusi Lengkap untuk Hermes Agent**, serta **Kode Modul Rust Utuh (`src/mascot/eyes.rs`, `src/notch/states.rs`, `src/notch/renderer.rs`, `src/notch/mod.rs`)** yang siap langsung diintegrasikan ke dalam `nusa-clone/`.

---

## 1. Diagnosis & Solusi 6 Poin Perbaikan v5.0

| # | Masalah / Permintaan Kamu | Akar Penyebab Sebelumnya | Solusi Teknis di Kode Baru (`v5.0`) |
| :- | :--- | :--- | :--- |
| **1** | **Hapus Section To-Do List** | Terdapat 6 tab (`Hub`, `Music`, `Control`, `Focus`, `Shelf`, `To-Do`). | Menghapus `NotchState::Todo` sepenuhnya. Sekarang bar navigasi berisi **5 section presisi (`Hub`, `Music`, `Control`, `Focus`, `Shelf`)** dengan pembagian klik `tabs_w / 5.0` yang pas. |
| **2** | **Background Base Notch Solid Black (`#000000`) Saat Mini Maupun Full** | Base *notch* sebelumnya memakai gradien abu-biru sehingga kurang menyatu dengan bezel hitam layar. | Mengubah *fill* utama `notch_geom` menjadi **Pure Solid Black (`#000000`, `r: 0.0, g: 0.0, b: 0.0, a: 1.0`)** baik saat tertutup (mini) maupun saat dibuka penuh (full), sementara kartu-kartu di dalamnya tetap menggunakan tema *semi-glass* bergradien modern. |
| **3** | **Ganti Notifikasi di `Hub` dengan Quotes / Word of the Day** | Bagian kanan `Hub` sebelumnya menampilkan daftar notifikasi statis. | Mengganti kartu kanan di `Hub` menjadi **Daily Motivational Quote & Word of the Day** yang otomatis berganti setiap hari berdasarkan `Local::now().ordinal()` (dan bisa di-klik atau di-*scroll* untuk melihat kutipan inspiratif lainnya). |
| **4** | **Cover Musik Full-Height & Lirik Otomatis Berjalan Sesuai Bagian Lagu** | • Ukuran tinggi kotak *cover* musik sebelumnya hanya `64px` sehingga menyisakan ruang kosong di bawahnya.<br>• Tag waktu `[mm:ss.xx]` pada lirik LRCLIB sebelumnya dihapus saat *parsing*, sehingga lirik tidak tahu detik ke berapa harus berganti. | • **Full-Height Cover**: Kotak *cover* musik di sebelah kiri kini merentang penuh dari `card.top + 10.0` hingga **`card.bottom - 10.0`** (menyatu penuh dengan tinggi kontainer kartu), sementara kontrol dan *progress bar* disusun rapi di kolom tengah.<br>• **Real Timestamp LRC Sync (`Vec<(f32, String)>`)**: Menyimpan *timestamp* asli setiap baris lirik (`[01:23.45]` → `83.45` detik) dan mencocokkannya secara *real-time* dengan `media_elapsed_exact` (`elapsed_secs + subsecond_dt`) sehingga **lirik berganti otomatis tepat mengikuti detik vokal lagu**! |
| **5** | **PENTING: Hilangkan Terminal Pop-up Flash & Cegah App/Game Minimize Saat Klik/Scroll Volume & Brightness** | • Pemanggilan `Command::new("powershell")` tanpa flag `CREATE_NO_WINDOW` (`0x08000000`) membuat jendela konsol hitam berkedip muncul setiap kali *slider* diklik.<br>• Window notch tidak memiliki `WS_EX_NOACTIVATE` dan tidak mencegat `WM_MOUSEACTIVATE`, sehingga setiap klik pada notch mencuri fokus (*foreground activation*) dan mem-*minimize* aplikasi/game yang sedang berjalan! | • **Zero Focus Stealing (`WS_EX_NOACTIVATE` + `MA_NOACTIVATE`)**: Menambahkan flag `WS_EX_NOACTIVATE` (`0x08000000`) pada `CreateWindowExW` dan mengembalikan `LRESULT(MA_NOACTIVATE as isize)` (`3`) saat menerima `WM_MOUSEACTIVATE`. Sekarang kamu bebas klik/scroll notch **tanpa pernah keluar atau mem-minimize aplikasi/game yang sedang aktif**!<br>• **In-Process Native Audio API (`IMMDeviceEnumerator` + `IAudioEndpointVolume`)**: Mengatur volume Windows secara langsung lewat COM API bawaan Windows (**0ms delay, tanpa PowerShell, tanpa terminal pop-up sama sekali!**).<br>• **Scroll Wheel Brightness & Volume**: Di tab `Control` maupun pada *Overlay*, kamu sekarang bisa langsung **scroll *mouse wheel* di atas angka/kartu Brightness atau Volume** untuk menaikkan/menurunkan nilainya per `5%` secara instan, dan proses WMI Brightness dijalankan di *single background worker* dengan `creation_flags(0x08000000)` (`CREATE_NO_WINDOW`) sehingga **100% hening tanpa pop-up terminal**. |
| **6** | **Redesign Maskot Asta Sesuai 2 Gambar Referensi (`Cheer Mode` & `Devil Union Crown Mode`)** | Sebelumnya hanya menggambar sepasang mata di latar kosong. | Meng-upgrade `src/mascot/eyes.rs` agar merender **wajah setengah atas (*upper-face bust*) Asta persis seperti kedua gambar referensimu**:<br>• **Mode 1 — Asta Cheer (`image_fa62bb.png`)**: Poni rambut *ash-grey* meruncing di tengah dahi (`V-bangs`), ikat kepala hitam di samping, alis pendek tebal terangkat ceria, mata besar **Hijau Olive-Emerald (`#7CB342` → `#9CCC65`)** dengan *eyeliner* hitam tebal, serta mulut terbuka tersenyum lebar (`:D`) saat menyapa!<br>• **Mode 2 — Asta Devil Union (`image_fab516.png`)**: Mahkota ikat kepala hitam bergerigi dengan **cahaya merah menyala berbentuk deretan bintang/salib 5-titik (`Red 5-Star/Cross Crown Glow`)** di dahi, **sepasang tanduk iblis hitam melengkung dengan garis cincin di kiri-kanan**, poni hitam meruncing di antara mata, mata merah menyala tajam dengan sklera putih lebar, serta **tanda silang hitam tebal (*cross-scars*) di kedua pipi bawah mata**, lengkap dengan animasi pedang *Demon-Slayer* melayang & tangan menangkap! |

---

## 2. Prompt Siap Pakai untuk Hermes Agent

Salin seluruh blok perintah di bawah ini dan berikan langsung ke **Hermes Agent** di terminal lokalmu (di folder `nusa-clone/`):

```markdown
Tolong baca file `DEVIL_UNION_UPGRADE.md` dan terapkan seluruh upgrade v5.0 (6 perbaikan utama) ke dalam repositori `nusa-clone` ini:

1. **Hapus Section To-Do & Gunakan 5 Section Presisi (`Hub`, `Music`, `Control`, `Focus`, `Shelf`)**:
   - Hapus `NotchState::Todo` dari `src/notch/states.rs`, `src/notch/renderer.rs`, dan `src/notch/mod.rs`.
   - Sesuaikan bar navigasi atas menjadi 5 tab (`Hub`, `Music`, `Control`, `Focus`, `Shelf`) beserta perhitungan klik `tabs_w / 5.0`.

2. **Ubah Background Base Notch Menjadi Solid Black (`#000000`)**:
   - Di `src/notch/renderer.rs` (`render_frame`), gambar geometri dasar notch (`notch_geom`) menggunakan warna **Pure Solid Black (`D2D1_COLOR_F { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }`)** baik saat mode mini/tertutup maupun saat dibuka penuh, agar menyatu sempurna dengan bezel atas layar. Pertahankan aksen semi-glass dan multi-stop gradient pada kartu-kartu di bagian dalamnya.

3. **Ganti Notifikasi di Section `Hub` dengan "Daily Quote / Word of the Day"**:
   - Di `draw_hub_panel` (`src/notch/renderer.rs`) dan `NotchController` (`src/notch/states.rs`), ganti daftar notifikasi di sebelah kanan jam dengan kartu **Quote / Word of the Day** yang memotivasi, otomatis berganti setiap hari berdasarkan tanggal (`Local::now().ordinal()`), dan bisa diklik atau di-scroll dengan mouse wheel untuk melihat kutipan motivasi berikutnya.

4. **Perbaiki Cover Musik Full-Height & Auto-Synced LRC Lyrics Berdasarkan Waktu Lagu**:
   - Di `draw_music_panel` (`src/notch/renderer.rs`), buat kotak cover musik/video di sebelah kiri merentang **full-height** mengikuti tinggi inner card (`top: card.top + 10.0` sampai `bottom: card.bottom - 10.0`) tanpa menyisakan ruang kosong di bawahnya. Letakkan judul, artis, progress bar, dan tombol kontrol di kolom tengah, serta panel lirik ala Spotify di kolom kanan.
   - Di `src/notch/states.rs`, simpan lirik sebagai `Vec<(f32, String)>` dengan mem-parsing tag waktu `[mm:ss.xx]` dari LRCLIB (atau mendistribusikan waktu secara proporsional terhadap `media_total_secs` bila hanya plain lyrics), lalu update `active_lyric_idx` secara otomatis setiap tick berdasarkan `media_elapsed_exact` sehingga lirik berganti sendiri secara akurat mengikuti bagian lagu yang sedang diputar!

5. **PENTING: Hilangkan Terminal Pop-Up Flash & Cegah Aplikasi/Game Minimize Saat Klik/Scroll Notch**:
   - Di `src/notch/mod.rs`, tambahkan flag `WS_EX_NOACTIVATE` pada `CreateWindowExW`, dan tambahkan handler `WM_MOUSEACTIVATE => return LRESULT(MA_NOACTIVATE as isize),` (nilai `3`) di `notch_wnd_proc`. Ini memastikan klik atau scroll pada notch TIDAK PERNAH mencuri fokus window aktif dan tidak mem-minimize game/aplikasi yang sedang berjalan!
   - Di `src/notch/states.rs`, HAPUS penggunaan PowerShell `WScript.Shell SendKeys` untuk Volume! Ganti dengan pemanggilan COM API native Windows `IMMDeviceEnumerator` + `IAudioEndpointVolume::SetMasterVolumeLevelScalar` (`windows::Win32::Media::Audio::Endpoints::IAudioEndpointVolume`) yang mengubah volume secara instan (0ms delay) tanpa membuka terminal sama sekali.
   - Untuk Brightness, gunakan worker background tunggal dengan `.creation_flags(0x08000000)` (`CREATE_NO_WINDOW`) dan dukung pengaturan nilai Brightness & Volume menggunakan **scroll mouse wheel (`WM_MOUSEWHEEL`)** langsung di atas kartu/angka Brightness maupun Volume (baik di tab `Control` maupun saat *Overlay* muncul)!

6. **Upgrade Maskot Asta Sesuai 2 Referensi Gambar (`Asta Cheer Mode` & `Asta Devil Union Crown Mode`)**:
   - Ganti `src/mascot/eyes.rs` dengan kode v5.0 dari `DEVIL_UNION_UPGRADE.md` yang menggambar:
     a. **Mode Devil Union (`image_fab516.png`)**: Mahkota hitam bergerigi di dahi dengan **cahaya merah 5-bintang/salib (`Red 5-Star/Cross Headband`)**, tanduk iblis melengkung di kiri-kanan, poni hitam meruncing di antara alis, mata merah menyala dengan sklera putih kontras, serta **tanda silang hitam tebal di kedua pipi bawah mata**, lengkap dengan animasi pedang *Demon-Slayer* melayang & tangan menangkap.
     b. **Mode Asta Cheer (`image_fa62bb.png`)**: Poni rambut *ash-grey* (`#9EA7B0`) meruncing di tengah dahi, alis pendek terangkat ceria, mata besar **Olive-Emerald (`#7CB342` → `#9CCC65`)** dengan *eyeliner* hitam tegas, dan mulut terbuka tersenyum senang (`:D`).

7. **Verifikasi Kompilasi**:
   - Pastikan feature `"Win32_Media_Audio"` dan `"Win32_Media_Audio_Endpoints"` ada di `Cargo.toml`, lalu jalankan `cargo check` dan `cargo build --release` sampai berhasil tanpa error.
```

---

## 3. Tambahan Feature Flag di `Cargo.toml` (Untuk Native Audio Volume Tanpa Terminal)

Pastikan di dalam `Cargo.toml` pada daftar `features` milik `windows = { version = "0.58", features = [ ... ] }` terdapat kedua feature audio native berikut:

```toml
    "Win32_Media_Audio",
    "Win32_Media_Audio_Endpoints",
    "Win32_System_Com",
    "Win32_System_Power",
```

---

## 4. Kode Lengkap Semua Modul (`v5.0`)

### A. `src/mascot/eyes.rs` — Redesign Maskot Asta Sesuai 2 Gambar Referensi (`Devil Union Crown Mode` & `Asta Cheer Mode` + Pedang Melayang)

Kode di bawah ini menerjemahkan kedua gambar referensimu secara langsung ke dalam vektor Direct2D:
- **Saat Mode Devil Union (`image_fab516.png`)**: Menampilkan **mahkota/ikat kepala hitam bergerigi dengan cahaya merah menyala berbentuk deretan 5 bintang/salib (`Red 5-Cross Crown`)**, **tanduk iblis melengkung beruas di kiri-kanan**, poni hitam tajam di antara mata, mata merah menyala dengan sklera putih lebar, dan **tanda silang hitam (`X-scars`) di kedua pipi bawah mata**, disertai aksi memanggil & menangkap pedang *Demon-Slayer*.
- **Saat Mode Asta Cheer (`image_fa62bb.png`)**: Menampilkan **poni rambut *ash-grey* khas Asta**, alis pendek terangkat antusias, **mata besar bulat berwarna *Olive-Emerald* (`#7CB342` → `#9CCC65`)** dengan *eyeliner* hitam tebal, serta **mulut terbuka ceria (`:D`)** dan mode lengkungan mata senang (`^ ^`)!

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
    /// Sesuai Referensi 2 (image_fab516): Devil Union dengan Mahkota 5-Salib Merah, Tanduk, Tanda Pipi & Tatapan Serius
    DevilUnionSerious { timer: f32 },
    /// Devil Union + Aksi Pedang Demon-Slayer Melayang & Tangan Asta Menangkap Gagangnya
    SwordSummonGrab { timer: f32 },
    /// Sesuai Referensi 1 (image_fa62bb): Asta Cheer Mode (Rambut Ash-Grey, Mata Olive-Emerald Besar, Mulut Senyum ':D')
    NormalEmeraldCheer { timer: f32 },
    /// Ekspresi Senang Mata Melengkung '^ ^'
    HappySmileGesture { timer: f32 },
    /// Istirahat Tenang
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
    /// 0.0 = Devil Union (Referensi 2), 1.0 = Asta Cheer Olive-Emerald (Referensi 1)
    pub cheer_blend: f32,
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
            openness: 0.2,
            blink_timer: 0.0,
            next_blink_in: 3.0,
            glance_timer: 1.8,
            aura_intensity: 1.0,
            pupil_scale: 1.0,
            cheer_blend: 0.0,
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
            rng_seed: 0xA57A_2026,
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
        let rx = (self.next_f32() - 0.5) * 32.0 + side * 20.0;
        let ry = (self.next_f32() - 0.5) * 8.0;
        let vx = (self.next_f32() - 0.5) * 10.0 + side * 3.0;
        let vy = -7.0 - self.next_f32() * 12.0;
        let max_life = 0.50 + self.next_f32() * 0.55;
        let life = if random_life {
            self.next_f32() * max_life
        } else {
            0.0
        };
        let size = 1.2 + self.next_f32() * 1.8;
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
        self.openness = 0.2;
        self.aura_intensity = 1.25;
    }

    pub fn trigger_wake_surge(&mut self) {
        self.mode = AstaIdleMode::NormalEmeraldCheer { timer: 0.0 };
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
                    self.mode = AstaIdleMode::NormalEmeraldCheer { timer: 0.0 };
                }
            }
        }

        if !cursor_near {
            self.glance_timer -= dt;
            if self.glance_timer <= 0.0 {
                let r = self.next_f32();
                if r < 0.38 {
                    self.target_look_x = 0.0;
                    self.target_look_y = 0.0;
                } else {
                    self.target_look_x = (self.next_f32() - 0.5) * 1.35;
                    self.target_look_y = (self.next_f32() - 0.5) * 0.50;
                }
                self.glance_timer = 1.8 + self.next_f32() * 2.4;
            }
        }

        let target_openness: f32;
        let target_aura: f32;
        let target_pupil: f32;
        let target_cheer: f32;

        match &mut self.mode {
            AstaIdleMode::DevilUnionSerious { timer } => {
                *timer += dt;
                target_openness = 0.98;
                target_cheer = 0.0;
                target_aura = if cursor_near {
                    1.20
                } else {
                    0.90 + 0.12 * (self.time * 3.2).sin()
                };
                target_pupil = if cursor_near { 0.88 } else { 1.0 };

                if *timer >= 6.8 {
                    if self.next_f32() < 0.58 {
                        self.mode = AstaIdleMode::SwordSummonGrab { timer: 0.0 };
                    } else {
                        self.mode = AstaIdleMode::NormalEmeraldCheer { timer: 0.0 };
                    }
                }
            }
            AstaIdleMode::SwordSummonGrab { timer } => {
                *timer += dt;
                target_openness = 1.02;
                target_cheer = 0.0;
                target_aura = 1.30;
                target_pupil = 0.84;

                let p = (*timer / 3.8).clamp(0.0, 1.0);
                self.target_look_x = 0.85 * (1.0 - (p - 0.5).abs() * 1.2).clamp(0.2, 1.0);
                self.target_look_y = -0.10 + (self.time * 4.0).sin() * 0.16;

                if *timer >= 3.8 {
                    self.mode = AstaIdleMode::NormalEmeraldCheer { timer: 0.0 };
                }
            }
            AstaIdleMode::NormalEmeraldCheer { timer } => {
                *timer += dt;
                target_openness = 1.06;
                target_cheer = 1.0;
                target_aura = 0.50;
                target_pupil = 1.05;

                if *timer >= 4.5 {
                    self.mode = AstaIdleMode::HappySmileGesture { timer: 0.0 };
                }
            }
            AstaIdleMode::HappySmileGesture { timer } => {
                *timer += dt;
                target_openness = 1.0;
                target_cheer = 1.0;
                target_aura = 0.65;
                target_pupil = 1.0;

                if *timer >= 2.2 {
                    self.mode = AstaIdleMode::DevilUnionSerious { timer: 0.0 };
                }
            }
            AstaIdleMode::Dozing { timer } => {
                *timer += dt;
                target_openness = 0.30 + 0.04 * (self.time * 1.4).sin();
                target_cheer = 0.0;
                target_aura = 0.35;
                target_pupil = 1.0;
            }
        }

        self.next_blink_in -= dt;
        if self.next_blink_in <= 0.0 {
            self.blink_timer = 0.13;
            self.next_blink_in = 2.7 + self.next_f32() * 3.3;
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
        self.cheer_blend += (target_cheer - self.cheer_blend) * (9.0 * dt).min(1.0);

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
        let center_y = (pill_rect.top + pill_rect.bottom) * 0.5 + 2.5;

        let sword_shift = if let AstaIdleMode::SwordSummonGrab { timer } = self.mode {
            let s = (timer * 2.5).min(1.0) * ((3.8 - timer) * 2.5).clamp(0.0, 1.0);
            -14.0 * s
        } else {
            0.0
        };

        let face_cx = center_x + sword_shift;
        let eye_spacing = 17.5;
        let cb = self.cheer_blend.clamp(0.0, 1.0);

        unsafe {
            self.draw_anti_magic_sparks(rt, face_cx, center_y, alpha_mult);

            // 1. Gambar Elemen Kepala / Dahi Sesuai Mode (Devil Union Crown & Horns vs Cheer Ash-Grey Hair)
            if cb < 0.55 {
                self.draw_devil_union_crown_and_horns(
                    factory,
                    rt,
                    face_cx,
                    pill_rect.top,
                    center_y,
                    eye_spacing,
                    alpha_mult * (1.0 - cb),
                );
            } else {
                self.draw_cheer_ash_grey_bangs_and_smile(
                    factory,
                    rt,
                    face_cx,
                    pill_rect.top,
                    center_y,
                    pill_rect.bottom,
                    alpha_mult * cb,
                );
            }

            // 2. Gambar Mata (Happy Arc '^ ^' atau Full Anime Reference Eye)
            if let AstaIdleMode::HappySmileGesture { timer } = self.mode {
                let bounce = (timer * 10.0).sin().abs() * -1.5;
                for side in [-1.0_f32, 1.0_f32] {
                    self.draw_happy_arc_eye(
                        factory,
                        rt,
                        face_cx + side * eye_spacing,
                        center_y + bounce,
                        side,
                        alpha_mult,
                    );
                }
            } else {
                for side in [-1.0_f32, 1.0_f32] {
                    self.draw_reference_anime_eye(
                        factory,
                        rt,
                        face_cx + side * eye_spacing,
                        center_y,
                        side,
                        alpha_mult,
                    );
                }
            }

            // 3. Jika Mode SwordSummonGrab: Gambar Pedang Demon-Slayer Melayang & Tangan Asta Menangkap!
            if let AstaIdleMode::SwordSummonGrab { timer } = self.mode {
                self.draw_floating_sword_and_grabbing_hand(
                    factory,
                    rt,
                    center_x + 40.0,
                    center_y - 1.5,
                    pill_rect.bottom,
                    timer,
                    alpha_mult,
                );
            }
        }
    }

    /// Sesuai Referensi 2 (`image_fab516.png`):
    /// Mahkota hitam bergerigi dengan deretan 5 bintang/salib merah menyala di dahi + Tanduk melengkung + Tanda silang hitam di pipi
    unsafe fn draw_devil_union_crown_and_horns(
        &self,
        factory: &ID2D1Factory,
        rt: &ID2D1HwndRenderTarget,
        cx: f32,
        top_y: f32,
        eye_y: f32,
        eye_spacing: f32,
        alpha: f32,
    ) {
        let horn_dark = D2D1_COLOR_F {
            r: 0.14,
            g: 0.15,
            b: 0.20,
            a: alpha,
        };
        let horn_ring = D2D1_COLOR_F {
            r: 0.34,
            g: 0.38,
            b: 0.48,
            a: 0.85 * alpha,
        };
        let crimson_glow = D2D1_COLOR_F {
            r: 1.0,
            g: 0.08,
            b: 0.14,
            a: alpha,
        };
        let scar_black = D2D1_COLOR_F {
            r: 0.12,
            g: 0.14,
            b: 0.18,
            a: 0.95 * alpha,
        };

        let Ok(horn_b) = rt.CreateSolidColorBrush(&horn_dark, None) else { return };
        let Ok(ring_b) = rt.CreateSolidColorBrush(&horn_ring, None) else { return };
        let Ok(red_b) = rt.CreateSolidColorBrush(&crimson_glow, None) else { return };
        let Ok(scar_b) = rt.CreateSolidColorBrush(&scar_black, None) else { return };

        // 1. Sepasang Tanduk Iblis Melengkung di Kiri & Kanan (`image_fab516.png`)
        for side in [-1.0_f32, 1.0_f32] {
            let hx = cx + side * (eye_spacing + 14.0);
            if let Ok(h_geom) = factory.CreatePathGeometry() {
                if let Ok(sink) = h_geom.Open() {
                    sink.BeginFigure(
                        D2D_POINT_2F {
                            x: hx - side * 3.0,
                            y: top_y + 16.0,
                        },
                        D2D1_FIGURE_BEGIN_FILLED,
                    );
                    sink.AddBezier(&D2D1_BEZIER_SEGMENT {
                        point1: D2D_POINT_2F {
                            x: hx + side * 6.0,
                            y: top_y + 11.0,
                        },
                        point2: D2D_POINT_2F {
                            x: hx + side * 8.5,
                            y: top_y + 4.0,
                        },
                        point3: D2D_POINT_2F {
                            x: hx + side * 4.5,
                            y: top_y + 1.0,
                        },
                    });
                    sink.AddLine(D2D_POINT_2F {
                        x: hx - side * 1.5,
                        y: top_y + 8.0,
                    });
                    sink.EndFigure(D2D1_FIGURE_END_CLOSED);
                    let _ = sink.Close();
                    rt.FillGeometry(&h_geom, &horn_b, None);
                    rt.DrawGeometry(&h_geom, &ring_b, 0.9, None);
                }
            }
        }

        // 2. Deretan 5 Bintang/Salib Merah Menyala di Dahi Khas Devil Union (`image_fab516.png`)
        let crown_y = top_y + 7.2;
        // Garis horizontal penghubung cahaya merah
        rt.DrawLine(
            D2D_POINT_2F {
                x: cx - 22.0,
                y: crown_y,
            },
            D2D_POINT_2F {
                x: cx + 22.0,
                y: crown_y,
            },
            &red_b,
            2.2,
            None,
        );

        // 5 Titik Salib/Bintang Vertikal pada Mahkota (Tengah paling tinggi)
        let star_offsets: [(f32, f32); 5] = [
            (-18.0, 2.8),
            (-9.5, 3.6),
            (0.0, 5.2),
            (9.5, 3.6),
            (18.0, 2.8),
        ];
        for (ox, half_h) in star_offsets {
            if let Ok(star_g) = self.create_tapered_plus_geometry(
                factory,
                cx + ox,
                crown_y,
                3.2,
                half_h,
                1.1,
            ) {
                rt.FillGeometry(&star_g, &red_b, None);
            }
        }

        // 3. Poni Hitam Runcing di Antara Kedua Mata (`image_fab516.png`)
        if let Ok(bang_g) = factory.CreatePathGeometry() {
            if let Ok(sink) = bang_g.Open() {
                sink.BeginFigure(
                    D2D_POINT_2F {
                        x: cx - 4.5,
                        y: crown_y + 3.5,
                    },
                    D2D1_FIGURE_BEGIN_FILLED,
                );
                sink.AddLines(&[
                    D2D_POINT_2F {
                        x: cx,
                        y: eye_y + 2.5,
                    },
                    D2D_POINT_2F {
                        x: cx + 4.5,
                        y: crown_y + 3.5,
                    },
                ]);
                sink.EndFigure(D2D1_FIGURE_END_CLOSED);
                let _ = sink.Close();
                rt.FillGeometry(&bang_g, &horn_b, None);
            }
        }

        // 4. Tanda Silang Hitam Tebal (Devil Union Cross-Scars) di Pipi Bawah Mata (`image_fab516.png`)
        for side in [-1.0_f32, 1.0_f32] {
            let ex = cx + side * eye_spacing;
            let sy = eye_y + 5.5;
            // Garis miring utama dari bawah mata ke pipi luar
            rt.DrawLine(
                D2D_POINT_2F {
                    x: ex + side * 1.5,
                    y: sy,
                },
                D2D_POINT_2F {
                    x: ex + side * 5.5,
                    y: sy + 6.5,
                },
                &scar_b,
                2.2,
                None,
            );
            // Garis silang pendek (crossbar)
            rt.DrawLine(
                D2D_POINT_2F {
                    x: ex + side * 0.5,
                    y: sy + 4.2,
                },
                D2D_POINT_2F {
                    x: ex + side * 6.2,
                    y: sy + 2.2,
                },
                &scar_b,
                1.8,
                None,
            );
        }
    }

    /// Sesuai Referensi 1 (`image_fa62bb.png`):
    /// Poni Rambut Ash-Grey Meruncing di Dahi + Mulut Terbuka Tersenyum Lebar (':D')
    unsafe fn draw_cheer_ash_grey_bangs_and_smile(
        &self,
        factory: &ID2D1Factory,
        rt: &ID2D1HwndRenderTarget,
        cx: f32,
        top_y: f32,
        eye_y: f32,
        bottom_y: f32,
        alpha: f32,
    ) {
        let ash_hair = D2D1_COLOR_F {
            r: 0.65,
            g: 0.68,
            b: 0.72,
            a: alpha,
        };
        let hair_line = D2D1_COLOR_F {
            r: 0.35,
            g: 0.38,
            b: 0.42,
            a: alpha,
        };
        let mouth_pink = D2D1_COLOR_F {
            r: 0.78,
            g: 0.45,
            b: 0.46,
            a: alpha,
        };
        let teeth_white = D2D1_COLOR_F {
            r: 0.96,
            g: 0.97,
            b: 0.99,
            a: alpha,
        };

        let Ok(hair_b) = rt.CreateSolidColorBrush(&ash_hair, None) else { return };
        let Ok(line_b) = rt.CreateSolidColorBrush(&hair_line, None) else { return };
        let Ok(mouth_b) = rt.CreateSolidColorBrush(&mouth_pink, None) else { return };
        let Ok(teeth_b) = rt.CreateSolidColorBrush(&teeth_white, None) else { return };

        // 1. Helai Poni Ash-Grey di Dahi Atas (`image_fa62bb.png`)
        if let Ok(b_geom) = factory.CreatePathGeometry() {
            if let Ok(sink) = b_geom.Open() {
                sink.BeginFigure(
                    D2D_POINT_2F {
                        x: cx - 22.0,
                        y: top_y,
                    },
                    D2D1_FIGURE_BEGIN_FILLED,
                );
                sink.AddLines(&[
                    D2D_POINT_2F {
                        x: cx - 14.0,
                        y: top_y + 9.5,
                    },
                    D2D_POINT_2F {
                        x: cx - 7.0,
                        y: top_y + 4.5,
                    },
                    D2D_POINT_2F {
                        x: cx,
                        y: eye_y - 2.0, // Helai V tengah khas Asta
                    },
                    D2D_POINT_2F {
                        x: cx + 7.0,
                        y: top_y + 4.5,
                    },
                    D2D_POINT_2F {
                        x: cx + 14.0,
                        y: top_y + 9.5,
                    },
                    D2D_POINT_2F {
                        x: cx + 22.0,
                        y: top_y,
                    },
                ]);
                sink.EndFigure(D2D1_FIGURE_END_CLOSED);
                let _ = sink.Close();
                rt.FillGeometry(&b_geom, &hair_b, None);
                rt.DrawGeometry(&b_geom, &line_b, 0.9, None);
            }
        }

        // 2. Mulut Terbuka Tersenyum Ceria (':D') di Bagian Tengah Bawah (`image_fa62bb.png`)
        let my = (eye_y + bottom_y) * 0.5 + 3.2;
        if let Ok(m_geom) = factory.CreatePathGeometry() {
            if let Ok(sink) = m_geom.Open() {
                sink.BeginFigure(
                    D2D_POINT_2F {
                        x: cx - 5.2,
                        y: my - 1.5,
                    },
                    D2D1_FIGURE_BEGIN_FILLED,
                );
                sink.AddLine(D2D_POINT_2F {
                    x: cx + 5.2,
                    y: my - 2.0,
                });
                sink.AddBezier(&D2D1_BEZIER_SEGMENT {
                    point1: D2D_POINT_2F {
                        x: cx + 5.0,
                        y: my + 4.2,
                    },
                    point2: D2D_POINT_2F {
                        x: cx - 5.0,
                        y: my + 4.2,
                    },
                    point3: D2D_POINT_2F {
                        x: cx - 5.2,
                        y: my - 1.5,
                    },
                });
                sink.EndFigure(D2D1_FIGURE_END_CLOSED);
                let _ = sink.Close();
                rt.FillGeometry(&m_geom, &mouth_b, None);
            }
        }
        // Garis gigi putih tipis di bagian atas mulut
        rt.DrawLine(
            D2D_POINT_2F {
                x: cx - 4.2,
                y: my - 1.2,
            },
            D2D_POINT_2F {
                x: cx + 4.2,
                y: my - 1.6,
            },
            &teeth_b,
            1.3,
            None,
        );
    }

    /// Gambar Mata Anime Asta Sesuai Kedua Gambar Referensi (`image_fa62bb.png` & `image_fab516.png`)
    unsafe fn draw_reference_anime_eye(
        &self,
        factory: &ID2D1Factory,
        rt: &ID2D1HwndRenderTarget,
        base_x: f32,
        base_y: f32,
        side: f32,
        alpha: f32,
    ) {
        let cb = self.cheer_blend.clamp(0.0, 1.0);
        let open = self.openness.clamp(0.08, 1.12);

        let w = 9.4;
        let h = (5.6 * open).max(0.8);

        // Kemiringan kelopak atas: Tajam mengernyit saat Devil Union (cb=0), Bulat besar saat Cheer (cb=1)
        let inner_drop = (1.0 - cb) * 2.4;
        let outer_lift = (1.0 - cb) * 1.2 + cb * 0.6;

        let inner_pt = D2D_POINT_2F {
            x: base_x - side * w,
            y: base_y + 0.5 + inner_drop * 0.35,
        };
        let outer_pt = D2D_POINT_2F {
            x: base_x + side * w,
            y: base_y - outer_lift * 0.4,
        };

        // 1. Alis Anime (Eyebrow): Tebal pendek ash-grey saat Cheer (`image_fa62bb`), Tajam menyatu saat Devil Union (`image_fab516`)
        let brow_col = D2D1_COLOR_F {
            r: 0.25 * (1.0 - cb) + 0.72 * cb,
            g: 0.27 * (1.0 - cb) + 0.75 * cb,
            b: 0.34 * (1.0 - cb) + 0.78 * cb,
            a: 0.92 * alpha,
        };
        if let Ok(bb) = rt.CreateSolidColorBrush(&brow_col, None) {
            let by_in = base_y - h - 1.5 + (1.0 - cb) * 2.2 - cb * 1.8;
            let by_out = base_y - h - 3.5;
            rt.DrawLine(
                D2D_POINT_2F {
                    x: base_x - side * (w * 0.75),
                    y: by_in,
                },
                D2D_POINT_2F {
                    x: base_x + side * (w * 0.65),
                    y: by_out,
                },
                &bb,
                2.0,
                None,
            );
        }

        if open < 0.16 {
            let lash_col = D2D1_COLOR_F {
                r: 0.90,
                g: 0.92,
                b: 0.96,
                a: 0.90 * alpha,
            };
            if let Ok(lb) = rt.CreateSolidColorBrush(&lash_col, None) {
                rt.DrawLine(inner_pt, outer_pt, &lb, 2.2, None);
            }
            return;
        }

        // 2. Sclera Putih Lebar Kontras (Sesuai Kedua Gambar Referensi)
        if let Ok(sclera_geom) = factory.CreatePathGeometry() {
            if let Ok(sink) = sclera_geom.Open() {
                sink.BeginFigure(inner_pt, D2D1_FIGURE_BEGIN_FILLED);
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

                let sclera_white = D2D1_COLOR_F {
                    r: 0.94,
                    g: 0.95,
                    b: 0.97,
                    a: 0.98 * alpha,
                };
                if let Ok(sb) = rt.CreateSolidColorBrush(&sclera_white, None) {
                    rt.FillGeometry(&sclera_geom, &sb, None);
                }
            }
        }

        // 3. Iris: Crimson Red Bulat Tajam (`image_fab516`) vs Olive-Emerald Besar (`image_fa62bb`)
        let shift_x = self.look_x * 2.8;
        let shift_y = self.look_y * 1.4;
        let ex = base_x + shift_x;
        let ey = base_y + shift_y;

        // Pada Devil Union (`image_fab516`), iris merah berukuran medium-kecil di tengah sklera putih sehingga terlihat intens!
        // Pada Cheer (`image_fa62bb`), iris Olive-Green berukuran besar memenuhi tinggi mata!
        let iris_rx = 3.8 * (1.0 - cb) + 5.2 * cb;
        let iris_ry = (3.8 * (1.0 - cb) + 5.2 * cb) * open;

        let c0 = D2D1_COLOR_F {
            r: 0.98 * (1.0 - cb) + 0.62 * cb,
            g: 0.14 * (1.0 - cb) + 0.82 * cb,
            b: 0.18 * (1.0 - cb) + 0.36 * cb,
            a: alpha,
        };
        let c1 = D2D1_COLOR_F {
            r: 0.86 * (1.0 - cb) + 0.46 * cb,
            g: 0.04 * (1.0 - cb) + 0.68 * cb,
            b: 0.08 * (1.0 - cb) + 0.24 * cb,
            a: alpha,
        };
        let c2 = D2D1_COLOR_F {
            r: 0.45 * (1.0 - cb) + 0.24 * cb,
            g: 0.02 * (1.0 - cb) + 0.42 * cb,
            b: 0.04 * (1.0 - cb) + 0.14 * cb,
            a: alpha,
        };

        let iris_stops = [
            D2D1_GRADIENT_STOP {
                position: 0.0,
                color: c0,
            },
            D2D1_GRADIENT_STOP {
                position: 0.70,
                color: c1,
            },
            D2D1_GRADIENT_STOP {
                position: 1.0,
                color: c2,
            },
        ];

        if let Ok(sc) =
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
            if let Ok(ib) = rt.CreateRadialGradientBrush(&props, None, &sc) {
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F { x: ex, y: ey },
                        radiusX: iris_rx,
                        radiusY: iris_ry,
                    },
                    &ib,
                );
            }
        }

        // 4. Pupil & Highlight Sesuai Referensi
        let pupil_dark = D2D1_COLOR_F {
            r: 0.05,
            g: 0.06,
            b: 0.09,
            a: alpha,
        };
        if cb < 0.55 {
            // Pupil '+' Devil Union
            if let Ok(plus_g) =
                self.create_tapered_plus_geometry(factory, ex, ey, 2.8, 2.8 * open, 0.85)
            {
                if let Ok(pb) = rt.CreateSolidColorBrush(&pupil_dark, None) {
                    rt.FillGeometry(&plus_g, &pb, None);
                }
            }
        } else {
            // Pupil vertikal kecil di tengah iris Olive-Green + Highlight putih bulat di kiri-atas (`image_fa62bb.png`)
            if let Ok(pb) = rt.CreateSolidColorBrush(&pupil_dark, None) {
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F { x: ex, y: ey },
                        radiusX: 1.15,
                        radiusY: 1.85 * open,
                    },
                    &pb,
                );
            }
            let glint_white = D2D1_COLOR_F {
                r: 0.99,
                g: 0.99,
                b: 1.0,
                a: 0.96 * alpha,
            };
            if let Ok(gb) = rt.CreateSolidColorBrush(&glint_white, None) {
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F {
                            x: ex - 1.8,
                            y: ey - iris_ry * 0.38,
                        },
                        radiusX: 1.55,
                        radiusY: 1.35 * open,
                    },
                    &gb,
                );
            }
        }

        // 5. Eyeliner / Garis Kelopak Atas & Bawah Hitam Tegas Khas Anime (`image_fa62bb` & `image_fab516`)
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

                let liner_col = D2D1_COLOR_F {
                    r: 0.08,
                    g: 0.09,
                    b: 0.12,
                    a: alpha,
                };
                if let Ok(lb) = rt.CreateSolidColorBrush(&liner_col, None) {
                    rt.DrawGeometry(&lash_geom, &lb, 2.5, None);
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
        _side: f32,
        alpha: f32,
    ) {
        if let Ok(geom) = factory.CreatePathGeometry() {
            if let Ok(sink) = geom.Open() {
                sink.BeginFigure(
                    D2D_POINT_2F {
                        x: ex - 7.0,
                        y: ey + 2.0,
                    },
                    D2D1_FIGURE_BEGIN_HOLLOW,
                );
                sink.AddBezier(&D2D1_BEZIER_SEGMENT {
                    point1: D2D_POINT_2F {
                        x: ex - 3.2,
                        y: ey - 5.5,
                    },
                    point2: D2D_POINT_2F {
                        x: ex + 3.2,
                        y: ey - 5.5,
                    },
                    point3: D2D_POINT_2F {
                        x: ex + 7.0,
                        y: ey + 2.0,
                    },
                });
                sink.EndFigure(D2D1_FIGURE_END_OPEN);
                let _ = sink.Close();

                let olive_col = D2D1_COLOR_F {
                    r: 0.55,
                    g: 0.82,
                    b: 0.32,
                    a: alpha,
                };
                if let Ok(b) = rt.CreateSolidColorBrush(&olive_col, None) {
                    rt.DrawGeometry(&geom, &b, 2.5, None);
                }
            }
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
        let hover_bob = (timer * 6.5).sin() * 2.0;
        let retract = if timer > 2.7 {
            ((timer - 2.7) / 1.1).clamp(0.0, 1.0).powi(2) * 18.0
        } else {
            0.0
        };

        let sword_x = sx;
        let sword_y = sy + hover_bob + retract;

        let blade_dark = D2D1_COLOR_F {
            r: 0.16,
            g: 0.19,
            b: 0.26,
            a: alpha,
        };
        let blade_edge = D2D1_COLOR_F {
            r: 0.56,
            g: 0.64,
            b: 0.78,
            a: alpha * 0.9,
        };
        let crimson_core = D2D1_COLOR_F {
            r: 0.88,
            g: 0.16,
            b: 0.24,
            a: alpha * 0.9,
        };
        let hand_obsidian = D2D1_COLOR_F {
            r: 0.10,
            g: 0.12,
            b: 0.18,
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
                        y: sword_y - 12.0,
                    },
                    D2D1_FIGURE_BEGIN_FILLED,
                );
                sink.AddLines(&[
                    D2D_POINT_2F {
                        x: sword_x + 4.0,
                        y: sword_y - 7.0,
                    },
                    D2D_POINT_2F {
                        x: sword_x + 3.2,
                        y: sword_y + 3.5,
                    },
                    D2D_POINT_2F {
                        x: sword_x - 3.2,
                        y: sword_y + 3.5,
                    },
                    D2D_POINT_2F {
                        x: sword_x - 4.0,
                        y: sword_y - 7.0,
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
                y: sword_y - 9.5,
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
                x: sword_x - 5.2,
                y: sword_y + 3.8,
            },
            D2D_POINT_2F {
                x: sword_x + 5.2,
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
                y: sword_y + 10.0,
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

            let hand_top_y = pill_bottom - (pill_bottom - (sword_y + 6.0)) * reach_progress;
            let arm_rect = D2D1_ROUNDED_RECT {
                rect: D2D_RECT_F {
                    left: sword_x - 3.0,
                    top: hand_top_y,
                    right: sword_x + 3.0,
                    bottom: pill_bottom,
                },
                radiusX: 2.0,
                radiusY: 2.0,
            };
            rt.FillRoundedRectangle(&arm_rect, &hand_b);
            rt.DrawRoundedRectangle(&arm_rect, &core_b, 0.9, None);
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

            let color = if self.cheer_blend > 0.5 {
                D2D1_COLOR_F {
                    r: 0.55,
                    g: 0.85,
                    b: 0.35,
                    a: spark_alpha * 0.50,
                }
            } else {
                match spark.kind {
                    0 => D2D1_COLOR_F {
                        r: 0.85,
                        g: 0.12,
                        b: 0.18,
                        a: spark_alpha * 0.70,
                    },
                    1 => D2D1_COLOR_F {
                        r: 0.48,
                        g: 0.56,
                        b: 0.72,
                        a: spark_alpha * 0.55,
                    },
                    _ => D2D1_COLOR_F {
                        r: 0.20,
                        g: 0.24,
                        b: 0.34,
                        a: spark_alpha * 0.75,
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

### B. `src/notch/states.rs` — 5 Section (Tanpa To-Do), Daily Motivational Quotes, Real Timestamp LRC Auto-Sync, & Zero-Flash Native Audio/Brightness Control

Perhatikan bagaimana fungsi `apply_system_volume` dan `apply_system_brightness` di bawah ini bekerja:
- **Volume**: Menggunakan **COM API Native `IMMDeviceEnumerator` + `IAudioEndpointVolume::SetMasterVolumeLevelScalar`** langsung dari memori Rust (**0ms delay, tidak ada PowerShell, tidak ada terminal pop-up, tidak ada pencurian fokus!**).
- **Brightness**: Menggunakan `CREATE_NO_WINDOW` (`0x08000000`) dan juga mendukung **scroll *mouse wheel*** langsung pada kartu atau angka Brightness/Volume!

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
use windows::Win32::UI::Shell::ShellExecuteW;
use windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;

use crate::mascot::chibi_liebe::ChibiLiebe;
use crate::mascot::eyes::DevilUnionEyes;

const CREATE_NO_WINDOW: u32 = 0x08000000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotchState {
    Pet,     // Closed mascot: Asta Devil Union Crown / Cheer Mode
    Idle,    // Closed notch: Jam Digital & Baterai iOS
    Hub,     // Tab 0: Jam Besar, Baterai iOS & Daily Motivational Quote / Word of the Day
    Music,   // Tab 1: Full-Height Cover + Controls + Auto-Synced LRC Lyrics
    Control, // Tab 2: All-in-One Control (Volume, Brightness, Wi-Fi, Energy Saver)
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
    /// Baris lirik beserta timestamp detiknya: `(timestamp_secs, lyric_text)`
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
            last_expanded_tab: NotchState::Music,
            is_hovered: false,
            minimal_mode: false,
            mascot_enabled: true,
            anim_time: 0.0,
            width: SpringValue::new(196.0, 310.0, 26.0),
            height: SpringValue::new(38.0, 310.0, 26.0),
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

    /// Background worker yang mem-poll Windows GSMTC & mem-parsing timestamp `[mm:ss.xx]` LRC secara akurat
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

    /// Parse timestamp `[mm:ss.xx]` dari LRCLIB agar lirik berganti tepat sesuai detik lagu!
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

                        // Fallback jika lagu hanya punya plainLyrics: bagi rata sepanjang durasi lagu
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
            self.width.set_target(290.0);
            self.height.set_target(46.0);
            self.corner_radius.set_target(18.0);
        }
    }

    /// 5 Section Presisi (Tanpa To-Do & Tanpa Water)
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
            NotchState::Pet => (196.0, 38.0, 16.0),
            NotchState::Idle => (216.0, 38.0, 16.0),
            NotchState::Hub => (470.0, 172.0, 24.0),
            NotchState::Music => (496.0, 174.0, 24.0),
            NotchState::Control => (460.0, 168.0, 24.0),
            NotchState::Pomo => (440.0, 162.0, 24.0),
            NotchState::Shelf => (465.0, 168.0, 24.0),
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

    /// Scroll Wheel: Atur Brightness/Volume dengan cepat atau ganti Quote/Lyrics!
    pub fn on_mouse_wheel(&mut self, delta: i16, lx: f32, pill_w: f32) {
        let step: i16 = if delta > 0 { 5 } else { -5 };

        // 1. Jika sedang di Overlay Volume/Brightness, scroll mengubah nilainya langsung!
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

        // 2. Jika sedang di tab Control, scroll di sisi kiri mengubah Volume, scroll di sisi kanan mengubah Brightness!
        if self.state == NotchState::Control {
            if lx < pill_w * 0.5 {
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

        // 3. Jika di Hub: scroll mengganti Quote inspiratif
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

        // 1. Sinkronisasi GSMTC + Interpolasi Waktu Lagu Real-Time agar Lirik Berganti Otomatis!
        if let Ok(mut guard) = self.live_media_arc.try_lock() {
            if let Some(snap) = guard.take() {
                let title_changed = self.media_title != snap.title;
                self.media_title = snap.title;
                self.media_artist = snap.artist;
                self.media_playing = snap.playing;
                if snap.total_secs > 1.0 {
                    self.media_total_secs = snap.total_secs;
                    // Sinkronkan elapsed_exact jika selisih > 1.8 detik dari posisi GSMTC
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

            // Cari baris lirik terakhir yang timestamp-nya <= media_elapsed_exact!
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

        // 2. Auto Time Peek setiap 30 menit (:00 dan :30)
        let now = Local::now();
        let hm = (now.hour(), now.minute());
        if (hm.1 == 0 || hm.1 == 30) && self.last_half_hour_marker != hm {
            self.last_half_hour_marker = hm;
            if !self.is_hovered && self.state == NotchState::Pet {
                self.closed_quiet_timer = -6.5;
                self.transition_to(NotchState::Idle);
            }
        }

        // 3. Poll Baterai & Charger
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

        // 5. Hover Expand / Collapse
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

    /// NATIVE WINDOWS COM AUDIO VOLUME (0ms Delay, Tanpa PowerShell, Tanpa Pop-up Terminal, Tidak Minimize App!)
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

    /// SILENT WMI BRIGHTNESS (Menggunakan `CREATE_NO_WINDOW = 0x08000000` sehingga TIDAK ADA Terminal Pop-Up!)
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
        let card_l = 14.0;
        let card_t = 30.0;
        let card_w = pill_w - 28.0;

        let cx = lx - card_l;
        let cy = ly - card_t;
        if cx < 0.0 || cy < 0.0 {
            return false;
        }

        match self.state {
            NotchState::Hub => {
                // Klik pada kartu Quote di sebelah kanan untuk mengganti ke kutipan motivasi berikutnya
                if cx >= 166.0 {
                    self.quote_offset = (self.quote_offset + 1) % MOTIVATIONAL_QUOTES.len();
                    return true;
                }
            }
            NotchState::Music => {
                // Kolom tengah (cx: 120..275): tombol Prev, Play/Pause, Next, Lyrics Toggle (cy: 80..128)
                if (76.0..=130.0).contains(&cy) && (114.0..=285.0).contains(&cx) {
                    if (114.0..=148.0).contains(&cx) {
                        Self::send_system_media_command("prev");
                        self.media_elapsed_exact = 0.0;
                        self.media_progress = 0.0;
                        return true;
                    } else if (150.0..=186.0).contains(&cx) {
                        Self::send_system_media_command("toggle");
                        self.media_playing = !self.media_playing;
                        return true;
                    } else if (188.0..=222.0).contains(&cx) {
                        Self::send_system_media_command("next");
                        self.media_elapsed_exact = 0.0;
                        self.media_progress = 0.0;
                        return true;
                    } else if (224.0..=282.0).contains(&cx) {
                        self.show_side_lyrics = !self.show_side_lyrics;
                        return true;
                    }
                }
                // Klik di kolom kanan (Lirik): lompat ke baris lirik berikutnya
                if cx > 282.0 && !self.timed_lyrics.is_empty() {
                    self.active_lyric_idx = (self.active_lyric_idx + 1) % self.timed_lyrics.len();
                    return true;
                }
            }
            NotchState::Control => {
                let half_w = card_w * 0.5;
                if (10.0..=64.0).contains(&cy) {
                    if cx < half_w - 5.0 {
                        let sl_l = 20.0;
                        let sl_r = half_w - 15.0;
                        let pct = (((cx - sl_l) / (sl_r - sl_l).max(1.0)).clamp(0.0, 1.0) * 100.0)
                            .round() as u8;
                        self.volume_level = pct;
                        Self::apply_system_volume(pct);
                        return true;
                    } else {
                        let sl_l = half_w + 15.0;
                        let sl_r = card_w - 20.0;
                        let pct = (((cx - sl_l) / (sl_r - sl_l).max(1.0)).clamp(0.0, 1.0) * 100.0)
                            .round() as u8;
                        self.brightness_level = pct;
                        Self::apply_system_brightness(pct);
                        return true;
                    }
                }
                if (68.0..=126.0).contains(&cy) {
                    if cx < half_w - 5.0 {
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
                    if (110.0..=204.0).contains(&cx) {
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
                if cy >= 30.0 && !self.shelf_files.is_empty() {
                    let slot_w = (card_w - 76.0) * 0.5;
                    let idx = if cx < 14.0 + slot_w { 0 } else { 1 };
                    if idx < self.shelf_files.len() {
                        let card_right_edge =
                            14.0 + (idx as f32 + 1.0) * slot_w + idx as f32 * 10.0;
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
            _ => {}
        }
        false
    }
}
```

---

### C. `src/notch/renderer.rs` — Solid Black Base Notch (`#000000`), Full-Height Cover Art, Daily Quote Card, & 5-Tab Navigation

Perhatikan pada `render_frame` di bawah ini:
1. **Base Notch Background** baik saat mini maupun saat terbuka penuh digambar dengan **`PILL_SOLID_BLACK` (`#000000`, `r: 0.0, g: 0.0, b: 0.0, a: 1.0`)** sehingga menyatu sempurna dengan bezel hitam layar laptop.
2. Pada `draw_music_panel`, **kotak Cover Musik/Video di sebelah kiri kini merentang *full-height*** dari `card.top + 10.0` hingga `card.bottom - 10.0` (`112x112` proporsional penuh mengikuti tinggi kartu) tanpa menyisakan ruang kosong di bawahnya!
3. Pada `draw_hub_panel`, sisi kanan kini menampilkan kartu **Word & Quote of the Day** dengan tipografi elegan.

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
    DWRITE_FONT_STRETCH_NORMAL, DWRITE_FONT_STYLE_ITALIC, DWRITE_FONT_STYLE_NORMAL,
    DWRITE_FONT_WEIGHT_BOLD, DWRITE_FONT_WEIGHT_MEDIUM, DWRITE_FONT_WEIGHT_SEMI_BOLD,
    DWRITE_PARAGRAPH_ALIGNMENT_CENTER, DWRITE_TEXT_ALIGNMENT_CENTER, DWRITE_TEXT_ALIGNMENT_LEADING,
};
use windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_B8G8R8A8_UNORM;
use windows::Win32::Graphics::Gdi::{BeginPaint, EndPaint, PAINTSTRUCT};

use super::states::{NotchController, NotchState, OverlayKind};

pub struct AntiMagicNeutralPalette;

impl AntiMagicNeutralPalette {
    /// SOLID BLACK (#000000) untuk Base Notch baik saat Mini maupun Full!
    pub const PILL_SOLID_BLACK: D2D1_COLOR_F = D2D1_COLOR_F {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
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
    pub fmt_quote_italic: IDWriteTextFormat,
    pub fmt_caption: IDWriteTextFormat,
    pub fmt_battery_inside: IDWriteTextFormat,
}

impl NotchRenderer {
    pub fn new(hwnd: HWND, width: u32, height: u32, dpi_scale: f32) -> windows::core::Result<Self> {
        unsafe {
            let factory: ID2D1Factory =
                D2D1CreateFactory(D2D1_FACTORY_TYPE_SINGLE_THREADED, None)?;
            let dwrite: IDWriteFactory = DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED)?;

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
                12.0,
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

            let fmt_quote_italic = dwrite.CreateTextFormat(
                w!("Segoe UI Variable Display"),
                None,
                DWRITE_FONT_WEIGHT_MEDIUM,
                DWRITE_FONT_STYLE_ITALIC,
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
                fmt_quote_italic,
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

            // 1. Gambar Base Notch dengan Warna PURE SOLID BLACK (#000000) Baik Saat Mini Maupun Full!
            if let Ok(notch_geom) =
                self.create_bezel_notch_geometry(pill_rect, controller.corner_radius.value)
            {
                if let Ok(black_brush) = self
                    .rt
                    .CreateSolidColorBrush(&AntiMagicNeutralPalette::PILL_SOLID_BLACK, None)
                {
                    self.rt.FillGeometry(&notch_geom, &black_brush, None);
                }

                // Garis tepi bawah tipis halus
                let border_color = D2D1_COLOR_F {
                    r: 0.20,
                    g: 0.26,
                    b: 0.38,
                    a: 0.45,
                };
                if let Ok(border_brush) = self.rt.CreateSolidColorBrush(&border_color, None) {
                    self.rt.DrawGeometry(&notch_geom, &border_brush, 1.2, None);
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
                let collapse_factor = ((62.0 - controller.height.value) / 24.0).clamp(0.0, 1.0);
                controller
                    .devil_eyes
                    .render(&self.factory, &self.rt, pill_rect, collapse_factor);
            } else if controller.state == NotchState::Idle && !controller.minimal_mode {
                let alpha = ((62.0 - controller.height.value) / 24.0).clamp(0.0, 1.0);
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

    /// Bar Navigasi Atas dengan 5 Section (`Hub`, `Music`, `Control`, `Focus`, `Shelf`) + Panah `<` dan `>`
    unsafe fn draw_expanded_panel_shell(
        &self,
        state: NotchState,
        pill_rect: D2D_RECT_F,
        alpha: f32,
    ) -> D2D_RECT_F {
        let tabs = ["Hub", "Music", "Control", "Focus", "Shelf"];
        let active_idx = match state {
            NotchState::Hub => 0,
            NotchState::Music => 1,
            NotchState::Control => 2,
            NotchState::Pomo => 3,
            NotchState::Shelf => 4,
            _ => 1,
        };

        let mut p_col = AntiMagicNeutralPalette::TEXT_PRIMARY;
        p_col.a *= alpha;
        let mut s_col = AntiMagicNeutralPalette::TEXT_SECONDARY;
        s_col.a *= alpha * 0.78;

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

        let tabs_left = pill_rect.left + 38.0;
        let tabs_right = pill_rect.right - 38.0;
        let slot_w = (tabs_right - tabs_left) / 5.0;

        for (i, label) in tabs.iter().enumerate() {
            let sl = tabs_left + slot_w * i as f32;
            let sr = sl + slot_w;
            let cx = (sl + sr) * 0.5;
            let is_active = i == active_idx;

            if is_active {
                self.fill_gradient_glass_card(
                    D2D_RECT_F {
                        left: sl + 4.0,
                        top: pill_rect.top + 4.0,
                        right: sr - 4.0,
                        bottom: pill_rect.top + 24.0,
                    },
                    7.0,
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
                            x: cx - 14.0,
                            y: pill_rect.top + 25.0,
                        },
                        D2D_POINT_2F {
                            x: cx + 14.0,
                            y: pill_rect.top + 25.0,
                        },
                        &lb,
                        2.0,
                        None,
                    );
                }
            }
        }

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
            1.3,
        );

        card_rect
    }

    // =========================================================================
    // SECTION 0: HUB (Jam Besar, Baterai iOS, & Daily Motivational Quote Card)
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
            right: card.left + 162.0,
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

        self.draw_ios_battery_icon(
            left_card.left + 12.0,
            left_card.bottom - 28.0,
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
                left: left_card.left + 56.0,
                top: left_card.bottom - 30.0,
                right: left_card.right - 6.0,
                bottom: left_card.bottom - 10.0,
            },
            AntiMagicNeutralPalette::ACCENT_CYAN,
        );

        // Kanan: Kartu WORD & QUOTE OF THE DAY (Semi-Glass Multi-Gradient)
        let q_card = D2D_RECT_F {
            left: left_card.right + 10.0,
            top: card.top + 10.0,
            right: card.right - 10.0,
            bottom: card.bottom - 10.0,
        };

        self.fill_gradient_glass_card(
            q_card,
            13.0,
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
            1.1,
        );

        let daily = ctrl.current_quote();
        let header = format!("✨ WORD OF THE DAY · {}", daily.word_of_the_day);
        self.draw_text_left(
            &header,
            &self.fmt_caption,
            D2D_RECT_F {
                left: q_card.left + 12.0,
                top: q_card.top + 8.0,
                right: q_card.right - 12.0,
                bottom: q_card.top + 26.0,
            },
            AntiMagicNeutralPalette::ACCENT_CYAN,
        );

        self.draw_text_left(
            daily.quote,
            &self.fmt_quote_italic,
            D2D_RECT_F {
                left: q_card.left + 12.0,
                top: q_card.top + 28.0,
                right: q_card.right - 12.0,
                bottom: q_card.bottom - 26.0,
            },
            p_col,
        );

        let footer = format!("— {}  (Click / Scroll for next)", daily.author);
        self.draw_text_left(
            &footer,
            &self.fmt_caption,
            D2D_RECT_F {
                left: q_card.left + 12.0,
                top: q_card.bottom - 24.0,
                right: q_card.right - 12.0,
                bottom: q_card.bottom - 6.0,
            },
            s_col,
        );
    }

    // =========================================================================
    // SECTION 1: MUSIC/VIDEO (Full-Height Cover Art + Controls + Auto-Synced Lyrics)
    // =========================================================================
    unsafe fn draw_music_panel(&self, ctrl: &NotchController, card: D2D_RECT_F, alpha: f32) {
        let mut p_col = AntiMagicNeutralPalette::TEXT_PRIMARY;
        p_col.a *= alpha;
        let mut s_col = AntiMagicNeutralPalette::TEXT_SECONDARY;
        s_col.a *= alpha;

        // 1. FULL-HEIGHT COVER ART CONTAINER (Merentang Penuh dari Atas ke Bawah Kartu!)
        let art_h = (card.bottom - card.top - 16.0).max(80.0);
        let art_rect = D2D_RECT_F {
            left: card.left + 8.0,
            top: card.top + 8.0,
            right: card.left + 8.0 + art_h,
            bottom: card.bottom - 8.0,
        };

        self.fill_gradient_glass_card(
            art_rect,
            13.0,
            D2D1_COLOR_F {
                r: 0.16,
                g: 0.24,
                b: 0.42,
                a: alpha,
            },
            D2D1_COLOR_F {
                r: 0.26,
                g: 0.18,
                b: 0.36,
                a: alpha,
            },
            D2D1_COLOR_F {
                r: 0.10,
                g: 0.16,
                b: 0.30,
                a: alpha,
            },
            D2D1_COLOR_F {
                r: 0.42,
                g: 0.58,
                b: 0.92,
                a: 0.70 * alpha,
            },
            1.2,
        );

        // Visualizer bar dinamis di bagian bawah Full-Height Cover
        if let Ok(eq_b) = self
            .rt
            .CreateSolidColorBrush(&AntiMagicNeutralPalette::ACCENT_CYAN, None)
        {
            let num_bars = 6;
            let total_eq_w = art_h - 24.0;
            let step_x = total_eq_w / num_bars as f32;
            for i in 0..num_bars {
                let phase = ctrl.anim_time * 6.0 + i as f32 * 1.1;
                let h = if ctrl.media_playing {
                    10.0 + phase.sin().abs() * (art_h * 0.42)
                } else {
                    6.0
                };
                let bx = art_rect.left + 14.0 + i as f32 * step_x;
                let by = art_rect.bottom - 12.0;
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
                    &eq_b,
                );
            }
        }

        // 2. Kolom Tengah: Judul, Artis, Progress Bar + Timecode, & Tombol Kontrol
        let mid_left = art_rect.right + 10.0;
        let mid_right = if ctrl.show_side_lyrics {
            card.left + 282.0
        } else {
            card.right - 50.0
        };

        self.draw_text_left(
            &ctrl.media_title,
            &self.fmt_title,
            D2D_RECT_F {
                left: mid_left,
                top: card.top + 8.0,
                right: mid_right,
                bottom: card.top + 28.0,
            },
            p_col,
        );
        self.draw_text_left(
            &ctrl.media_artist,
            &self.fmt_caption,
            D2D_RECT_F {
                left: mid_left,
                top: card.top + 28.0,
                right: mid_right,
                bottom: card.top + 46.0,
            },
            s_col,
        );

        let bar_top = card.top + 54.0;
        let bar_w = (mid_right - mid_left).max(16.0);
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
                        left: mid_left,
                        top: bar_top,
                        right: mid_right,
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
                left: mid_left,
                top: bar_top,
                right: mid_left + prog_w,
                bottom: bar_top + 4.5,
            },
            2.2,
            AntiMagicNeutralPalette::ACCENT_BLUE,
            AntiMagicNeutralPalette::ACCENT_CYAN,
            AntiMagicNeutralPalette::ACCENT_CYAN,
            AntiMagicNeutralPalette::ACCENT_CYAN,
            0.0,
        );

        let el_s = ctrl.media_elapsed_exact as u32;
        let tot_s = ctrl.media_total_secs as u32;
        let time_str = format!(
            "{:02}:{:02} / {:02}:{:02}",
            el_s / 60,
            el_s % 60,
            tot_s / 60,
            tot_s % 60
        );
        self.draw_text_left(
            &time_str,
            &self.fmt_caption,
            D2D_RECT_F {
                left: mid_left,
                top: bar_top + 6.0,
                right: mid_right,
                bottom: bar_top + 22.0,
            },
            s_col,
        );

        // Tombol Kontrol: [ |<< ] [ Play/Pause ] [ >>| ] [ Lyrics ]
        let btn_y = card.bottom - 22.0;
        let btns = [
            (mid_left + 15.0, "|<<"),
            (
                mid_left + 51.0,
                if ctrl.media_playing { "||" } else { "|>" },
            ),
            (mid_left + 87.0, ">>|"),
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
            left: mid_left + 108.0,
            top: btn_y - 11.0,
            right: mid_left + 160.0,
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

        // 3. Kolom Kanan: SPOTIFY-STYLE AUTO-SYNCED LYRICS PANE
        if ctrl.show_side_lyrics {
            let lyr_card = D2D_RECT_F {
                left: mid_right + 6.0,
                top: card.top + 8.0,
                right: card.right - 42.0,
                bottom: card.bottom - 8.0,
            };

            self.fill_gradient_glass_card(
                lyr_card,
                13.0,
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

            if !ctrl.timed_lyrics.is_empty() {
                let idx = ctrl.active_lyric_idx.min(ctrl.timed_lyrics.len() - 1);
                let prev_l = if idx > 0 {
                    ctrl.timed_lyrics[idx - 1].1.as_str()
                } else {
                    "♪ · · ·"
                };
                let curr_l = ctrl.timed_lyrics[idx].1.as_str();
                let next_l = if idx + 1 < ctrl.timed_lyrics.len() {
                    ctrl.timed_lyrics[idx + 1].1.as_str()
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
                        bottom: lyr_card.top + 82.0,
                    },
                    p_col,
                );
                self.draw_text_left(
                    next_l,
                    &self.fmt_caption,
                    D2D_RECT_F {
                        left: lyr_card.left + 10.0,
                        top: lyr_card.top + 84.0,
                        right: lyr_card.right - 8.0,
                        bottom: lyr_card.bottom - 6.0,
                    },
                    s_col,
                );
            }
        }
    }

    // =========================================================================
    // SECTION 2: ALL-IN-ONE CONTROL CENTER (Click & Scroll Wheel Supported!)
    // =========================================================================
    unsafe fn draw_control_panel(&self, ctrl: &NotchController, card: D2D_RECT_F, alpha: f32) {
        let mut p_col = AntiMagicNeutralPalette::TEXT_PRIMARY;
        p_col.a *= alpha;
        let mut s_col = AntiMagicNeutralPalette::TEXT_SECONDARY;
        s_col.a *= alpha;

        let card_w = card.right - card.left;
        let half_w = card_w * 0.5;

        let sliders = [
            (
                card.left + 10.0,
                card.left + half_w - 5.0,
                "🔊 Volume (Click/Scroll)",
                ctrl.volume_level,
            ),
            (
                card.left + half_w + 5.0,
                card.right - 10.0,
                "🔆 Brightness (Click/Scroll)",
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
    // SECTION 4: FILE SHELF
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
}
```

---

### D. `src/notch/mod.rs` — Pencegahan Aplikasi Minimize (`WS_EX_NOACTIVATE` + `MA_NOACTIVATE`), 5-Slot Navigation, & Scroll Wheel Brightness/Volume

Perhatikan penambahan **`WS_EX_NOACTIVATE`** di `CreateWindowExW` dan **`WM_MOUSEACTIVATE => return LRESULT(MA_NOACTIVATE as isize)`** di `notch_wnd_proc`: ini menjamin aplikasi/game apa pun yang sedang kamu jalankan **tidak akan pernah keluar atau ter-minimize** saat kamu mengklik atau men-*scroll* notch!

```rust
// src/notch/mod.rs
//! Notch window and rendering for Nusa Notch Clone (v5.0)

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
    pub last_cursor_logical_x: f32,
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
            last_cursor_logical_x: 0.0,
        })
    }

    pub fn current_pill_rect(&self) -> D2D_RECT_F {
        let logical_win_w = NOTCH_WINDOW_WIDTH as f32;
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
                        let step = (self.controller.media_total_secs.max(60.0))
                            / (lines.len().max(1) as f32);
                        self.controller.timed_lyrics = lines
                            .into_iter()
                            .enumerate()
                            .map(|(i, l)| (i as f32 * step, l))
                            .collect();
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

        let pill_rect = self.current_pill_rect();
        let mut pt = POINT::default();
        unsafe {
            let _ = GetCursorPos(&mut pt);
        }

        let local_logical_x = (pt.x - self.win_x) as f32 / self.dpi_scale;
        let local_logical_y = (pt.y - self.win_y) as f32 / self.dpi_scale;
        self.last_cursor_logical_x = local_logical_x;

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

        // Bar Navigasi Atas (`<`, 5 Tab: Hub/Music/Control/Focus/Shelf, dan `>`)
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
                let slot = ((lx - 38.0) / (tabs_w / 5.0)) as usize;
                let next = match slot {
                    0 => NotchState::Hub,
                    1 => NotchState::Music,
                    2 => NotchState::Control,
                    3 => NotchState::Pomo,
                    _ => NotchState::Shelf,
                };
                self.controller.transition_to(next);
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

        // PENTING: Tambahkan WS_EX_NOACTIVATE agar klik pada notch TIDAK PERNAH mencuri fokus / mem-minimize app!
        let hwnd = CreateWindowExW(
            WS_EX_TOPMOST
                | WS_EX_TOOLWINDOW
                | WS_EX_LAYERED
                | WS_EX_TRANSPARENT
                | WS_EX_ACCEPTFILES
                | WS_EX_NOACTIVATE,
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
            // PENTING: Cegah Windows mengaktifkan window notch saat diklik (agar game/app tidak minimize!)
            WM_MOUSEACTIVATE => return LRESULT(MA_NOACTIVATE as isize),
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
                let pill_rect = app.current_pill_rect();
                let pill_w = pill_rect.right - pill_rect.left;
                let lx = app.last_cursor_logical_x - pill_rect.left;
                app.controller.on_mouse_wheel(delta, lx, pill_w);
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