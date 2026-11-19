#![windows_subsystem = "windows"]

//! Namic Notch - A living notch for Windows
//! 
//! Clean native Win32/Direct2D implementation running UI on main thread.

mod notch;
mod tray;
mod media;
mod hooks;
mod shelf;
mod timers;
mod settings;
mod mascot;
mod utils;
pub mod icon;

use anyhow::Result;
use single_instance::SingleInstance;
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc;
use windows::Win32::Foundation::HINSTANCE;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;

fn main() -> Result<()> {
    // 1. Single instance check
    let _instance = SingleInstance::new("NamicNotch_SingleInstance_Mutex").ok();

    // 2. Load settings
    let settings = Arc::new(RwLock::new(settings::Settings::load().unwrap_or_default()));

    // 3. Setup Tokio runtime for background async workers
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    // 4. Inter-thread channels
    let (notch_tx, notch_rx) = mpsc::channel::<notch::NotchEvent>(64);
    let (media_tx, media_rx) = mpsc::channel::<media::MediaEvent>(64);

    // 5. Spawn background workers
    let media = media::MediaManager::new(media_tx, media_rx, settings.clone())?;
    rt.spawn(async move {
        let _ = media.run().await;
    });

    let timers = timers::TimerManager::new(settings.clone(), notch_tx.clone())?;
    rt.spawn(async move {
        let _ = timers.run().await;
    });

    let shelf = shelf::Shelf::new(settings.clone())?;
    rt.spawn(async move {
        let _ = shelf.run().await;
    });

    let overlay_enabled = settings.read().unwrap().enable_hotkey_overlay;
    let hook_config = hooks::HookConfig {
        enable_media_keys: overlay_enabled,
        enable_volume_keys: overlay_enabled,
        ..hooks::HookConfig::default()
    };
    let hooks = hooks::HookManager::new(hook_config, notch_tx.clone())?;
    std::thread::spawn(move || {
        let _ = hooks.run();
    });

    // 6. Run GUI on Main Thread (Win32 requirement)
    let hinstance: HINSTANCE = unsafe { GetModuleHandleW(None)? }.into();
    notch::run_ui(hinstance, notch_rx, settings)?;

    Ok(())
}
