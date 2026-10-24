//! Global hooks and hotkeys for Nusa Notch Clone

use anyhow::Result;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{info, warn};
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    SetWindowsHookExW, UnhookWindowsHookEx, CallNextHookEx, GetMessageW,
    WH_KEYBOARD_LL, WH_MOUSE_LL, WM_KEYDOWN, WM_KEYUP, WM_SYSKEYDOWN, WM_SYSKEYUP,
    KBDLLHOOKSTRUCT, MSLLHOOKSTRUCT,
};
use crate::notch::NotchEvent;

// Media key virtual key codes (not in windows crate)
const VK_VOLUME_MUTE: u32 = 0xAD;
const VK_VOLUME_DOWN: u32 = 0xAE;
const VK_VOLUME_UP: u32 = 0xAF;
const VK_MEDIA_NEXT_TRACK: u32 = 0xB0;
const VK_MEDIA_PREV_TRACK: u32 = 0xB1;
const VK_MEDIA_STOP: u32 = 0xB2;
const VK_MEDIA_PLAY_PAUSE: u32 = 0xB3;

/// Hook configuration
#[derive(Debug, Clone, Default)]
pub struct HookConfig {
    pub enable_media_keys: bool,
    pub enable_volume_keys: bool,
    pub enable_brightness_keys: bool,
}

/// Hook manager for global keyboard/mouse hooks
pub struct HookManager {
    config: HookConfig,
    event_tx: mpsc::Sender<NotchEvent>,
    keyboard_hook: Option<windows::Win32::UI::WindowsAndMessaging::HHOOK>,
    mouse_hook: Option<windows::Win32::UI::WindowsAndMessaging::HHOOK>,
}

unsafe impl Send for HookManager {}

impl HookManager {
    pub fn new(
        config: HookConfig,
        event_tx: mpsc::Sender<NotchEvent>,
    ) -> Result<Self> {
        Ok(Self {
            config,
            event_tx,
            keyboard_hook: None,
            mouse_hook: None,
        })
    }
    
    pub async fn run(mut self) -> Result<()> {
        info!("Hook manager started");
        
        // Install low-level keyboard hook
        if self.config.enable_media_keys || self.config.enable_volume_keys {
            self.install_keyboard_hook()?;
        }
        
        // Install low-level mouse hook (for future use)
        // self.install_mouse_hook()?;
        
        // Message loop to keep hooks alive
        let mut msg = windows::Win32::UI::WindowsAndMessaging::MSG::default();
        
        loop {
            unsafe {
                let result = GetMessageW(&mut msg, HWND::default(), 0, 0);
                if result.0 <= 0 {
                    break;
                }
            }
            
            // Yield to async runtime
            tokio::task::yield_now().await;
        }
        
        // Cleanup hooks
        self.uninstall_hooks()?;
        
        info!("Hook manager stopped");
        Ok(())
    }
    
    fn install_keyboard_hook(&mut self) -> Result<()> {
        use windows::Win32::UI::WindowsAndMessaging::SetWindowsHookExW;
        use std::ffi::c_void;
        
        unsafe {
            let hook = SetWindowsHookExW(
                WH_KEYBOARD_LL,
                Some(keyboard_hook_proc),
                HINSTANCE::default(),
                0,
            )?;
            self.keyboard_hook = Some(hook);
            info!("Keyboard hook installed");
        }
        
        Ok(())
    }
    
    fn install_mouse_hook(&mut self) -> Result<()> {
        use windows::Win32::UI::WindowsAndMessaging::SetWindowsHookExW;
        
        unsafe {
            let hook = SetWindowsHookExW(
                WH_MOUSE_LL,
                Some(mouse_hook_proc),
                HINSTANCE::default(),
                0,
            )?;
            self.mouse_hook = Some(hook);
            info!("Mouse hook installed");
        }
        
        Ok(())
    }
    
    fn uninstall_hooks(&mut self) -> Result<()> {
        use windows::Win32::UI::WindowsAndMessaging::UnhookWindowsHookEx;
        
        if let Some(hook) = self.keyboard_hook.take() {
            unsafe { UnhookWindowsHookEx(hook)?; }
            info!("Keyboard hook uninstalled");
        }
        
        if let Some(hook) = self.mouse_hook.take() {
            unsafe { UnhookWindowsHookEx(hook)?; }
            info!("Mouse hook uninstalled");
        }
        
        Ok(())
    }
}

/// Low-level keyboard hook callback
/// This runs in the context of the thread that installed the hook
extern "system" fn keyboard_hook_proc(
    n_code: i32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if n_code >= 0 {
        let kbd = unsafe { &*(lparam.0 as *const KBDLLHOOKSTRUCT) };
        let vk_code = kbd.vkCode as u32;
        let flags = kbd.flags;
        let is_key_up = (flags.0 & 0x80) != 0; // LLKHF_UP
        
        let message = wparam.0 as u32;
        let is_key_down = matches!(message, WM_KEYDOWN | WM_SYSKEYDOWN);
        
        // Handle media keys on key down
        if is_key_down && !is_key_up {
            match vk_code {
                VK_MEDIA_PLAY_PAUSE => {
                    // Send media play/pause event
                }
                VK_MEDIA_NEXT_TRACK => {
                    // Send next track event
                }
                VK_MEDIA_PREV_TRACK => {
                    // Send previous track event
                }
                VK_MEDIA_STOP => {
                    // Send stop event
                }
                VK_VOLUME_UP => {
                    // Send volume up event
                }
                VK_VOLUME_DOWN => {
                    // Send volume down event
                }
                VK_VOLUME_MUTE => {
                    // Send mute toggle event
                }
                _ => {}
            }
        }
    }
    
    unsafe { CallNextHookEx(None, n_code, wparam, lparam) }
}

/// Low-level mouse hook callback
extern "system" fn mouse_hook_proc(
    n_code: i32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if n_code >= 0 {
        // Could track mouse position for notch hover detection
        // But we use window messages instead
    }
    
    unsafe { CallNextHookEx(None, n_code, wparam, lparam) }
}

/// Register global hotkeys (alternative to hooks)
pub fn register_hotkeys(_hwnd: HWND) -> Result<()> {
    // MOD_NOREPEAT = 0x4000
    // VK_MEDIA keys don't work well with RegisterHotKey
    // Use hooks instead
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hook_config_default() {
        let config = HookConfig::default();
        assert!(!config.enable_media_keys);
        assert!(!config.enable_volume_keys);
    }
}