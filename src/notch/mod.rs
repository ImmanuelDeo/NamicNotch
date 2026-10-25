// src/notch/mod.rs
//! Notch window and rendering for Nusa Notch Clone (v5.0)

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
                cyTopHeight: -1,
                cyBottomHeight: -1,
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
                NotchEvent::UpdateTimer(info) => {
                    self.controller.pomo_remaining_secs = info.remaining_seconds;
                    self.controller.pomo_total_secs = info.total_seconds;
                    self.controller.pomo_running = info.running;
                    self.controller.pomo_is_break = matches!(info.mode, crate::timers::PomodoroMode::ShortBreak | crate::timers::PomodoroMode::LongBreak);
                }
                NotchEvent::UpdateShelf(items) => {
                    self.controller.shelf_files = items
                        .into_iter()
                        .map(|item| (item.name.clone(), format!("{} · {:?}", item.size, item.item_type), item.path.to_string_lossy().to_string()))
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

        // Bar Navigasi Atas (`<`, 6 Tab: Hub/Music/Control/Focus/Shelf/Stages, dan `>`)
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
                            _ => NotchState::StageDock,
                        };
                        if next == NotchState::StageDock {
                            self.controller.trigger_stagedock();
                        } else {
                            self.controller.transition_to(next);
                        }
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