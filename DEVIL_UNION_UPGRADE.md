# Nusa Clone v7.0: Clean Borderless Nusa Original UI, Pomodoro 25m/5m Selector, GSMTC Music Timeline Fix, Custom App/Tray Icon, & Dynamic Idle Aura (Devil Union Crimson Lightning & Healing Green Particles)

Dokumen ini berisi **Analisis 5 Perubahan Utama (`v7.0`)**, **Prompt Siap Pakai untuk [Hermes Agent](https://github.com/nousresearch/hermes-agent)**, serta **Kode Lengkap Modul Rust (`src/icon.rs`, `src/tray.rs`, `src/mascot/eyes.rs`, `src/notch/states.rs`, `src/notch/renderer.rs`, `src/notch/mod.rs`, dan `build.rs`)** yang siap dikompilasi di lokalmu.

---

## 1. Ringkasan 5 Perbaikan & Fitur Baru di Versi 7.0

| # | Fitur / Masalah yang Diperbaiki | Akar Penyebab Sebelumnya | Solusi Arsitektur di Versi 7.0 |
| :- | :--- | :--- | :--- |
| **1** | **Desain *Full Notch* Tanpa Banyak *Card Container* (Langsung di Atas Background Hitam `#000000` Seperti [Nusa](https://rama2555.github.io/nusa/) Original)** | Sebelumnya setiap section (`Hub`, `Music`, `Control`, `Focus`, `Shelf`) membungkus konten dengan `fill_gradient_glass_card` berlapis-lapis sehingga terlihat penuh kotak. | **Direct-on-Black Layout (Sesuai Referensi `Screenshot 2026-10-25 210657.png` & `210820.png`)**:<br>• Menghapus seluruh *nested card container* di dalam notch.<br>• Konten digambar langsung di atas *base* hitam pekat (`#000000`) dengan teks putih tegas (`#FFFFFF`), abu-abu perak (`#9CA3AF`), dan aksen biru-hitam Anti-Magic (`#3B82F6` / `#38BDF8`).<br>• Tombol navigasi bulat `<` dan `>` berada di tepi kiri & kanan tengah, indikator baterai bundar hijau (`⚡`) + tanggal + jam (`Sun 25 Oct 2026 21:08`) di kanan atas, *dropzone* garis putus-putus (*dashed border*) di `Shelf`, dan layout `Music` persis seperti referensi (`00:03 ●─── -04:37` + tombol `💬`, `⏮`, `▶`, `⏭` + lirik di sisi kanan). |
| **2** | **Opsi Pomodoro: *Focus 25 Menit* vs *Break 5 Menit*** | Sebelumnya mode `Break 5m` hanya aktif otomatis setelah 25 menit selesai, belum ada tombol pilihan langsung. | **Dual Mode Selector Pills (`Focus 25m` & `Break 5m`)**:<br>Menambahkan 2 tombol pil di section `Focus` (`NotchState::Pomo`): **`🎯 Focus 25m`** dan **`☕ Break 5m`** di samping tombol **`▶ Start / ⏸ Pause`** dan **`↺ Reset`**, sehingga kamu bisa berpindah antara fokus 25 menit atau istirahat 5 menit kapan saja dengan sekali klik. |
| **3** | **Timeline Musik *Looping* di Detik ke-2 (`00:02`) & Lirik Tidak Mengikuti Lagu** | Pada API Windows GSMTC (`GetTimelineProperties()`), nilai `Position()` **tidak bertambah tiap detik secara otomatis**, melainkan membeku di posisi saat status lagu terakhir diperbarui (misal `2.0s`). Karena *poller* membaca `2.0s` setiap `900ms` dan mereset `media_elapsed_exact` setiap selisihnya `> 1.8s`, waktu selalu kembali ke `00:02` setiap mencapai `00:03`! | **Smart GSMTC Anchor & Delta Tracking + Click-to-Seek**:<br>• Menyimpan `last_raw_gsmtc_pos` di `NotchController`.<br>• Selama lagu yang sama diputar dan nilai mentah `raw_gsmtc_pos` dari Windows tidak berubah (tidak ada *seek* baru), `media_elapsed_exact` **tidak pernah di-reset ke angka lama**, melainkan terus maju mulus (`+ dt`) setiap *frame* dari `00:00` sampai akhir lagu (`-04:37`) dan menggerakkan lirik LRC tepat waktu.<br>• Menambahkan fitur klik langsung pada garis *timeline* musik untuk menggeser waktu/lirik secara manual. |
| **4** | **Ikon Aplikasi & System Tray Tidak Muncul (Hanya Ikon Default `.exe` Windows)** | Fungsi `LoadIconW(None, IDI_APPLICATION)` hanya memuat ikon standar Windows karena belum ada *custom `HICON`* atau *embedded `.ico`*. | **Runtime Win32 `HICON` Generator (`src/icon.rs`) + `build.rs` `.ico` Embedder**:<br>• Modul `src/icon.rs` membuat `HICON` 32×32 piksel bergambar **Lambang Anti-Magic Clover / Bintang Biru-Hitam** secara langsung di memori untuk **System Tray (`NOTIFYICONDATAW`)** dan **Window Icon (`WNDCLASSEXW`)**.<br>• Menambahkan `build.rs` yang otomatis men-generate file `assets/nusa.ico` agar file `.exe` di File Explorer juga memiliki ikon khusus. |
| **5** | **Animasi Background Dinamis Saat Idle (`NotchState::Pet`) Sesuai Mode Asta (Devil Union Merah-Hitam Berkilat vs Normal/Happy Hijau *Healing*)** | Sebelumnya saat *idle* tertutup, background *pill* selalu hitam statis. | **Dual Transient Idle Aura Burst (Aktif ~4 Detik Lalu Kembali Hitam Normal)**:<br>• **Saat Asta Mode Devil Union Memegang Pedang**: Selama `4.0 detik` pertama transisi/siklus, background *pill* berubah menjadi gradasi **Merah-Hitam Anti-Magic yang bergerak** disertai **kilatan petir merah (`#EF4444` / `#DC2626`)** menyambar di belakang Asta, lalu perlahan kembali ke hitam pekat (`#000000`).<br>• **Saat Asta Mode Normal & Happy Face (`Cheer Mode`)**: Selama `4.0 detik` pertama transisi/siklus, background *pill* berubah menjadi gelombang **Hijau-Hitam *Healing* yang bergerak** disertai **partikel *healing* (`+` & cahaya emerald `#10B981` / `#6EE7B7`)** yang melayang naik seperti sedang di-*heal*, lalu perlahan kembali ke hitam pekat (`#000000`). |

---

## 2. Prompt Instruksi Siap Pakai untuk Hermes Agent (Versi 7.0)

Salin seluruh blok di bawah ini dan tempelkan langsung ke **[Hermes Agent](https://github.com/nousresearch/hermes-agent)** di terminal lokalmu:

```markdown
Tolong baca panduan `DEVIL_UNION_UPGRADE.md` (Versi 7.0) dan terapkan 5 pembaruan berikut secara lengkap dan presisi:

1. **Desain Clean Tanpa Nested Card Containers (Langsung di Atas Background Hitam `#000000` Seperti Nusa Original)**:
   - Di `src/notch/renderer.rs`, hapus kotak `fill_gradient_glass_card` pembungkus utama di `draw_expanded_panel_shell` serta kotak-kotak kartu di dalam `draw_hub_panel`, `draw_music_panel`, `draw_control_panel`, `draw_pomo_panel`, dan `draw_shelf_panel`.
   - Biarkan seluruh konten tampil langsung di atas background notch hitam pekat (`#000000`) dengan teks putih (`#FFFFFF`), sekunder abu-abu (`#9CA3AF`), dan aksen biru-hitam Anti-Magic (`#3B82F6` / `#38BDF8`).
   - Di header atas (`y: 4..24`), tampilkan tab navigasi minimalis (`Hub`, `Music`, `Control`, `Focus`, `Shelf`, `Stages`) di sisi kiri-tengah dan indikator baterai bundar hijau (`⚡`) + tanggal + jam tebal (`Sun 25 Oct 2026 21:08`) di kanan atas, serta tombol bulat `<` dan `>` di tepi kiri dan kanan tengah (`cy = pill_center_y`).
   - Pada section `Music`, tampilkan cover album rounded di kiri, judul putih tebal + nama artis abu-abu, timeline bar tipis (`00:03 ●────── -04:37`), tombol `💬` (toggle lirik), `⏮`, `▶/⏸`, `⏭` di bawah, dan lirik otomatis di sebelah kanan.
   - Pada section `Shelf`, tampilkan area dropzone langsung di atas background hitam dengan garis tepi putus-putus (`dashed rounded rect`), ikon tray `📥` di tengah, teks `"Drop files here"`, dan subteks `"Drag out anytime · click to open"`.

2. **Tambahkan Pilihan Mode `Focus 25m` dan `Break 5m` di Section Pomodoro (`Focus`)**:
   - Di `src/notch/states.rs` dan `src/notch/renderer.rs`, tambahkan fungsi `set_pomo_mode(&mut self, is_break: bool)` dan gambar 2 pil pemilih mode (`"Focus 25m"` dan `"Break 5m"`) di samping tombol `"Start / Pause"` dan `"Reset"`.

3. **Perbaiki Timeline Musik yang Looping di Detik ke-2 (`00:02`) & Sinkronisasi Lirik**:
   - Di `src/notch/states.rs`, tambahkan field `last_raw_gsmtc_pos: f32` pada `NotchController`.
   - Karena `GlobalSystemMediaTransportControlsSession::GetTimelineProperties().Position()` di Windows bersifat statis di antara event seek, jangan menimpa `self.media_elapsed_exact` jika `snap.elapsed_secs` dari GSMTC masih sama dengan `self.last_raw_gsmtc_pos` (`(snap.elapsed_secs - self.last_raw_gsmtc_pos).abs() < 0.5`) dan judul lagu tidak berubah.
   - Biarkan `self.media_elapsed_exact += dt` berjalan mulus setiap frame sehingga waktu dan lirik LRC bergerak akurat dari awal hingga akhir lagu tanpa pernah melompat balik ke detik ke-2.

4. **Tambahkan Ikon Aplikasi & System Tray (`src/icon.rs`, `src/tray.rs`, & `build.rs`)**:
   - Buat modul `src/icon.rs` yang menghasilkan `HICON` 32x32 Anti-Magic Clover/Star (`create_antimagic_hicon()`) menggunakan `CreateDIBSection` + `CreateIconIndirect`.
   - Pasang `create_antimagic_hicon()` pada `NOTIFYICONDATAW.hIcon` di `src/tray.rs` dan pada `WNDCLASSEXW.hIcon` / `hIconSm` di `src/notch/mod.rs` agar ikon langsung muncul di System Tray Windows.
   - Tambahkan juga pembuatan file ikon di `build.rs` jika menggunakan resource `.ico`.

5. **Tambahkan Animasi Background Dinamis Saat Idle (`NotchState::Pet`) Sesuai Mode Asta**:
   - Di `src/mascot/eyes.rs` dan `src/notch/renderer.rs`, tambahkan `mode_aura_timer: f32` (berdurasi `4.2 detik` setiap kali Asta berganti mode di state tertutup/idle, lalu meredup kembali ke hitam normal `#000000`):
     - **Saat Asta Mode Devil Union Memegang Pedang (`!is_cheer_mode`)**: Gambar gelombang background Merah-Hitam Anti-Magic yang bergerak beserta kilatan-kilatan petir merah (`#EF4444` / `#DC2626`) di dalam pill, lalu kembali ke hitam pekat setelah `4.2 detik`.
     - **Saat Asta Mode Normal & Happy Face (`is_cheer_mode`)**: Gambar gelombang background Hijau-Hitam yang bergerak beserta partikel-partikel healing (`+` dan orb hijau `#10B981` / `#6EE7B7`) yang melayang naik seperti sedang di-heal, lalu kembali ke hitam pekat setelah `4.2 detik`.

6. **Kompilasi & Uji**:
   - Pastikan `mod icon;` terdaftar di `src/main.rs`, lalu jalankan `cargo check` dan `cargo build --release` hingga berhasil tanpa error.
```

---

## 3. Kode Lengkap Modul Rust (`v7.0`)

### A. File Baru: `src/icon.rs` — Generator Ikon Native Win32 (`HICON`) untuk System Tray & Window App

Buat file `src/icon.rs` dan tambahkan `pub mod icon;` di `src/main.rs`. Modul ini menggambar ikon 32×32 **Anti-Magic Star / Clover** (Obsidian Hitam + Biru Elektrik `#3B82F6` + Inti Cyan `#38BDF8` + Kilau Merah Anti-Magic) secara langsung di memori tanpa bergantung pada file eksternal yang rawan gagal dimuat:

```rust
// src/icon.rs
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Gdi::{
    CreateBitmap, CreateDIBSection, DeleteObject, GetDC, ReleaseDC, BITMAPINFO, BITMAPINFOHEADER,
    BI_RGB, DIB_RGB_COLORS,
};
use windows::Win32::UI::WindowsAndMessaging::{CreateIconIndirect, HICON, ICONINFO};

/// Membuat HICON 32x32 bertema Nusa Anti-Magic (Hitam Obsidian + Biru Royal + Kilat Merah/Cyan)
/// untuk digunakan di System Tray dan Window Class.
pub fn create_antimagic_hicon() -> HICON {
    unsafe {
        let size = 32i32;
        let bmi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: size,
                biHeight: -size, // Top-down DIB
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB.0,
                ..Default::default()
            },
            ..Default::default()
        };

        let mut bits: *mut std::ffi::c_void = std::ptr::null_mut();
        let hdc = GetDC(HWND::default());
        let hbm_color = CreateDIBSection(hdc, &bmi, DIB_RGB_COLORS, &mut bits, None, 0)
            .unwrap_or_default();
        ReleaseDC(HWND::default(), hdc);

        if !hbm_color.is_invalid() && !bits.is_null() {
            let pixels = std::slice::from_raw_parts_mut(bits as *mut u32, (size * size) as usize);
            let center = 15.5_f32;

            for y in 0..size {
                for x in 0..size {
                    let fx = x as f32 - center;
                    let fy = y as f32 - center;
                    let dist = (fx * fx + fy * fy).sqrt();

                    // Bentuk dasar: Rounded Shield / Lingkaran Hitam Anti-Magic
                    if dist <= 14.8 {
                        // Base Obsidian Black (#070A12)
                        let mut r = 7u32;
                        let mut g = 10u32;
                        let mut b = 18u32;
                        let a = if dist > 13.8 {
                            ((14.8 - dist).clamp(0.0, 1.0) * 255.0) as u32
                        } else {
                            255u32
                        };

                        // Ring tepi luar Biru Royal (#3B82F6)
                        if (12.2..=14.0).contains(&dist) {
                            r = 59;
                            g = 130;
                            b = 246;
                        }

                        // Bintang / Pedang Anti-Magic 4-Sudut di tengah
                        let ax = fx.abs();
                        let ay = fy.abs();
                        let star = (ax * 0.35 + ay).min(ay * 0.35 + ax);
                        if star < 3.6 && dist < 11.8 {
                            // Gradien Cyan ke Biru Royal
                            r = 56;
                            g = 189;
                            b = 248;
                        } else if star < 5.0 && dist < 10.5 {
                            r = 37;
                            g = 99;
                            b = 235;
                        }

                        // Inti Merah Devil Union di pusat bintang
                        if dist < 2.6 {
                            r = 239;
                            g = 68;
                            b = 68;
                        }

                        // Pre-multiplied BGRA (0xAARRGGBB)
                        pixels[(y * size + x) as usize] = (a << 24) | (r << 16) | (g << 8) | b;
                    } else {
                        pixels[(y * size + x) as usize] = 0x00000000;
                    }
                }
            }
        }

        let hbm_mask = CreateBitmap(size, size, 1, 1, None);
        let icon_info = ICONINFO {
            fIcon: true.into(),
            xHotspot: 0,
            yHotspot: 0,
            hbmMask: hbm_mask,
            hbmColor: hbm_color,
        };

        let hicon = CreateIconIndirect(&icon_info).unwrap_or_default();
        let _ = DeleteObject(hbm_color);
        let _ = DeleteObject(hbm_mask);
        hicon
    }
}
```

Pastikan di **`src/tray.rs`**, saat mengisi `NOTIFYICONDATAW`, kamu memanggil `crate::icon::create_antimagic_hicon()`:

```rust
// Di dalam src/tray.rs saat inisialisasi tray icon:
let hicon = crate::icon::create_antimagic_hicon();
// Pasang pada nid.hIcon = hicon; dan pastikan nid.uFlags menyertakan NIF_ICON!
```

---

### B. `src/mascot/eyes.rs` — Asta Mode Devil Union + Pedang (Aura Merah-Hitam & Kilatan Petir) vs Mode Normal Happy Face (Aura Hijau *Healing* & Partikel `+`)

Salin kode ini ke `src/mascot/eyes.rs`. Perhatikan field **`pub mode_aura_timer: f32`** yang otomatis menyala selama **`4.2 detik`** setiap kali Asta berganti mode antara **Devil Union (Memegang Pedang)** dan **Normal Cheer/Happy Face**, lalu meredup halus kembali ke hitam pekat (`#000000`):

```rust
// src/mascot/eyes.rs
use windows::Win32::Graphics::Direct2D::Common::{
    D2D1_BEZIER_SEGMENT, D2D1_COLOR_F, D2D1_FIGURE_BEGIN_FILLED, D2D1_FIGURE_BEGIN_HOLLOW,
    D2D1_FIGURE_END_CLOSED, D2D1_FIGURE_END_OPEN, D2D_POINT_2F, D2D_RECT_F,
};
use windows::Win32::Graphics::Direct2D::{
    ID2D1DeviceContext, ID2D1Factory1, D2D1_ELLIPSE, D2D1_ROUNDED_RECT,
};

pub struct DevilUnionEyes {
    pub phase_time: f32,
    pub blink_timer: f32,
    pub blink_progress: f32, // 0.0 = terbuka, 1.0 = tertutup
    pub look_x: f32,
    pub look_y: f32,
    pub target_look_x: f32,
    pub target_look_y: f32,
    pub flare_intensity: f32,

    // Siklus Ekspresi Hidup Asta + Aura Background Dinamis
    pub idle_cycle_timer: f32,
    pub is_cheer_mode: bool,    // false = Devil Union (Pedang + Petir Merah), true = Normal Happy (Healing Hijau)
    pub mode_aura_timer: f32,   // Berjalan 0.0..4.2 detik di awal pergantian mode, lalu kembali normal (hitam)
    pub sword_float_phase: f32,
    pub hand_grab_progress: f32,
}

impl DevilUnionEyes {
    pub fn new() -> Self {
        Self {
            phase_time: 0.0,
            blink_timer: 2.8,
            blink_progress: 0.0,
            look_x: 0.0,
            look_y: 0.0,
            target_look_x: 0.0,
            target_look_y: 0.0,
            flare_intensity: 1.0,
            idle_cycle_timer: 0.0,
            is_cheer_mode: false,
            mode_aura_timer: 0.0,
            sword_float_phase: 0.0,
            hand_grab_progress: 0.0,
        }
    }

    pub fn trigger_awakening(&mut self) {
        self.flare_intensity = 1.8;
        self.blink_progress = 1.0;
        self.mode_aura_timer = 0.0;
    }

    pub fn trigger_wake_surge(&mut self) {
        self.flare_intensity = 2.4;
    }

    /// Mengembalikan intensitas efek aura background (0.0 = hitam normal, 0.0..1.0 = aura aktif)
    /// Hanya aktif selama 4.2 detik pertama setiap kali Asta berganti mode, lalu kembali 0.0 (hitam normal)
    pub fn current_aura_envelope(&self) -> f32 {
        let dur = 4.2_f32;
        if self.mode_aura_timer >= dur {
            0.0
        } else if self.mode_aura_timer < 0.6 {
            (self.mode_aura_timer / 0.6).clamp(0.0, 1.0)
        } else if self.mode_aura_timer > dur - 1.1 {
            ((dur - self.mode_aura_timer) / 1.1).clamp(0.0, 1.0)
        } else {
            1.0
        }
    }

    pub fn update(&mut self, dt: f32, cursor_offset: Option<(f32, f32)>) {
        self.phase_time += dt;
        self.idle_cycle_timer += dt;
        self.mode_aura_timer += dt;
        self.sword_float_phase += dt * 3.1;

        // Berganti mode setiap 9.0 detik:
        // 0..9s   = Devil Union Mode (4.2s pertama: Aura Merah-Hitam + Kilatan Petir Merah, 4.8s berikutnya: Hitam Normal)
        // 9..18s  = Normal Happy Mode (4.2s pertama: Aura Hijau Healing + Partikel +, 4.8s berikutnya: Hitam Normal)
        let cycle = self.idle_cycle_timer % 18.0;
        let next_cheer = cycle >= 9.0;
        if next_cheer != self.is_cheer_mode {
            self.is_cheer_mode = next_cheer;
            self.mode_aura_timer = 0.0; // Picu efek background selama 4.2 detik!
            self.blink_progress = 0.85;
        }

        let grab_wave = (self.sword_float_phase * 0.75).sin();
        self.hand_grab_progress = ((grab_wave + 0.3) / 1.3).clamp(0.0, 1.0);

        if let Some((dx, dy)) = cursor_offset {
            self.target_look_x = (dx / 180.0).clamp(-1.0, 1.0) * 3.0;
            self.target_look_y = (dy / 100.0).clamp(-1.0, 1.0) * 1.4;
        } else {
            self.target_look_x = (self.phase_time * 0.9).sin() * 1.6;
            self.target_look_y = (self.phase_time * 0.6).cos() * 0.6;
        }

        let follow = (dt * 10.0).min(1.0);
        self.look_x += (self.target_look_x - self.look_x) * follow;
        self.look_y += (self.target_look_y - self.look_y) * follow;

        if self.flare_intensity > 1.0 {
            self.flare_intensity = (self.flare_intensity - dt * 2.2).max(1.0);
        }

        self.blink_timer -= dt;
        if self.blink_timer <= 0.0 {
            self.blink_progress += dt * 14.0;
            if self.blink_progress >= 1.0 {
                self.blink_progress = 0.0;
                self.blink_timer = 2.6 + (self.phase_time.sin().abs() * 2.2);
            }
        }
    }

    /// Menggambar efek background dinamis di dalam pill saat Idle (`NotchState::Pet`):
    /// - Mode Devil Union (`!is_cheer_mode`): Gelombang Merah-Hitam + Kilatan Petir Merah Anti-Magic
    /// - Mode Normal Happy (`is_cheer_mode`): Gelombang Hijau Healing + Partikel '+' & Orb Emerald
    /// Setelah 4.2 detik, `env` menjadi 0.0 sehingga background kembali Hitam Pekat (#000000) normal!
    pub unsafe fn draw_idle_aura_background(
        &self,
        rt: &ID2D1DeviceContext,
        factory: &ID2D1Factory1,
        pill_rect: D2D_RECT_F,
        alpha: f32,
    ) {
        let env = self.current_aura_envelope() * alpha;
        if env <= 0.01 {
            return;
        }

        let pw = pill_rect.right - pill_rect.left;
        let ph = pill_rect.bottom - pill_rect.top;
        let cx = (pill_rect.left + pill_rect.right) * 0.5;
        let cy = (pill_rect.top + pill_rect.bottom) * 0.5;

        if !self.is_cheer_mode {
            // =================================================================
            // 1. DEVIL UNION SWORD AURA: Merah-Hitam Bergerak + Kilatan Petir Merah
            // =================================================================
            let wave_x = (self.phase_time * 3.8).sin() * (pw * 0.22);
            let pulse = 0.70 + 0.30 * (self.phase_time * 9.5).sin().abs();

            // Lapisan kabut merah-hitam gelap di dalam pill
            if let Ok(bg_crimson) = rt.CreateSolidColorBrush(
                &D2D1_COLOR_F {
                    r: 0.24 * pulse,
                    g: 0.02,
                    b: 0.04,
                    a: 0.72 * env,
                },
                None,
            ) {
                rt.FillRoundedRectangle(
                    &D2D1_ROUNDED_RECT {
                        rect: D2D_RECT_F {
                            left: pill_rect.left + 3.0,
                            top: pill_rect.top + 2.0,
                            right: pill_rect.right - 3.0,
                            bottom: pill_rect.bottom - 2.0,
                        },
                        radiusX: 14.0,
                        radiusY: 14.0,
                    },
                    &bg_crimson,
                );
            }

            // 2 Orb energi merah-hitam yang bergerak menyapu kiri-kanan
            if let Ok(orb_brush) = rt.CreateSolidColorBrush(
                &D2D1_COLOR_F {
                    r: 0.55,
                    g: 0.05,
                    b: 0.09,
                    a: 0.48 * env * pulse,
                },
                None,
            ) {
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F {
                            x: cx + wave_x,
                            y: cy,
                        },
                        radiusX: pw * 0.30,
                        radiusY: ph * 0.46,
                    },
                    &orb_brush,
                );
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F {
                            x: cx - wave_x * 0.7,
                            y: cy,
                        },
                        radiusX: pw * 0.22,
                        radiusY: ph * 0.40,
                    },
                    &orb_brush,
                );
            }

            // Kilatan-kilatan Petir Merah Anti-Magic (Red Lightning Flashes)
            let flash_trigger = ((self.phase_time * 13.0).sin() + (self.phase_time * 23.0).cos()) * 0.5;
            if flash_trigger > -0.15 {
                let bolt_alpha = ((flash_trigger + 0.2) * 1.2).clamp(0.25, 1.0) * env;
                if let Ok(bolt_brush) = rt.CreateSolidColorBrush(
                    &D2D1_COLOR_F {
                        r: 0.96,
                        g: 0.20,
                        b: 0.26,
                        a: bolt_alpha,
                    },
                    None,
                ) {
                    // Gambar 3 sambaran petir zig-zag di kiri, tengah, dan kanan
                    let offsets = [-pw * 0.30, 0.0, pw * 0.26];
                    for (idx, ox) in offsets.iter().enumerate() {
                        let phase_shift = (self.phase_time * 16.0 + idx as f32 * 2.1).sin();
                        let bx = cx + ox + phase_shift * 8.0;
                        let by = pill_rect.top + 5.0;

                        if let Ok(path) = factory.CreatePathGeometry() {
                            if let Ok(sink) = path.Open() {
                                sink.BeginFigure(
                                    D2D_POINT_2F { x: bx - 10.0, y: by + 2.0 },
                                    D2D1_FIGURE_BEGIN_HOLLOW,
                                );
                                sink.AddLines(&[
                                    D2D_POINT_2F { x: bx - 2.0, y: by + 10.0 },
                                    D2D_POINT_2F { x: bx - 6.0, y: by + 13.0 },
                                    D2D_POINT_2F { x: bx + 5.0, y: by + 24.0 },
                                    D2D_POINT_2F { x: bx + 1.0, y: by + 16.0 },
                                    D2D_POINT_2F { x: bx + 11.0, y: by + 26.0 },
                                ]);
                                sink.EndFigure(D2D1_FIGURE_END_OPEN);
                                let _ = sink.Close();
                                rt.DrawGeometry(&path, &bolt_brush, 1.6, None);
                            }
                        }
                    }
                }
            }
        } else {
            // =================================================================
            // 2. NORMAL HAPPY / CHEER MODE AURA: Hijau Bergerak + Partikel Healing (+)
            // =================================================================
            let wave_x = (self.phase_time * 2.4).sin() * (pw * 0.20);
            let gentle_pulse = 0.75 + 0.25 * (self.phase_time * 4.0).sin();

            // Lapisan dasar hijau zamrud gelap yang menenangkan
            if let Ok(bg_heal) = rt.CreateSolidColorBrush(
                &D2D1_COLOR_F {
                    r: 0.02,
                    g: 0.22 * gentle_pulse,
                    b: 0.14 * gentle_pulse,
                    a: 0.68 * env,
                },
                None,
            ) {
                rt.FillRoundedRectangle(
                    &D2D1_ROUNDED_RECT {
                        rect: D2D_RECT_F {
                            left: pill_rect.left + 3.0,
                            top: pill_rect.top + 2.0,
                            right: pill_rect.right - 3.0,
                            bottom: pill_rect.bottom - 2.0,
                        },
                        radiusX: 14.0,
                        radiusY: 14.0,
                    },
                    &bg_heal,
                );
            }

            // Gelombang cahaya hijau zamrud bergerak lembut
            if let Ok(wave_brush) = rt.CreateSolidColorBrush(
                &D2D1_COLOR_F {
                    r: 0.06,
                    g: 0.52,
                    b: 0.34,
                    a: 0.42 * env * gentle_pulse,
                },
                None,
            ) {
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F {
                            x: cx + wave_x,
                            y: cy + 2.0,
                        },
                        radiusX: pw * 0.32,
                        radiusY: ph * 0.44,
                    },
                    &wave_brush,
                );
            }

            // Partikel-partikel Healing (Simbol '+' dan Orb Hijau Mint yang Melayang Naik)
            if let Ok(part_brush) = rt.CreateSolidColorBrush(
                &D2D1_COLOR_F {
                    r: 0.43,
                    g: 0.91,
                    b: 0.72,
                    a: 0.88 * env,
                },
                None,
            ) {
                let slots = [-0.36_f32, -0.22, -0.08, 0.10, 0.24, 0.38];
                for (i, rel_x) in slots.iter().enumerate() {
                    let speed = 14.0 + (i as f32 * 2.5);
                    let raw_y = (self.phase_time * speed + i as f32 * 7.0) % (ph - 8.0);
                    let py = pill_rect.bottom - 5.0 - raw_y;
                    let px = cx + rel_x * pw + (self.phase_time * 3.0 + i as f32).sin() * 3.0;

                    if i % 2 == 0 {
                        // Gambar simbol Healing '+'
                        let arm = 2.8_f32;
                        rt.DrawLine(
                            D2D_POINT_2F { x: px - arm, y: py },
                            D2D_POINT_2F { x: px + arm, y: py },
                            &part_brush,
                            1.5,
                            None,
                        );
                        rt.DrawLine(
                            D2D_POINT_2F { x: px, y: py - arm },
                            D2D_POINT_2F { x: px, y: py + arm },
                            &part_brush,
                            1.5,
                            None,
                        );
                    } else {
                        // Gambar orb cahaya healing kecil
                        rt.FillEllipse(
                            &D2D1_ELLIPSE {
                                point: D2D_POINT_2F { x: px, y: py },
                                radiusX: 2.0,
                                radiusY: 2.0,
                            },
                            &part_brush,
                        );
                    }
                }
            }
        }
    }

    pub unsafe fn draw(
        &self,
        rt: &ID2D1DeviceContext,
        factory: &ID2D1Factory1,
        center_x: f32,
        center_y: f32,
        alpha: f32,
    ) {
        if alpha <= 0.02 {
            return;
        }

        let eye_spacing = 15.0;
        let open_factor = (1.0 - self.blink_progress).clamp(0.08, 1.0);

        let skin_peach = D2D1_COLOR_F {
            r: 0.96,
            g: 0.82,
            b: 0.72,
            a: alpha,
        };
        let hair_ash_light = D2D1_COLOR_F {
            r: 0.85,
            g: 0.87,
            b: 0.91,
            a: alpha,
        };
        let hair_ash_mid = D2D1_COLOR_F {
            r: 0.64,
            g: 0.68,
            b: 0.75,
            a: alpha,
        };
        let headband_obsidian = D2D1_COLOR_F {
            r: 0.08,
            g: 0.10,
            b: 0.14,
            a: alpha,
        };
        let crimson_core = D2D1_COLOR_F {
            r: 0.94,
            g: 0.18,
            b: 0.24,
            a: alpha,
        };
        let emerald_green = D2D1_COLOR_F {
            r: 0.16,
            g: 0.82,
            b: 0.48,
            a: alpha,
        };
        let sclera_white = D2D1_COLOR_F {
            r: 0.97,
            g: 0.98,
            b: 1.00,
            a: alpha,
        };

        let Ok(skin_brush) = rt.CreateSolidColorBrush(&skin_peach, None) else { return };
        let Ok(hair_light_brush) = rt.CreateSolidColorBrush(&hair_ash_light, None) else { return };
        let Ok(hair_mid_brush) = rt.CreateSolidColorBrush(&hair_ash_mid, None) else { return };
        let Ok(band_brush) = rt.CreateSolidColorBrush(&headband_obsidian, None) else { return };
        let Ok(crimson_brush) = rt.CreateSolidColorBrush(&crimson_core, None) else { return };
        let Ok(emerald_brush) = rt.CreateSolidColorBrush(&emerald_green, None) else { return };
        let Ok(sclera_brush) = rt.CreateSolidColorBrush(&sclera_white, None) else { return };

        // 1. KEPALA & WAJAH CHIBI ASTA
        rt.FillRoundedRectangle(
            &D2D1_ROUNDED_RECT {
                rect: D2D_RECT_F {
                    left: center_x - 24.0,
                    top: center_y - 10.0,
                    right: center_x + 24.0,
                    bottom: center_y + 13.0,
                },
                radiusX: 10.0,
                radiusY: 10.0,
            },
            &skin_brush,
        );

        // 2. IKAT KEPALA BLACK BULLS
        rt.FillRoundedRectangle(
            &D2D1_ROUNDED_RECT {
                rect: D2D_RECT_F {
                    left: center_x - 25.0,
                    top: center_y - 11.5,
                    right: center_x + 25.0,
                    bottom: center_y - 4.0,
                },
                radiusX: 3.5,
                radiusY: 3.5,
            },
            &band_brush,
        );

        // Insignia Cross Merah di sisi kiri headband
        let bx = center_x - 16.5;
        let by = center_y - 7.8;
        rt.DrawLine(
            D2D_POINT_2F { x: bx - 3.0, y: by },
            D2D_POINT_2F { x: bx + 3.0, y: by },
            &crimson_brush,
            1.6,
            None,
        );
        rt.DrawLine(
            D2D_POINT_2F { x: bx, y: by - 3.0 },
            D2D_POINT_2F { x: bx, y: by + 3.0 },
            &crimson_brush,
            1.6,
            None,
        );

        // 3. RAMBUT SPIKY ABU-ABU PERAK ASTA
        let spikes = [
            (-26.0, -5.0, -32.0, -14.0, -18.0, -12.0),
            (-18.0, -10.0, -20.0, -19.0, -8.0, -12.0),
            (-9.0, -11.0, -6.0, -20.0, 3.0, -11.0),
            (1.0, -11.0, 8.0, -20.0, 14.0, -10.0),
            (11.0, -10.0, 21.0, -18.0, 23.0, -8.0),
            (18.0, -8.0, 31.0, -13.0, 25.0, -3.0),
        ];
        for (i, &(x1, y1, tx, ty, x2, y2)) in spikes.iter().enumerate() {
            if let Ok(path) = factory.CreatePathGeometry() {
                if let Ok(sink) = path.Open() {
                    sink.BeginFigure(
                        D2D_POINT_2F {
                            x: center_x + x1,
                            y: center_y + y1,
                        },
                        D2D1_FIGURE_BEGIN_FILLED,
                    );
                    sink.AddLines(&[
                        D2D_POINT_2F {
                            x: center_x + tx,
                            y: center_y + ty,
                        },
                        D2D_POINT_2F {
                            x: center_x + x2,
                            y: center_y + y2,
                        },
                    ]);
                    sink.EndFigure(D2D1_FIGURE_END_CLOSED);
                    let _ = sink.Close();
                    let b = if i % 2 == 0 {
                        &hair_light_brush
                    } else {
                        &hair_mid_brush
                    };
                    rt.FillGeometry(&path, b, None);
                }
            }
        }

        // 4. TANDUK DEVIL UNION (Saat Mode Devil Union) ATAU BINTANG KILAU (Saat Mode Cheer)
        if !self.is_cheer_mode {
            for side in [-1.0_f32, 1.0_f32] {
                if let Ok(h_path) = factory.CreatePathGeometry() {
                    if let Ok(sink) = h_path.Open() {
                        let hx = center_x + side * 14.0;
                        let hy = center_y - 11.0;
                        sink.BeginFigure(D2D_POINT_2F { x: hx, y: hy }, D2D1_FIGURE_BEGIN_FILLED);
                        sink.AddBezier(&D2D1_BEZIER_SEGMENT {
                            point1: D2D_POINT_2F {
                                x: hx + side * 8.0,
                                y: hy - 6.0,
                            },
                            point2: D2D_POINT_2F {
                                x: hx + side * 6.0,
                                y: hy - 14.0,
                            },
                            point3: D2D_POINT_2F {
                                x: hx + side * 1.5,
                                y: hy - 17.5,
                            },
                        });
                        sink.AddBezier(&D2D1_BEZIER_SEGMENT {
                            point1: D2D_POINT_2F {
                                x: hx + side * 2.5,
                                y: hy - 11.0,
                            },
                            point2: D2D_POINT_2F {
                                x: hx - side * 1.5,
                                y: hy - 5.0,
                            },
                            point3: D2D_POINT_2F {
                                x: hx - side * 4.5,
                                y: hy,
                            },
                        });
                        sink.EndFigure(D2D1_FIGURE_END_CLOSED);
                        let _ = sink.Close();
                        rt.FillGeometry(&h_path, &band_brush, None);
                        rt.DrawGeometry(&h_path, &crimson_brush, 0.9, None);
                    }
                }
            }
        } else {
            let sx = center_x - 32.0;
            let sy = center_y - 8.0 + (self.phase_time * 5.0).sin() * 1.5;
            rt.DrawLine(
                D2D_POINT_2F { x: sx - 3.5, y: sy },
                D2D_POINT_2F { x: sx + 3.5, y: sy },
                &emerald_brush,
                1.4,
                None,
            );
            rt.DrawLine(
                D2D_POINT_2F { x: sx, y: sy - 3.5 },
                D2D_POINT_2F { x: sx, y: sy + 3.5 },
                &emerald_brush,
                1.4,
                None,
            );
        }

        // 5. MATA ASTA
        for side in [-1.0_f32, 1.0_f32] {
            let ex = center_x + side * eye_spacing;
            let ey = center_y + 2.0;

            if self.is_cheer_mode && self.blink_progress < 0.4 {
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F { x: ex, y: ey },
                        radiusX: 5.8,
                        radiusY: 4.5 * open_factor,
                    },
                    &sclera_brush,
                );
                let px = ex + self.look_x * 0.55;
                let py = ey + self.look_y * 0.55;
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F { x: px, y: py },
                        radiusX: 3.4,
                        radiusY: 3.5 * open_factor,
                    },
                    &emerald_brush,
                );
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F {
                            x: px - 1.1,
                            y: py - 1.1,
                        },
                        radiusX: 1.1,
                        radiusY: 1.1,
                    },
                    &sclera_brush,
                );
                rt.DrawLine(
                    D2D_POINT_2F {
                        x: ex - 5.5,
                        y: ey - 5.2,
                    },
                    D2D_POINT_2F {
                        x: ex + 5.0,
                        y: ey - 4.2,
                    },
                    &band_brush,
                    1.5,
                    None,
                );
            } else {
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F { x: ex, y: ey },
                        radiusX: 6.2,
                        radiusY: 3.8 * open_factor,
                    },
                    &sclera_brush,
                );
                let px = ex + self.look_x * 0.65;
                let py = ey + self.look_y * 0.65;
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F { x: px, y: py },
                        radiusX: 3.6,
                        radiusY: 3.3 * open_factor,
                    },
                    &crimson_brush,
                );
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F { x: px, y: py },
                        radiusX: 0.9,
                        radiusY: 2.9 * open_factor,
                    },
                    &band_brush,
                );
                rt.DrawLine(
                    D2D_POINT_2F {
                        x: ex - side * 5.0,
                        y: ey - 2.6,
                    },
                    D2D_POINT_2F {
                        x: ex + side * 6.2,
                        y: ey - 5.2,
                    },
                    &band_brush,
                    1.8,
                    None,
                );
            }
        }

        // 6. PEDANG DEMON-SLAYER & TANGAN MENGGENGGAM
        let sword_bob = self.sword_float_phase.sin() * 2.2;
        let sx = center_x + 42.0;
        let sy = center_y + sword_bob * 0.5;

        if let Ok(sword_path) = factory.CreatePathGeometry() {
            if let Ok(sink) = sword_path.Open() {
                sink.BeginFigure(
                    D2D_POINT_2F {
                        x: sx - 10.0,
                        y: sy - 2.5,
                    },
                    D2D1_FIGURE_BEGIN_FILLED,
                );
                sink.AddLines(&[
                    D2D_POINT_2F {
                        x: sx + 12.0,
                        y: sy - 7.5,
                    },
                    D2D_POINT_2F {
                        x: sx + 18.0,
                        y: sy,
                    },
                    D2D_POINT_2F {
                        x: sx + 12.0,
                        y: sy + 7.5,
                    },
                    D2D_POINT_2F {
                        x: sx - 10.0,
                        y: sy + 2.5,
                    },
                ]);
                sink.EndFigure(D2D1_FIGURE_END_CLOSED);
                let _ = sink.Close();

                rt.FillGeometry(&sword_path, &band_brush, None);
                rt.DrawGeometry(&sword_path, &crimson_brush, 1.1, None);
            }
        }

        rt.DrawLine(
            D2D_POINT_2F {
                x: sx - 16.0,
                y: sy,
            },
            D2D_POINT_2F {
                x: sx - 10.0,
                y: sy,
            },
            &crimson_brush,
            2.4,
            None,
        );

        let reach_x = (center_x + 25.0) + self.hand_grab_progress * 8.0;
        let reach_y = center_y + 3.0 + sword_bob * 0.35;
        rt.FillEllipse(
            &D2D1_ELLIPSE {
                point: D2D_POINT_2F {
                    x: reach_x,
                    y: reach_y,
                },
                radiusX: 3.4,
                radiusY: 3.0,
            },
            &skin_brush,
        );
    }
}
```

---

### C. `src/notch/states.rs` — Perbaikan Permanen Timeline Musik GSMTC, Mode Pomodoro (`Focus 25m` / `Break 5m`), & Click Coordinates

Salin kode ini ke `src/notch/states.rs`:

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
```

---

### D. `src/notch/renderer.rs` — Clean Borderless Nusa Layout (Tanpa *Nested Cards*), Pomodoro 25m/5m Selector, Dashed Shelf, & Animasi Background Aura Idle

Salin kode ini ke `src/notch/renderer.rs`:

```rust
// src/notch/renderer.rs
use chrono::Local;
use windows::core::PCWSTR;
use windows::Win32::Graphics::Direct2D::Common::{
    D2D1_BEZIER_SEGMENT, D2D1_COLOR_F, D2D1_FIGURE_BEGIN_FILLED, D2D1_FIGURE_END_CLOSED,
    D2D1_GRADIENT_STOP, D2D_POINT_2F, D2D_RECT_F,
};
use windows::Win32::Graphics::Direct2D::{
    ID2D1DeviceContext, ID2D1Factory1, ID2D1PathGeometry, ID2D1StrokeStyle, D2D1_CAP_STYLE_ROUND,
    D2D1_DASH_STYLE_DASH, D2D1_ELLIPSE, D2D1_EXTEND_MODE_CLAMP, D2D1_GAMMA_2_2,
    D2D1_LINE_JOIN_ROUND, D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES, D2D1_ROUNDED_RECT,
    D2D1_STROKE_STYLE_PROPERTIES,
};
use windows::Win32::Graphics::DirectWrite::{
    DWriteCreateFactory, IDWriteFactory, IDWriteTextFormat, DWRITE_FACTORY_TYPE_SHARED,
    DWRITE_FONT_STRETCH_NORMAL, DWRITE_FONT_STYLE_ITALIC, DWRITE_FONT_STYLE_NORMAL,
    DWRITE_FONT_WEIGHT_BOLD, DWRITE_FONT_WEIGHT_MEDIUM, DWRITE_FONT_WEIGHT_SEMI_BOLD,
    DWRITE_PARAGRAPH_ALIGNMENT_CENTER, DWRITE_TEXT_ALIGNMENT_CENTER, DWRITE_TEXT_ALIGNMENT_LEADING,
    DWRITE_TEXT_ALIGNMENT_TRAILING,
};

use crate::notch::states::{NotchController, NotchState, OverlayKind};
use crate::theme::AntiMagicNeutralPalette;

pub struct NotchRenderer {
    pub factory: ID2D1Factory1,
    pub rt: ID2D1DeviceContext,
    pub dwrite: IDWriteFactory,
    pub dashed_stroke: Option<ID2D1StrokeStyle>,
    pub fmt_clock_hero: IDWriteTextFormat,
    pub fmt_title: IDWriteTextFormat,
    pub fmt_body: IDWriteTextFormat,
    pub fmt_quote_italic: IDWriteTextFormat,
    pub fmt_caption: IDWriteTextFormat,
}

impl NotchRenderer {
    pub unsafe fn new(
        factory: ID2D1Factory1,
        rt: ID2D1DeviceContext,
        dpi_scale: f32,
    ) -> windows::core::Result<Self> {
        rt.SetDpi(96.0 * dpi_scale, 96.0 * dpi_scale);

        let dwrite: IDWriteFactory = DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED)?;
        let font_family: Vec<u16> = "Segoe UI Variable Display\0".encode_utf16().collect();
        let locale: Vec<u16> = "en-us\0".encode_utf16().collect();

        // Stroke garis putus-putus (dashed border) untuk Dropzone Shelf seperti Nusa Original
        let dashed_stroke = factory
            .CreateStrokeStyle(
                &D2D1_STROKE_STYLE_PROPERTIES {
                    startCap: D2D1_CAP_STYLE_ROUND,
                    endCap: D2D1_CAP_STYLE_ROUND,
                    dashCap: D2D1_CAP_STYLE_ROUND,
                    lineJoin: D2D1_LINE_JOIN_ROUND,
                    miterLimit: 10.0,
                    dashStyle: D2D1_DASH_STYLE_DASH,
                    dashOffset: 0.0,
                },
                None,
            )
            .ok();

        let fmt_clock_hero = dwrite.CreateTextFormat(
            PCWSTR(font_family.as_ptr()),
            None,
            DWRITE_FONT_WEIGHT_BOLD,
            DWRITE_FONT_STYLE_NORMAL,
            DWRITE_FONT_STRETCH_NORMAL,
            22.0,
            PCWSTR(locale.as_ptr()),
        )?;
        fmt_clock_hero.SetParagraphAlignment(DWRITE_PARAGRAPH_ALIGNMENT_CENTER)?;

        let fmt_title = dwrite.CreateTextFormat(
            PCWSTR(font_family.as_ptr()),
            None,
            DWRITE_FONT_WEIGHT_BOLD,
            DWRITE_FONT_STYLE_NORMAL,
            DWRITE_FONT_STRETCH_NORMAL,
            13.0,
            PCWSTR(locale.as_ptr()),
        )?;
        fmt_title.SetParagraphAlignment(DWRITE_PARAGRAPH_ALIGNMENT_CENTER)?;

        let fmt_body = dwrite.CreateTextFormat(
            PCWSTR(font_family.as_ptr()),
            None,
            DWRITE_FONT_WEIGHT_SEMI_BOLD,
            DWRITE_FONT_STYLE_NORMAL,
            DWRITE_FONT_STRETCH_NORMAL,
            11.5,
            PCWSTR(locale.as_ptr()),
        )?;
        fmt_body.SetParagraphAlignment(DWRITE_PARAGRAPH_ALIGNMENT_CENTER)?;

        let fmt_quote_italic = dwrite.CreateTextFormat(
            PCWSTR(font_family.as_ptr()),
            None,
            DWRITE_FONT_WEIGHT_MEDIUM,
            DWRITE_FONT_STYLE_ITALIC,
            DWRITE_FONT_STRETCH_NORMAL,
            11.0,
            PCWSTR(locale.as_ptr()),
        )?;
        fmt_quote_italic.SetParagraphAlignment(DWRITE_PARAGRAPH_ALIGNMENT_CENTER)?;

        let fmt_caption = dwrite.CreateTextFormat(
            PCWSTR(font_family.as_ptr()),
            None,
            DWRITE_FONT_WEIGHT_MEDIUM,
            DWRITE_FONT_STYLE_NORMAL,
            DWRITE_FONT_STRETCH_NORMAL,
            10.0,
            PCWSTR(locale.as_ptr()),
        )?;
        fmt_caption.SetParagraphAlignment(DWRITE_PARAGRAPH_ALIGNMENT_CENTER)?;

        Ok(Self {
            factory,
            rt,
            dwrite,
            dashed_stroke,
            fmt_clock_hero,
            fmt_title,
            fmt_body,
            fmt_quote_italic,
            fmt_caption,
        })
    }

    unsafe fn draw_text_left(
        &self,
        text: &str,
        format: &IDWriteTextFormat,
        rect: D2D_RECT_F,
        color: D2D1_COLOR_F,
    ) {
        let _ = format.SetTextAlignment(DWRITE_TEXT_ALIGNMENT_LEADING);
        let wide: Vec<u16> = text.encode_utf16().collect();
        if let Ok(brush) = self.rt.CreateSolidColorBrush(&color, None) {
            self.rt
                .DrawText(&wide, format, &rect, &brush, Default::default(), Default::default());
        }
    }

    unsafe fn draw_text_right(
        &self,
        text: &str,
        format: &IDWriteTextFormat,
        rect: D2D_RECT_F,
        color: D2D1_COLOR_F,
    ) {
        let _ = format.SetTextAlignment(DWRITE_TEXT_ALIGNMENT_TRAILING);
        let wide: Vec<u16> = text.encode_utf16().collect();
        if let Ok(brush) = self.rt.CreateSolidColorBrush(&color, None) {
            self.rt
                .DrawText(&wide, format, &rect, &brush, Default::default(), Default::default());
        }
    }

    unsafe fn draw_text_centered(
        &self,
        text: &str,
        format: &IDWriteTextFormat,
        rect: D2D_RECT_F,
        color: D2D1_COLOR_F,
    ) {
        let _ = format.SetTextAlignment(DWRITE_TEXT_ALIGNMENT_CENTER);
        let wide: Vec<u16> = text.encode_utf16().collect();
        if let Ok(brush) = self.rt.CreateSolidColorBrush(&color, None) {
            self.rt
                .DrawText(&wide, format, &rect, &brush, Default::default(), Default::default());
        }
    }

    unsafe fn create_top_bezel_notch_geometry(
        &self,
        rect: D2D_RECT_F,
        bottom_radius: f32,
        top_flare: f32,
    ) -> windows::core::Result<ID2D1PathGeometry> {
        let path = self.factory.CreatePathGeometry()?;
        let sink = path.Open()?;

        let h = (rect.bottom - rect.top).max(4.0);
        let r = bottom_radius.min(h * 0.48).max(2.0);
        let flare = top_flare.min(r).max(4.0);

        let left = rect.left;
        let right = rect.right;
        let top = rect.top;
        let bottom = rect.bottom;

        sink.BeginFigure(
            D2D_POINT_2F {
                x: left - flare,
                y: top,
            },
            D2D1_FIGURE_BEGIN_FILLED,
        );
        sink.AddLine(D2D_POINT_2F {
            x: right + flare,
            y: top,
        });
        sink.AddBezier(&D2D1_BEZIER_SEGMENT {
            point1: D2D_POINT_2F {
                x: right + flare * 0.25,
                y: top,
            },
            point2: D2D_POINT_2F {
                x: right,
                y: top + flare * 0.35,
            },
            point3: D2D_POINT_2F {
                x: right,
                y: top + flare,
            },
        });
        sink.AddLine(D2D_POINT_2F {
            x: right,
            y: bottom - r,
        });
        sink.AddBezier(&D2D1_BEZIER_SEGMENT {
            point1: D2D_POINT_2F {
                x: right,
                y: bottom - r * 0.45,
            },
            point2: D2D_POINT_2F {
                x: right - r * 0.45,
                y: bottom,
            },
            point3: D2D_POINT_2F {
                x: right - r,
                y: bottom,
            },
        });
        sink.AddLine(D2D_POINT_2F {
            x: left + r,
            y: bottom,
        });
        sink.AddBezier(&D2D1_BEZIER_SEGMENT {
            point1: D2D_POINT_2F {
                x: left + r * 0.45,
                y: bottom,
            },
            point2: D2D_POINT_2F {
                x: left,
                y: bottom - r * 0.45,
            },
            point3: D2D_POINT_2F {
                x: left,
                y: bottom - r,
            },
        });
        sink.AddLine(D2D_POINT_2F {
            x: left,
            y: top + flare,
        });
        sink.AddBezier(&D2D1_BEZIER_SEGMENT {
            point1: D2D_POINT_2F {
                x: left,
                y: top + flare * 0.35,
            },
            point2: D2D_POINT_2F {
                x: left - flare * 0.25,
                y: top,
            },
            point3: D2D_POINT_2F {
                x: left - flare,
                y: top,
            },
        });

        sink.EndFigure(D2D1_FIGURE_END_CLOSED);
        sink.Close()?;
        Ok(path)
    }

    /// Indikator Baterai Lingkaran Hijau (`⚡`) ala Nusa Original (`Screenshot 2026-10-25 210657.png`)
    unsafe fn draw_nusa_ring_battery(&self, cx: f32, cy: f32, pct: u8, charging: bool, alpha: f32) {
        let ring_col = if charging || pct > 20 {
            D2D1_COLOR_F {
                r: 0.13,
                g: 0.85,
                b: 0.38,
                a: alpha,
            }
        } else {
            D2D1_COLOR_F {
                r: 0.94,
                g: 0.27,
                b: 0.27,
                a: alpha,
            }
        };

        if let Ok(rb) = self.rt.CreateSolidColorBrush(&ring_col, None) {
            self.rt.DrawEllipse(
                &D2D1_ELLIPSE {
                    point: D2D_POINT_2F { x: cx, y: cy },
                    radiusX: 7.0,
                    radiusY: 7.0,
                },
                &rb,
                2.0,
                None,
            );
        }

        let bolt = if charging { "⚡" } else { "•" };
        self.draw_text_centered(
            bolt,
            &self.fmt_caption,
            D2D_RECT_F {
                left: cx - 8.0,
                top: cy - 8.0,
                right: cx + 8.0,
                bottom: cy + 8.0,
            },
            D2D1_COLOR_F {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: alpha,
            },
        );
    }

    pub unsafe fn draw_frame(&self, ctrl: &NotchController, pill_rect: D2D_RECT_F) {
        let radius = ctrl.corner_radius.value;
        let alpha = ctrl.content_alpha.value.clamp(0.0, 1.0);
        let flare = if pill_rect.bottom - pill_rect.top > 60.0 {
            20.0
        } else {
            13.0
        };

        // 1. BASE NOTCH SOLID BLACK (#000000) TANPA BORDER MENGGANGGU
        let pure_black = D2D1_COLOR_F {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        };
        if let Ok(bezel_geom) = self.create_top_bezel_notch_geometry(pill_rect, radius, flare) {
            if let Ok(base_brush) = self.rt.CreateSolidColorBrush(&pure_black, None) {
                self.rt.FillGeometry(&bezel_geom, &base_brush, None);
            }
        } else if let Ok(base_brush) = self.rt.CreateSolidColorBrush(&pure_black, None) {
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: pill_rect,
                    radiusX: radius,
                    radiusY: radius,
                },
                &base_brush,
            );
        }

        // 2. SAAT TERTUTUP + ADA SYSTEM OVERLAY
        if !ctrl.is_hovered {
            if let Some(ref ov) = ctrl.active_overlay {
                self.draw_system_overlay(ov, pill_rect, alpha);
                return;
            }
        }

        // 3. KONTEN SESUAI STATE
        if ctrl.height.value < 55.0 {
            if ctrl.state == NotchState::Idle {
                self.draw_closed_clock_pill(ctrl, pill_rect, alpha);
            } else {
                // ANIMASI AURA BACKGROUND DINAMIS SAAT IDLE (Merah-Hitam Petir / Hijau Healing -> Kembali Hitam)
                ctrl.devil_eyes.draw_idle_aura_background(
                    &self.rt,
                    &self.factory,
                    pill_rect,
                    alpha,
                );
                let cx = (pill_rect.left + pill_rect.right) * 0.5;
                let cy = (pill_rect.top + pill_rect.bottom) * 0.5;
                ctrl.devil_eyes
                    .draw(&self.rt, &self.factory, cx, cy, alpha);
            }
        } else {
            let content_rect = self.draw_clean_nusa_shell(ctrl, pill_rect, alpha);
            match ctrl.state {
                NotchState::Hub => self.draw_hub_panel(ctrl, content_rect, alpha),
                NotchState::Music => self.draw_music_panel(ctrl, content_rect, alpha),
                NotchState::Control => self.draw_control_panel(ctrl, content_rect, alpha),
                NotchState::Pomo => self.draw_pomo_panel(ctrl, content_rect, alpha),
                NotchState::Shelf => self.draw_shelf_panel(ctrl, content_rect, alpha),
                _ => {}
            }
        }

        // 4. MASKOT CHIBI LIEBE
        ctrl.chibi_liebe
            .draw(&self.rt, &self.factory, pill_rect, ctrl.state);
    }

    unsafe fn draw_system_overlay(
        &self,
        ov: &OverlayKind,
        pill_rect: D2D_RECT_F,
        alpha: f32,
    ) {
        let mut p_col = AntiMagicNeutralPalette::TEXT_PRIMARY;
        p_col.a *= alpha;

        match ov {
            OverlayKind::Volume(val) | OverlayKind::Brightness(val) => {
                let is_vol = matches!(ov, OverlayKind::Volume(_));
                let icon = if is_vol { "🔊" } else { "🔆" };
                let bar_col = if is_vol {
                    AntiMagicNeutralPalette::ACCENT_BLUE
                } else {
                    AntiMagicNeutralPalette::ACCENT_CYAN
                };

                self.draw_text_left(
                    icon,
                    &self.fmt_title,
                    D2D_RECT_F {
                        left: pill_rect.left + 16.0,
                        top: pill_rect.top,
                        right: pill_rect.left + 48.0,
                        bottom: pill_rect.bottom,
                    },
                    p_col,
                );

                let bar_l = pill_rect.left + 52.0;
                let bar_r = pill_rect.right - 52.0;
                let cy = (pill_rect.top + pill_rect.bottom) * 0.5;

                if let Ok(tb) = self.rt.CreateSolidColorBrush(
                    &D2D1_COLOR_F {
                        r: 0.20,
                        g: 0.22,
                        b: 0.28,
                        a: 0.9 * alpha,
                    },
                    None,
                ) {
                    self.rt.FillRoundedRectangle(
                        &D2D1_ROUNDED_RECT {
                            rect: D2D_RECT_F {
                                left: bar_l,
                                top: cy - 3.0,
                                right: bar_r,
                                bottom: cy + 3.0,
                            },
                            radiusX: 3.0,
                            radiusY: 3.0,
                        },
                        &tb,
                    );
                }

                let fill_w = (bar_r - bar_l) * (*val as f32 / 100.0);
                if let Ok(fb) = self.rt.CreateSolidColorBrush(&bar_col, None) {
                    self.rt.FillRoundedRectangle(
                        &D2D1_ROUNDED_RECT {
                            rect: D2D_RECT_F {
                                left: bar_l,
                                top: cy - 3.0,
                                right: (bar_l + fill_w).max(bar_l + 6.0),
                                bottom: cy + 3.0,
                            },
                            radiusX: 3.0,
                            radiusY: 3.0,
                        },
                        &fb,
                    );
                }

                self.draw_text_right(
                    &format!("{}%", val),
                    &self.fmt_caption,
                    D2D_RECT_F {
                        left: bar_r + 4.0,
                        top: pill_rect.top,
                        right: pill_rect.right - 14.0,
                        bottom: pill_rect.bottom,
                    },
                    p_col,
                );
            }
            OverlayKind::CapsLock(on) => {
                let txt = if *on {
                    "🅰  CAPS LOCK : ON"
                } else {
                    "🅰  CAPS LOCK : OFF"
                };
                self.draw_text_centered(txt, &self.fmt_body, pill_rect, p_col);
            }
            OverlayKind::Charging { percent, plugged } => {
                let txt = if *plugged {
                    format!("⚡ Charging · {}%", percent)
                } else {
                    format!("🔋 On Battery · {}%", percent)
                };
                self.draw_text_centered(&txt, &self.fmt_body, pill_rect, p_col);
            }
        }
    }

    unsafe fn draw_closed_clock_pill(
        &self,
        ctrl: &NotchController,
        pill_rect: D2D_RECT_F,
        alpha: f32,
    ) {
        let now = Local::now();
        let time_str = now.format("%H:%M").to_string();

        let mut p_col = AntiMagicNeutralPalette::TEXT_PRIMARY;
        p_col.a *= alpha;

        self.draw_text_left(
            &time_str,
            &self.fmt_title,
            D2D_RECT_F {
                left: pill_rect.left + 16.0,
                top: pill_rect.top,
                right: pill_rect.left + 90.0,
                bottom: pill_rect.bottom,
            },
            p_col,
        );

        let cy = (pill_rect.top + pill_rect.bottom) * 0.5;
        self.draw_nusa_ring_battery(
            pill_rect.right - 44.0,
            cy,
            ctrl.battery_percent,
            ctrl.is_charging,
            alpha,
        );
        self.draw_text_right(
            &format!("{}%", ctrl.battery_percent),
            &self.fmt_caption,
            D2D_RECT_F {
                left: pill_rect.right - 36.0,
                top: pill_rect.top,
                right: pill_rect.right - 12.0,
                bottom: pill_rect.bottom,
            },
            p_col,
        );
    }

    /// Shell Expanded Notch Bersih Seperti Nusa Original (`Screenshot 2026-10-25 210657.png` & `210820.png`):
    /// - TIDAK ADA nested card box pembungkus konten (langsung di atas background hitam `#000000`)
    /// - Tombol navigasi bulat `<` dan `>` di tepi kiri & kanan tengah (`cy = pill_center_y`)
    /// - Bar navigasi atas yang bersih + baterai bundar hijau & jam di kanan
    unsafe fn draw_clean_nusa_shell(
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

        let white = D2D1_COLOR_F {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: alpha,
        };
        let muted = D2D1_COLOR_F {
            r: 0.58,
            g: 0.63,
            b: 0.72,
            a: alpha * 0.85,
        };

        // 1. Tombol Bulat `<` dan `>` di Tepi Kiri & Kanan Tengah (Persis Referensi Nusa Original!)
        let mid_y = (pill_rect.top + pill_rect.bottom) * 0.5 + 4.0;
        let left_circle = D2D_POINT_2F {
            x: pill_rect.left + 16.0,
            y: mid_y,
        };
        let right_circle = D2D_POINT_2F {
            x: pill_rect.right - 16.0,
            y: mid_y,
        };

        if let Ok(cb) = self.rt.CreateSolidColorBrush(
            &D2D1_COLOR_F {
                r: 0.22,
                g: 0.24,
                b: 0.28,
                a: 0.85 * alpha,
            },
            None,
        ) {
            self.rt.FillEllipse(
                &D2D1_ELLIPSE {
                    point: left_circle,
                    radiusX: 9.5,
                    radiusY: 9.5,
                },
                &cb,
            );
            self.rt.FillEllipse(
                &D2D1_ELLIPSE {
                    point: right_circle,
                    radiusX: 9.5,
                    radiusY: 9.5,
                },
                &cb,
            );
        }
        self.draw_text_centered(
            "‹",
            &self.fmt_title,
            D2D_RECT_F {
                left: left_circle.x - 9.5,
                top: left_circle.y - 10.5,
                right: left_circle.x + 9.5,
                bottom: left_circle.y + 8.5,
            },
            white,
        );
        self.draw_text_centered(
            "›",
            &self.fmt_title,
            D2D_RECT_F {
                left: right_circle.x - 9.5,
                top: right_circle.y - 10.5,
                right: right_circle.x + 9.5,
                bottom: right_circle.y + 8.5,
            },
            white,
        );

        // 2. Bar Navigasi 6-Slot di Atas (Tanpa Kotak Tebal, Bersih & Minimalis)
        let tabs_left = pill_rect.left + 36.0;
        let tabs_right = pill_rect.right - 36.0;
        let slot_w = (tabs_right - tabs_left) / 6.0;

        for (i, label) in tabs.iter().enumerate() {
            let sl = tabs_left + slot_w * i as f32;
            let sr = sl + slot_w;
            let cx = (sl + sr) * 0.5;
            let is_active = i == active_idx || (i == 5 && ctrl.stagedock_flash_timer > 0.0);

            let text_col = if is_active {
                white
            } else if i == 5 {
                AntiMagicNeutralPalette::ACCENT_CYAN
            } else {
                muted
            };

            self.draw_text_centered(
                label,
                &self.fmt_caption,
                D2D_RECT_F {
                    left: sl,
                    top: pill_rect.top + 4.0,
                    right: sr,
                    bottom: pill_rect.top + 22.0,
                },
                text_col,
            );

            // Garis bawah tipis biru-putih hanya pada tab yang aktif
            if is_active {
                if let Ok(lb) = self
                    .rt
                    .CreateSolidColorBrush(&AntiMagicNeutralPalette::ACCENT_BLUE, None)
                {
                    self.rt.DrawLine(
                        D2D_POINT_2F {
                            x: cx - 11.0,
                            y: pill_rect.top + 22.5,
                        },
                        D2D_POINT_2F {
                            x: cx + 11.0,
                            y: pill_rect.top + 22.5,
                        },
                        &lb,
                        2.0,
                        None,
                    );
                }
            }
        }

        // 3. Indikator Titik / Pill Tipis di Bagian Paling Bawah Notch (Persis Nusa Original)
        let dot_y = pill_rect.bottom - 4.5;
        let total_dots = 5;
        let dots_w = 44.0;
        let dot_start_x = (pill_rect.left + pill_rect.right - dots_w) * 0.5;
        for d in 0..total_dots {
            let dx = dot_start_x + d as f32 * 9.5;
            let is_cur = d == active_idx;
            let w = if is_cur { 10.0 } else { 4.0 };
            let col = if is_cur {
                white
            } else {
                D2D1_COLOR_F {
                    r: 0.35,
                    g: 0.38,
                    b: 0.45,
                    a: 0.7 * alpha,
                }
            };
            if let Ok(db) = self.rt.CreateSolidColorBrush(&col, None) {
                self.rt.FillRoundedRectangle(
                    &D2D1_ROUNDED_RECT {
                        rect: D2D_RECT_F {
                            left: dx,
                            top: dot_y - 1.5,
                            right: dx + w,
                            bottom: dot_y + 1.5,
                        },
                        radiusX: 1.5,
                        radiusY: 1.5,
                    },
                    &db,
                );
            }
        }

        // Kembalikan area konten langsung di atas background hitam (tanpa menggambar card container!)
        D2D_RECT_F {
            left: pill_rect.left + 32.0,
            top: pill_rect.top + 26.0,
            right: pill_rect.right - 32.0,
            bottom: pill_rect.bottom - 10.0,
        }
    }

    // =========================================================================
    // SECTION 0: HUB (Langsung di Atas Background Hitam, Tanpa Nested Cards!)
    // =========================================================================
    unsafe fn draw_hub_panel(&self, ctrl: &NotchController, area: D2D_RECT_F, alpha: f32) {
        let white = D2D1_COLOR_F {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: alpha,
        };
        let muted = D2D1_COLOR_F {
            r: 0.62,
            g: 0.66,
            b: 0.74,
            a: alpha,
        };

        let now = Local::now();
        let time_big = now.format("%H:%M").to_string();
        let date_str = now.format("%a %d %b %Y").to_string();

        // Kolom Kiri (x: area.left .. area.left + 138.0): Jam Besar, Tanggal, & Ring Baterai Hijau
        self.draw_text_left(
            &time_big,
            &self.fmt_clock_hero,
            D2D_RECT_F {
                left: area.left + 4.0,
                top: area.top + 8.0,
                right: area.left + 138.0,
                bottom: area.top + 40.0,
            },
            white,
        );

        self.draw_text_left(
            &date_str,
            &self.fmt_caption,
            D2D_RECT_F {
                left: area.left + 4.0,
                top: area.top + 42.0,
                right: area.left + 138.0,
                bottom: area.top + 60.0,
            },
            muted,
        );

        self.draw_nusa_ring_battery(
            area.left + 12.0,
            area.bottom - 18.0,
            ctrl.battery_percent,
            ctrl.is_charging,
            alpha,
        );
        let bat_label = if ctrl.is_charging {
            format!("{}% · Charging", ctrl.battery_percent)
        } else {
            format!("{}% · Battery", ctrl.battery_percent)
        };
        self.draw_text_left(
            &bat_label,
            &self.fmt_caption,
            D2D_RECT_F {
                left: area.left + 24.0,
                top: area.bottom - 27.0,
                right: area.left + 138.0,
                bottom: area.bottom - 9.0,
            },
            white,
        );

        // Garis pemisah vertikal tipis berwarna biru-abu gelap
        if let Ok(div_b) = self.rt.CreateSolidColorBrush(
            &D2D1_COLOR_F {
                r: 0.18,
                g: 0.24,
                b: 0.36,
                a: 0.65 * alpha,
            },
            None,
        ) {
            self.rt.DrawLine(
                D2D_POINT_2F {
                    x: area.left + 142.0,
                    y: area.top + 10.0,
                },
                D2D_POINT_2F {
                    x: area.left + 142.0,
                    y: area.bottom - 10.0,
                },
                &div_b,
                1.0,
                None,
            );
        }

        // Kolom Kanan: Word / Quote of the Day langsung di atas background hitam
        let q_left = area.left + 154.0;
        let daily = ctrl.current_quote();
        let header = format!("✨ {}", daily.word_of_the_day);
        self.draw_text_left(
            &header,
            &self.fmt_caption,
            D2D_RECT_F {
                left: q_left,
                top: area.top + 8.0,
                right: area.right - 4.0,
                bottom: area.top + 24.0,
            },
            AntiMagicNeutralPalette::ACCENT_CYAN,
        );

        self.draw_text_left(
            daily.quote,
            &self.fmt_quote_italic,
            D2D_RECT_F {
                left: q_left,
                top: area.top + 25.0,
                right: area.right - 4.0,
                bottom: area.bottom - 24.0,
            },
            white,
        );

        let footer = format!("— {}  (Click / Scroll)", daily.author);
        self.draw_text_left(
            &footer,
            &self.fmt_caption,
            D2D_RECT_F {
                left: q_left,
                top: area.bottom - 23.0,
                right: area.right - 4.0,
                bottom: area.bottom - 6.0,
            },
            muted,
        );
    }

    // =========================================================================
    // SECTION 1: MUSIC (Persis Referensi `Screenshot 2026-10-25 210657.png` + Side Lyrics!)
    // =========================================================================
    unsafe fn draw_music_panel(&self, ctrl: &NotchController, area: D2D_RECT_F, alpha: f32) {
        let white = D2D1_COLOR_F {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: alpha,
        };
        let muted = D2D1_COLOR_F {
            r: 0.60,
            g: 0.64,
            b: 0.70,
            a: alpha,
        };

        let total_w = area.right - area.left;
        let player_right = if ctrl.show_side_lyrics {
            area.left + total_w * 0.57
        } else {
            area.right - 4.0
        };

        // 1. Album Art Rounded Square di Kiri Atas (`48x48`)
        let art_rect = D2D_RECT_F {
            left: area.left + 4.0,
            top: area.top + 4.0,
            right: area.left + 52.0,
            bottom: area.top + 52.0,
        };
        if let Ok(art_bg) = self.rt.CreateSolidColorBrush(
            &D2D1_COLOR_F {
                r: 0.14,
                g: 0.22,
                b: 0.34,
                a: alpha,
            },
            None,
        ) {
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: art_rect,
                    radiusX: 10.0,
                    radiusY: 10.0,
                },
                &art_bg,
            );
        }
        self.draw_text_centered("🎵", &self.fmt_title, art_rect, white);

        // 2. Judul Lagu (Putih Tebal) & Nama Artis (Abu-abu) di Sebelah Kanan Cover
        self.draw_text_left(
            &ctrl.media_title,
            &self.fmt_title,
            D2D_RECT_F {
                left: art_rect.right + 10.0,
                top: area.top + 6.0,
                right: player_right,
                bottom: area.top + 28.0,
            },
            white,
        );
        self.draw_text_left(
            &ctrl.media_artist,
            &self.fmt_caption,
            D2D_RECT_F {
                left: art_rect.right + 10.0,
                top: area.top + 27.0,
                right: player_right,
                bottom: area.top + 46.0,
            },
            muted,
        );

        // 3. Baris Timeline (`00:03  ●──────────  -04:37`) Langsung di Atas Hitam
        let elapsed_s = ctrl.media_elapsed_exact.max(0.0) as u32;
        let total_s = ctrl.media_total_secs.max(1.0) as u32;
        let rem_s = total_s.saturating_sub(elapsed_s);

        let left_time = format!("{:02}:{:02}", elapsed_s / 60, elapsed_s % 60);
        let right_time = format!("-{:02}:{:02}", rem_s / 60, rem_s % 60);

        let bar_y = area.top + 64.0;
        self.draw_text_left(
            &left_time,
            &self.fmt_caption,
            D2D_RECT_F {
                left: area.left + 4.0,
                top: bar_y - 9.0,
                right: area.left + 40.0,
                bottom: bar_y + 9.0,
            },
            white,
        );
        self.draw_text_right(
            &right_time,
            &self.fmt_caption,
            D2D_RECT_F {
                left: player_right - 44.0,
                top: bar_y - 9.0,
                right: player_right,
                bottom: bar_y + 9.0,
            },
            white,
        );

        let track_l = area.left + 42.0;
        let track_r = (player_right - 46.0).max(track_l + 30.0);
        if let Ok(tb) = self.rt.CreateSolidColorBrush(
            &D2D1_COLOR_F {
                r: 0.28,
                g: 0.30,
                b: 0.34,
                a: alpha,
            },
            None,
        ) {
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: D2D_RECT_F {
                        left: track_l,
                        top: bar_y - 2.0,
                        right: track_r,
                        bottom: bar_y + 2.0,
                    },
                    radiusX: 2.0,
                    radiusY: 2.0,
                },
                &tb,
            );
        }

        let dot_x = track_l + (track_r - track_l) * ctrl.media_progress.clamp(0.0, 1.0);
        if let Ok(fb) = self
            .rt
            .CreateSolidColorBrush(&AntiMagicNeutralPalette::ACCENT_BLUE, None)
        {
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: D2D_RECT_F {
                        left: track_l,
                        top: bar_y - 2.0,
                        right: dot_x.max(track_l + 2.0),
                        bottom: bar_y + 2.0,
                    },
                    radiusX: 2.0,
                    radiusY: 2.0,
                },
                &fb,
            );
        }
        if let Ok(wb) = self.rt.CreateSolidColorBrush(&white, None) {
            self.rt.FillEllipse(
                &D2D1_ELLIPSE {
                    point: D2D_POINT_2F { x: dot_x, y: bar_y },
                    radiusX: 3.5,
                    radiusY: 3.5,
                },
                &wb,
            );
        }

        // 4. Baris Tombol Bawah: `[💬]` di kiri, `[⏮]  [▶/⏸]  [⏭]` di tengah
        let ctrl_y = area.top + 88.0;
        let mid_x = (area.left + player_right) * 0.52;

        let lyr_col = if ctrl.show_side_lyrics {
            AntiMagicNeutralPalette::ACCENT_CYAN
        } else {
            muted
        };
        self.draw_text_centered(
            "💬",
            &self.fmt_body,
            D2D_RECT_F {
                left: mid_x - 76.0,
                top: ctrl_y - 10.0,
                right: mid_x - 48.0,
                bottom: ctrl_y + 12.0,
            },
            lyr_col,
        );
        self.draw_text_centered(
            "⏮",
            &self.fmt_title,
            D2D_RECT_F {
                left: mid_x - 44.0,
                top: ctrl_y - 10.0,
                right: mid_x - 16.0,
                bottom: ctrl_y + 12.0,
            },
            white,
        );
        self.draw_text_centered(
            if ctrl.media_playing { "⏸" } else { "▶" },
            &self.fmt_title,
            D2D_RECT_F {
                left: mid_x - 14.0,
                top: ctrl_y - 10.0,
                right: mid_x + 14.0,
                bottom: ctrl_y + 12.0,
            },
            white,
        );
        self.draw_text_centered(
            "⏭",
            &self.fmt_title,
            D2D_RECT_F {
                left: mid_x + 16.0,
                top: ctrl_y - 10.0,
                right: mid_x + 44.0,
                bottom: ctrl_y + 12.0,
            },
            white,
        );

        // 5. Panel Lirik Live di Sebelah Kanan (Langsung di Atas Hitam Tanpa Kotak Penuh)
        if ctrl.show_side_lyrics {
            let lyr_left = player_right + 10.0;
            if let Ok(div_b) = self.rt.CreateSolidColorBrush(
                &D2D1_COLOR_F {
                    r: 0.18,
                    g: 0.24,
                    b: 0.36,
                    a: 0.6 * alpha,
                },
                None,
            ) {
                self.rt.DrawLine(
                    D2D_POINT_2F {
                        x: player_right + 4.0,
                        y: area.top + 6.0,
                    },
                    D2D_POINT_2F {
                        x: player_right + 4.0,
                        y: area.bottom - 8.0,
                    },
                    &div_b,
                    1.0,
                    None,
                );
            }

            let total_lines = ctrl.timed_lyrics.len().max(1);
            let cur_idx = ctrl.active_lyric_idx % total_lines;
            let prev_line = if cur_idx > 0 {
                ctrl.timed_lyrics
                    .get(cur_idx - 1)
                    .map(|(_, s)| s.as_str())
                    .unwrap_or("")
            } else {
                "♪ · · ·"
            };
            let cur_line = ctrl
                .timed_lyrics
                .get(cur_idx)
                .map(|(_, s)| s.as_str())
                .unwrap_or("♪ ...");
            let next_line = ctrl
                .timed_lyrics
                .get((cur_idx + 1) % total_lines)
                .map(|(_, s)| s.as_str())
                .unwrap_or("");

            self.draw_text_left(
                prev_line,
                &self.fmt_caption,
                D2D_RECT_F {
                    left: lyr_left,
                    top: area.top + 6.0,
                    right: area.right,
                    bottom: area.top + 26.0,
                },
                muted,
            );
            self.draw_text_left(
                cur_line,
                &self.fmt_body,
                D2D_RECT_F {
                    left: lyr_left,
                    top: area.top + 28.0,
                    right: area.right,
                    bottom: area.top + 72.0,
                },
                white,
            );
            self.draw_text_left(
                next_line,
                &self.fmt_caption,
                D2D_RECT_F {
                    left: lyr_left,
                    top: area.top + 74.0,
                    right: area.right,
                    bottom: area.top + 96.0,
                },
                muted,
            );
        }
    }

    // =========================================================================
    // SECTION 2: CONTROL (Langsung di Atas Background Hitam Tanpa Kotak Kartu!)
    // =========================================================================
    unsafe fn draw_control_panel(&self, ctrl: &NotchController, area: D2D_RECT_F, alpha: f32) {
        let white = D2D1_COLOR_F {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: alpha,
        };
        let area_h = area.bottom - area.top;
        let row_h = area_h * 0.46;

        let rows = [
            (
                area.top + 4.0,
                "🔊  Volume",
                ctrl.volume_level,
                AntiMagicNeutralPalette::ACCENT_BLUE,
            ),
            (
                area.top + 8.0 + row_h,
                "🔆  Brightness",
                ctrl.brightness_level,
                AntiMagicNeutralPalette::ACCENT_CYAN,
            ),
        ];

        for (ry, label, val, accent) in rows {
            let cy = ry + row_h * 0.5;

            // 1. Kolom Label Kiri
            self.draw_text_left(
                label,
                &self.fmt_body,
                D2D_RECT_F {
                    left: area.left + 4.0,
                    top: ry,
                    right: area.left + 100.0,
                    bottom: ry + row_h,
                },
                white,
            );

            // 2. Track Slider Tipis di Tengah
            let track_l = area.left + 104.0;
            let track_r = (area.right - 52.0).max(track_l + 20.0);

            if let Ok(tb) = self.rt.CreateSolidColorBrush(
                &D2D1_COLOR_F {
                    r: 0.22,
                    g: 0.25,
                    b: 0.32,
                    a: alpha,
                },
                None,
            ) {
                self.rt.FillRoundedRectangle(
                    &D2D1_ROUNDED_RECT {
                        rect: D2D_RECT_F {
                            left: track_l,
                            top: cy - 3.0,
                            right: track_r,
                            bottom: cy + 3.0,
                        },
                        radiusX: 3.0,
                        radiusY: 3.0,
                    },
                    &tb,
                );
            }

            let fill_x = track_l + (track_r - track_l) * (val.min(100) as f32 / 100.0);
            if let Ok(fb) = self.rt.CreateSolidColorBrush(&accent, None) {
                self.rt.FillRoundedRectangle(
                    &D2D1_ROUNDED_RECT {
                        rect: D2D_RECT_F {
                            left: track_l,
                            top: cy - 3.0,
                            right: fill_x.max(track_l + 4.0),
                            bottom: cy + 3.0,
                        },
                        radiusX: 3.0,
                        radiusY: 3.0,
                    },
                    &fb,
                );
            }

            if let Ok(kb) = self.rt.CreateSolidColorBrush(&white, None) {
                self.rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F { x: fill_x, y: cy },
                        radiusX: 5.0,
                        radiusY: 5.0,
                    },
                    &kb,
                );
            }

            // 3. Angka Persentase di Kanan
            self.draw_text_right(
                &format!("{}%", val),
                &self.fmt_body,
                D2D_RECT_F {
                    left: area.right - 48.0,
                    top: ry,
                    right: area.right - 4.0,
                    bottom: ry + row_h,
                },
                white,
            );
        }
    }

    // =========================================================================
    // SECTION 3: POMODORO TIMER (Dengan Opsi Tombol `Focus 25m` & `Break 5m`!)
    // =========================================================================
    unsafe fn draw_pomo_panel(&self, ctrl: &NotchController, area: D2D_RECT_F, alpha: f32) {
        let white = D2D1_COLOR_F {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: alpha,
        };
        let muted = D2D1_COLOR_F {
            r: 0.60,
            g: 0.64,
            b: 0.72,
            a: alpha,
        };

        let mins = ctrl.pomo_remaining_secs / 60;
        let secs = ctrl.pomo_remaining_secs % 60;
        let time_str = format!("{:02}:{:02}", mins, secs);

        // Kiri: Angka Countdown Besar & Label Status
        self.draw_text_left(
            &time_str,
            &self.fmt_clock_hero,
            D2D_RECT_F {
                left: area.left + 6.0,
                top: area.top + 14.0,
                right: area.left + 112.0,
                bottom: area.top + 54.0,
            },
            white,
        );
        self.draw_text_left(
            if ctrl.pomo_is_break {
                "☕ Break Mode"
            } else {
                "🎯 Focus Mode"
            },
            &self.fmt_caption,
            D2D_RECT_F {
                left: area.left + 6.0,
                top: area.top + 56.0,
                right: area.left + 112.0,
                bottom: area.top + 78.0,
            },
            AntiMagicNeutralPalette::ACCENT_CYAN,
        );

        // Kanan Baris 1 (top + 12..42): Opsi Pemilih Mode [Focus 25m] dan [Break 5m]
        let focus_mode_rect = D2D_RECT_F {
            left: area.left + 118.0,
            top: area.top + 12.0,
            right: area.left + 214.0,
            bottom: area.top + 42.0,
        };
        let break_mode_rect = D2D_RECT_F {
            left: area.left + 222.0,
            top: area.top + 12.0,
            right: area.left + 316.0,
            bottom: area.top + 42.0,
        };

        let active_pill_col = D2D1_COLOR_F {
            r: 0.23,
            g: 0.51,
            b: 0.96,
            a: 0.90 * alpha,
        };
        let inactive_pill_col = D2D1_COLOR_F {
            r: 0.14,
            g: 0.17,
            b: 0.24,
            a: 0.90 * alpha,
        };

        if let Ok(fb) = self.rt.CreateSolidColorBrush(
            if !ctrl.pomo_is_break {
                &active_pill_col
            } else {
                &inactive_pill_col
            },
            None,
        ) {
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: focus_mode_rect,
                    radiusX: 8.0,
                    radiusY: 8.0,
                },
                &fb,
            );
        }
        if let Ok(bb) = self.rt.CreateSolidColorBrush(
            if ctrl.pomo_is_break {
                &active_pill_col
            } else {
                &inactive_pill_col
            },
            None,
        ) {
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: break_mode_rect,
                    radiusX: 8.0,
                    radiusY: 8.0,
                },
                &bb,
            );
        }

        self.draw_text_centered(
            "Focus 25m",
            &self.fmt_caption,
            focus_mode_rect,
            if !ctrl.pomo_is_break { white } else { muted },
        );
        self.draw_text_centered(
            "Break 5m",
            &self.fmt_caption,
            break_mode_rect,
            if ctrl.pomo_is_break { white } else { muted },
        );

        // Kanan Baris 2 (top + 54..86): Tombol [▶ Start / ⏸ Pause] dan [↺ Reset]
        let start_rect = D2D_RECT_F {
            left: area.left + 118.0,
            top: area.top + 54.0,
            right: area.left + 214.0,
            bottom: area.top + 86.0,
        };
        let reset_rect = D2D_RECT_F {
            left: area.left + 222.0,
            top: area.top + 54.0,
            right: area.left + 300.0,
            bottom: area.top + 86.0,
        };

        if let Ok(sb) = self.rt.CreateSolidColorBrush(
            &D2D1_COLOR_F {
                r: 0.18,
                g: 0.24,
                b: 0.36,
                a: alpha,
            },
            None,
        ) {
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: start_rect,
                    radiusX: 8.0,
                    radiusY: 8.0,
                },
                &sb,
            );
        }
        if let Ok(rb) = self.rt.CreateSolidColorBrush(&inactive_pill_col, None) {
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: reset_rect,
                    radiusX: 8.0,
                    radiusY: 8.0,
                },
                &rb,
            );
        }

        self.draw_text_centered(
            if ctrl.pomo_running {
                "⏸ Pause"
            } else {
                "▶ Start"
            },
            &self.fmt_body,
            start_rect,
            white,
        );
        self.draw_text_centered("↺ Reset", &self.fmt_caption, reset_rect, muted);
    }

    // =========================================================================
    // SECTION 4: SHELF (Persis Referensi `Screenshot 2026-10-25 210820.png`: Dashed Border!)
    // =========================================================================
    unsafe fn draw_shelf_panel(&self, ctrl: &NotchController, area: D2D_RECT_F, alpha: f32) {
        let white = D2D1_COLOR_F {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: alpha,
        };
        let muted = D2D1_COLOR_F {
            r: 0.56,
            g: 0.60,
            b: 0.68,
            a: alpha,
        };

        let drop_rect = D2D_RECT_F {
            left: area.left + 4.0,
            top: area.top + 4.0,
            right: area.right - 4.0,
            bottom: area.bottom - 4.0,
        };

        // Garis tepi putus-putus (Dashed Rounded Rect) langsung di atas background hitam!
        if let Ok(border_b) = self.rt.CreateSolidColorBrush(
            &D2D1_COLOR_F {
                r: 0.32,
                g: 0.36,
                b: 0.44,
                a: 0.85 * alpha,
            },
            None,
        ) {
            self.rt.DrawRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: drop_rect,
                    radiusX: 14.0,
                    radiusY: 14.0,
                },
                &border_b,
                1.4,
                self.dashed_stroke.as_ref(),
            );
        }

        if ctrl.shelf_files.is_empty() {
            let cx = (drop_rect.left + drop_rect.right) * 0.5;
            self.draw_text_centered(
                "📥",
                &self.fmt_clock_hero,
                D2D_RECT_F {
                    left: cx - 30.0,
                    top: drop_rect.top + 8.0,
                    right: cx + 30.0,
                    bottom: drop_rect.top + 44.0,
                },
                white,
            );
            self.draw_text_centered(
                "Drop files here",
                &self.fmt_title,
                D2D_RECT_F {
                    left: drop_rect.left,
                    top: drop_rect.top + 44.0,
                    right: drop_rect.right,
                    bottom: drop_rect.top + 68.0,
                },
                white,
            );
            self.draw_text_centered(
                "Drag out anytime · click to open",
                &self.fmt_caption,
                D2D_RECT_F {
                    left: drop_rect.left,
                    top: drop_rect.top + 66.0,
                    right: drop_rect.right,
                    bottom: drop_rect.bottom - 6.0,
                },
                muted,
            );
        } else {
            let slot_w = (drop_rect.right - drop_rect.left - 24.0) * 0.5;
            for (i, (name, size, _)) in ctrl.shelf_files.iter().take(2).enumerate() {
                let fl = drop_rect.left + 8.0 + i as f32 * (slot_w + 8.0);
                let f_rect = D2D_RECT_F {
                    left: fl,
                    top: drop_rect.top + 10.0,
                    right: fl + slot_w,
                    bottom: drop_rect.bottom - 10.0,
                };
                if let Ok(ib) = self.rt.CreateSolidColorBrush(
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
                            rect: f_rect,
                            radiusX: 10.0,
                            radiusY: 10.0,
                        },
                        &ib,
                    );
                }
                self.draw_text_left(
                    &format!("📄 {}", name),
                    &self.fmt_body,
                    D2D_RECT_F {
                        left: f_rect.left + 10.0,
                        top: f_rect.top + 8.0,
                        right: f_rect.right - 24.0,
                        bottom: f_rect.top + 34.0,
                    },
                    white,
                );
                self.draw_text_left(
                    &format!("{} · Click to open", size),
                    &self.fmt_caption,
                    D2D_RECT_F {
                        left: f_rect.left + 10.0,
                        top: f_rect.top + 36.0,
                        right: f_rect.right - 10.0,
                        bottom: f_rect.bottom - 6.0,
                    },
                    AntiMagicNeutralPalette::ACCENT_CYAN,
                );
                self.draw_text_centered(
                    "✕",
                    &self.fmt_caption,
                    D2D_RECT_F {
                        left: f_rect.right - 22.0,
                        top: f_rect.top + 6.0,
                        right: f_rect.right - 6.0,
                        bottom: f_rect.top + 22.0,
                    },
                    muted,
                );
            }
        }
    }
}
```

---

### E. `src/notch/mod.rs` — Klik Tombol Bulat `<` / `>` di Tepi Tengah, 6-Slot Tab Bar, & Pemasangan Custom Window Icon (`create_antimagic_hicon()`)

Di `src/notch/mod.rs`, pastikan:
1. Saat mendaftarkan `WNDCLASSEXW`, pasang `let app_icon = crate::icon::create_antimagic_hicon(); wc.hIcon = app_icon; wc.hIconSm = app_icon;`.
2. Fungsi `on_lbutton_down` mendeteksi klik baik pada **tombol bulat `<` dan `>` di tepi kiri/kanan tengah (`lx <= 28.0` atau `lx >= pill_w - 28.0`)** maupun **6 slot tab di atas (`Hub`, `Music`, `Control`, `Focus`, `Shelf`, `Stages`)**:

```rust
    pub fn on_lbutton_down(&mut self, phys_x: f32, phys_y: f32) {
        let x = phys_x / self.dpi_scale;
        let y = phys_y / self.dpi_scale;

        let pill_rect = self.current_pill_rect();
        let pill_w = pill_rect.right - pill_rect.left;
        let pill_h = pill_rect.bottom - pill_rect.top;
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

        if self.controller.height.value > 65.0 {
            // 1. Klik Tombol Bulat `<` (kiri) atau `>` (kanan) di tepi notch (seperti Nusa Original)
            if (0.0..=pill_h).contains(&ly) {
                if (4.0..=28.0).contains(&lx) {
                    self.controller.prev_tab();
                    unsafe {
                        let _ = InvalidateRect(self.hwnd, None, false);
                    }
                    return;
                } else if lx >= pill_w - 28.0 && lx <= pill_w - 4.0 {
                    self.controller.next_tab();
                    unsafe {
                        let _ = InvalidateRect(self.hwnd, None, false);
                    }
                    return;
                }
            }

            // 2. Klik Bar Navigasi 6-Slot di Atas (Hub, Music, Control, Focus, Shelf, Stages)
            if (0.0..=26.0).contains(&ly) && lx >= 36.0 && lx <= pill_w - 36.0 {
                let tabs_w = (pill_w - 72.0).max(1.0);
                let slot = (((lx - 36.0) / (tabs_w / 6.0)).floor() as usize).min(5);
                match slot {
                    0 => self.controller.transition_to(NotchState::Hub),
                    1 => self.controller.transition_to(NotchState::Music),
                    2 => self.controller.transition_to(NotchState::Control),
                    3 => self.controller.transition_to(NotchState::Pomo),
                    4 => self.controller.transition_to(NotchState::Shelf),
                    _ => {
                        self.controller.trigger_stagedock();
                    }
                }
                unsafe {
                    let _ = InvalidateRect(self.hwnd, None, false);
                }
                return;
            }

            // 3. Klik Konten di Dalam Section Aktif
            if self.controller.handle_panel_click(lx, ly, pill_w) {
                unsafe {
                    let _ = InvalidateRect(self.hwnd, None, false);
                }
                return;
            }
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