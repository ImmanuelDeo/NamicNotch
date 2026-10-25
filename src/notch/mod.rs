//! Notch window and rendering for Nusa Notch Clone

use anyhow::Result;
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc;
use tracing::info;
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, POINT, RECT, WPARAM, BOOL};
use windows::Win32::Graphics::Direct2D::{
    D2D1_FACTORY_TYPE_SINGLE_THREADED, D2D1CreateFactory, ID2D1Factory1, ID2D1HwndRenderTarget,
    D2D1_RENDER_TARGET_PROPERTIES, D2D1_HWND_RENDER_TARGET_PROPERTIES,
    D2D1_RENDER_TARGET_USAGE_NONE, D2D1_FEATURE_LEVEL_DEFAULT,
    D2D1_ROUNDED_RECT, D2D1_ELLIPSE,
};
use windows::Win32::Graphics::Direct2D::Common::{
    D2D1_PIXEL_FORMAT, D2D1_ALPHA_MODE_PREMULTIPLIED,
    D2D_SIZE_U, D2D_RECT_F, D2D1_COLOR_F, D2D_POINT_2F,
};
use windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_B8G8R8A8_UNORM;
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, RegisterClassExW, SetWindowPos, ShowWindow,
    GetClientRect, GetCursorPos, DestroyWindow, PostQuitMessage,
    SetWindowLongPtrW, GetWindowLongPtrW, GWLP_USERDATA,
    WM_PAINT, WM_MOUSEMOVE, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_RBUTTONUP,
    WM_TIMER, WM_DESTROY,
    WS_EX_TOPMOST, WS_EX_TOOLWINDOW,
    WS_POPUP, SW_SHOW, HWND_TOPMOST, SWP_NOACTIVATE, SWP_NOOWNERZORDER,
    CS_HREDRAW, CS_VREDRAW, WNDCLASSEXW,
    GetMessageW, TranslateMessage, DispatchMessageW,
    MSG, HICON, HCURSOR, HMENU,
    SetTimer, KillTimer,
};
use windows::Win32::Graphics::Gdi::{
    GetMonitorInfoW, MonitorFromWindow, MONITORINFO, MONITOR_DEFAULTTONEAREST,
    HBRUSH, HDC, PAINTSTRUCT, BeginPaint, EndPaint, InvalidateRect,
    CreateRoundRectRgn, SetWindowRgn,
};
use windows::Win32::UI::HiDpi::GetDpiForWindow;
use windows::core::PCWSTR;
use crate::settings::Settings;
use crate::tray::{setup_tray_icon, remove_tray_icon, show_tray_menu, handle_tray_command, TRAY_MESSAGE};

/// Notch states matching the original Nusa
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NotchState {
    #[default]
    Pet,
    Idle,
    Music,
    Lyrics,
    Pomo,
    Water,
    Shelf,
    Todo,
    Bt,
    Charge,
}

impl NotchState {
    pub fn is_expanded(&self) -> bool {
        !matches!(self, NotchState::Pet | NotchState::Idle)
    }
}

/// Notch events for inter-component communication
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

const NOTCH_CLOSED_WIDTH: i32 = 160;
const NOTCH_CLOSED_HEIGHT: i32 = 32;
const NOTCH_EXPANDED_WIDTH: i32 = 420;
const NOTCH_EXPANDED_HEIGHT: i32 = 140;

pub struct NotchUI {
    hwnd: HWND,
    factory: ID2D1Factory1,
    render_target: Option<ID2D1HwndRenderTarget>,
    settings: Arc<RwLock<Settings>>,
    event_rx: mpsc::Receiver<NotchEvent>,
    state: NotchState,
    is_expanded: bool,
    dpi_scale: f32,
    monitor_rect: RECT,
    cur_width: i32,
    cur_height: i32,
    
    // Media & Timers
    media_info: Option<crate::media::MediaInfo>,
    timer_info: Option<crate::timers::TimerInfo>,
    
    // Mascot animation
    eye_blink: f32,
    eye_look: f32,
    anim_tick: u32,
}

impl NotchUI {
    pub fn new(
        hwnd: HWND,
        factory: ID2D1Factory1,
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

        Ok(Self {
            hwnd,
            factory,
            render_target: None,
            settings,
            event_rx,
            state: NotchState::Pet,
            is_expanded: false,
            dpi_scale,
            monitor_rect,
            cur_width: NOTCH_CLOSED_WIDTH,
            cur_height: NOTCH_CLOSED_HEIGHT,
            media_info: None,
            timer_info: None,
            eye_blink: 1.0,
            eye_look: 0.0,
            anim_tick: 0,
        })
    }

    pub fn recreate_render_target(&mut self) -> Result<()> {
        let mut rect = RECT::default();
        unsafe { GetClientRect(self.hwnd, &mut rect)?; }
        let width = (rect.right - rect.left).max(1);
        let height = (rect.bottom - rect.top).max(1);

        let props = D2D1_RENDER_TARGET_PROPERTIES {
            r#type: windows::Win32::Graphics::Direct2D::D2D1_RENDER_TARGET_TYPE_DEFAULT,
            pixelFormat: D2D1_PIXEL_FORMAT {
                format: DXGI_FORMAT_B8G8R8A8_UNORM,
                alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED,
            },
            dpiX: 96.0 * self.dpi_scale,
            dpiY: 96.0 * self.dpi_scale,
            usage: D2D1_RENDER_TARGET_USAGE_NONE,
            minLevel: D2D1_FEATURE_LEVEL_DEFAULT,
        };

        let hwnd_props = D2D1_HWND_RENDER_TARGET_PROPERTIES {
            hwnd: self.hwnd,
            pixelSize: D2D_SIZE_U { width: width as u32, height: height as u32 },
            presentOptions: windows::Win32::Graphics::Direct2D::D2D1_PRESENT_OPTIONS_NONE,
        };

        let rt = unsafe { self.factory.CreateHwndRenderTarget(&props, &hwnd_props)? };
        self.render_target = Some(rt);
        Ok(())
    }

    pub fn update_layout(&mut self, expand: bool) -> Result<()> {
        self.is_expanded = expand;
        let (w, h) = if expand {
            (NOTCH_EXPANDED_WIDTH, NOTCH_EXPANDED_HEIGHT)
        } else {
            let s = self.settings.read().unwrap();
            if s.minimal_notch {
                (NOTCH_CLOSED_WIDTH / 2, 4)
            } else {
                (NOTCH_CLOSED_WIDTH, NOTCH_CLOSED_HEIGHT)
            }
        };

        self.cur_width = w;
        self.cur_height = h;

        let phys_w = (w as f32 * self.dpi_scale).round() as i32;
        let phys_h = (h as f32 * self.dpi_scale).round() as i32;
        let screen_w = self.monitor_rect.right - self.monitor_rect.left;
        let x = self.monitor_rect.left + (screen_w - phys_w) / 2;
        let y = self.monitor_rect.top;

        unsafe {
            let _ = SetWindowPos(
                self.hwnd,
                HWND_TOPMOST,
                x,
                y,
                phys_w,
                phys_h,
                SWP_NOACTIVATE | SWP_NOOWNERZORDER,
            );

            // Shape round bottom corners
            let radius = (16.0 * self.dpi_scale).round() as i32;
            let rgn = CreateRoundRectRgn(0, -radius, phys_w, phys_h, radius * 2, radius * 2);
            SetWindowRgn(self.hwnd, rgn, true);
        }

        self.recreate_render_target()?;
        Ok(())
    }

    pub fn handle_timer(&mut self) -> Result<()> {
        // 1. Drain background events
        while let Ok(event) = self.event_rx.try_recv() {
            match event {
                NotchEvent::SetState(s) => self.state = s,
                NotchEvent::UpdateMedia(info) => {
                    let playing = info.playing;
                    self.media_info = Some(info);
                    if playing && !self.is_expanded {
                        self.state = NotchState::Music;
                    }
                }
                NotchEvent::UpdateTimer(info) => self.timer_info = Some(info),
                _ => {}
            }
        }

        // 2. Animate mascot
        self.anim_tick += 1;
        // Blink every ~120 frames (4 seconds)
        let blink_phase = self.anim_tick % 120;
        if blink_phase < 8 {
            self.eye_blink = (blink_phase as f32 / 8.0) * 0.1; // closed
        } else {
            self.eye_blink = 1.0;
        }

        // Look around every ~180 frames (6 seconds)
        let look_phase = (self.anim_tick / 180) % 3;
        self.eye_look = match look_phase {
            1 => -0.5, // look left
            2 => 0.5,  // look right
            _ => 0.0,  // center
        };

        // 3. Check hover expansion
        unsafe {
            let mut pt = POINT::default();
            GetCursorPos(&mut pt)?;
            let mut rect = RECT::default();
            windows::Win32::UI::WindowsAndMessaging::GetWindowRect(self.hwnd, &mut rect)?;
            
            // Inflate detect zone a bit
            let hover = pt.x >= rect.left && pt.x <= rect.right && pt.y >= rect.top - 5 && pt.y <= rect.bottom + 10;
            if hover && !self.is_expanded {
                self.update_layout(true)?;
            } else if !hover && self.is_expanded {
                self.update_layout(false)?;
            }

            let _ = InvalidateRect(self.hwnd, None, false);
        }

        Ok(())
    }

    pub fn render(&mut self) -> Result<()> {
        let Some(rt) = &self.render_target else { return Ok(()); };

        let mut rect = RECT::default();
        unsafe { GetClientRect(self.hwnd, &mut rect)?; }
        let w = (rect.right - rect.left) as f32;
        let h = (rect.bottom - rect.top) as f32;

        unsafe {
            rt.BeginDraw();
            rt.Clear(Some(&D2D1_COLOR_F { r: 0.07, g: 0.07, b: 0.07, a: 1.0 }));

            // Background notch
            let radius = 16.0 * self.dpi_scale;
            let bg_rect = D2D1_ROUNDED_RECT {
                rect: D2D_RECT_F { left: 0.0, top: 0.0, right: w, bottom: h },
                radiusX: radius,
                radiusY: radius,
            };
            let bg_brush = rt.CreateSolidColorBrush(&D2D1_COLOR_F { r: 0.05, g: 0.05, b: 0.05, a: 1.0 }, None)?;
            rt.FillRoundedRectangle(&bg_rect, &bg_brush);

            if !self.is_expanded {
                // Draw cute mascot eyes in center
                self.draw_eyes(rt, w / 2.0, h / 2.0)?;
            } else {
                // Draw expanded music / info view
                self.draw_expanded(rt, w, h)?;
            }

            let _ = rt.EndDraw(None, None);
        }

        Ok(())
    }

    fn draw_eyes(&self, rt: &ID2D1HwndRenderTarget, cx: f32, cy: f32) -> Result<()> {
        let scale = self.dpi_scale;
        let eye_r = 3.5 * scale;
        let eye_spacing = 9.0 * scale;

        unsafe {
            let white_brush = rt.CreateSolidColorBrush(&D2D1_COLOR_F { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }, None)?;
            let black_brush = rt.CreateSolidColorBrush(&D2D1_COLOR_F { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }, None)?;

            let left_center = D2D_POINT_2F { x: cx - eye_spacing, y: cy };
            let right_center = D2D_POINT_2F { x: cx + eye_spacing, y: cy };

            let l_eye = D2D1_ELLIPSE { point: left_center, radiusX: eye_r, radiusY: eye_r * self.eye_blink };
            let r_eye = D2D1_ELLIPSE { point: right_center, radiusX: eye_r, radiusY: eye_r * self.eye_blink };

            rt.FillEllipse(&l_eye, &white_brush);
            rt.FillEllipse(&r_eye, &white_brush);

            // Pupils with looking direction
            let pupil_r = eye_r * 0.45;
            let look_offset = self.eye_look * (eye_r * 0.35);
            let l_pupil = D2D1_ELLIPSE {
                point: D2D_POINT_2F { x: left_center.x + look_offset, y: left_center.y },
                radiusX: pupil_r,
                radiusY: pupil_r * self.eye_blink,
            };
            let r_pupil = D2D1_ELLIPSE {
                point: D2D_POINT_2F { x: right_center.x + look_offset, y: right_center.y },
                radiusX: pupil_r,
                radiusY: pupil_r * self.eye_blink,
            };

            rt.FillEllipse(&l_pupil, &black_brush);
            rt.FillEllipse(&r_pupil, &black_brush);
        }

        Ok(())
    }

    fn draw_expanded(&self, rt: &ID2D1HwndRenderTarget, w: f32, h: f32) -> Result<()> {
        let scale = self.dpi_scale;
        unsafe {
            // Draw header bar
            let header_brush = rt.CreateSolidColorBrush(&D2D1_COLOR_F { r: 0.2, g: 0.2, b: 0.2, a: 1.0 }, None)?;
            let text_brush = rt.CreateSolidColorBrush(&D2D1_COLOR_F { r: 0.9, g: 0.9, b: 0.9, a: 1.0 }, None)?;

            // Small album art square on left
            let art_rect = D2D1_ROUNDED_RECT {
                rect: D2D_RECT_F { left: 20.0 * scale, top: 25.0 * scale, right: 80.0 * scale, bottom: 85.0 * scale },
                radiusX: 6.0 * scale,
                radiusY: 6.0 * scale,
            };
            rt.FillRoundedRectangle(&art_rect, &header_brush);

            // Mascot eyes on right
            self.draw_eyes(rt, w - (40.0 * scale), 30.0 * scale)?;

            // Media progress bar at bottom
            let bar_rect = D2D1_ROUNDED_RECT {
                rect: D2D_RECT_F { left: 20.0 * scale, top: h - (25.0 * scale), right: w - (20.0 * scale), bottom: h - (21.0 * scale) },
                radiusX: 2.0 * scale,
                radiusY: 2.0 * scale,
            };
            rt.FillRoundedRectangle(&bar_rect, &header_brush);

            // Progress fill (35%)
            let fill_rect = D2D1_ROUNDED_RECT {
                rect: D2D_RECT_F { left: 20.0 * scale, top: h - (25.0 * scale), right: (20.0 + (w - 40.0) * 0.35) * scale, bottom: h - (21.0 * scale) },
                radiusX: 2.0 * scale,
                radiusY: 2.0 * scale,
            };
            let accent_brush = rt.CreateSolidColorBrush(&D2D1_COLOR_F { r: 0.2, g: 0.8, b: 0.5, a: 1.0 }, None)?;
            rt.FillRoundedRectangle(&fill_rect, &accent_brush);
        }

        Ok(())
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
        hCursor: HCURSOR::default(),
        hbrBackground: HBRUSH::default(),
        lpszMenuName: PCWSTR::null(),
        lpszClassName: PCWSTR(class_name.as_ptr()),
        hIconSm: HICON::default(),
    };

    unsafe {
        RegisterClassExW(&wc);

        let hwnd = CreateWindowExW(
            WS_EX_TOPMOST | WS_EX_TOOLWINDOW,
            PCWSTR(class_name.as_ptr()),
            PCWSTR(to_wide_string("NusaNotch\0").as_ptr()),
            WS_POPUP,
            0,
            0,
            NOTCH_CLOSED_WIDTH,
            NOTCH_CLOSED_HEIGHT,
            HWND::default(),
            HMENU::default(),
            hinstance,
            None,
        )?;

        let factory: ID2D1Factory1 = D2D1CreateFactory(D2D1_FACTORY_TYPE_SINGLE_THREADED, None)?;

        let mut ui = Box::new(NotchUI::new(hwnd, factory, settings.clone(), event_rx)?);
        ui.update_layout(false)?;

        let raw_ptr = Box::into_raw(ui);
        SetWindowLongPtrW(hwnd, GWLP_USERDATA, raw_ptr as isize);

        // Setup System Tray icon bound directly to this window
        let nid = setup_tray_icon(hwnd)?;

        // Set 30 FPS timer
        SetTimer(hwnd, 1, 33, None);

        // Show window immediately
        let _ = ShowWindow(hwnd, SW_SHOW);

        // Standard Win32 Message Loop
        let mut msg = MSG::default();
        while GetMessageW(&mut msg, HWND::default(), 0, 0).as_bool() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

        // Cleanup
        remove_tray_icon(&nid);
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
    let ptr = unsafe { GetWindowLongPtrW(hwnd, GWLP_USERDATA) } as *mut NotchUI;

    if !ptr.is_null() {
        let ui = unsafe { &mut *ptr };

        match msg {
            WM_TIMER => {
                let _ = ui.handle_timer();
                return LRESULT(0);
            }
            WM_PAINT => {
                let mut ps = PAINTSTRUCT::default();
                unsafe {
                    BeginPaint(hwnd, &mut ps);
                    let _ = ui.render();
                    EndPaint(hwnd, &ps);
                }
                return LRESULT(0);
            }
            TRAY_MESSAGE => {
                let event = lparam.0 as u32;
                if event == WM_RBUTTONUP {
                    if let Ok(cmd) = show_tray_menu(hwnd, &ui.settings) {
                        if handle_tray_command(cmd, hwnd, &ui.settings).unwrap_or(false) {
                            unsafe {
                                KillTimer(hwnd, 1);
                                let _ = DestroyWindow(hwnd);
                            }
                        }
                    }
                } else if event == WM_LBUTTONUP {
                    let _ = ui.update_layout(!ui.is_expanded);
                }
                return LRESULT(0);
            }
            WM_DESTROY => {
                unsafe {
                    KillTimer(hwnd, 1);
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
