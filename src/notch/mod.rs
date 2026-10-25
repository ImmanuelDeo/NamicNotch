// src/notch/mod.rs
//! Notch window and rendering for Nusa Notch Clone

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
    SetState(NotchState),
    Show,
    Hide,
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
                cyTopHeight: -1,
                cyBottomHeight: -1,
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
                NotchEvent::SetState(s) => self.controller.transition_to(s),
                NotchEvent::Show => unsafe { let _ = ShowWindow(self.hwnd, SW_SHOW); },
                NotchEvent::Hide => unsafe { let _ = ShowWindow(self.hwnd, windows::Win32::UI::WindowsAndMessaging::SW_HIDE); },
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
                        self.controller.transition_to(NotchState::Music);
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