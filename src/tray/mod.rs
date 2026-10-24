//! System tray implementation for Nusa Notch Clone

use anyhow::Result;
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc;
use tracing::{info, warn};
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM, BOOL};
use windows::Win32::UI::Shell::{
    NIM_ADD, NIM_DELETE, NIF_ICON, NIF_MESSAGE, NIF_TIP, NOTIFYICONDATAW,
    Shell_NotifyIconW,
};
use windows::Win32::UI::WindowsAndMessaging::{
    CreatePopupMenu, AppendMenuW, TrackPopupMenuEx,
    GetCursorPos, TPM_RIGHTBUTTON, TPM_BOTTOMALIGN, TPM_LEFTALIGN, TPM_RETURNCMD,
    WM_USER, WM_LBUTTONUP, WM_RBUTTONUP, WM_MOUSEMOVE,
    MF_STRING, MF_SEPARATOR, MF_CHECKED, MF_UNCHECKED,
    DestroyMenu, PeekMessageW, TranslateMessage, DispatchMessageW, PM_REMOVE,
    RegisterClassExW, CreateWindowExW, DefWindowProcW, WNDCLASSEXW, CS_HREDRAW, CS_VREDRAW,
    WS_OVERLAPPEDWINDOW, CW_USEDEFAULT, HWND_MESSAGE,
    CheckMenuItem, HMENU, HICON, HCURSOR, MSG,
};
use windows::Win32::Graphics::Gdi::HBRUSH;
use windows::Win32::System::Registry::{
    RegOpenKeyExW, RegSetValueExW, RegDeleteValueW, RegCloseKey, HKEY_CURRENT_USER,
    KEY_SET_VALUE, REG_SZ, HKEY,
};
use windows::core::PCWSTR;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use crate::settings::Settings;

/// Custom message for tray icon
const TRAY_MESSAGE: u32 = WM_USER + 1;
const TRAY_ID: u32 = 1;

/// Tray menu item IDs
const ID_START_WITH_WINDOWS: u32 = 1001;
const ID_SHOW_MASCOT: u32 = 1002;
const ID_MINIMAL_NOTCH: u32 = 1003;
const ID_ABOUT: u32 = 1004;
const ID_EXIT: u32 = 1005;

/// Tray events
#[derive(Debug, Clone)]
pub enum TrayEvent {
    ShowNotch,
    HideNotch,
    ToggleStartWithWindows,
    ToggleMascot,
    ToggleMinimalNotch,
    ShowAbout,
    Exit,
}

/// System tray manager
pub struct Tray {
    hinstance: HINSTANCE,
    hwnd: HWND,
    event_tx: mpsc::Sender<TrayEvent>,
    settings: Arc<RwLock<Settings>>,
    nid: NOTIFYICONDATAW,
    menu_handle: HMENU,
}

unsafe impl Send for Tray {}

impl Tray {
    pub fn new(
        hinstance: HINSTANCE,
        event_tx: mpsc::Sender<TrayEvent>,
        settings: Arc<RwLock<Settings>>,
    ) -> Result<Self> {
        // Create a hidden message window for tray callbacks
        let hwnd = create_message_window(hinstance)?;
        
        // Create tray icon
        let icon = create_tray_icon()?;
        
        let mut nid = NOTIFYICONDATAW::default();
        nid.cbSize = std::mem::size_of::<NOTIFYICONDATAW>() as u32;
        nid.hWnd = hwnd;
        nid.uID = TRAY_ID;
        nid.uFlags = NIF_MESSAGE | NIF_ICON | NIF_TIP;
        nid.uCallbackMessage = TRAY_MESSAGE;
        nid.hIcon = icon;
        
        let tip = to_wide_string("Nusa Notch Clone\0");
        nid.szTip[..tip.len().min(127)].copy_from_slice(&tip[..tip.len().min(127)]);
        
        // Add tray icon
        unsafe {
            let res = Shell_NotifyIconW(NIM_ADD, &nid);
            if res.0 == 0 {
                return Err(anyhow::anyhow!("Failed to add tray icon"));
            }
        }
        
        // Create context menu
        let menu_handle = create_context_menu(&settings)?;
        
        Ok(Self {
            hinstance,
            hwnd,
            event_tx,
            settings,
            nid,
            menu_handle,
        })
    }
    
    pub async fn run(mut self, mut event_rx: mpsc::Receiver<TrayEvent>) -> Result<()> {
        info!("Tray started");
        
        // Message loop
        let mut msg = MSG::default();
        
        loop {
            // Check for async events
            tokio::select! {
                event = event_rx.recv() => {
                    if let Some(event) = event {
                        match event {
                            TrayEvent::Exit => break,
                            TrayEvent::ToggleStartWithWindows => {
                                self.toggle_start_with_windows()?;
                            }
                            TrayEvent::ToggleMascot => {
                                self.toggle_mascot()?;
                            }
                            TrayEvent::ToggleMinimalNotch => {
                                self.toggle_minimal_notch()?;
                            }
                            _ => {}
                        }
                    } else {
                        break; // Channel closed
                    }
                }
                _ = tokio::time::sleep(std::time::Duration::from_millis(16)) => {}
            }
            
            // Process window messages
            while unsafe {
                PeekMessageW(&mut msg, HWND::default(), 0, 0, PM_REMOVE).as_bool()
            } {
                if msg.message == TRAY_MESSAGE {
                    self.handle_tray_message(msg.lParam)?;
                } else {
                    unsafe {
                        TranslateMessage(&msg);
                        DispatchMessageW(&msg);
                    }
                }
            }
        }
        
        // Cleanup
        unsafe {
            let _ = Shell_NotifyIconW(NIM_DELETE, &self.nid);
            DestroyMenu(self.menu_handle);
        }
        
        info!("Tray stopped");
        Ok(())
    }
    
    fn handle_tray_message(&self, lparam: LPARAM) -> Result<()> {
        let message = lparam.0 as u32;
        
        match message {
            WM_RBUTTONUP | WM_LBUTTONUP => {
                self.show_context_menu()?;
            }
            WM_MOUSEMOVE => {
                // Could show tooltip
            }
            _ => {}
        }
        Ok(())
    }
    
    fn show_context_menu(&self) -> Result<()> {
        unsafe {
            let mut pt = windows::Win32::Foundation::POINT::default();
            GetCursorPos(&mut pt)?;
            
            // Update menu item states before showing
            self.update_menu_states()?;
            
            let flags = TPM_RIGHTBUTTON.0 | TPM_BOTTOMALIGN.0 | TPM_LEFTALIGN.0 | TPM_RETURNCMD.0;
            let cmd = TrackPopupMenuEx(
                self.menu_handle,
                flags,
                pt.x,
                pt.y,
                self.hwnd,
                None,
            );
            
            if cmd.0 != 0 {
                self.handle_menu_command(cmd.0 as u32)?;
            }
        }
        Ok(())
    }
    
    fn update_menu_states(&self) -> Result<()> {
        unsafe {
            let settings = self.settings.read().unwrap();
            
            // Update Start with Windows
            let state = if settings.start_with_windows { MF_CHECKED } else { MF_UNCHECKED };
            CheckMenuItem(self.menu_handle, ID_START_WITH_WINDOWS, state.0 as u32);
            
            // Update Show Mascot
            let state = if settings.show_mascot { MF_CHECKED } else { MF_UNCHECKED };
            CheckMenuItem(self.menu_handle, ID_SHOW_MASCOT, state.0 as u32);
            
            // Update Minimal Notch
            let state = if settings.minimal_notch { MF_CHECKED } else { MF_UNCHECKED };
            CheckMenuItem(self.menu_handle, ID_MINIMAL_NOTCH, state.0 as u32);
        }
        Ok(())
    }
    
    fn handle_menu_command(&self, cmd: u32) -> Result<()> {
        match cmd {
            ID_START_WITH_WINDOWS => {
                let _ = self.event_tx.try_send(TrayEvent::ToggleStartWithWindows);
            }
            ID_SHOW_MASCOT => {
                let _ = self.event_tx.try_send(TrayEvent::ToggleMascot);
            }
            ID_MINIMAL_NOTCH => {
                let _ = self.event_tx.try_send(TrayEvent::ToggleMinimalNotch);
            }
            ID_ABOUT => {
                let _ = self.event_tx.try_send(TrayEvent::ShowAbout);
            }
            ID_EXIT => {
                let _ = self.event_tx.try_send(TrayEvent::Exit);
            }
            _ => {}
        }
        Ok(())
    }
    
    fn toggle_start_with_windows(&self) -> Result<()> {
        let mut settings = self.settings.write().unwrap();
        settings.start_with_windows = !settings.start_with_windows;
        
        if settings.start_with_windows {
            self.enable_start_with_windows()?;
        } else {
            self.disable_start_with_windows()?;
        }
        
        settings.save()?;
        Ok(())
    }
    
    fn enable_start_with_windows(&self) -> Result<()> {
        unsafe {
            let path = std::env::current_exe()?;
            let path_str = path.to_string_lossy().to_string();
            let path_wide: Vec<u16> = OsStr::new(&path_str).encode_wide().chain(Some(0)).collect();
            
            let key_name = to_wide_string(r"Software\Microsoft\Windows\CurrentVersion\Run\0");
            let mut key = HKEY::default();
            RegOpenKeyExW(
                HKEY_CURRENT_USER,
                PCWSTR(key_name.as_ptr()),
                0,
                KEY_SET_VALUE,
                &mut key,
            ).ok()?;
            
            let value_name = to_wide_string("NusaNotchClone\0");
            let bytes = std::slice::from_raw_parts(path_wide.as_ptr() as *const u8, path_wide.len() * 2);
            RegSetValueExW(
                key,
                PCWSTR(value_name.as_ptr()),
                0,
                REG_SZ,
                Some(bytes),
            ).ok()?;
            
            RegCloseKey(key).ok()?;
        }
        Ok(())
    }
    
    fn disable_start_with_windows(&self) -> Result<()> {
        unsafe {
            let key_name = to_wide_string(r"Software\Microsoft\Windows\CurrentVersion\Run\0");
            let mut key = HKEY::default();
            RegOpenKeyExW(
                HKEY_CURRENT_USER,
                PCWSTR(key_name.as_ptr()),
                0,
                KEY_SET_VALUE,
                &mut key,
            ).ok()?;
            
            let value_name = to_wide_string("NusaNotchClone\0");
            RegDeleteValueW(key, PCWSTR(value_name.as_ptr())).ok()?;
            
            RegCloseKey(key).ok()?;
        }
        Ok(())
    }
    
    fn toggle_mascot(&self) -> Result<()> {
        let mut settings = self.settings.write().unwrap();
        settings.show_mascot = !settings.show_mascot;
        settings.save()?;
        Ok(())
    }
    
    fn toggle_minimal_notch(&self) -> Result<()> {
        let mut settings = self.settings.write().unwrap();
        settings.minimal_notch = !settings.minimal_notch;
        settings.save()?;
        Ok(())
    }
}

fn create_message_window(hinstance: HINSTANCE) -> Result<HWND> {
    let class_name = to_wide_string("NusaNotchCloneTrayWindow\0");
    
    let wc = WNDCLASSEXW {
        cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(wnd_proc),
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
            windows::Win32::UI::WindowsAndMessaging::WINDOW_EX_STYLE::default(),
            PCWSTR(class_name.as_ptr()),
            PCWSTR(to_wide_string("NusaNotchCloneTray\0").as_ptr()),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT,
            HWND_MESSAGE,
            HMENU::default(),
            hinstance,
            None,
        )?;
        
        Ok(hwnd)
    }
}

extern "system" fn wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
}

fn create_tray_icon() -> Result<HICON> {
    Ok(HICON::default())
}

fn create_context_menu(settings: &Arc<RwLock<Settings>>) -> Result<HMENU> {
    let settings = settings.read().unwrap();
    unsafe {
        let menu = CreatePopupMenu()?;
        
        // Start with Windows
        let text = to_wide_string("Start with Windows\0");
        AppendMenuW(menu, MF_STRING | if settings.start_with_windows { MF_CHECKED } else { MF_UNCHECKED }, ID_START_WITH_WINDOWS as usize, PCWSTR(text.as_ptr()))?;
        
        // Show Mascot
        let text = to_wide_string("Nusa Mascot\0");
        AppendMenuW(menu, MF_STRING | if settings.show_mascot { MF_CHECKED } else { MF_UNCHECKED }, ID_SHOW_MASCOT as usize, PCWSTR(text.as_ptr()))?;
        
        // Minimal Notch
        let text = to_wide_string("Minimal Notch\0");
        AppendMenuW(menu, MF_STRING | if settings.minimal_notch { MF_CHECKED } else { MF_UNCHECKED }, ID_MINIMAL_NOTCH as usize, PCWSTR(text.as_ptr()))?;
        
        // Separator
        AppendMenuW(menu, MF_SEPARATOR, 0, PCWSTR::null())?;
        
        // About
        let text = to_wide_string("About Nusa Notch Clone\0");
        AppendMenuW(menu, MF_STRING, ID_ABOUT as usize, PCWSTR(text.as_ptr()))?;
        
        // Exit
        let text = to_wide_string("Exit\0");
        AppendMenuW(menu, MF_STRING, ID_EXIT as usize, PCWSTR(text.as_ptr()))?;
        
        Ok(menu)
    }
}

fn to_wide_string(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().collect()
}