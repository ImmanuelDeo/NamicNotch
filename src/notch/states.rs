// src/notch/states.rs
use crate::mascot::chibi_liebe::ChibiLiebe;
use crate::mascot::eyes::DevilUnionEyes;
use windows::Media::Control::GlobalSystemMediaTransportControlsSessionManager;
use windows::Win32::Graphics::Direct2D::Common::D2D_RECT_F;
use windows::Win32::System::Power::{GetSystemPowerStatus, SYSTEM_POWER_STATUS};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotchState {
    Pet,    // Idle mascot: Mata Asta (Devil Union <-> Normal Emerald + Floating Sword)
    Idle,   // Closed notch dengan Jam Digital & Baterai
    Music,  // Tab 0: Music Player + Controls
    Lyrics, // Tab 1: Synced Lyrics
    Pomo,   // Tab 2: Pomodoro Focus Timer
    Water,  // Tab 3: Water Reminder & Hourly Chart
    Shelf,  // Tab 4: File Shelf Drag & Drop
    Todo,   // Tab 5: To-Do List
}

/// Pop-up Overlay sementara seperti di Nusa Original (Volume, Brightness, CapsLock, Charging, Headphones)
#[derive(Debug, Clone, PartialEq)]
pub enum OverlayKind {
    Volume(u8),
    Brightness(u8),
    CapsLock(bool),
    Charging { percent: u8, plugged: bool },
    Headphones { name: String, percent: u8 },
}

#[derive(Debug, Clone, Copy)]
pub struct SpringValue {
    pub value: f32,
    pub target: f32,
    pub velocity: f32,
    pub stiffness: f32,
    pub damping: f32,
}

impl SpringValue {
    pub fn new(initial: f32, stiffness: f32, damping: f32) -> Self {
        Self {
            value: initial,
            target: initial,
            velocity: 0.0,
            stiffness,
            damping,
        }
    }

    pub fn set_target(&mut self, target: f32) {
        self.target = target;
    }

    pub fn update(&mut self, dt: f32) {
        let step_dt = (dt * 0.5).clamp(0.0005, 0.016);
        for _ in 0..2 {
            let displacement = self.value - self.target;
            let spring_force = -self.stiffness * displacement;
            let damping_force = -self.damping * self.velocity;
            let acceleration = spring_force + damping_force;

            self.velocity += acceleration * step_dt;
            self.value += self.velocity * step_dt;
        }

        if self.is_settled() {
            self.value = self.target;
            self.velocity = 0.0;
        }
    }

    pub fn is_settled(&self) -> bool {
        (self.value - self.target).abs() < 0.15 && self.velocity.abs() < 0.8
    }
}

pub struct NotchController {
    pub state: NotchState,
    pub last_expanded_tab: NotchState,
    pub is_hovered: bool,
    pub minimal_mode: bool,
    pub mascot_enabled: bool,
    pub anim_time: f32,

    pub width: SpringValue,
    pub height: SpringValue,
    pub corner_radius: SpringValue,
    pub content_alpha: SpringValue,

    pub closed_quiet_timer: f32,
    pub hover_grace_timer: f32,

    // --- Pop-up Overlay (Volume, Brightness, CapsLock, Charging, Headphones) ---
    pub active_overlay: Option<OverlayKind>,
    pub overlay_timer: f32,

    // --- Status Baterai & Charger Sistem ---
    pub battery_percent: u8,
    pub is_charging: bool,
    power_poll_timer: f32,

    // --- Data Fitur 1: Music ---
    pub media_title: String,
    pub media_artist: String,
    pub media_playing: bool,
    pub media_progress: f32,
    pub media_elapsed_secs: u32,
    pub media_total_secs: u32,

    // --- Data Fitur 2: Synced Lyrics ---
    pub lyrics_lines: Vec<String>,
    pub active_lyric_idx: usize,

    // --- Data Fitur 3: Pomodoro Timer ---
    pub pomo_remaining_secs: u32,
    pub pomo_total_secs: u32,
    pub pomo_running: bool,
    pub pomo_is_break: bool,
    pomo_accum: f32,

    // --- Data Fitur 4: Water Tracker ---
    pub water_current_ml: u32,
    pub water_goal_ml: u32,
    pub water_cup_ml: u32,
    pub water_hourly_ml: [u32; 6],

    // --- Data Fitur 5: File Shelf ---
    pub shelf_files: Vec<(String, String)>,

    // --- Data Fitur 6: To-Do List ---
    pub todos: Vec<(String, bool)>,

    // --- Maskot ---
    pub devil_eyes: DevilUnionEyes,
    pub chibi_liebe: ChibiLiebe,
}

impl NotchController {
    pub fn new() -> Self {
        let (bat_pct, charging) = Self::read_windows_power();
        let mut ctrl = Self {
            state: NotchState::Pet,
            last_expanded_tab: NotchState::Music,
            is_hovered: false,
            minimal_mode: false,
            mascot_enabled: true,
            anim_time: 0.0,
            width: SpringValue::new(184.0, 310.0, 26.0),
            height: SpringValue::new(34.0, 310.0, 26.0),
            corner_radius: SpringValue::new(16.0, 310.0, 26.0),
            content_alpha: SpringValue::new(1.0, 240.0, 28.0),
            closed_quiet_timer: 0.0,
            hover_grace_timer: 0.0,

            active_overlay: None,
            overlay_timer: 0.0,
            battery_percent: bat_pct,
            is_charging: charging,
            power_poll_timer: 0.0,

            media_title: String::from("Haruka Mirai (Anti-Magic Edit)"),
            media_artist: String::from("Kankaku Piero · Black Clover OP 1"),
            media_playing: true,
            media_progress: 0.42,
            media_elapsed_secs: 84,
            media_total_secs: 200,

            lyrics_lines: vec![
                String::from("♪ Calling out through the darkened sky"),
                String::from("Even without magic, I'll surpass my limits right here!"),
                String::from("Unite the shadows, carve the path ahead"),
                String::from("Haruka mirai e — never giving up is my magic"),
            ],
            active_lyric_idx: 1,

            pomo_remaining_secs: 25 * 60,
            pomo_total_secs: 25 * 60,
            pomo_running: false,
            pomo_is_break: false,
            pomo_accum: 0.0,

            water_current_ml: 750,
            water_goal_ml: 1500,
            water_cup_ml: 250,
            water_hourly_ml: [250, 250, 0, 250, 0, 0],

            shelf_files: vec![
                (String::from("ARCHITECTURE.md"), String::from("4.2 KB · Markdown")),
                (String::from("nusa_config.toml"), String::from("1.8 KB · Config")),
            ],

            todos: vec![
                (String::from("Integrasi mata Asta Devil Union & Emerald"), true),
                (String::from("Bentuk notch menyatu bezel atas laptop"), true),
                (String::from("Uji Volume, Brightness, Charging & Music"), false),
            ],

            devil_eyes: DevilUnionEyes::new(),
            chibi_liebe: ChibiLiebe::new(),
        };
        ctrl.chibi_liebe.sync_with_state(ctrl.state);
        ctrl
    }

    fn read_windows_power() -> (u8, bool) {
        unsafe {
            let mut status = SYSTEM_POWER_STATUS::default();
            if GetSystemPowerStatus(&mut status).is_ok() {
                let pct = if status.BatteryLifePercent <= 100 {
                    status.BatteryLifePercent
                } else {
                    100
                };
                let charging = status.ACLineStatus == 1;
                (pct, charging)
            } else {
                (100, false)
            }
        }
    }

    /// Tampilkan pop-up overlay (Volume, Brightness, CapsLock, Charging, Headphones)
    pub fn trigger_overlay(&mut self, kind: OverlayKind) {
        self.active_overlay = Some(kind);
        self.overlay_timer = 2.4;
        if !self.is_hovered {
            self.width.set_target(256.0);
            self.height.set_target(42.0);
            self.corner_radius.set_target(18.0);
        }
    }

    pub fn target_dimensions_for(state: NotchState, minimal_mode: bool) -> (f32, f32, f32) {
        if minimal_mode && matches!(state, NotchState::Pet | NotchState::Idle) {
            return (130.0, 6.0, 3.0);
        }

        match state {
            NotchState::Pet => (184.0, 34.0, 16.0),
            NotchState::Idle => (204.0, 34.0, 16.0),
            NotchState::Music => (430.0, 150.0, 22.0),
            NotchState::Lyrics => (465.0, 158.0, 22.0),
            NotchState::Pomo => (410.0, 148.0, 22.0),
            NotchState::Water => (420.0, 152.0, 22.0),
            NotchState::Shelf => (440.0, 160.0, 22.0),
            NotchState::Todo => (430.0, 164.0, 22.0),
        }
    }

    pub fn transition_to(&mut self, next_state: NotchState) {
        if self.state == next_state {
            return;
        }

        if next_state == NotchState::Pet {
            self.devil_eyes.trigger_awakening();
        } else if matches!(
            next_state,
            NotchState::Music
                | NotchState::Lyrics
                | NotchState::Pomo
                | NotchState::Water
                | NotchState::Shelf
                | NotchState::Todo
        ) {
            self.last_expanded_tab = next_state;
        }

        self.state = next_state;
        self.chibi_liebe.sync_with_state(next_state);

        let (tw, th, tr) = Self::target_dimensions_for(next_state, self.minimal_mode);
        self.width.set_target(tw);
        self.height.set_target(th);
        self.corner_radius.set_target(tr);

        self.content_alpha.value = 0.25;
        self.content_alpha.set_target(1.0);
    }

    pub fn update(&mut self, dt: f32, cursor_in_pill: bool, cursor_offset: Option<(f32, f32)>) {
        self.anim_time += dt.clamp(0.001, 0.05);

        // 1. Poll status charger & baterai setiap 2.0 detik
        self.power_poll_timer += dt;
        if self.power_poll_timer >= 2.0 {
            self.power_poll_timer = 0.0;
            let (pct, charging) = Self::read_windows_power();
            if charging != self.is_charging {
                self.is_charging = charging;
                self.battery_percent = pct;
                self.trigger_overlay(OverlayKind::Charging {
                    percent: pct,
                    plugged: charging,
                });
            } else {
                self.battery_percent = pct;
            }
        }

        // 2. Kurangi timer pop-up overlay jika sedang tampil
        if self.active_overlay.is_some() {
            self.overlay_timer -= dt;
            if self.overlay_timer <= 0.0 {
                self.active_overlay = None;
                if !self.is_hovered {
                    let (tw, th, tr) = Self::target_dimensions_for(self.state, self.minimal_mode);
                    self.width.set_target(tw);
                    self.height.set_target(th);
                    self.corner_radius.set_target(tr);
                }
            }
        }

        // 3. Hover buka/tutup otomatis
        if cursor_in_pill {
            self.hover_grace_timer = 0.14;
            if !self.is_hovered {
                self.is_hovered = true;
                self.active_overlay = None;
                self.devil_eyes.trigger_wake_surge();
                self.transition_to(self.last_expanded_tab);
            }
        } else if self.is_hovered {
            self.hover_grace_timer -= dt;
            if self.hover_grace_timer <= 0.0 {
                self.is_hovered = false;
                self.closed_quiet_timer = 0.0;
                // Saat baru ditutup, tampilkan jam & baterai (Idle) selama 3.5 detik, lalu masuk ke Mata Asta (Pet)
                self.transition_to(NotchState::Idle);
            }
        } else if self.state == NotchState::Idle && self.mascot_enabled {
            self.closed_quiet_timer += dt;
            if self.closed_quiet_timer >= 3.5 {
                self.transition_to(NotchState::Pet);
            }
        }

        // 4. Countdown Pomodoro
        if self.pomo_running {
            self.pomo_accum += dt;
            if self.pomo_accum >= 1.0 {
                self.pomo_accum -= 1.0;
                if self.pomo_remaining_secs > 0 {
                    self.pomo_remaining_secs -= 1;
                } else {
                    self.pomo_is_break = !self.pomo_is_break;
                    self.pomo_total_secs = if self.pomo_is_break { 5 * 60 } else { 25 * 60 };
                    self.pomo_remaining_secs = self.pomo_total_secs;
                }
            }
        }

        // 5. Progress bar musik
        if self.media_playing && self.media_total_secs > 0 {
            let next_prog = self.media_progress + (dt / self.media_total_secs as f32);
            self.media_progress = if next_prog > 1.0 { 0.0 } else { next_prog };
            self.media_elapsed_secs =
                (self.media_progress * self.media_total_secs as f32) as u32;
        }

        self.width.update(dt);
        self.height.update(dt);
        self.corner_radius.update(dt);
        self.content_alpha.update(dt);

        if self.state == NotchState::Pet || self.height.value < 55.0 {
            self.devil_eyes.update(dt, cursor_offset);
        }

        if self.chibi_liebe.is_visible() {
            self.chibi_liebe.update(dt);
        }
    }

    /// Kirim perintah kontrol media sesungguhnya ke Windows GSMTC (Spotify / Browser / Apple Music)
    fn send_system_media_command(cmd: &'static str) {
        std::thread::spawn(move || {
            if let Ok(op) = GlobalSystemMediaTransportControlsSessionManager::RequestAsync() {
                if let Ok(manager) = op.get() {
                    if let Ok(session) = manager.GetCurrentSession() {
                        match cmd {
                            "toggle" => {
                                let _ = session.TryTogglePlayPauseAsync();
                            }
                            "next" => {
                                let _ = session.TrySkipNextAsync();
                            }
                            "prev" => {
                                let _ = session.TrySkipPreviousAsync();
                            }
                            _ => {}
                        }
                    }
                }
            }
        });
    }

    pub fn handle_panel_click(&mut self, x: f32, y: f32, pill_rect: D2D_RECT_F, dpi: f32) -> bool {
        let lx = (x - pill_rect.left) / dpi;
        let ly = (y - pill_rect.top) / dpi;

        match self.state {
            NotchState::Music => {
                if (98.0..=142.0).contains(&ly) {
                    if (110.0..=146.0).contains(&lx) {
                        Self::send_system_media_command("prev");
                        self.media_progress = 0.0;
                        self.media_elapsed_secs = 0;
                        return true;
                    } else if (152.0..=192.0).contains(&lx) {
                        Self::send_system_media_command("toggle");
                        self.media_playing = !self.media_playing;
                        return true;
                    } else if (198.0..=234.0).contains(&lx) {
                        Self::send_system_media_command("next");
                        self.media_progress = 0.0;
                        self.media_elapsed_secs = 0;
                        return true;
                    } else if (246.0..=312.0).contains(&lx) {
                        self.transition_to(NotchState::Lyrics);
                        return true;
                    }
                }
            }
            NotchState::Lyrics => {
                if ly > 36.0 && !self.lyrics_lines.is_empty() {
                    self.active_lyric_idx = (self.active_lyric_idx + 1) % self.lyrics_lines.len();
                    return true;
                }
            }
            NotchState::Pomo => {
                if (90.0..=136.0).contains(&ly) {
                    if (128.0..=218.0).contains(&lx) {
                        self.pomo_running = !self.pomo_running;
                        return true;
                    } else if (226.0..=294.0).contains(&lx) {
                        self.pomo_running = false;
                        self.pomo_remaining_secs = self.pomo_total_secs;
                        return true;
                    }
                }
            }
            NotchState::Water => {
                if (44.0..=104.0).contains(&ly) && (240.0..=398.0).contains(&lx) {
                    self.water_current_ml =
                        (self.water_current_ml + self.water_cup_ml).min(self.water_goal_ml * 2);
                    let slot = ((self.water_current_ml / self.water_cup_ml) as usize) % 6;
                    self.water_hourly_ml[slot] += self.water_cup_ml;
                    return true;
                }
            }
            NotchState::Todo => {
                if ly >= 38.0 {
                    let row = ((ly - 38.0) / 33.0) as usize;
                    if let Some(item) = self.todos.get_mut(row) {
                        item.1 = !item.1;
                        return true;
                    }
                }
            }
            _ => {}
        }
        false
    }
}