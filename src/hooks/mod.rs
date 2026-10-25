//! Global hooks and hotkeys for Nusa Notch Clone

use anyhow::Result;
use tokio::sync::mpsc;
use tracing::info;
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    SetWindowsHookExW, UnhookWindowsHookEx, CallNextHookEx, GetMessageW,
    TranslateMessage, DispatchMessageW,
    WH_KEYBOARD_LL, WM_KEYDOWN, WM_SYSKEYDOWN,
    KBDLLHOOKSTRUCT,
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
        })
    }
    
    pub fn run(mut self) -> Result<()> {
        info!("Hook manager started");
        
        if self.config.enable_media_keys || self.config.enable_volume_keys {
            self.install_keyboard_hook()?;
        }
        
        let mut msg = windows::Win32::UI::WindowsAndMessaging::MSG::default();
        
        loop {
            unsafe {
                let result = GetMessageW(&mut msg, HWND::default(), 0, 0);
                if result.0 <= 0 {
                    break;
                }
                let _ = TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
        
        self.uninstall_hooks()?;
        Ok(())
    }
    
    fn install_keyboard_hook(&mut self) -> Result<()> {
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
    
    fn uninstall_hooks(&mut self) -> Result<()> {
        if let Some(hook) = self.keyboard_hook.take() {
            unsafe { UnhookWindowsHookEx(hook)?; }
            info!("Keyboard hook uninstalled");
        }
        
        Ok(())
    }
}

extern "system" fn keyboard_hook_proc(
    n_code: i32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if n_code >= 0 {
        let kbd = unsafe { &*(lparam.0 as *const KBDLLHOOKSTRUCT) };
        let vk_code = kbd.vkCode as u32;
        let flags = kbd.flags;
        let is_key_up = (flags.0 & 0x80) != 0;
        
        let message = wparam.0 as u32;
        let is_key_down = matches!(message, WM_KEYDOWN | WM_SYSKEYDOWN);
        
        if is_key_down && !is_key_up {
            match vk_code {
                VK_MEDIA_PLAY_PAUSE => {}
                VK_MEDIA_NEXT_TRACK => {}
                VK_MEDIA_PREV_TRACK => {}
                VK_MEDIA_STOP => {}
                VK_VOLUME_UP => {}
                VK_VOLUME_DOWN => {}
                VK_VOLUME_MUTE => {}
                _ => {}
            }
        }
    }
    
    unsafe { CallNextHookEx(None, n_code, wparam, lparam) }
}
