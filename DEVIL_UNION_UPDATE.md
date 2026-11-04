# Nusa Clone v7.2 (Khusus Repositori `D:\NusaNotch-RE`): Perbaikan Total Notch Muncul Kembali, Clean Nusa Original UI, Pomodoro 25m/5m, Fix Timeline Musik, Custom Icon, & Dynamic Idle Aura

Terima kasih sudah mengirimkan isi `src/main.rs` dan `src/notch/mod.rs` dari **`D:\NusaNotch-RE`**! Dari kedua file tersebut, akar masalah mengapa notch tidak muncul (atau gagal dikompilasi) terlihat sangat jelas.

---

## 1. Diagnosis Pasti dari Kode `D:\NusaNotch-RE` Kamu & Solusi di `v7.2`

| # | Temuan di Kode `D:\NusaNotch-RE` Kamu | Mengapa Menyebabkan Notch Tidak Muncul | Solusi Presisi di Kode `v7.2` Ini |
| :- | :--- | :--- | :--- |
| **1** | **Di `src/notch/mod.rs` baris `run_ui()`, Hermes menghapus `WS_EX_LAYERED` (`// TANPA WS_EX_LAYERED!`), namun di `on_tick()` ia memanggil `SetWindowLongPtrW(..., style \| WS_EX_TRANSPARENT)`** | Pada Windows 11, menyetel `WS_EX_TRANSPARENT` pada window yang **tidak memiliki `WS_EX_LAYERED`** merusak komposisi DWM Direct2D sehingga window tidak digambar ke layar sama sekali. | Mengembalikan `WS_EX_LAYERED` pada `CreateWindowExW` diikuti **`SetLayeredWindowAttributes(hwnd, COLORREF(0), 255, LWA_ALPHA)`** + `DwmExtendFrameIntoClientArea` (persis kombinasi yang berhasil memunculkan notch di versi sebelumnya). |
| **2** | **`src/notch/mod.rs` Memanggil `NotchRenderer::new(hwnd, win_w, win_h, dpi_scale)` & `app.renderer.render_frame(&mut app.controller, pill_rect)`** | Di panduan v7.0/v7.1 sebelumnya, fungsi di `renderer.rs` bernama `draw_frame` (tanpa `BeginDraw()` / `EndDraw()`) dan memakai `use crate::theme::AntiMagicNeutralPalette;` (padahal di `src/main.rs` tidak ada `mod theme;`). Akibatnya `BeginDraw()`/`EndDraw()` tidak terpanggil saat `WM_PAINT` atau build gagal! | Di **v7.2**, `src/notch/renderer.rs` disesuaikan **100% identik dengan struktur `D:\NusaNotch-RE`**: menggunakan `NotchRenderer::new(hwnd, width, height, dpi_scale)` berbasis `ID2D1HwndRenderTarget`, memiliki `pub fn render_frame(&mut self, ctrl: &mut NotchController, pill_rect: D2D_RECT_F)` yang memanggil `BeginDraw()` ➔ `Clear()` ➔ `draw_frame()` ➔ `EndDraw()`, dan menyertakan `AntiMagicNeutralPalette` langsung di dalam `renderer.rs`. |
| **3** | **`self.controller.minimal_mode = s.minimal_notch` di `on_tick()`** | Jika di file `settings.json` nilai `minimal_notch` tersimpan `true`, maka setiap 8ms tinggi notch dipaksa menyusut menjadi `6px`. | Memastikan saat aplikasi dijalankan, `minimal_mode` diinisialisasi ke `false` dan tidak ditimpa paksa setiap *tick* kecuali diubah dari menu *System Tray*. |
| **4** | **`NotchEvent::UpdateMedia` dari `src/media.rs` Ikut Menimpa Timeline Musik** | Selain *poller* internal, `src/main.rs` menjalankan `media::MediaManager` yang mengirim `NotchEvent::UpdateMedia(info)` ke `src/notch/mod.rs`. | Mengamankan handler `NotchEvent::UpdateMedia` di `src/notch/mod.rs` agar hanya memperbarui judul/artis/status *play* tanpa mereset `media_elapsed_exact` ke detik ke-2 (`00:02`). |

---

## 2. Prompt Siap Pakai untuk Hermes Agent (Khusus `D:\NusaNotch-RE` — Versi 7.2)

Salin seluruh blok perintah di bawah ini dan tempelkan langsung ke **[Hermes Agent](https://github.com/nousresearch/hermes-agent)** di terminal `D:\NusaNotch-RE`:

```markdown
Tolong baca panduan `DEVIL_UNION_UPGRADE.md` (Versi 7.2 khusus repositori `D:\NusaNotch-RE`) dan terapkan perbaikan berikut secara persis tanpa mengubah signature fungsi penghubung antar modul:

1. **Matikan Proses Lama**:
   - Jalankan `taskkill /F /IM nusa-clone.exe` atau matikan binary yang sedang berjalan.

2. **Ganti `src/notch/mod.rs` dengan Kode Utuh v7.2**:
   - Pertahankan struktur `NotchEvent`, `NotchApp`, dan `pub fn run_ui(hinstance: HINSTANCE, event_rx: mpsc::Receiver<NotchEvent>, settings: Arc<RwLock<Settings>>) -> Result<()>` milik `D:\NusaNotch-RE`.
   - PENTING: Di `run_ui()`, WAJIB sertakan `WS_EX_LAYERED` pada `ex_style` (`WS_EX_LAYERED | WS_EX_TOPMOST | WS_EX_TOOLWINDOW | WS_EX_ACCEPTFILES | WS_EX_NOACTIVATE`) dan panggil `SetLayeredWindowAttributes(hwnd, COLORREF(0), 255, LWA_ALPHA)` tepat setelah `CreateWindowExW` serta `DwmExtendFrameIntoClientArea` dengan `MARGINS { -1, -1, -1, -1 }` agar window 100% muncul di layar dan dynamic click-through (`WS_EX_TRANSPARENT`) berfungsi normal.

3. **Ganti `src/notch/renderer.rs` dengan Kode Utuh v7.2**:
   - Gunakan signature `pub fn new(hwnd: HWND, width: u32, height: u32, dpi_scale: f32) -> Result<Self>` dengan `ID2D1HwndRenderTarget` dan `ID2D1Factory`.
   - Definisikan `AntiMagicNeutralPalette` langsung di dalam `src/notch/renderer.rs` (jangan mengimpor `crate::theme` karena tidak ada di `src/main.rs`).
   - Pastikan `pub fn render_frame(&mut self, ctrl: &mut NotchController, pill_rect: D2D_RECT_F)` memanggil `self.rt.BeginDraw()`, `self.rt.Clear(...)`, `self.draw_frame(ctrl, pill_rect)`, dan `self.rt.EndDraw(None, None)`.

4. **Perbarui `src/notch/states.rs`, `src/mascot/eyes.rs`, dan `src/icon.rs` Sesuai Kode v7.2**:
   - Di `src/mascot/eyes.rs`, gunakan parameter `rt: &ID2D1HwndRenderTarget, factory: &ID2D1Factory` agar cocok 100% dengan `NotchRenderer` di `src/notch/renderer.rs` (cek juga `src/mascot/chibi_liebe.rs` agar parameter `draw` memakai `&ID2D1HwndRenderTarget` dan `&ID2D1Factory`).
   - Pastikan `src/main.rs` bersih tanpa baris duplikat dan memiliki `pub mod icon;`.

5. **Kompilasi & Verifikasi**:
   - Jalankan `cargo check` dan `cargo build --release`, pastikan kompilasi sukses (0 errors), lalu jalankan binary-nya.
```

---

## 3. Kode Lengkap Modul Rust (`v7.2` — 100% Cocok dengan `D:\NusaNotch-RE`)

### A. `src/main.rs` — Bersih Tanpa Baris Duplikat

Pastikan `src/main.rs` di `D:\NusaNotch-RE` bersih seperti ini:

```rust
// src/main.rs
#![windows_subsystem = "windows"]
//! Nusa Notch Clone - A living notch for Windows
//!
//! Clean native Win32/Direct2D implementation running UI on main thread.

mod hooks;
pub mod icon;
mod mascot;
mod media;
mod notch;
mod settings;
mod shelf;
mod timers;
mod tray;
mod utils;

use anyhow::Result;
use single_instance::SingleInstance;
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc;
use windows::Win32::Foundation::HINSTANCE;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;

fn main() -> Result<()> {
    // 1. Single instance check
    let instance = SingleInstance::new("NusaNotchClone_SingleInstance_Mutex")?;
    if !instance.is_single() {
        return Ok(());
    }

    // 2. Load settings
    let settings = Arc::new(RwLock::new(settings::Settings::load().unwrap_or_default()));

    // 3. Setup Tokio runtime for background async workers
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    // 4. Inter-thread channels
    let (notch_tx, notch_rx) = mpsc::channel::<notch::NotchEvent>(64);
    let (media_tx, media_rx) = mpsc::channel::<media::MediaEvent>(64);

    // 5. Spawn background workers
    let media_mgr = media::MediaManager::new(media_tx, media_rx, notch_tx.clone(), settings.clone())?;
    rt.spawn(async move {
        let _ = media_mgr.run().await;
    });

    let timer_mgr = timers::TimerManager::new(settings.clone(), notch_tx.clone())?;
    rt.spawn(async move {
        let _ = timer_mgr.run().await;
    });

    let _shelf = shelf::Shelf::new(settings.clone())?;

    let hooks_mgr = hooks::HooksManager::new(notch_tx.clone(), settings.clone())?;
    std::thread::spawn(move || {
        let _ = hooks_mgr.run();
    });

    // 6. Run Win32 UI on main thread
    let hinstance: HINSTANCE = unsafe { GetModuleHandleW(None)? }.into();
    notch::run_ui(hinstance, notch_rx, settings)?;

    Ok(())
}
```
*(Catatan untuk Hermes Agent: jika konstruktor `media::MediaManager::new` di `src/media.rs` hanya menerima 3 argumen `(media_tx, media_rx, settings.clone())`, pertahankan argumen di `src/main.rs` yang sudah ada dan cukup hapus baris-baris yang terduplikasi).*

---

### B. `src/notch/mod.rs` — Kode Lengkap Utuh untuk `D:\NusaNotch-RE` (Dengan `WS_EX_LAYERED` + `LWA_ALPHA` + `DwmExtendFrameIntoClientArea`)

Salin kode utuh ini ke `src/notch/mod.rs`:

```rust
// src/notch/mod.rs
//! Notch window and rendering for Nusa Notch Clone (v7.2 - D:\NusaNotch-RE)

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
    BeginPaint, EndPaint, GetMonitorInfoW, InvalidateRect, MonitorFromWindow, UpdateWindow, HBRUSH,
    MONITORINFO, MONITOR_DEFAULTTOPRIMARY, PAINTSTRUCT,
};
use windows::Win32::UI::Controls::MARGINS;
use windows::Win32::UI::HiDpi::GetDpiForWindow;
use windows::Win32::UI::Shell::{DragAcceptFiles, DragFinish, DragQueryFileW, HDROP};
use windows::Win32::UI::WindowsAndMessaging::*;

use crate::settings::Settings;
use crate::tray::{
    create_tray_icon, handle_tray_command, remove_tray_icon, show_tray_menu, WM_TRAYICON,
};

pub use renderer::NotchRenderer;
pub use states::{NotchController, NotchState, OverlayKind};

pub const NOTCH_WINDOW_WIDTH: i32 = 540;
pub const NOTCH_WINDOW_HEIGHT: i32 = 220;
const TIMER_ID_ANIMATION: usize = 1;

#[derive(Debug, Clone)]
pub enum NotchEvent {
    SetState(NotchState),
    Show,
    Hide,
    Toggle,
    UpdateMedia(crate::media::MediaInfo),
    UpdateLyrics(Vec<String>),
    UpdateTimer(crate::timers::TimerInfo),
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
    pub last_cursor_logical_y: f32,
}

impl NotchApp {
    pub fn new(
        hwnd: HWND,
        settings: Arc<RwLock<Settings>>,
        event_rx: mpsc::Receiver<NotchEvent>,
    ) -> Result<Self> {
        let dpi = unsafe { GetDpiForWindow(hwnd) };
        let dpi_scale = if dpi == 0 { 1.0 } else { dpi as f32 / 96.0 };

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
                SWP_NOACTIVATE,
            );

            // WAJIB: SetLayeredWindowAttributes dengan LWA_ALPHA (255) + DwmExtendFrameIntoClientArea
            // agar window 100% terlihat dan mendukung klik tembus (WS_EX_TRANSPARENT) di luar pill!
            let _ = SetLayeredWindowAttributes(hwnd, COLORREF(0), 255, LWA_ALPHA);

            let margins = MARGINS {
                cxLeftWidth: -1,
                cxRightWidth: -1,
                cyTopHeight: -1,
                cyBottomHeight: -1,
            };
            let _ = DwmExtendFrameIntoClientArea(hwnd, &margins);
        }

        let renderer = NotchRenderer::new(hwnd, win_w as u32, win_h as u32, dpi_scale)?;
        let mut controller = NotchController::new();
        controller.mascot_enabled = true;
        controller.minimal_mode = false;

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
            last_cursor_logical_y: 0.0,
        })
    }

    pub fn current_pill_rect(&self) -> D2D_RECT_F {
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

    pub fn on_tick(&mut self) -> Result<()> {
        let now = Instant::now();
        let dt = now.duration_since(self.last_tick).as_secs_f32();
        self.last_tick = now;

        while let Ok(ev) = self.event_rx.try_recv() {
            match ev {
                NotchEvent::SetState(s) => self.controller.transition_to(s),
                NotchEvent::Show => unsafe {
                    let _ = ShowWindow(self.hwnd, SW_SHOWNOACTIVATE);
                },
                NotchEvent::Hide => unsafe {
                    let _ = ShowWindow(self.hwnd, SW_HIDE);
                },
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
                        let title_changed = self.controller.media_title != info.title;
                        self.controller.media_title = info.title;
                        if title_changed {
                            self.controller.media_elapsed_exact = 0.0;
                            self.controller.media_progress = 0.0;
                        }
                    }
                    if !info.artist.is_empty() {
                        self.controller.media_artist = info.artist;
                    }
                    self.controller.media_playing = info.playing;
                }
                NotchEvent::UpdateLyrics(lines) => {
                    if !lines.is_empty() && self.controller.timed_lyrics.len() <= 4 {
                        let step = (self.controller.media_total_secs.max(60.0))
                            / (lines.len().max(1) as f32);
                        self.controller.timed_lyrics = lines
                            .into_iter()
                            .enumerate()
                            .map(|(i, l)| (i as f32 * step, l))
                            .collect();
                    }
                }
                NotchEvent::UpdateTimer(info) => {
                    self.controller.pomo_remaining_secs = info.remaining_seconds;
                    self.controller.pomo_total_secs = info.total_seconds;
                    self.controller.pomo_running = info.running;
                    self.controller.pomo_is_break = matches!(
                        info.mode,
                        crate::timers::PomodoroMode::ShortBreak
                            | crate::timers::PomodoroMode::LongBreak
                    );
                }
                NotchEvent::UpdateShelf(items) => {
                    self.controller.shelf_files = items
                        .into_iter()
                        .map(|item| {
                            (
                                item.name.clone(),
                                format!("{} · {:?}", item.size, item.item_type),
                                item.path.to_string_lossy().to_string(),
                            )
                        })
                        .collect();
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
            }
        }

        let mut pt = POINT::default();
        unsafe {
            let _ = GetCursorPos(&mut pt);
        }

        let local_logical_x = (pt.x - self.win_x) as f32 / self.dpi_scale;
        let local_logical_y = (pt.y - self.win_y) as f32 / self.dpi_scale;
        self.last_cursor_logical_x = local_logical_x;
        self.last_cursor_logical_y = local_logical_y;

        let pill_rect = self.current_pill_rect();
        let pad = if self.controller.is_hovered { 10.0 } else { 6.0 };
        let cursor_in_pill = local_logical_x >= pill_rect.left - pad
            && local_logical_x <= pill_rect.right + pad
            && local_logical_y >= pill_rect.top
            && local_logical_y <= pill_rect.bottom + pad;
        self.cursor_in_pill = cursor_in_pill;

        // Dynamic click-through: bekerja mulus karena window memiliki WS_EX_LAYERED!
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
        let cursor_offset = Some((local_logical_x - pill_cx, local_logical_y - pill_cy));

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
            // 1. Klik Tombol Bulat `<` (kiri) atau `>` (kanan) di tepi notch
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
}

pub fn run_ui(
    hinstance: HINSTANCE,
    event_rx: mpsc::Receiver<NotchEvent>,
    settings: Arc<RwLock<Settings>>,
) -> Result<()> {
    let class_name = to_wide_string("NusaNotchCloneWindow\0");
    let app_icon = crate::icon::create_antimagic_hicon();

    let wc = WNDCLASSEXW {
        cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(notch_wnd_proc),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: hinstance,
        hIcon: app_icon,
        hCursor: unsafe { LoadCursorW(None, IDC_ARROW).unwrap_or_default() },
        hbrBackground: HBRUSH::default(),
        lpszMenuName: PCWSTR::null(),
        lpszClassName: PCWSTR(class_name.as_ptr()),
        hIconSm: app_icon,
    };

    unsafe {
        RegisterClassExW(&wc);

        // WAJIB menyertakan WS_EX_LAYERED bersama WS_EX_TOPMOST | WS_EX_TOOLWINDOW | WS_EX_ACCEPTFILES | WS_EX_NOACTIVATE
        let ex_style = WS_EX_LAYERED
            | WS_EX_TOPMOST
            | WS_EX_TOOLWINDOW
            | WS_EX_ACCEPTFILES
            | WS_EX_NOACTIVATE;

        let hwnd = CreateWindowExW(
            ex_style,
            PCWSTR(class_name.as_ptr()),
            PCWSTR(to_wide_string("Notch\0").as_ptr()),
            WS_POPUP | WS_VISIBLE,
            0,
            0,
            NOTCH_WINDOW_WIDTH,
            NOTCH_WINDOW_HEIGHT,
            HWND::default(),
            HMENU::default(),
            hinstance,
            None,
        )?;

        let _ = SetLayeredWindowAttributes(hwnd, COLORREF(0), 255, LWA_ALPHA);
        DragAcceptFiles(hwnd, true);

        let app = Box::new(NotchApp::new(hwnd, settings.clone(), event_rx)?);
        let win_x = app.win_x;
        let win_y = app.win_y;
        let win_w = app.win_w;
        let win_h = app.win_h;

        let raw_ptr = Box::into_raw(app);
        SetWindowLongPtrW(hwnd, GWLP_USERDATA, raw_ptr as isize);

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
        let _ = UpdateWindow(hwnd);
        let nid = create_tray_icon(hwnd)?;
        let _ = SetTimer(hwnd, TIMER_ID_ANIMATION, 8, None);
        let _ = InvalidateRect(hwnd, None, false);

        let mut msg = MSG::default();
        while GetMessageW(&mut msg, HWND::default(), 0, 0).as_bool() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

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
            WM_MOUSEACTIVATE => return LRESULT(MA_NOACTIVATE as isize),
            WM_TIMER => {
                if wparam.0 == TIMER_ID_ANIMATION {
                    let _ = app.on_tick();
                }
                return LRESULT(0);
            }
            WM_PAINT => {
                let pill_rect = app.current_pill_rect();
                unsafe {
                    let mut ps = PAINTSTRUCT::default();
                    let _ = BeginPaint(hwnd, &mut ps);
                    app.renderer.render_frame(&mut app.controller, pill_rect);
                    let _ = EndPaint(hwnd, &ps);
                }
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
                let lx = (((lparam.0 & 0xFFFF) as i16 - app.win_x as i16) as f32 / app.dpi_scale)
                    - pill_rect.left;
                let ly = ((((lparam.0 >> 16) & 0xFFFF) as i16 - app.win_y as i16) as f32
                    / app.dpi_scale)
                    - pill_rect.top;
                app.controller.on_mouse_wheel(delta, lx, ly, pill_w);
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
                            let full_path = String::from_utf16_lossy(&buf);
                            app.controller.add_dropped_file(full_path);
                        }
                    }
                    DragFinish(hdrop);
                    let _ = InvalidateRect(hwnd, None, false);
                }
                return LRESULT(0);
            }
            WM_TRAYICON => {
                let event = lparam.0 as u32;
                if event == WM_RBUTTONUP {
                    if let Ok(cmd) = show_tray_menu(hwnd, &app.settings) {
                        if handle_tray_command(cmd, hwnd, &app.settings).unwrap_or(false) {
                            unsafe {
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

### C. `src/notch/renderer.rs` — Kode Lengkap Utuh untuk `D:\NusaNotch-RE` (Dengan `NotchRenderer::new(hwnd, width, height, dpi_scale)` & `render_frame` Lengkap `BeginDraw`/`EndDraw`)

Salin kode utuh ini ke `src/notch/renderer.rs`:

```rust
// src/notch/renderer.rs
use anyhow::Result;
use chrono::Local;
use windows::core::PCWSTR;
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Direct2D::Common::{
    D2D1_ALPHA_MODE_PREMULTIPLIED, D2D1_BEZIER_SEGMENT, D2D1_COLOR_F, D2D1_FIGURE_BEGIN_FILLED,
    D2D1_FIGURE_END_CLOSED, D2D1_PIXEL_FORMAT, D2D_POINT_2F, D2D_RECT_F, D2D_SIZE_U,
};
use windows::Win32::Graphics::Direct2D::{
    D2D1CreateFactory, ID2D1Factory, ID2D1HwndRenderTarget, ID2D1PathGeometry, ID2D1StrokeStyle,
    D2D1_CAP_STYLE_ROUND, D2D1_DASH_STYLE_DASH, D2D1_ELLIPSE, D2D1_FACTORY_TYPE_SINGLE_THREADED,
    D2D1_FEATURE_LEVEL_DEFAULT, D2D1_HWND_RENDER_TARGET_PROPERTIES, D2D1_LINE_JOIN_ROUND,
    D2D1_PRESENT_OPTIONS_IMMEDIATELY, D2D1_RENDER_TARGET_PROPERTIES,
    D2D1_RENDER_TARGET_TYPE_DEFAULT, D2D1_RENDER_TARGET_USAGE_NONE, D2D1_ROUNDED_RECT,
    D2D1_STROKE_STYLE_PROPERTIES,
};
use windows::Win32::Graphics::DirectWrite::{
    DWriteCreateFactory, IDWriteFactory, IDWriteTextFormat, DWRITE_FACTORY_TYPE_SHARED,
    DWRITE_FONT_STRETCH_NORMAL, DWRITE_FONT_STYLE_ITALIC, DWRITE_FONT_STYLE_NORMAL,
    DWRITE_FONT_WEIGHT_BOLD, DWRITE_FONT_WEIGHT_MEDIUM, DWRITE_FONT_WEIGHT_SEMI_BOLD,
    DWRITE_PARAGRAPH_ALIGNMENT_CENTER, DWRITE_TEXT_ALIGNMENT_CENTER, DWRITE_TEXT_ALIGNMENT_LEADING,
    DWRITE_TEXT_ALIGNMENT_TRAILING,
};
use windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_B8G8R8A8_UNORM;

use crate::notch::states::{NotchController, NotchState, OverlayKind};

/// Palet Warna Anti-Magic Biru-Hitam Netral (Didefinisikan langsung agar tidak butuh `mod theme`)
pub struct AntiMagicNeutralPalette;

impl AntiMagicNeutralPalette {
    pub const TEXT_PRIMARY: D2D1_COLOR_F = D2D1_COLOR_F {
        r: 0.97,
        g: 0.98,
        b: 1.00,
        a: 1.0,
    };
    pub const TEXT_SECONDARY: D2D1_COLOR_F = D2D1_COLOR_F {
        r: 0.60,
        g: 0.65,
        b: 0.74,
        a: 1.0,
    };
    pub const ACCENT_BLUE: D2D1_COLOR_F = D2D1_COLOR_F {
        r: 0.23,
        g: 0.51,
        b: 0.96,
        a: 1.0,
    };
    pub const ACCENT_CYAN: D2D1_COLOR_F = D2D1_COLOR_F {
        r: 0.22,
        g: 0.74,
        b: 0.97,
        a: 1.0,
    };
}

pub struct NotchRenderer {
    pub factory: ID2D1Factory,
    pub rt: ID2D1HwndRenderTarget,
    pub dwrite: IDWriteFactory,
    pub dashed_stroke: Option<ID2D1StrokeStyle>,
    pub fmt_clock_hero: IDWriteTextFormat,
    pub fmt_title: IDWriteTextFormat,
    pub fmt_body: IDWriteTextFormat,
    pub fmt_quote_italic: IDWriteTextFormat,
    pub fmt_caption: IDWriteTextFormat,
}

impl NotchRenderer {
    pub fn new(hwnd: HWND, width: u32, height: u32, dpi_scale: f32) -> Result<Self> {
        unsafe {
            let factory: ID2D1Factory =
                D2D1CreateFactory(D2D1_FACTORY_TYPE_SINGLE_THREADED, None)?;

            let rt_props = D2D1_RENDER_TARGET_PROPERTIES {
                r#type: D2D1_RENDER_TARGET_TYPE_DEFAULT,
                pixelFormat: D2D1_PIXEL_FORMAT {
                    format: DXGI_FORMAT_B8G8R8A8_UNORM,
                    alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED,
                },
                dpiX: 96.0 * dpi_scale,
                dpiY: 96.0 * dpi_scale,
                usage: D2D1_RENDER_TARGET_USAGE_NONE,
                minLevel: D2D1_FEATURE_LEVEL_DEFAULT,
            };

            let hwnd_props = D2D1_HWND_RENDER_TARGET_PROPERTIES {
                hwnd,
                pixelSize: D2D_SIZE_U {
                    width: width.max(1),
                    height: height.max(1),
                },
                presentOptions: D2D1_PRESENT_OPTIONS_IMMEDIATELY,
            };

            let rt = factory.CreateHwndRenderTarget(&rt_props, &hwnd_props)?;
            let dwrite: IDWriteFactory = DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED)?;

            let font_family: Vec<u16> = "Segoe UI\0".encode_utf16().collect();
            let locale: Vec<u16> = "en-us\0".encode_utf16().collect();

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
    }

    /// Dipanggil langsung oleh `WM_PAINT` di `src/notch/mod.rs`
    pub fn render_frame(&mut self, ctrl: &mut NotchController, pill_rect: D2D_RECT_F) {
        unsafe {
            self.rt.BeginDraw();
            self.rt.Clear(Some(&D2D1_COLOR_F {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            }));

            self.draw_frame(ctrl, pill_rect);

            let _ = self.rt.EndDraw(None, None);
        }
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

        // Base Notch Obsidian Pitch-Black (#040406)
        let pitch_black = D2D1_COLOR_F {
            r: 0.016,
            g: 0.016,
            b: 0.022,
            a: 1.0,
        };
        if let Ok(bezel_geom) = self.create_top_bezel_notch_geometry(pill_rect, radius, flare) {
            if let Ok(base_brush) = self.rt.CreateSolidColorBrush(&pitch_black, None) {
                self.rt.FillGeometry(&bezel_geom, &base_brush, None);
            }
        } else if let Ok(base_brush) = self.rt.CreateSolidColorBrush(&pitch_black, None) {
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: pill_rect,
                    radiusX: radius,
                    radiusY: radius,
                },
                &base_brush,
            );
        }

        if !ctrl.is_hovered {
            if let Some(ref ov) = ctrl.active_overlay {
                self.draw_system_overlay(ov, pill_rect, alpha);
                return;
            }
        }

        if ctrl.height.value < 55.0 {
            if ctrl.state == NotchState::Idle {
                self.draw_closed_clock_pill(ctrl, pill_rect, alpha);
            } else {
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

        D2D_RECT_F {
            left: pill_rect.left + 32.0,
            top: pill_rect.top + 26.0,
            right: pill_rect.right - 32.0,
            bottom: pill_rect.bottom - 10.0,
        }
    }

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

### D. `src/mascot/eyes.rs` — Menggunakan `&ID2D1HwndRenderTarget` dan `&ID2D1Factory` agar 100% Kompatibel dengan `NotchRenderer` di `D:\NusaNotch-RE`

Di `src/mascot/eyes.rs`, pastikan tipe parameter `draw_idle_aura_background` dan `draw` menggunakan `rt: &ID2D1HwndRenderTarget, factory: &ID2D1Factory` (bukan `ID2D1DeviceContext` / `ID2D1Factory1`):

```rust
// Di bagian atas src/mascot/eyes.rs:
use windows::Win32::Graphics::Direct2D::Common::{
    D2D1_BEZIER_SEGMENT, D2D1_COLOR_F, D2D1_FIGURE_BEGIN_FILLED, D2D1_FIGURE_BEGIN_HOLLOW,
    D2D1_FIGURE_END_CLOSED, D2D1_FIGURE_END_OPEN, D2D_POINT_2F, D2D_RECT_F,
};
use windows::Win32::Graphics::Direct2D::{
    ID2D1Factory, ID2D1HwndRenderTarget, D2D1_ELLIPSE, D2D1_ROUNDED_RECT,
};

// Dan pada kedua fungsi gambar di impl DevilUnionEyes:
    pub unsafe fn draw_idle_aura_background(
        &self,
        rt: &ID2D1HwndRenderTarget,
        factory: &ID2D1Factory,
        pill_rect: D2D_RECT_F,
        alpha: f32,
    ) {
        // ... (isi fungsi sama seperti di v7.0)
    }

    pub unsafe fn draw(
        &self,
        rt: &ID2D1HwndRenderTarget,
        factory: &ID2D1Factory,
        center_x: f32,
        center_y: f32,
        alpha: f32,
    ) {
        // ... (isi fungsi sama seperti di v7.0)
    }
```
*(Pastikan `src/mascot/chibi_liebe.rs` juga menggunakan `rt: &ID2D1HwndRenderTarget, factory: &ID2D1Factory` pada fungsi `pub unsafe fn draw(...)`).*