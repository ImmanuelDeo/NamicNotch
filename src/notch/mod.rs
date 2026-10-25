//! Notch window and rendering for Nusa Notch Clone

pub mod renderer;
pub mod states;

use anyhow::Result;
use std::sync::{Arc, RwLock};
use std::time::Instant;
use tokio::sync::mpsc;
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, POINT, WPARAM, COLORREF};
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, RegisterClassExW, SetWindowPos, ShowWindow,
    GetCursorPos, DestroyWindow, PostQuitMessage,
    SetWindowLongPtrW, GetWindowLongPtrW, GWLP_USERDATA, GWL_EXSTYLE,
    WM_PAINT, WM_LBUTTONDOWN, WM_RBUTTONUP, WM_TIMER, WM_DESTROY,
    WM_ERASEBKGND,
    WS_EX_TOPMOST, WS_EX_TOOLWINDOW, WS_EX_LAYERED, WS_EX_TRANSPARENT,
    WS_POPUP, SW_SHOW, SW_SHOWNOACTIVATE, HWND_TOPMOST, SWP_NOACTIVATE, SWP_NOOWNERZORDER, SWP_SHOWWINDOW,
    CS_HREDRAW, CS_VREDRAW, WNDCLASSEXW,
    GetMessageW, TranslateMessage, DispatchMessageW,
    MSG, HICON, HCURSOR, HMENU,
    SetTimer, KillTimer,
    SetLayeredWindowAttributes, LWA_ALPHA,
};
use windows::Win32::Graphics::Gdi::UpdateWindow;
use windows::Win32::UI::Controls::MARGINS;
use windows::Win32::Graphics::Dwm::DwmExtendFrameIntoClientArea;
use windows::Win32::Graphics::Gdi::{
    GetMonitorInfoW, MonitorFromWindow, MONITORINFO, MONITOR_DEFAULTTONEAREST,
    HBRUSH, PAINTSTRUCT, BeginPaint, EndPaint, InvalidateRect,
};
use windows::Win32::Graphics::Direct2D::Common::D2D_RECT_F;
use windows::Win32::UI::HiDpi::GetDpiForWindow;
use windows::core::PCWSTR;

use crate::settings::Settings;
use crate::tray::{setup_tray_icon, remove_tray_icon, show_tray_menu, handle_tray_command, TRAY_MESSAGE};
pub use renderer::NotchRenderer;
pub use states::{NotchController, NotchState};

pub const NOTCH_WINDOW_WIDTH: i32 = 540;
pub const NOTCH_WINDOW_HEIGHT: i32 = 240;

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
    UpdateBattery(u8),
    UpdateCharging(bool),
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
        let dpi_scale = (dpi as f32 / 96.0).max(1.0);

        let monitor_rect = unsafe {
            let monitor = MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST);
            let mut info = MONITORINFO {
                cbSize: std::mem::size_of::<MONITORINFO>() as u32,
                ..Default::default()
            };
            GetMonitorInfoW(monitor, &mut info).ok()?;
            info.rcWork
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
                SWP_NOACTIVATE | SWP_NOOWNERZORDER,
            );

            // True glass transparency via DWM
            let margins = MARGINS {
                cxLeftWidth: -1,
                cxRightWidth: -1,
                cyTopHeight: -1,
                cyBottomHeight: -1,
            };
            let _ = DwmExtendFrameIntoClientArea(hwnd, &margins);
        }

        let renderer = NotchRenderer::new(hwnd, win_w as u32, win_h as u32)?;
        let mut controller = NotchController::new();

        {
            let s = settings.read().unwrap();
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
        let right = left + pw;
        let top = 0.0;
        let bottom = ph;

        D2D_RECT_F {
            left,
            top,
            right,
            bottom,
        }
    }

    pub fn on_tick(&mut self) -> Result<()> {
        let now = Instant::now();
        let dt = (now.duration_since(self.last_tick).as_secs_f32()).clamp(0.001, 0.05);
        self.last_tick = now;

        // 1. Drain background events
        while let Ok(event) = self.event_rx.try_recv() {
            match event {
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
                NotchEvent::UpdateTimer(_) => {}
                _ => {}
            }
        }

        // 2. Sync settings flags
        {
            if let Ok(s) = self.settings.read() {
                self.controller.mascot_enabled = s.show_mascot;
                self.controller.minimal_mode = s.minimal_notch;
            }
        }

        // 3. Dynamic Hit-Testing: check cursor against pill bounding box
        let pill_rect = self.current_pill_rect();
        let pill_screen_left = self.win_x as f32 + pill_rect.left;
        let pill_screen_right = self.win_x as f32 + pill_rect.right;
        let pill_screen_top = self.win_y as f32 + pill_rect.top;
        let pill_screen_bottom = self.win_y as f32 + pill_rect.bottom;

        let mut pt = POINT::default();
        unsafe { let _ = GetCursorPos(&mut pt); }

        let cursor_in_pill = pt.x as f32 >= pill_screen_left - 4.0
            && pt.x as f32 <= pill_screen_right + 4.0
            && pt.y as f32 >= pill_screen_top - 4.0
            && pt.y as f32 <= pill_screen_bottom + 6.0;

        self.cursor_in_pill = cursor_in_pill;

        // Dynamic click-through: strip WS_EX_TRANSPARENT when inside pill so click works
        unsafe {
            let style = GetWindowLongPtrW(self.hwnd, GWL_EXSTYLE);
            if cursor_in_pill {
                if (style & (WS_EX_TRANSPARENT.0 as isize)) != 0 {
                    let _ = SetWindowLongPtrW(self.hwnd, GWL_EXSTYLE, style & !(WS_EX_TRANSPARENT.0 as isize));
                }
            } else {
                if (style & (WS_EX_TRANSPARENT.0 as isize)) == 0 {
                    let _ = SetWindowLongPtrW(self.hwnd, GWL_EXSTYLE, style | (WS_EX_TRANSPARENT.0 as isize));
                }
            }
        }

        // Eye cursor tracking offset (clamped to 360px radius)
        let pill_center_x = (pill_screen_left + pill_screen_right) * 0.5;
        let pill_center_y = (pill_screen_top + pill_screen_bottom) * 0.5;
        let dx = pt.x as f32 - pill_center_x;
        let dy = pt.y as f32 - pill_center_y;
        let dist = (dx * dx + dy * dy).sqrt();
        let cursor_offset = if dist < 380.0 {
            Some((dx, dy))
        } else {
            None
        };

        // 4. Update spring physics and animations
        self.controller.update(dt, cursor_in_pill, cursor_offset);

        // 5. Trigger repaint
        unsafe {
            let _ = InvalidateRect(self.hwnd, None, false);
        }

        Ok(())
    }

    pub fn on_lbutton_down(&mut self, x: f32, y: f32) {
        // First check if Chibi Liebe was poked
        if self.controller.chibi_liebe.handle_click(x, y) {
            unsafe { let _ = InvalidateRect(self.hwnd, None, false); }
            return;
        }

        let pill_rect = self.current_pill_rect();

        // Check top tab clicks when expanded
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
                    4 => self.controller.transition_to(NotchState::Shelf),
                    _ => {}
                }
                unsafe { let _ = InvalidateRect(self.hwnd, None, false); }
                return;
            }
        }

        // Toggle state or awaken if clicked inside pill
        if self.cursor_in_pill {
            if self.controller.state == NotchState::Pet {
                self.controller.devil_eyes.trigger_wake_surge();
                let target = self.controller.last_expanded_tab;
                self.controller.transition_to(target);
            } else {
                self.controller.transition_to(NotchState::Pet);
            }
            unsafe { let _ = InvalidateRect(self.hwnd, None, false); }
        }
    }
}

pub struct NotchWindow {
    pub hwnd: HWND,
    _raw_app: *mut NotchApp,
    _tray_nid: windows::Win32::UI::Shell::NOTIFYICONDATAW,
}

impl NotchWindow {
    pub fn create(
        hinstance: HINSTANCE,
        event_rx: mpsc::Receiver<NotchEvent>,
        settings: Arc<RwLock<Settings>>,
    ) -> Result<Self> {
        let class_name = to_wide_string("NusaNotchCloneWindow\0");

        let wc = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(notch_wnd_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: hinstance,
            hIcon: HICON::default(),
            hCursor: HCURSOR::default(),
            hbrBackground: HBRUSH::default(),
            lpszMenuName: PCWSTR::null(),
            lpszClassName: PCWSTR(class_name.as_ptr()),
            hIconSm: HICON::default(),
        };

        unsafe {
            RegisterClassExW(&wc);

            // CRITICAL BUG FIX 1a: NO WS_VISIBLE flag in CreateWindowExW
            let hwnd = CreateWindowExW(
                WS_EX_TOPMOST | WS_EX_TOOLWINDOW | WS_EX_LAYERED,
                PCWSTR(class_name.as_ptr()),
                PCWSTR(to_wide_string("NusaNotch\0").as_ptr()),
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

            // CRITICAL BUG FIX 1b: Set opacity for layered window (otherwise invisible on Win10/11)
            SetLayeredWindowAttributes(hwnd, COLORREF(0), 255, LWA_ALPHA)?;

            let app = Box::new(NotchApp::new(hwnd, settings.clone(), event_rx)?);
            let win_x = app.win_x;
            let win_y = app.win_y;
            let win_w = app.win_w;
            let win_h = app.win_h;
            let raw_ptr = Box::into_raw(app);
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, raw_ptr as isize);

            // Setup System Tray icon
            let nid = setup_tray_icon(hwnd)?;

            // Critically damped ~120 FPS timer (8ms)
            SetTimer(hwnd, 1, 8, None);

            // CRITICAL BUG FIX 1c: Explicitly show & position window properly after USERDATA and timer are set
            let _ = ShowWindow(hwnd, SW_SHOWNOACTIVATE);
            let _ = SetWindowPos(
                hwnd,
                HWND_TOPMOST,
                win_x,
                win_y,
                win_w,
                win_h,
                SWP_SHOWWINDOW | SWP_NOACTIVATE,
            );
            let _ = InvalidateRect(hwnd, None, false);
            let _ = UpdateWindow(hwnd);

            Ok(Self {
                hwnd,
                _raw_app: raw_ptr,
                _tray_nid: nid,
            })
        }
    }
}

impl Drop for NotchWindow {
    fn drop(&mut self) {
        unsafe {
            remove_tray_icon(&self._tray_nid);
            if !self._raw_app.is_null() {
                let _ = Box::from_raw(self._raw_app);
                self._raw_app = std::ptr::null_mut();
            }
        }
    }
}

/// Run standard Win32 message loop on the main thread
pub fn run_message_loop() {
    unsafe {
        let mut msg = MSG::default();
        while GetMessageW(&mut msg, HWND::default(), 0, 0).as_bool() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}

pub fn run_ui(
    hinstance: HINSTANCE,
    event_rx: mpsc::Receiver<NotchEvent>,
    settings: Arc<RwLock<Settings>>,
) -> Result<()> {
    let _window = NotchWindow::create(hinstance, event_rx, settings)?;
    run_message_loop();
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
                // CRITICAL BUG FIX 2: Return 1 to suppress GDI background erase (Direct2D handles it)
                return LRESULT(1);
            }
            WM_TIMER => {
                let _ = app.on_tick();
                return LRESULT(0);
            }
            WM_PAINT => {
                let mut ps = PAINTSTRUCT::default();
                unsafe {
                    BeginPaint(hwnd, &mut ps);
                    let pill_rect = app.current_pill_rect();
                    let _ = app.renderer.render_frame(&mut app.controller, pill_rect);
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
            TRAY_MESSAGE => {
                let event = lparam.0 as u32;
                if event == WM_RBUTTONUP {
                    if let Ok(cmd) = show_tray_menu(hwnd, &app.settings) {
                        if handle_tray_command(cmd, hwnd, &app.settings).unwrap_or(false) {
                            unsafe {
                                let _ = KillTimer(hwnd, 1);
                                let _ = DestroyWindow(hwnd);
                            }
                        }
                    }
                } else if event == windows::Win32::UI::WindowsAndMessaging::WM_LBUTTONUP {
                    if app.controller.is_hovered || app.controller.state != NotchState::Pet {
                        app.controller.is_hovered = false;
                        app.controller.transition_to(NotchState::Pet);
                    } else {
                        app.controller.is_hovered = true;
                        let target = app.controller.last_expanded_tab;
                        app.controller.transition_to(target);
                    }
                    unsafe { let _ = InvalidateRect(hwnd, None, false); }
                }
                return LRESULT(0);
            }
            WM_DESTROY => {
                            unsafe {
                                let _ = KillTimer(hwnd, 1);
                                PostQuitMessage(0);
                            }
                            return LRESULT(0);
                        }
            _ => {}
        }
    } else {
        // CRITICAL BUG FIX 2b: If ptr is null (early WM_CREATE/WM_NCCREATE), suppress background erase
        if msg == WM_ERASEBKGND {
            return LRESULT(1);
        }
    }

    unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
}

fn to_wide_string(s: &str) -> Vec<u16> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    OsStr::new(s).encode_wide().collect()
}
