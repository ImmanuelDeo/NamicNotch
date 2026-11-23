//! Global hooks and hotkeys for Nusa Notch Clone

use anyhow::Result;
use tokio::sync::mpsc;
use tracing::info;
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetKeyState, VK_CAPITAL, VK_MEDIA_NEXT_TRACK, VK_MEDIA_PLAY_PAUSE, VK_MEDIA_PREV_TRACK,
    VK_MEDIA_STOP, VK_VOLUME_DOWN, VK_VOLUME_MUTE, VK_VOLUME_UP,
};
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, DispatchMessageW, GetMessageW, SetWindowsHookExW, TranslateMessage,
    UnhookWindowsHookEx, HHOOK, KBDLLHOOKSTRUCT, WH_KEYBOARD_LL, WM_KEYDOWN, WM_SYSKEYDOWN,
};
use crate::notch::NotchEvent;
use std::sync::atomic::{AtomicBool, Ordering};

/// Set by the UI while the quick-note input is focused. The hook then routes
/// printable keys to the note instead of letting them fall through.
pub static NOTE_INPUT_ACTIVE: AtomicBool = AtomicBool::new(false);

/// Which overlay a media/volume key should surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaKey {
    Volume,
    Media,
}

/// Pure classification of a virtual-key code into the overlay it should trigger.
/// Free of Win32 calls so it can be unit-tested on any host.
pub fn classify_media_key(vk: u32) -> Option<MediaKey> {
    match vk {
        x if x == VK_VOLUME_UP.0 as u32
            || x == VK_VOLUME_DOWN.0 as u32
            || x == VK_VOLUME_MUTE.0 as u32 =>
        {
            Some(MediaKey::Volume)
        }
        x if x == VK_MEDIA_PLAY_PAUSE.0 as u32
            || x == VK_MEDIA_NEXT_TRACK.0 as u32
            || x == VK_MEDIA_PREV_TRACK.0 as u32
            || x == VK_MEDIA_STOP.0 as u32 =>
        {
            Some(MediaKey::Media)
        }
        _ => None,
    }
}

/// Hook configuration
#[derive(Debug, Clone)]
pub struct HookConfig {
    pub enable_media_keys: bool,
    pub enable_volume_keys: bool,
    pub enable_brightness_keys: bool,
}

impl Default for HookConfig {
    fn default() -> Self {
        Self {
            // The overlay peek (volume/media/caps) is the headline dynamic-notch
            // behaviour. It previously never fired because every flag defaulted to
            // false, leaving the WH_KEYBOARD_LL hook uninstalled.
            enable_media_keys: true,
            enable_volume_keys: true,
            enable_brightness_keys: false,
        }
    }
}

/// Read the master volume (0-100) from the default render endpoint.
/// Spawns its own COM apartment; returns None when audio endpoints are absent.
pub fn read_system_volume() -> Option<u8> {
    std::thread::spawn(|| unsafe {
        use windows::Win32::Media::Audio::Endpoints::IAudioEndpointVolume;
        use windows::Win32::Media::Audio::{
            eConsole, eRender, IMMDeviceEnumerator, MMDeviceEnumerator,
        };
        use windows::Win32::System::Com::{
            CoCreateInstance, CoInitializeEx, CoUninitialize, CLSCTX_ALL, COINIT_MULTITHREADED,
        };

        let _ = CoInitializeEx(None, COINIT_MULTITHREADED);
        let mut pct: Option<u8> = None;
        if let Ok(enumerator) =
            CoCreateInstance::<_, IMMDeviceEnumerator>(&MMDeviceEnumerator, None, CLSCTX_ALL)
        {
            if let Ok(device) = enumerator.GetDefaultAudioEndpoint(eRender, eConsole) {
                if let Ok(endpoint) = device.Activate::<IAudioEndpointVolume>(CLSCTX_ALL, None) {
                    if let Ok(scalar) = endpoint.GetMasterVolumeLevelScalar() {
                        pct = Some((scalar.clamp(0.0, 1.0) * 100.0).round() as u8);
                    }
                }
            }
        }
        CoUninitialize();
        pct
    })
    .join()
    .ok()
    .flatten()
}

/// Hook manager for global keyboard/mouse hooks
pub struct HookManager {
    config: HookConfig,
    event_tx: mpsc::Sender<NotchEvent>,
    keyboard_hook: Option<HHOOK>,
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

        // The hook callback is a bare extern fn with no closure state, so the
        // channel is published to the thread that owns the message loop.
        install_event_tx(self.event_tx.clone());

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
        let is_key_up = (kbd.flags.0 & 0x80) != 0;

        let message = wparam.0 as u32;
        let is_key_down = matches!(message, WM_KEYDOWN | WM_SYSKEYDOWN);

        if is_key_down && !is_key_up {
            if NOTE_INPUT_ACTIVE.load(Ordering::Relaxed) {
                // Feed the quick-note editor. VK codes 0x30-0x5A cover 0-9/A-Z;
                // shift state selects the shifted glyph.
                let shifted = unsafe {
                    (GetKeyState(0x10) as u16 & 0x8000) != 0 // VK_SHIFT
                };
                let caps = unsafe { (GetKeyState(VK_CAPITAL.0 as i32) as u16 & 1) != 0 };
                let ch: Option<char> = match vk_code {
                    0x0D => { post(NotchEvent::NoteCommit); None }
                    0x1B => { post(NotchEvent::NoteCancel); None }
                    0x08 => { post(NotchEvent::NoteBackspace); None }
                    0x20 => Some(' '),
                    0x30..=0x39 => {
                        const SHIFTED: &[u8] = b")!@#$%^&*(";
                        let d = (vk_code - 0x30) as usize;
                        Some(if shifted { SHIFTED[d] as char } else { (b'0' + d as u8) as char })
                    }
                    0x41..=0x5A => {
                        let base = b'a' + (vk_code - 0x41) as u8;
                        let upper = shifted ^ caps;
                        Some(if upper { base.to_ascii_uppercase() as char } else { base as char })
                    }
                    0xBC => Some(if shifted { '<' } else { ',' }),
                    0xBE => Some(if shifted { '>' } else { '.' }),
                    0xBD => Some(if shifted { '_' } else { '-' }),
                    0xBF => Some(if shifted { '?' } else { '/' }),
                    0xBA => Some(if shifted { ':' } else { ';' }),
                    0xDE => Some(if shifted { '"' } else { '\'' }),
                    _ => None,
                };
                if let Some(c) = ch {
                    post(NotchEvent::NoteChar(c));
                }
                // Swallow nothing: keys still reach the foreground app too, but
                // the note editor is only active while the notch is hovered.
            }
            if let Some(kind) = classify_media_key(vk_code) {
                // Windows applies the volume change during key-down, so reading the
                // endpoint here yields the level the user just set.
                if kind == MediaKey::Volume {
                    let volume = read_system_volume().unwrap_or(0);
                    post(NotchEvent::HotkeyVolume(volume));
                } else {
                    post(NotchEvent::HotkeyMedia);
                }
            } else if vk_code == VK_CAPITAL.0 as u32 {
                // WH_KEYBOARD_LL runs BEFORE Windows processes the key, so the
                // latch bit still holds the PRE-toggle state. Invert it to get
                // the state the keypress is about to produce.
                let was_on = unsafe { GetKeyState(VK_CAPITAL.0 as i32) as u16 & 1 } != 0;
                post(NotchEvent::HotkeyCapsLock(next_caps_state(was_on)));
            }
        }
    }

    unsafe { CallNextHookEx(None, n_code, wparam, lparam) }
}

fn post(event: NotchEvent) {
    EVENT_TX.with(|cell| {
        if let Some(tx) = cell.borrow().as_ref() {
            let _ = tx.try_send(event);
        }
    });
}

thread_local! {
    static EVENT_TX: std::cell::RefCell<Option<mpsc::Sender<NotchEvent>>> =
        const { std::cell::RefCell::new(None) };
}

fn install_event_tx(tx: mpsc::Sender<NotchEvent>) {
    EVENT_TX.with(|cell| {
        *cell.borrow_mut() = Some(tx);
    });
}

/// CapsLock latch bit read inside WH_KEYBOARD_LL is the PRE-toggle state;
/// the keypress flips it. Pure helper so the inversion is unit-testable.
fn next_caps_state(was_on: bool) -> bool {
    !was_on
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capslock_overlay_shows_post_toggle_state() {
        // Regression: notch used to show OFF when the user turned CapsLock ON.
        assert!(next_caps_state(false), "pressing Caps while OFF must report ON");
        assert!(!next_caps_state(true), "pressing Caps while ON must report OFF");
    }

    #[test]
    fn classifies_volume_keys() {
        assert_eq!(classify_media_key(VK_VOLUME_UP.0 as u32), Some(MediaKey::Volume));
        assert_eq!(classify_media_key(VK_VOLUME_DOWN.0 as u32), Some(MediaKey::Volume));
        assert_eq!(classify_media_key(VK_VOLUME_MUTE.0 as u32), Some(MediaKey::Volume));
    }

    #[test]
    fn classifies_media_keys() {
        assert_eq!(classify_media_key(VK_MEDIA_PLAY_PAUSE.0 as u32), Some(MediaKey::Media));
        assert_eq!(classify_media_key(VK_MEDIA_NEXT_TRACK.0 as u32), Some(MediaKey::Media));
        assert_eq!(classify_media_key(VK_MEDIA_PREV_TRACK.0 as u32), Some(MediaKey::Media));
        assert_eq!(classify_media_key(VK_MEDIA_STOP.0 as u32), Some(MediaKey::Media));
    }

    #[test]
    fn ignores_unrelated_keys() {
        assert_eq!(classify_media_key(0x41), None); // 'A'
        assert_eq!(classify_media_key(VK_CAPITAL.0 as u32), None);
    }

    #[test]
    fn default_config_enables_overlay_hooks() {
        let cfg = HookConfig::default();
        assert!(cfg.enable_media_keys, "media overlay must be on by default");
        assert!(cfg.enable_volume_keys, "volume overlay must be on by default");
    }
}
