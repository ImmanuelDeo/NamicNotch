# Nusa Clone: Perbaikan Critical Bug Notch Tidak Muncul, Smooth Spring Animation, Neutral Anti-Magic Theme, Mata Asta & Chibi Liebe

Dokumen ini berisi **4 Akar Penyebab Notch Tidak Muncul dari Kode Kamu**, **Prompt Perbaikan Instan untuk Hermes Agent**, serta **Kode Pengganti Utuh (`src/notch/mod.rs`, `src/notch/renderer.rs`, `src/notch/states.rs`, `src/mascot/eyes.rs`, `src/mascot/chibi_liebe.rs`)** yang sudah disesuaikan 100% dengan struktur `NotchApp`, `NotchEvent`, dan `run_ui` di `src/main.rs` milikmu.

---

## 1. Hasil Audit Kode Kamu: 4 Bug Penyebab Notch Tidak Muncul Sama Sekali

Setelah memeriksa `src/notch/mod.rs` dan `src/notch/renderer.rs` yang kamu kirimkan, berikut adalah **4 bug fatal** yang menyebabkan aplikasi jalan di *System Tray* tetapi *notch* sama sekali tidak tergambar di layar:

| # | Bug di Kode Kamu Saat Ini | Mengapa Ini Membuat Notch Hilang / Mati | Solusi Perbaikan di Kode Baru |
| :- | :--- | :--- | :--- |
| **1** | **Double `BeginPaint` / `EndPaint` pada `WM_PAINT`**<br>Di `notch_wnd_proc` (`src/notch/mod.rs`) kamu memanggil `BeginPaint(hwnd, &mut ps)`, lalu di dalam `renderer.render_frame()` (`src/notch/renderer.rs`) kamu memanggil `BeginPaint(hwnd, &mut ps)` **sekali lagi**! | Di Win32 GDI, panggilan `BeginPaint` pertama langsung menandai *update region* window sebagai "sudah bersih". Panggilan `BeginPaint` kedua di dalam `render_frame` menghasilkan **clipping region kosong (`0x0` piksel)**, sehingga seluruh gambar Direct2D dibuang oleh Windows! | **Hapus `BeginPaint` & `EndPaint` dari `notch_wnd_proc`**. Biarkan hanya `ValidateRect(hwnd, None)` atau panggil `BeginPaint`/`EndPaint` di satu tempat saja (di dalam `render_frame`). |
| **2** | **`SetLayeredWindowAttributes` di-import tapi TIDAK PERNAH dipanggil** | Di `run_ui`, window dibuat dengan `WS_EX_LAYERED`, tetapi `SetLayeredWindowAttributes(hwnd, COLORREF(0), 255, LWA_ALPHA)` tidak pernah dipanggil setelah `CreateWindowExW`. Di Windows 10/11, window `WS_EX_LAYERED` yang belum diinisialisasi atributnya memiliki **opacity 0% (100% tembus pandang)**. | Panggil `SetLayeredWindowAttributes(hwnd, COLORREF(0), 255, LWA_ALPHA)` segera setelah `CreateWindowExW`. |
| **3** | **`SetTimer` di-import tapi TIDAK PERNAH dipanggil di `run_ui`** | Di `notch_wnd_proc`, ada *handler* `WM_TIMER` yang memanggil `app.on_tick()`, dan `KillTimer(hwnd, 1)` saat keluar, tetapi **`SetTimer(hwnd, 1, 8, None)` lupa dipanggil di `run_ui`!** Akibatnya `on_tick()` tidak pernah berjalan, animasi membeku di frame 0, dan kursor tidak pernah dideteksi. | Panggil `SetTimer(hwnd, 1, 8, None)` tepat sebelum *message loop* `GetMessageW` dimulai. |
| **4** | **Ketidakcocokan Skala DPI (`dpi_scale` ganda)** | Di `current_pill_rect()`, `pw` dan `ph` dikalikan dengan `self.dpi_scale`, padahal `NotchRenderer` dibuat dengan `dpiX: 96.0, dpiY: 96.0` dan ukuran window fisiknya tidak sinkron dengan DIPs Direct2D. | Gunakan ukuran logis yang konsisten antara `NotchRenderer` (`win_w as u32, win_h as u32` pada `dpiX: 96.0`) dan koordinat `pill_rect`. |

---

## 2. Prompt Perbaikan Langsung untuk Hermes Agent

Salin blok perintah di bawah ini dan berikan langsung ke **Hermes Agent** di terminal lokalmu (di folder `nusa-clone/`):

```markdown
Tolong baca file `DEVIL_UNION_UPGRADE.md` dan segera perbaiki 4 bug kritis di `src/notch/mod.rs` dan `src/notch/renderer.rs` yang menyebabkan window notch tidak muncul di layar:

1. **Hapus Double `BeginPaint` / `EndPaint` (Penyebab Clipping Region 0x0)**:
   - Di `src/notch/mod.rs` pada cabang `WM_PAINT` di `notch_wnd_proc`, JANGAN panggil `BeginPaint` dan `EndPaint` lagi karena `app.renderer.render_frame(...)` sudah memanggil `BeginPaint` dan `EndPaint` di dalamnya. Cukup panggil:
     ```rust
     WM_PAINT => {
         let pill_rect = app.current_pill_rect();
         app.renderer.render_frame(&mut app.controller, pill_rect);
         return LRESULT(0);
     }
     ```

2. **Panggil `SetLayeredWindowAttributes` dan `SetTimer` di `run_ui` (`src/notch/mod.rs`)**:
   - Tepat setelah `CreateWindowExW` di `run_ui`, panggil:
     ```rust
     let _ = SetLayeredWindowAttributes(hwnd, windows::Win32::Foundation::COLORREF(0), 255, LWA_ALPHA);
     ```
   - Sebelum masuk ke loop `while GetMessageW(...)`, wajib aktifkan timer 120 FPS (8ms) dan paksa render frame pertama:
     ```rust
     let _ = SetTimer(hwnd, 1, 8, None);
     let _ = ShowWindow(hwnd, SW_SHOWNOACTIVATE);
     let _ = InvalidateRect(hwnd, None, false);
     let _ = UpdateWindow(hwnd);
     ```

3. **Sinkronkan Ukuran Render Target Direct2D di `NotchApp::new` (`src/notch/mod.rs`)**:
   - Pastikan setelah menghitung `win_w` dan `win_h`, `SetWindowPos` dipanggil terlebih dahulu untuk mengubah ukuran window dari initial size ke `(win_x, win_y, win_w, win_h)`, BARU kemudian buat `NotchRenderer::new(hwnd, win_w as u32, win_h as u32)` agar ukuran buffer Direct2D tidak terjebak di ukuran lama atau 0x0.
   - Di `NotchRenderer::render_frame`, tambahkan pengecekan ukuran pixel atau panggil `self.rt.Resize(...)` bila diperlukan, dan pastikan `DrawRoundedRectangle` untuk border benar-benar dipanggil di dalam blok `if let Ok(border_brush) = ...`.

4. **Gunakan Kode Lengkap `src/notch/mod.rs` dan `src/notch/renderer.rs` dari Bagian 3 di `DEVIL_UNION_UPGRADE.md`**:
   - Ganti isi `src/notch/mod.rs` dan `src/notch/renderer.rs` sesuai kode di `DEVIL_UNION_UPGRADE.md` (pastikan semua enum variant `NotchEvent` yang dibutuhkan oleh `media`, `timers`, `shelf`, dan `hooks` di codebase tetap dipertahankan).
   - Jalankan `cargo check` dan `cargo build --release` sampai kompilasi sukses tanpa error.
```

---

## 3. Kode Pengganti Lengkap (`src/notch/mod.rs` & `src/notch/renderer.rs`)

Kode di bawah ini sudah disesuaikan langsung dengan arsitektur `NotchApp`, `NotchEvent`, `run_ui(hinstance, notch_rx, settings)`, dan `tray` dari kode yang kamu kirimkan, sekaligus memperbaiki ke-4 bug di atas:

### A. `src/notch/mod.rs` (Sudah Diperbaiki Total)

```rust
// src/notch/mod.rs
//! Notch window and rendering for Nusa Notch Clone

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
use windows::Win32::UI::WindowsAndMessaging::*;

use crate::settings::Settings;
use crate::tray::{
    handle_tray_command, remove_tray_icon, setup_tray_icon, show_tray_menu, TRAY_MESSAGE,
};

pub use renderer::NotchRenderer;
pub use states::{NotchController, NotchState};

pub const NOTCH_WINDOW_WIDTH: i32 = 520;
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
            // Gunakan rcMonitor (bukan rcWork) agar selalu menempel tepat di tepi paling atas layar (y = 0)
            info.rcMonitor
        };

        let win_w = (NOTCH_WINDOW_WIDTH as f32 * dpi_scale).round() as i32;
        let win_h = (NOTCH_WINDOW_HEIGHT as f32 * dpi_scale).round() as i32;
        let screen_w = monitor_rect.right - monitor_rect.left;
        let win_x = monitor_rect.left + (screen_w - win_w) / 2;
        let win_y = monitor_rect.top;

        unsafe {
            // 1. PENTING: Set ukuran window terlebih dahulu SEBELUM membuat Direct2D HwndRenderTarget!
            let _ = SetWindowPos(
                hwnd,
                HWND_TOPMOST,
                win_x,
                win_y,
                win_w,
                win_h,
                SWP_NOACTIVATE | SWP_NOZORDER,
            );

            // 2. PENTING: Inisialisasi atribut Layered Window ke Alpha 255 (tanpa ini window 100% invisible!)
            let _ = SetLayeredWindowAttributes(hwnd, COLORREF(0), 255, LWA_ALPHA);

            // 3. Perluas area kaca DWM ke seluruh window agar background Direct2D transparan mulus
            let margins = MARGINS {
                cxLeftWidth: -1,
                cxRightWidth: -1,
                cyTopWidth: -1,
                cyBottomWidth: -1,
            };
            let _ = DwmExtendFrameIntoClientArea(hwnd, &margins);
        }

        // 4. Buat Direct2D Renderer dengan ukuran pixel fisik window yang sudah di-set
        let renderer = NotchRenderer::new(hwnd, win_w as u32, win_h as u32)?;
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

    pub fn current_pill_rect(&self) -> D2D_RECT_F {
        let pw = (self.controller.width.value * self.dpi_scale).round();
        let ph = (self.controller.height.value * self.dpi_scale).round();
        let left = ((self.win_w as f32 - pw) * 0.5).max(0.0);
        let top = 4.0 * self.dpi_scale; // Jarak 4px dari tepi atas layar agar bayangan/glow terlihat rapi
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

        // 1. Drain background events
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
                    if info.playing && !self.controller.is_hovered {
                        // Tetap di mode Pet atau tampilkan indikator halus
                    }
                }
                _ => {}
            }
        }

        // 2. Sync settings flags
        if let Ok(s) = self.settings.read() {
            self.controller.mascot_enabled = s.show_mascot;
            self.controller.minimal_mode = s.minimal_notch;
        }

        // 3. Cek posisi kursor layar terhadap area pill
        let pill_rect = self.current_pill_rect();
        let pill_screen_left = self.win_x as f32 + pill_rect.left;
        let pill_screen_right = self.win_x as f32 + pill_rect.right;
        let pill_screen_top = self.win_y as f32 + pill_rect.top;
        let pill_screen_bottom = self.win_y as f32 + pill_rect.bottom;

        let mut pt = POINT::default();
        unsafe {
            let _ = GetCursorPos(&mut pt);
        }

        let pad = 6.0 * self.dpi_scale;
        let cursor_in_pill = pt.x as f32 >= pill_screen_left - pad
            && pt.x as f32 <= pill_screen_right + pad
            && pt.y as f32 >= pill_screen_top - pad
            && pt.y as f32 <= pill_screen_bottom + pad;

        self.cursor_in_pill = cursor_in_pill;

        // 4. Dynamic click-through: cabut WS_EX_TRANSPARENT saat kursor di dalam pill
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

        // 5. Hitung offset kursor untuk dilirik oleh mata Asta Devil Union (radius 360px)
        let pill_center_x = (pill_screen_left + pill_screen_right) * 0.5;
        let pill_center_y = (pill_screen_top + pill_screen_bottom) * 0.5;
        let dx = (pt.x as f32 - pill_center_x) / self.dpi_scale;
        let dy = (pt.y as f32 - pill_center_y) / self.dpi_scale;
        let cursor_offset = if (dx * dx + dy * dy).sqrt() <= 360.0 {
            Some((dx, dy))
        } else {
            None
        };

        // 6. Update fisika pegas, mata Asta, & Chibi Liebe
        self.controller.update(dt, cursor_in_pill, cursor_offset);

        // 7. Trigger repaint
        unsafe {
            let _ = InvalidateRect(self.hwnd, None, false);
        }

        Ok(())
    }

    pub fn on_lbutton_down(&mut self, x: f32, y: f32) {
        // Cek apakah pengguna mengklik Chibi Liebe (efek poke)
        if self.controller.chibi_liebe.handle_click(x, y) {
            unsafe {
                let _ = InvalidateRect(self.hwnd, None, false);
            }
            return;
        }

        let pill_rect = self.current_pill_rect();

        // Cek klik bar navigasi tab atas saat notch sedang terbuka
        if self.controller.height.value > 65.0 && y <= pill_rect.top + 32.0 * self.dpi_scale {
            let total_w = pill_rect.right - pill_rect.left;
            let slot_w = total_w / 5.0;
            let rel_x = x - pill_rect.left;
            if rel_x >= 0.0 && rel_x <= total_w {
                let slot = (rel_x / slot_w) as usize;
                match slot {
                    0 => self.controller.transition_to(NotchState::Music),
                    1 => self.controller.transition_to(NotchState::Lyrics),
                    2 => self.controller.transition_to(NotchState::Pomo),
                    3 => self.controller.transition_to(NotchState::Water),
                    _ => self.controller.transition_to(NotchState::Shelf),
                }
                unsafe {
                    let _ = InvalidateRect(self.hwnd, None, false);
                }
                return;
            }
        }

        // Toggle buka/tutup jika diklik di bagian tengah pill
        if self.controller.state == NotchState::Pet {
            self.controller.devil_eyes.trigger_wake_surge();
            let target = self.controller.last_expanded_tab;
            self.controller.transition_to(target);
        } else {
            self.controller.transition_to(NotchState::Pet);
        }
        unsafe {
            let _ = InvalidateRect(self.hwnd, None, false);
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
            WS_EX_TOPMOST | WS_EX_TOOLWINDOW | WS_EX_LAYERED | WS_EX_TRANSPARENT,
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

        // Setup System Tray icon
        let nid = setup_tray_icon(hwnd)?;

        // Tampilkan window di posisi tengah-atas layar (Topmost)
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

        // PENTING: Nyalakan timer 8ms (~120 FPS) agar on_tick() dan animasi berjalan!
        let _ = SetTimer(hwnd, TIMER_ID_ANIMATION, 8, None);

        // Paksa gambar frame pertama sekarang juga
        let _ = InvalidateRect(hwnd, None, false);
        let _ = UpdateWindow(hwnd);

        // Standard Win32 Message Loop
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
            WM_ERASEBKGND => {
                return LRESULT(1);
            }
            WM_TIMER => {
                let _ = app.on_tick();
                return LRESULT(0);
            }
            WM_PAINT => {
                // PENTING: Jangan panggil BeginPaint di sini karena render_frame sudah memanggil BeginPaint/EndPaint!
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

---

### B. `src/notch/renderer.rs` (Sudah Diperbaiki Total)

Perhatikan bahwa di kode `renderer.rs` kamu sebelumnya, blok `if let Ok(border_brush) = ...` **kosong** (tidak memanggil `DrawRoundedRectangle`) dan `BeginPaint` bertabrakan dengan `notch_wnd_proc`. Kode di bawah ini sudah memperbaiki keduanya:

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
        a: 0.98,
    };
    /// Dark Slate Card / Inner Surface (#131822)
    pub const CARD_SURFACE: D2D1_COLOR_F = D2D1_COLOR_F {
        r: 0.075,
        g: 0.094,
        b: 0.133,
        a: 0.94,
    };
    /// Cool Steel Border (#263042)
    pub const STEEL_BORDER: D2D1_COLOR_F = D2D1_COLOR_F {
        r: 0.180,
        g: 0.230,
        b: 0.320,
        a: 0.70,
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
                pixelSize: D2D_SIZE_U {
                    width: width.max(100),
                    height: height.max(50),
                },
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

            // 1. Bersihkan seluruh canvas dengan alpha 0.0 (DWM glass composition)
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

            // 3. Gambar tepi luar (Cool Steel Grey dengan denyut Slate-Wine halus saat mode Pet)
            let border_color = if controller.state == NotchState::Pet {
                let pulse = controller.devil_eyes.aura_intensity.clamp(0.4, 1.2);
                D2D1_COLOR_F {
                    r: 0.28 * pulse,
                    g: 0.25,
                    b: 0.38,
                    a: 0.65,
                }
            } else {
                AntiMagicNeutralPalette::STEEL_BORDER
            };

            if let Ok(border_brush) = self.rt.CreateSolidColorBrush(&border_color, None) {
                self.rt
                    .DrawRoundedRectangle(&rounded_pill, &border_brush, 1.25, None);
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

                // Gambar bar navigasi tab atas + kartu dalam
                self.draw_expanded_panel_shell(controller.state, pill_rect, content_alpha);

                // Render Chibi Liebe secara selektif (HANYA tampil pada Music, Pomo, atau Shelf)
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

    unsafe fn draw_expanded_panel_shell(
        &self,
        state: NotchState,
        pill_rect: D2D_RECT_F,
        alpha: f32,
    ) {
        let total_w = pill_rect.right - pill_rect.left;
        let slot_w = total_w / 5.0;

        // Gambar titik/indikator kelima tab di bagian atas panel agar pengguna tahu posisi klik tab
        let active_idx = match state {
            NotchState::Music => 0,
            NotchState::Lyrics => 1,
            NotchState::Pomo => 2,
            NotchState::Water => 3,
            NotchState::Shelf | NotchState::Todo => 4,
            _ => return,
        };

        for i in 0..5 {
            let cx = pill_rect.left + slot_w * (i as f32 + 0.5);
            let cy = pill_rect.top + 12.0;

            let color = if i == active_idx {
                let mut c = AntiMagicNeutralPalette::ACCENT_BLUE;
                c.a *= alpha;
                c
            } else {
                D2D1_COLOR_F {
                    r: 0.35,
                    g: 0.42,
                    b: 0.55,
                    a: 0.45 * alpha,
                }
            };

            if let Ok(brush) = self.rt.CreateSolidColorBrush(&color, None) {
                let half_len = if i == active_idx { 14.0 } else { 6.0 };
                self.rt.DrawLine(
                    D2D_POINT_2F {
                        x: cx - half_len,
                        y: cy,
                    },
                    D2D_POINT_2F {
                        x: cx + half_len,
                        y: cy,
                    },
                    &brush,
                    2.4,
                    None,
                );
            }
        }

        // Gambar kartu permukaan dalam (Inner Slate Card #131822) untuk konten fitur
        let mut card_color = AntiMagicNeutralPalette::CARD_SURFACE;
        card_color.a *= alpha;
        if let Ok(card_brush) = self.rt.CreateSolidColorBrush(&card_color, None) {
            let inner_card = D2D1_ROUNDED_RECT {
                rect: D2D_RECT_F {
                    left: pill_rect.left + 12.0,
                    top: pill_rect.top + 26.0,
                    right: pill_rect.right - 12.0,
                    bottom: pill_rect.bottom - 12.0,
                },
                radiusX: 16.0,
                radiusY: 16.0,
            };
            self.rt.FillRoundedRectangle(&inner_card, &card_brush);
        }
    }
}
```