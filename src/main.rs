#![windows_subsystem = "windows"]

//! Nusa Notch Clone - A living notch for Windows
//! 
//! This is a reverse-engineered clone of Nusa Notch, built from scratch
//! using native Windows APIs and Rust.

mod notch;
mod tray;
mod media;
mod hooks;
mod shelf;
mod timers;
mod settings;
mod mascot;
mod utils;

use anyhow::Result;
use single_instance::SingleInstance;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{info, error};
use windows::Win32::Foundation::HINSTANCE;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Starting Nusa Notch Clone v0.1.0");

    // Ensure single instance
    let _instance = SingleInstance::new("NusaNotchClone").expect("Another instance is already running");

    // Get module handle
    let hinstance: HINSTANCE = unsafe { GetModuleHandleW(None)? }.into();

    // Create channels for inter-component communication
    let (tray_tx, tray_rx) = mpsc::channel::<tray::TrayEvent>(32);
    let (notch_tx, notch_rx) = mpsc::channel::<notch::NotchEvent>(32);
    let (media_tx, media_rx) = mpsc::channel::<media::MediaEvent>(32);
    let (settings_tx, settings_rx) = mpsc::channel::<settings::SettingsEvent>(32);

    // Initialize settings (wrapped in RwLock for thread-safe access)
    let settings = Arc::new(std::sync::RwLock::new(settings::Settings::load()?));
    info!("Settings loaded from {}", settings::SETTINGS_PATH);

    // Initialize components
    let tray = tray::Tray::new(hinstance, tray_tx.clone(), settings.clone())?;
    let notch = notch::Notch::new(hinstance, notch_tx.clone(), notch_rx, settings.clone())?;
    let media = media::MediaManager::new(media_tx, media_rx, settings.clone())?;
    let hooks = hooks::HookManager::new(hooks::HookConfig::default(), notch_tx.clone())?;
    let shelf = shelf::Shelf::new(settings.clone())?;
    let timers = timers::TimerManager::new(settings.clone(), notch_tx.clone())?;
    let mascot = mascot::Mascot::new(settings.clone(), notch_tx.clone())?;

    // Start background tasks
    let tray_thread = std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed to create tray runtime");
        let _ = rt.block_on(tray.run(tray_rx));
    });

    let notch_thread = std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed to create notch runtime");
        let _ = rt.block_on(notch.run());
    });

    let media_thread = std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed to create media runtime");
        let _ = rt.block_on(media.run());
    });

    let hooks_thread = std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed to create hooks runtime");
        let _ = rt.block_on(hooks.run());
    });

    let shelf_handle = tokio::spawn(shelf.run());
    let timers_handle = tokio::spawn(timers.run());
    let mascot_handle = tokio::spawn(mascot.run());

    // Wait for shutdown signal (from tray exit)
    let _ = tray_thread.join();

    info!("Shutting down...");

    // Clean shutdown
    shelf_handle.abort();
    timers_handle.abort();
    mascot_handle.abort();
    let _ = notch_thread.join();
    let _ = media_thread.join();
    let _ = hooks_thread.join();

    Ok(())
}
