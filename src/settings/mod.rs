//! Settings persistence for Nusa Notch Clone

use anyhow::Result;
use dirs::data_local_dir;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc;
use tracing::{info, warn};

/// Path to settings file
pub const SETTINGS_PATH: &str = "NusaShelf/settings.txt";

/// Settings events for inter-component communication
#[derive(Debug)]
pub enum SettingsEvent {
    Get(String, tokio::sync::oneshot::Sender<Option<String>>),
    Set(String, String),
    Save,
}

/// Main settings structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    /// General settings
    pub start_with_windows: bool,
    pub show_mascot: bool,
    pub minimal_notch: bool,
    
    /// Notch appearance
    pub notch_width: i32,
    pub notch_height: i32,
    pub corner_radius: i32,
    
    /// Timers
    pub pomodoro_work_minutes: u32,
    pub pomodoro_short_break_minutes: u32,
    pub pomodoro_long_break_minutes: u32,
    pub water_goal_ml: u32,
    pub water_reminder_hours: u32,
    
    /// Media
    pub show_lyrics: bool,
    pub lyrics_provider: String,
    
    /// Shelf
    pub shelf_max_items: usize,
    
    /// Custom key-value storage for extensibility
    #[serde(flatten)]
    pub custom: HashMap<String, String>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            start_with_windows: false,
            show_mascot: true,
            minimal_notch: false,
            notch_width: 156,
            notch_height: 34,
            corner_radius: 15,
            pomodoro_work_minutes: 25,
            pomodoro_short_break_minutes: 5,
            pomodoro_long_break_minutes: 15,
            water_goal_ml: 1500,
            water_reminder_hours: 2,
            show_lyrics: true,
            lyrics_provider: "lrclib".to_string(),
            shelf_max_items: 10,
            custom: HashMap::new(),
        }
    }
}

impl Settings {
    /// Get the settings file path
    pub fn get_path() -> PathBuf {
        data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(SETTINGS_PATH)
    }

    /// Load settings from file
    pub fn load() -> Result<Self> {
        let path = Self::get_path();
        
        if !path.exists() {
            info!("Settings file not found at {}, using defaults", path.display());
            return Ok(Self::default());
        }

        let content = fs::read_to_string(&path)?;
        let settings = Self::parse(&content)?;
        info!("Loaded settings from {}", path.display());
        Ok(settings)
    }

    /// Parse settings from INI-like format
    fn parse(content: &str) -> Result<Self> {
        let mut settings = Self::default();
        
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
                continue;
            }
            
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim();
                settings.set_key(key, value);
            }
        }
        
        Ok(settings)
    }

    /// Set a key-value pair
    fn set_key(&mut self, key: &str, value: &str) {
        match key {
            "start_with_windows" => self.start_with_windows = value.parse().unwrap_or(false),
            "show_mascot" => self.show_mascot = value.parse().unwrap_or(true),
            "minimal_notch" => self.minimal_notch = value.parse().unwrap_or(false),
            "notch_width" => self.notch_width = value.parse().unwrap_or(150),
            "notch_height" => self.notch_height = value.parse().unwrap_or(30),
            "corner_radius" => self.corner_radius = value.parse().unwrap_or(14),
            "pomodoro_work_minutes" => self.pomodoro_work_minutes = value.parse().unwrap_or(25),
            "pomodoro_short_break_minutes" => self.pomodoro_short_break_minutes = value.parse().unwrap_or(5),
            "pomodoro_long_break_minutes" => self.pomodoro_long_break_minutes = value.parse().unwrap_or(15),
            "water_goal_ml" => self.water_goal_ml = value.parse().unwrap_or(1500),
            "water_reminder_hours" => self.water_reminder_hours = value.parse().unwrap_or(2),
            "show_lyrics" => self.show_lyrics = value.parse().unwrap_or(true),
            "lyrics_provider" => self.lyrics_provider = value.to_string(),
            "shelf_max_items" => self.shelf_max_items = value.parse().unwrap_or(10),
            _ => { self.custom.insert(key.to_string(), value.to_string()); }
        }
    }

    /// Save settings to file
    pub fn save(&self) -> Result<()> {
        let path = Self::get_path();
        
        // Create parent directory if needed
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = self.to_string();
        fs::write(&path, content)?;
        info!("Saved settings to {}", path.display());
        Ok(())
    }

    /// Get a value by key
    pub fn get(&self, key: &str) -> Option<String> {
        match key {
            "start_with_windows" => Some(self.start_with_windows.to_string()),
            "show_mascot" => Some(self.show_mascot.to_string()),
            "minimal_notch" => Some(self.minimal_notch.to_string()),
            "notch_width" => Some(self.notch_width.to_string()),
            "notch_height" => Some(self.notch_height.to_string()),
            "corner_radius" => Some(self.corner_radius.to_string()),
            "pomodoro_work_minutes" => Some(self.pomodoro_work_minutes.to_string()),
            "pomodoro_short_break_minutes" => Some(self.pomodoro_short_break_minutes.to_string()),
            "pomodoro_long_break_minutes" => Some(self.pomodoro_long_break_minutes.to_string()),
            "water_goal_ml" => Some(self.water_goal_ml.to_string()),
            "water_reminder_hours" => Some(self.water_reminder_hours.to_string()),
            "show_lyrics" => Some(self.show_lyrics.to_string()),
            "lyrics_provider" => Some(self.lyrics_provider.clone()),
            "shelf_max_items" => Some(self.shelf_max_items.to_string()),
            _ => self.custom.get(key).cloned(),
        }
    }

    /// Set a value by key
    pub fn set(&mut self, key: &str, value: &str) {
        self.set_key(key, value);
    }
}

impl std::fmt::Display for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "# Nusa Notch Clone Settings")?;
        writeln!(f, "# Generated automatically - edit with care")?;
        writeln!(f, "")?;
        
        writeln!(f, "[General]")?;
        writeln!(f, "start_with_windows={}", self.start_with_windows)?;
        writeln!(f, "show_mascot={}", self.show_mascot)?;
        writeln!(f, "minimal_notch={}", self.minimal_notch)?;
        writeln!(f, "")?;
        
        writeln!(f, "[Notch]")?;
        writeln!(f, "notch_width={}", self.notch_width)?;
        writeln!(f, "notch_height={}", self.notch_height)?;
        writeln!(f, "corner_radius={}", self.corner_radius)?;
        writeln!(f, "")?;
        
        writeln!(f, "[Timers]")?;
        writeln!(f, "pomodoro_work_minutes={}", self.pomodoro_work_minutes)?;
        writeln!(f, "pomodoro_short_break_minutes={}", self.pomodoro_short_break_minutes)?;
        writeln!(f, "pomodoro_long_break_minutes={}", self.pomodoro_long_break_minutes)?;
        writeln!(f, "water_goal_ml={}", self.water_goal_ml)?;
        writeln!(f, "water_reminder_hours={}", self.water_reminder_hours)?;
        writeln!(f, "")?;
        
        writeln!(f, "[Media]")?;
        writeln!(f, "show_lyrics={}", self.show_lyrics)?;
        writeln!(f, "lyrics_provider={}", self.lyrics_provider)?;
        writeln!(f, "")?;
        
        writeln!(f, "[Shelf]")?;
        writeln!(f, "shelf_max_items={}", self.shelf_max_items)?;
        writeln!(f, "")?;
        
        if !self.custom.is_empty() {
            writeln!(f, "[Custom]")?;
            for (key, value) in &self.custom {
                writeln!(f, "{}={}", key, value)?;
            }
        }
        
        Ok(())
    }
}

/// Settings manager with async interface
pub struct SettingsManager {
    settings: Arc<RwLock<Settings>>,
    event_tx: mpsc::Sender<SettingsEvent>,
    event_rx: mpsc::Receiver<SettingsEvent>,
}

impl SettingsManager {
    pub fn new() -> Result<(Self, mpsc::Sender<SettingsEvent>)> {
        let settings = Arc::new(RwLock::new(Settings::load()?));
        let (event_tx, event_rx) = mpsc::channel(32);
        
        let manager = Self {
            settings: settings.clone(),
            event_tx: event_tx.clone(),
            event_rx,
        };
        
        Ok((manager, event_tx))
    }

    pub async fn run(mut self) -> Result<()> {
        while let Some(event) = self.event_rx.recv().await {
            match event {
                SettingsEvent::Get(key, responder) => {
                    let value = self.settings.read().unwrap().get(&key);
                    let _ = responder.send(value);
                }
                SettingsEvent::Set(key, value) => {
                    self.settings.write().unwrap().set(&key, &value);
                }
                SettingsEvent::Save => {
                    if let Err(e) = self.settings.read().unwrap().save() {
                        warn!("Failed to save settings: {}", e);
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_parse() {
        let content = r#"
start_with_windows=true
show_mascot=false
notch_width=200
"#;
        let settings = Settings::parse(content).unwrap();
        assert!(settings.start_with_windows);
        assert!(!settings.show_mascot);
        assert_eq!(settings.notch_width, 200);
    }

    #[test]
    fn test_settings_display() {
        let settings = Settings::default();
        let output = settings.to_string();
        assert!(output.contains("start_with_windows=false"));
        assert!(output.contains("show_mascot=true"));
    }
}