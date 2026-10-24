//! Notch window and rendering for Nusa Notch Clone

use anyhow::Result;
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc;
use tracing::{info, warn};
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, POINT, RECT, WPARAM, BOOL};
use windows::Win32::Graphics::Direct2D::{
    D2D1_FACTORY_TYPE_SINGLE_THREADED, D2D1CreateFactory, ID2D1Factory1, ID2D1HwndRenderTarget,
    ID2D1RenderTarget, D2D1_RENDER_TARGET_PROPERTIES, D2D1_HWND_RENDER_TARGET_PROPERTIES,
    D2D1_RENDER_TARGET_USAGE_NONE, D2D1_FEATURE_LEVEL_DEFAULT,
    D2D1_ROUNDED_RECT, D2D1_ELLIPSE,
};
use windows::Win32::Graphics::Direct2D::Common::{
    D2D1_PIXEL_FORMAT, D2D1_ALPHA_MODE_PREMULTIPLIED,
    D2D_SIZE_U, D2D_RECT_F, D2D1_COLOR_F,
};
use windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_B8G8R8A8_UNORM;
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, RegisterClassExW, SetWindowPos, ShowWindow,
    GetClientRect, GetCursorPos,
    WM_PAINT, WM_MOUSEMOVE, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_RBUTTONUP,
    WM_TIMER, WM_DESTROY, WM_SIZE, WM_DPICHANGED,
    WS_EX_LAYERED, WS_EX_TRANSPARENT, WS_EX_TOPMOST, WS_EX_NOACTIVATE,
    WS_POPUP, SW_SHOW, SW_HIDE, HWND_TOPMOST, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE,
    SWP_FRAMECHANGED, CS_HREDRAW, CS_VREDRAW, WNDCLASSEXW,
    PeekMessageW, TranslateMessage, DispatchMessageW, PM_REMOVE,
    MSG, HICON, HCURSOR, HMENU,
};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    TrackMouseEvent, TRACKMOUSEEVENT, TME_LEAVE,
};
use windows::Win32::UI::Controls::MARGINS;
use windows::Win32::Graphics::Dwm::DwmExtendFrameIntoClientArea;
use windows::Win32::Graphics::Gdi::{
    GetMonitorInfoW, MonitorFromWindow, MONITORINFO, MONITOR_DEFAULTTONEAREST,
    ScreenToClient, MonitorFromPoint, MONITOR_DEFAULTTOPRIMARY,
    HBRUSH,
};
use windows::Win32::UI::HiDpi::GetDpiForWindow;
use windows::core::PCWSTR;
use crate::settings::Settings;
use crate::utils::{get_monitor_work_area, get_dpi_scale, set_window_pos_no_activate, center_rect};

/// Custom messages
const NOTCH_MESSAGE_UPDATE: u32 = windows::Win32::UI::WindowsAndMessaging::WM_USER + 100;

/// Notch states matching the original Nusa
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NotchState {
    #[default]
    Pet,      // Resting mascot with eyes
    Idle,     // Closed notch with time/battery
    Music,    // Now playing
    Lyrics,   // Synced lyrics
    Pomo,     // Pomodoro timer
    Water,    // Water tracker
    Shelf,    // File shelf
    Todo,     // To-do list
    Bt,       // Headphones battery
    Charge,   // Charging battery
}

impl NotchState {
    pub fn as_str(&self) -> &'static str {
        match self {
            NotchState::Pet => "pet",
            NotchState::Idle => "idle",
            NotchState::Music => "music",
            NotchState::Lyrics => "lyrics",
            NotchState::Pomo => "pomo",
            NotchState::Water => "water",
            NotchState::Shelf => "shelf",
            NotchState::Todo => "todo",
            NotchState::Bt => "bt",
            NotchState::Charge => "charge",
        }
    }
    
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
    MouseEnter,
    MouseLeave,
    Click,
}

/// Notch window dimensions (in logical pixels)
const NOTCH_CLOSED_WIDTH: i32 = 150;
const NOTCH_CLOSED_HEIGHT: i32 = 30;
const NOTCH_EXPANDED_WIDTH: i32 = 420;
const NOTCH_EXPANDED_HEIGHT: i32 = 140;
const NOTCH_LYRICS_WIDTH: i32 = 520;
const NOTCH_LYRICS_HEIGHT: i32 = 140;
const NOTCH_BT_WIDTH: i32 = 310;
const NOTCH_BT_HEIGHT: i32 = 60;

/// Main notch window
pub struct Notch {
    hinstance: HINSTANCE,
    hwnd: HWND,
    factory: ID2D1Factory1,
    render_target: Option<ID2D1HwndRenderTarget>,
    event_tx: mpsc::Sender<NotchEvent>,
    event_rx: mpsc::Receiver<NotchEvent>,
    settings: Arc<RwLock<Settings>>,
    state: NotchState,
    visible: bool,
    mouse_inside: bool,
    dpi_scale: f32,
    monitor_rect: RECT,
    media_info: Option<crate::media::MediaInfo>,
    lyrics: Vec<String>,
    timer_info: Option<crate::timers::TimerInfo>,
    water_info: Option<crate::timers::WaterInfo>,
    battery_level: u8,
    is_charging: bool,
}

unsafe impl Send for Notch {}

impl Notch {
    pub fn new(
        hinstance: HINSTANCE,
        event_tx: mpsc::Sender<NotchEvent>,
        event_rx: mpsc::Receiver<NotchEvent>,
        settings: Arc<RwLock<Settings>>,
    ) -> Result<Self> {
        let hwnd = create_notch_window(hinstance)?;
        
        // Create Direct2D factory
        let factory: ID2D1Factory1 = unsafe {
            D2D1CreateFactory(D2D1_FACTORY_TYPE_SINGLE_THREADED, None)?
        };
        
        // Get monitor info
        let monitor_rect = get_monitor_work_area(hwnd)?;
        let dpi_scale = get_dpi_scale(hwnd);
        
        // Extend frame into client area for custom rendering
        unsafe {
            let margins = MARGINS {
                cxLeftWidth: -1,
                cxRightWidth: -1,
                cyTopHeight: -1,
                cyBottomHeight: -1,
            };
            DwmExtendFrameIntoClientArea(hwnd, &margins).ok();
        }
        
        Ok(Self {
            hinstance,
            hwnd,
            factory,
            render_target: None,
            event_tx,
            event_rx,
            settings,
            state: NotchState::default(),
            visible: false,
            mouse_inside: false,
            dpi_scale,
            monitor_rect,
            media_info: None,
            lyrics: Vec::new(),
            timer_info: None,
            water_info: None,
            battery_level: 100,
            is_charging: false,
        })
    }
    
    pub async fn run(mut self) -> Result<()> {
        info!("Notch window started");
        
        // Initialize render target
        self.create_render_target()?;
        
        // Position notch at top center
        self.position_notch()?;
        
        // Show window immediately
        self.visible = true;
        unsafe {
            let _ = ShowWindow(self.hwnd, SW_SHOW);
        }
        
        // Message loop
        let mut msg = MSG::default();
        
        loop {
            // Check for async events
            tokio::select! {
                event = self.event_rx.recv() => {
                    if let Some(event) = event {
                        self.handle_event(event)?;
                    } else {
                        break; // Channel closed
                    }
                }
                _ = tokio::time::sleep(std::time::Duration::from_millis(16)) => {}
            }
            
            // Process window messages
            while unsafe {
                PeekMessageW(&mut msg, self.hwnd, 0, 0, PM_REMOVE).as_bool()
            } {
                unsafe {
                    TranslateMessage(&msg);
                    DispatchMessageW(&msg);
                }
            }
            
            // Render if visible
            if self.visible {
                self.render()?;
            }
        }
        
        info!("Notch window stopped");
        Ok(())
    }
    
    fn create_render_target(&mut self) -> Result<()> {
        let mut rect = RECT::default();
        unsafe { GetClientRect(self.hwnd, &mut rect)?; }
        
        let width = rect.right - rect.left;
        let height = rect.bottom - rect.top;
        
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
        
        let rt: ID2D1HwndRenderTarget = unsafe {
            self.factory.CreateHwndRenderTarget(&props, &hwnd_props)?
        };
        
        self.render_target = Some(rt);
        Ok(())
    }
    
    fn position_notch(&mut self) -> Result<()> {
        let (width, height) = {
            let settings = self.settings.read().unwrap();
            match self.state {
                NotchState::Pet | NotchState::Idle => {
                    if settings.minimal_notch {
                        (2, 2) // Minimal: thin line
                    } else {
                        (NOTCH_CLOSED_WIDTH, NOTCH_CLOSED_HEIGHT)
                    }
                }
                NotchState::Lyrics => (NOTCH_LYRICS_WIDTH, NOTCH_LYRICS_HEIGHT),
                NotchState::Bt | NotchState::Charge => (NOTCH_BT_WIDTH, NOTCH_BT_HEIGHT),
                _ => (NOTCH_EXPANDED_WIDTH, NOTCH_EXPANDED_HEIGHT),
            }
        };
        
        let physical_width = (width as f32 * self.dpi_scale).round() as i32;
        let physical_height = (height as f32 * self.dpi_scale).round() as i32;
        
        let x = self.monitor_rect.left + (self.monitor_rect.right - self.monitor_rect.left - physical_width) / 2;
        let y = self.monitor_rect.top;
        
        set_window_pos_no_activate(self.hwnd, x, y, physical_width, physical_height)?;
        
        // Recreate render target with new size
        self.create_render_target()?;
        
        Ok(())
    }
    
    fn handle_event(&mut self, event: NotchEvent) -> Result<()> {
        match event {
            NotchEvent::SetState(new_state) => {
                if self.state != new_state {
                    self.state = new_state;
                    self.position_notch()?;
                }
            }
            NotchEvent::Show => {
                if !self.visible {
                    self.visible = true;
                    unsafe { ShowWindow(self.hwnd, SW_SHOW); }
                }
            }
            NotchEvent::Hide => {
                if self.visible {
                    self.visible = false;
                    unsafe { ShowWindow(self.hwnd, SW_HIDE); }
                }
            }
            NotchEvent::Toggle => {
                if self.visible {
                    self.visible = false;
                    unsafe { ShowWindow(self.hwnd, SW_HIDE); }
                } else {
                    self.visible = true;
                    unsafe { ShowWindow(self.hwnd, SW_SHOW); }
                }
            }
            NotchEvent::UpdateMedia(info) => {
                self.media_info = Some(info);
                if self.state == NotchState::Music || self.state == NotchState::Lyrics {
                    self.position_notch()?;
                }
            }
            NotchEvent::UpdateLyrics(lyrics) => {
                self.lyrics = lyrics;
            }
            NotchEvent::UpdateTimer(info) => {
                self.timer_info = Some(info);
            }
            NotchEvent::UpdateWater(info) => {
                self.water_info = Some(info);
            }
            NotchEvent::UpdateBattery(level) => {
                self.battery_level = level;
            }
            NotchEvent::UpdateCharging(charging) => {
                self.is_charging = charging;
                if charging {
                    self.state = NotchState::Charge;
                    self.position_notch()?;
                }
            }
            NotchEvent::MouseEnter => {
                self.mouse_inside = true;
                if self.state == NotchState::Idle || self.state == NotchState::Pet {
                    self.state = NotchState::Music; // Expand on hover
                    self.position_notch()?;
                }
            }
            NotchEvent::MouseLeave => {
                self.mouse_inside = false;
                if self.state != NotchState::Pet && self.state != NotchState::Idle {
                    // Auto-collapse after delay
                    // For now, just collapse immediately
                    self.state = NotchState::Idle;
                    self.position_notch()?;
                }
            }
            NotchEvent::Click => {
                // Handle click based on state
            }
        }
        Ok(())
    }
    
    fn render(&self) -> Result<()> {
        let Some(rt) = &self.render_target else { return Ok(()); };
        
        unsafe {
            rt.BeginDraw();
            rt.Clear(Some(&D2D1_COLOR_F {
                r: 0.0, g: 0.0, b: 0.0, a: 1.0,
            }));
            
            // Draw notch background
            self.draw_notch_background(rt)?;
            
            // Draw state-specific content
            match self.state {
                NotchState::Pet => self.draw_pet(rt)?,
                NotchState::Idle => self.draw_idle(rt)?,
                NotchState::Music => self.draw_music(rt)?,
                NotchState::Lyrics => self.draw_lyrics(rt)?,
                NotchState::Pomo => self.draw_pomo(rt)?,
                NotchState::Water => self.draw_water(rt)?,
                NotchState::Shelf => self.draw_shelf(rt)?,
                NotchState::Todo => self.draw_todo(rt)?,
                NotchState::Bt => self.draw_bt(rt)?,
                NotchState::Charge => self.draw_charge(rt)?,
            }
            
            rt.EndDraw(None, None)?;
        }
        
        Ok(())
    }
    
    fn draw_notch_background(&self, rt: &ID2D1HwndRenderTarget) -> Result<()> {
        let mut rect = RECT::default();
        unsafe { GetClientRect(self.hwnd, &mut rect)?; }
        
        let width = rect.right as f32;
        let height = rect.bottom as f32;
        let radius = self.settings.read().unwrap().corner_radius as f32 * self.dpi_scale;
        
        let rounded_rect = D2D1_ROUNDED_RECT {
            rect: D2D_RECT_F {
                left: 0.0,
                top: 0.0,
                right: width,
                bottom: height,
            },
            radiusX: radius,
            radiusY: radius,
        };
        
        // Create black brush
        unsafe {
            let brush = rt.CreateSolidColorBrush(
                &D2D1_COLOR_F { r: 0.0, g: 0.0, b: 0.0, a: 1.0 },
                None,
            )?;
            rt.FillRoundedRectangle(&rounded_rect, &brush);
        }
        
        Ok(())
    }
    
    // State-specific drawing methods
    fn draw_pet(&self, rt: &ID2D1HwndRenderTarget) -> Result<()> {
        let mut rect = RECT::default();
        unsafe { GetClientRect(self.hwnd, &mut rect)?; }
        let width = rect.right as f32;
        let height = rect.bottom as f32;
        
        let center_x = width / 2.0;
        let center_y = height / 2.0;
        let eye_radius = 4.0;
        
        let mascot = crate::mascot::Mascot::new(self.settings.clone(), self.event_tx.clone())?;
        mascot.draw_eyes(rt, center_x, center_y, eye_radius, self.dpi_scale)?;
        Ok(())
    }

    fn draw_idle(&self, rt: &ID2D1HwndRenderTarget) -> Result<()> {
        self.draw_pet(rt)
    }

    fn draw_music(&self, rt: &ID2D1HwndRenderTarget) -> Result<()> {
        self.draw_pet(rt)
    }

    fn draw_lyrics(&self, rt: &ID2D1HwndRenderTarget) -> Result<()> { Ok(()) }
    fn draw_pomo(&self, rt: &ID2D1HwndRenderTarget) -> Result<()> { Ok(()) }
    fn draw_water(&self, rt: &ID2D1HwndRenderTarget) -> Result<()> { Ok(()) }
    fn draw_shelf(&self, rt: &ID2D1HwndRenderTarget) -> Result<()> { Ok(()) }
    fn draw_todo(&self, rt: &ID2D1HwndRenderTarget) -> Result<()> { Ok(()) }
    fn draw_bt(&self, rt: &ID2D1HwndRenderTarget) -> Result<()> { Ok(()) }
    fn draw_charge(&self, rt: &ID2D1HwndRenderTarget) -> Result<()> { Ok(()) }
}

fn create_notch_window(hinstance: HINSTANCE) -> Result<HWND> {
    let class_name = to_wide_string("NusaNotchCloneNotchWindow\0");
    
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
            WS_EX_LAYERED | WS_EX_TOPMOST | windows::Win32::UI::WindowsAndMessaging::WS_EX_TOOLWINDOW,
            PCWSTR(class_name.as_ptr()),
            PCWSTR(to_wide_string("NusaNotchCloneNotch\0").as_ptr()),
            WS_POPUP,
            0, 0, NOTCH_CLOSED_WIDTH, NOTCH_CLOSED_HEIGHT,
            HWND::default(),
            HMENU::default(),
            hinstance,
            None,
        )?;
        
        Ok(hwnd)
    }
}

extern "system" fn notch_wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    // Forward to the notch instance via window user data
    // For now, just use default handling
    unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
}

fn to_wide_string(s: &str) -> Vec<u16> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    OsStr::new(s).encode_wide().collect()
}