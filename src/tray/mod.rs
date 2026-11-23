//! System tray implementation for Nusa Notch Clone

use anyhow::Result;
use std::sync::{Arc, RwLock};
use windows::Win32::Foundation::{HINSTANCE, HWND};
use windows::Win32::UI::Shell::{
    NIM_ADD, NIM_DELETE, NIF_ICON, NIF_MESSAGE, NIF_TIP, NOTIFYICONDATAW,
    Shell_NotifyIconW,
};
use windows::Win32::UI::WindowsAndMessaging::{
    CreatePopupMenu, AppendMenuW, TrackPopupMenuEx,
    GetCursorPos, SetForegroundWindow, GetSystemMetrics,
    TPM_RIGHTBUTTON, TPM_BOTTOMALIGN, TPM_LEFTALIGN, TPM_RETURNCMD,
    WM_USER,
    MF_STRING, MF_SEPARATOR, MF_CHECKED, MF_UNCHECKED,
    DestroyMenu, HMENU, HICON,
    MessageBoxW, MB_OK, MB_ICONINFORMATION,
};
use windows::Win32::System::Registry::{
    RegOpenKeyExW, RegSetValueExW, RegDeleteValueW, RegCloseKey, RegGetValueW,
    RRF_RT_REG_SZ, HKEY_CURRENT_USER,
    KEY_SET_VALUE, REG_SZ, HKEY,
};
use windows::core::PCWSTR;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use crate::settings::Settings;

pub const WM_TRAYICON: u32 = WM_USER + 1;
pub const TRAY_MESSAGE: u32 = WM_USER + 1;
pub const TRAY_ID: u32 = 1;

pub const ID_START_WITH_WINDOWS: u32 = 1001;
pub const ID_SHOW_MASCOT: u32 = 1002;
pub const ID_MINIMAL_NOTCH: u32 = 1003;
pub const ID_HOTKEY_OVERLAY: u32 = 1006;
pub const ID_CLIPBOARD_HISTORY: u32 = 1007;
pub const ID_OCR_CAPTURE: u32 = 1008;
pub const ID_SHOW_CLIPS: u32 = 1009;
pub const ID_ABOUT: u32 = 1004;
pub const ID_EXIT: u32 = 1005;

pub fn create_tray_icon(hwnd: HWND) -> Result<NOTIFYICONDATAW> {
    let icon = crate::icon::get_app_icon(true);
    
    let mut nid = NOTIFYICONDATAW::default();
    nid.cbSize = std::mem::size_of::<NOTIFYICONDATAW>() as u32;
    nid.hWnd = hwnd;
    nid.uID = TRAY_ID;
    nid.uFlags = NIF_MESSAGE | NIF_ICON | NIF_TIP;
    nid.uCallbackMessage = TRAY_MESSAGE;
    nid.hIcon = icon;
    
    let tip = to_wide_string("Namic Notch\0");
    let tip_len = tip.len().min(127);
    nid.szTip[..tip_len].copy_from_slice(&tip[..tip_len]);
    
    unsafe {
        let res = Shell_NotifyIconW(NIM_ADD, &nid);
        if res.0 == 0 {
            return Err(anyhow::anyhow!("Failed to add tray icon"));
        }
    }
    
    Ok(nid)
}

pub fn remove_tray_icon(nid: &NOTIFYICONDATAW) {
    unsafe {
        let _ = Shell_NotifyIconW(NIM_DELETE, nid);
    }
}

pub fn show_tray_menu(hwnd: HWND, settings: &Arc<RwLock<Settings>>) -> Result<u32> {
    unsafe {
        let mut pt = windows::Win32::Foundation::POINT::default();
        GetCursorPos(&mut pt)?;
        
        let menu = create_context_menu(settings)?;
        
        let _ = SetForegroundWindow(hwnd);
        
        let flags = TPM_RIGHTBUTTON.0 | TPM_BOTTOMALIGN.0 | TPM_LEFTALIGN.0 | TPM_RETURNCMD.0;
        let cmd = TrackPopupMenuEx(menu, flags, pt.x, pt.y, hwnd, None);
        
        let _ = DestroyMenu(menu);
        Ok(cmd.0 as u32)
    }
}

pub fn handle_tray_command(cmd: u32, hwnd: HWND, settings: &Arc<RwLock<Settings>>) -> Result<bool> {
    match cmd {
        ID_START_WITH_WINDOWS => {
            let mut s = settings.write().unwrap();
            let enabled = is_start_with_windows_enabled()?;
            if enabled {
                disable_start_with_windows()?;
            } else {
                enable_start_with_windows()?;
            }
            s.start_with_windows = !enabled;
            s.save()?;
            Ok(false)
        }
        ID_SHOW_MASCOT => {
            let mut s = settings.write().unwrap();
            s.show_mascot = !s.show_mascot;
            let _ = s.save();
            Ok(false)
        }
        ID_MINIMAL_NOTCH => {
            let mut s = settings.write().unwrap();
            s.minimal_notch = !s.minimal_notch;
            let _ = s.save();
            Ok(false)
        }
        ID_HOTKEY_OVERLAY => {
            let mut s = settings.write().unwrap();
            s.enable_hotkey_overlay = !s.enable_hotkey_overlay;
            let _ = s.save();
            Ok(false)
        }
        ID_CLIPBOARD_HISTORY => {
            let mut s = settings.write().unwrap();
            s.enable_clipboard_history = !s.enable_clipboard_history;
            let _ = s.save();
            Ok(false)
        }
        ID_OCR_CAPTURE => {
            unsafe {
                let _ = windows::Win32::UI::WindowsAndMessaging::PostMessageW(
                    hwnd,
                    windows::Win32::UI::WindowsAndMessaging::WM_HOTKEY,
                    windows::Win32::Foundation::WPARAM(crate::notch::HOTKEY_ID_OCR as usize),
                    windows::Win32::Foundation::LPARAM(0),
                );
            }
            Ok(false)
        }
        ID_SHOW_CLIPS => {
            unsafe {
                let _ = windows::Win32::UI::WindowsAndMessaging::PostMessageW(
                    hwnd,
                    windows::Win32::UI::WindowsAndMessaging::WM_HOTKEY,
                    windows::Win32::Foundation::WPARAM(crate::notch::HOTKEY_ID_CLIPS as usize),
                    windows::Win32::Foundation::LPARAM(0),
                );
            }
            Ok(false)
        }
        ID_ABOUT => {
            unsafe {
                let text = to_wide_string("Namic Notch v1.0.0\nA living dynamic notch for Windows\nNative Rust + Direct2D & DirectWrite.\0");
                let title = to_wide_string("About Namic Notch\0");
                MessageBoxW(
                    hwnd,
                    PCWSTR(text.as_ptr()),
                    PCWSTR(title.as_ptr()),
                    MB_OK | MB_ICONINFORMATION,
                );
            }
            Ok(false)
        }
        ID_EXIT => {
            Ok(true) // Signal exit
        }
        _ => Ok(false),
    }
}

fn load_tray_icon_fallback() -> Result<HICON> {
    unsafe {
        let hinstance = windows::Win32::System::LibraryLoader::GetModuleHandleW(None)?;
        
        // Coba load icon IDI_ICON1 (1) yang di-embed dari resource .rc
        if let Ok(icon) = windows::Win32::UI::WindowsAndMessaging::LoadImageW(
            hinstance,
            PCWSTR(1 as *const u16), // IDI_ICON1 = 1
            windows::Win32::UI::WindowsAndMessaging::IMAGE_ICON,
            GetSystemMetrics(windows::Win32::UI::WindowsAndMessaging::SM_CXSMICON),
            GetSystemMetrics(windows::Win32::UI::WindowsAndMessaging::SM_CYSMICON),
            windows::Win32::UI::WindowsAndMessaging::LR_DEFAULTCOLOR,
        ) {
            let hicon = HICON(icon.0);
            if !hicon.is_invalid() {
                return Ok(hicon);
            }
        }

        // Coba load dari file resources/icon.ico atau icon.ico di folder saat ini jika ada
        let icon_paths = ["resources\\icon.ico\0", "icon.ico\0"];
        for p in icon_paths {
            let wide = to_wide_string(p);
            if let Ok(icon) = windows::Win32::UI::WindowsAndMessaging::LoadImageW(
                HINSTANCE::default(),
                PCWSTR(wide.as_ptr()),
                windows::Win32::UI::WindowsAndMessaging::IMAGE_ICON,
                GetSystemMetrics(windows::Win32::UI::WindowsAndMessaging::SM_CXSMICON),
                GetSystemMetrics(windows::Win32::UI::WindowsAndMessaging::SM_CYSMICON),
                windows::Win32::UI::WindowsAndMessaging::LR_LOADFROMFILE | windows::Win32::UI::WindowsAndMessaging::LR_DEFAULTCOLOR,
            ) {
                let hicon = HICON(icon.0);
                if !hicon.is_invalid() {
                    return Ok(hicon);
                }
            }
        }

        // Fallback terakhir
        let icon = windows::Win32::UI::WindowsAndMessaging::LoadIconW(
            HINSTANCE::default(),
            windows::Win32::UI::WindowsAndMessaging::IDI_APPLICATION,
        )?;
        Ok(icon)
    }
}

fn create_context_menu(settings: &Arc<RwLock<Settings>>) -> Result<HMENU> {
    let settings = settings.read().unwrap();
    unsafe {
        let menu = CreatePopupMenu()?;
        
        let text = to_wide_string("Start with Windows\0");
        AppendMenuW(
            menu,
            MF_STRING | if is_start_with_windows_enabled()? { MF_CHECKED } else { MF_UNCHECKED },
            ID_START_WITH_WINDOWS as usize,
            PCWSTR(text.as_ptr()),
        )?;
        
        let text = to_wide_string("Asta Mascot\0");
        AppendMenuW(
            menu,
            MF_STRING | if settings.show_mascot { MF_CHECKED } else { MF_UNCHECKED },
            ID_SHOW_MASCOT as usize,
            PCWSTR(text.as_ptr()),
        )?;
        
        let text = to_wide_string("Minimal Notch\0");
        AppendMenuW(
            menu,
            MF_STRING | if settings.minimal_notch { MF_CHECKED } else { MF_UNCHECKED },
            ID_MINIMAL_NOTCH as usize,
            PCWSTR(text.as_ptr()),
        )?;

        let text = to_wide_string("Hotkey Overlay\0");
        AppendMenuW(
            menu,
            MF_STRING | if settings.enable_hotkey_overlay { MF_CHECKED } else { MF_UNCHECKED },
            ID_HOTKEY_OVERLAY as usize,
            PCWSTR(text.as_ptr()),
        )?;

        let text = to_wide_string("Clipboard History\0");
        AppendMenuW(
            menu,
            MF_STRING | if settings.enable_clipboard_history { MF_CHECKED } else { MF_UNCHECKED },
            ID_CLIPBOARD_HISTORY as usize,
            PCWSTR(text.as_ptr()),
        )?;

        AppendMenuW(menu, MF_SEPARATOR, 0, PCWSTR::null())?;

        let text = to_wide_string("Capture Text (Ctrl+Alt+S)\0");
        AppendMenuW(menu, MF_STRING, ID_OCR_CAPTURE as usize, PCWSTR(text.as_ptr()))?;

        let text = to_wide_string("Clipboard & Notes (Ctrl+Alt+V)\0");
        AppendMenuW(menu, MF_STRING, ID_SHOW_CLIPS as usize, PCWSTR(text.as_ptr()))?;

        AppendMenuW(menu, MF_SEPARATOR, 0, PCWSTR::null())?;
        
        let text = to_wide_string("About Namic Notch\0");
        AppendMenuW(menu, MF_STRING, ID_ABOUT as usize, PCWSTR(text.as_ptr()))?;
        
        let text = to_wide_string("Exit\0");
        AppendMenuW(menu, MF_STRING, ID_EXIT as usize, PCWSTR(text.as_ptr()))?;
        
        Ok(menu)
    }
}

fn is_start_with_windows_enabled() -> Result<bool> {
    let key = to_wide_string("Software\\Microsoft\\Windows\\CurrentVersion\\Run\0");
    let name = to_wide_string("NamicNotch\0");
    let mut size = 0;
    let result = unsafe {
        RegGetValueW(HKEY_CURRENT_USER, PCWSTR(key.as_ptr()), PCWSTR(name.as_ptr()),
            RRF_RT_REG_SZ, None, None, Some(&mut size))
    };
    if result == windows::Win32::Foundation::ERROR_FILE_NOT_FOUND {
        return Ok(false);
    }
    result.ok()?;
    Ok(size > 2)
}

fn enable_start_with_windows() -> Result<()> {
    unsafe {
        let path = std::env::current_exe()?;
        let path_str = format!("\"{}\"", path.display());
        let path_wide: Vec<u16> = OsStr::new(&path_str).encode_wide().chain(Some(0)).collect();
        
        let key_name = to_wide_string("Software\\Microsoft\\Windows\\CurrentVersion\\Run\0");
        let mut key = HKEY::default();
        RegOpenKeyExW(
            HKEY_CURRENT_USER,
            PCWSTR(key_name.as_ptr()),
            0,
            KEY_SET_VALUE,
            &mut key,
        ).ok()?;
        
        let value_name = to_wide_string("NamicNotch\0");
        let bytes = std::slice::from_raw_parts(path_wide.as_ptr() as *const u8, path_wide.len() * 2);
        let result = RegSetValueExW(
            key,
            PCWSTR(value_name.as_ptr()),
            0,
            REG_SZ,
            Some(bytes),
        ).ok();
        let _ = RegCloseKey(key);
        result?;
    }
    Ok(())
}

fn disable_start_with_windows() -> Result<()> {
    unsafe {
        let key_name = to_wide_string("Software\\Microsoft\\Windows\\CurrentVersion\\Run\0");
        let mut key = HKEY::default();
        RegOpenKeyExW(
            HKEY_CURRENT_USER,
            PCWSTR(key_name.as_ptr()),
            0,
            KEY_SET_VALUE,
            &mut key,
        ).ok()?;
        
        let value_name = to_wide_string("NamicNotch\0");
        let result = RegDeleteValueW(key, PCWSTR(value_name.as_ptr()));
        let _ = RegCloseKey(key);
        if result != windows::Win32::Foundation::ERROR_FILE_NOT_FOUND {
            result.ok()?;
        }
    }
    Ok(())
}

fn to_wide_string(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().collect()
}
