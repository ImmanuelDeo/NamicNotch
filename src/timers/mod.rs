//! Timers for Nusa Notch Clone - Pomodoro and Water Reminder

use anyhow::Result;
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc;
use tokio::time::{interval, Duration, Instant};
use tracing::{info, warn};
use chrono::{DateTime, Local, Timelike};
use crate::settings::Settings;
use crate::notch::NotchEvent;

/// Pomodoro mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PomodoroMode {
    Work,
    ShortBreak,
    LongBreak,
}

impl Default for PomodoroMode {
    fn default() -> Self {
        PomodoroMode::Work
    }
}

/// Timer info for notch display
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct TimerInfo {
    pub mode: PomodoroMode,
    pub remaining_seconds: u32,
    pub total_seconds: u32,
    pub running: bool,
    pub completed_sessions: u32,
}

impl TimerInfo {
    pub fn new(mode: PomodoroMode, total_seconds: u32) -> Self {
        Self {
            mode,
            remaining_seconds: total_seconds,
            total_seconds,
            running: false,
            completed_sessions: 0,
        }
    }
    
    pub fn progress(&self) -> f32 {
        if self.total_seconds == 0 { 0.0 } else {
            1.0 - (self.remaining_seconds as f32 / self.total_seconds as f32)
        }
    }
    
    pub fn format_time(&self) -> String {
        let mins = self.remaining_seconds / 60;
        let secs = self.remaining_seconds % 60;
        format!("{:02}:{:02}", mins, secs)
    }
}

/// Water info for notch display
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct WaterInfo {
    pub current_ml: u32,
    pub goal_ml: u32,
    pub last_drink: Option<DateTime<Local>>,
    pub reminder_hours: u32,
}

impl WaterInfo {
    pub fn new(goal_ml: u32, reminder_hours: u32) -> Self {
        Self {
            current_ml: 0,
            goal_ml,
            last_drink: Some(Local::now()),
            reminder_hours,
        }
    }
    
    pub fn percentage(&self) -> f32 {
        if self.goal_ml == 0 { 0.0 } else {
            (self.current_ml as f32 / self.goal_ml as f32).min(1.0)
        }
    }
    
    pub fn format(&self) -> String {
        format!("{} / {} ml", self.current_ml, self.goal_ml)
    }
}

/// Timer events
#[derive(Debug, Clone)]
pub enum TimerEvent {
    PomodoroStart,
    PomodoroPause,
    PomodoroReset,
    PomodoroSkip,
    WaterDrink(u32), // ml
    WaterReset,
    Tick, // Internal tick
}

/// Timer manager
pub struct TimerManager {
    settings: Arc<RwLock<Settings>>,
    notch_tx: mpsc::Sender<NotchEvent>,
    event_tx: mpsc::Sender<TimerEvent>,
    event_rx: mpsc::Receiver<TimerEvent>,
    
    // Pomodoro state
    pomodoro: TimerInfo,
    pomodoro_end_time: Option<Instant>,
    
    // Water state
    water: WaterInfo,
    water_reminder_end: Option<Instant>,
    
    // Persistence
    state_dir: std::path::PathBuf,
}

impl TimerManager {
    pub fn new(
        settings: Arc<RwLock<Settings>>,
        notch_tx: mpsc::Sender<NotchEvent>,
    ) -> Result<Self> {
        let (event_tx, event_rx) = mpsc::channel(32);
        
        let state_dir = dirs::data_local_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join("NusaShelf")
            .join("Timers");
        
        std::fs::create_dir_all(&state_dir)?;
        
        // Load saved state
        let pomodoro = Self::load_pomodoro(&state_dir).unwrap_or_else(||
            TimerInfo::new(
                PomodoroMode::Work,
                settings.read().unwrap().pomodoro_work_minutes * 60
            )
        );
        
        let water = Self::load_water(&state_dir).unwrap_or_else(||
            WaterInfo::new(
                settings.read().unwrap().water_goal_ml,
                settings.read().unwrap().water_reminder_hours
            )
        );
        
        Ok(Self {
            settings,
            notch_tx,
            event_tx,
            event_rx,
            pomodoro,
            pomodoro_end_time: None,
            water,
            water_reminder_end: None,
            state_dir,
        })
    }
    
    pub async fn run(mut self) -> Result<()> {
        info!("Timer manager started");
        
        // Start reminder checks
        let mut pomodoro_tick = interval(Duration::from_secs(1));
        let mut water_check = interval(Duration::from_secs(60)); // Check every minute
        let mut save_interval = interval(Duration::from_secs(10));
        
        // Send initial state
        self.send_notch_update().await?;
        
        loop {
            tokio::select! {
                event = self.event_rx.recv() => {
                    if let Some(event) = event {
                        self.handle_event(event).await?;
                    } else {
                        break;
                    }
                }
                _ = pomodoro_tick.tick() => {
                    self.tick_pomodoro().await?;
                }
                _ = water_check.tick() => {
                    self.check_water_reminder().await?;
                }
                _ = save_interval.tick() => {
                    self.save_state()?;
                }
            }
        }
        
        // Final save
        self.save_state()?;
        
        info!("Timer manager stopped");
        Ok(())
    }
    
    async fn handle_event(&mut self, event: TimerEvent) -> Result<()> {
        match event {
            TimerEvent::PomodoroStart => {
                if !self.pomodoro.running {
                    self.pomodoro.running = true;
                    self.pomodoro_end_time = Some(Instant::now() + Duration::from_secs(self.pomodoro.remaining_seconds as u64));
                    self.send_notch_update().await?;
                }
            }
            TimerEvent::PomodoroPause => {
                if self.pomodoro.running {
                    self.pomodoro.running = false;
                    if let Some(end) = self.pomodoro_end_time {
                        let remaining = end.saturating_duration_since(Instant::now()).as_secs() as u32;
                        self.pomodoro.remaining_seconds = remaining;
                    }
                    self.pomodoro_end_time = None;
                    self.send_notch_update().await?;
                }
            }
            TimerEvent::PomodoroReset => {
                self.pomodoro.running = false;
                self.pomodoro_end_time = None;
                let (work_m, short_m, long_m) = {
                    let s = self.settings.read().unwrap();
                    (s.pomodoro_work_minutes, s.pomodoro_short_break_minutes, s.pomodoro_long_break_minutes)
                };
                self.pomodoro.remaining_seconds = match self.pomodoro.mode {
                    PomodoroMode::Work => work_m * 60,
                    PomodoroMode::ShortBreak => short_m * 60,
                    PomodoroMode::LongBreak => long_m * 60,
                };
                self.pomodoro.total_seconds = self.pomodoro.remaining_seconds;
                self.send_notch_update().await?;
            }
            TimerEvent::PomodoroSkip => {
                self.complete_pomodoro_session().await?;
            }
            TimerEvent::WaterDrink(ml) => {
                self.water.current_ml = (self.water.current_ml + ml).min(self.water.goal_ml);
                self.water.last_drink = Some(Local::now());
                self.water_reminder_end = Some(Instant::now() + Duration::from_secs(self.water.reminder_hours as u64 * 3600));
                self.send_notch_update().await?;
            }
            TimerEvent::WaterReset => {
                self.water.current_ml = 0;
                self.water.last_drink = Some(Local::now());
                self.water_reminder_end = Some(Instant::now() + Duration::from_secs(self.water.reminder_hours as u64 * 3600));
                self.send_notch_update().await?;
            }
            TimerEvent::Tick => {
                self.tick_pomodoro().await?;
            }
        }
        Ok(())
    }
    
    async fn tick_pomodoro(&mut self) -> Result<()> {
        if !self.pomodoro.running {
            return Ok(());
        }
        
        if let Some(end) = self.pomodoro_end_time {
            if Instant::now() >= end {
                // Timer completed
                self.complete_pomodoro_session().await?;
            } else {
                let remaining = end.saturating_duration_since(Instant::now()).as_secs() as u32;
                if self.pomodoro.remaining_seconds != remaining {
                    self.pomodoro.remaining_seconds = remaining;
                    self.send_notch_update().await?;
                }
            }
        }
        
        Ok(())
    }
    
    async fn complete_pomodoro_session(&mut self) -> Result<()> {
        self.pomodoro.running = false;
        self.pomodoro_end_time = None;
        self.pomodoro.completed_sessions += 1;
        
        // Determine next mode
        let (work_m, short_m, long_m) = {
            let s = self.settings.read().unwrap();
            (s.pomodoro_work_minutes, s.pomodoro_short_break_minutes, s.pomodoro_long_break_minutes)
        };
        
        self.pomodoro.mode = match self.pomodoro.mode {
            PomodoroMode::Work => {
                if self.pomodoro.completed_sessions % 4 == 0 {
                    PomodoroMode::LongBreak
                } else {
                    PomodoroMode::ShortBreak
                }
            }
            PomodoroMode::ShortBreak | PomodoroMode::LongBreak => {
                PomodoroMode::Work
            }
        };
        
        self.pomodoro.remaining_seconds = match self.pomodoro.mode {
            PomodoroMode::Work => work_m * 60,
            PomodoroMode::ShortBreak => short_m * 60,
            PomodoroMode::LongBreak => long_m * 60,
        };
        self.pomodoro.total_seconds = self.pomodoro.remaining_seconds;
        
        // Switch notch to pomo state
        let _ = self.notch_tx.try_send(NotchEvent::SetState(crate::notch::NotchState::Pomo));
        
        self.send_notch_update().await?;
        Ok(())
    }
    
    async fn check_water_reminder(&mut self) -> Result<()> {
        if self.water.current_ml >= self.water.goal_ml {
            return Ok(()); // Goal reached
        }
        
        Ok(())
    }
    
    async fn send_notch_update(&self) -> Result<()> {
        let _ = self.notch_tx.try_send(NotchEvent::UpdateTimer(self.pomodoro.clone()));
        Ok(())
    }
    
    fn save_state(&self) -> Result<()> {
        Self::save_pomodoro(&self.state_dir, &self.pomodoro)?;
        Self::save_water(&self.state_dir, &self.water)?;
        Ok(())
    }
    
    fn load_pomodoro(state_dir: &std::path::Path) -> Option<TimerInfo> {
        let path = state_dir.join("pomodoro.json");
        if path.exists() {
            if let Ok(content) = std::fs::read_to_string(&path) {
                if let Ok(info) = serde_json::from_str(&content) {
                    return Some(info);
                }
            }
        }
        None
    }
    
    fn save_pomodoro(state_dir: &std::path::Path, info: &TimerInfo) -> Result<()> {
        let path = state_dir.join("pomodoro.json");
        let content = serde_json::to_string_pretty(info)?;
        std::fs::write(path, content)?;
        Ok(())
    }
    
    fn load_water(state_dir: &std::path::Path) -> Option<WaterInfo> {
        let path = state_dir.join("water.json");
        if path.exists() {
            if let Ok(content) = std::fs::read_to_string(&path) {
                if let Ok(info) = serde_json::from_str(&content) {
                    return Some(info);
                }
            }
        }
        None
    }
    
    fn save_water(state_dir: &std::path::Path, info: &WaterInfo) -> Result<()> {
        let path = state_dir.join("water.json");
        let content = serde_json::to_string_pretty(info)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_timer_info_progress() {
        let mut timer = TimerInfo::new(PomodoroMode::Work, 1500); // 25 min
        timer.remaining_seconds = 750; // Halfway
        assert!((timer.progress() - 0.5).abs() < 0.01);
    }
    
    #[test]
    fn test_water_info_percentage() {
        let mut water = WaterInfo::new(2000, 2);
        water.current_ml = 1000;
        assert!((water.percentage() - 0.5).abs() < 0.01);
    }
}