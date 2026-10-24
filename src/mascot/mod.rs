//! Mascot animation for Nusa Notch Clone - idle eyes animation

use anyhow::Result;
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc;
use tokio::time::{interval, Duration};
use tracing::{info, debug};
use crate::settings::Settings;
use crate::notch::NotchEvent;

/// Mascot state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MascotState {
    Sleeping,
    Awake,
    Blinking,
    LookingLeft,
    LookingRight,
    Happy,
    Sad,
}

impl Default for MascotState {
    fn default() -> Self {
        MascotState::Awake
    }
}

/// Mascot eye position
#[derive(Debug, Clone, Copy, Default)]
pub struct EyePosition {
    pub x: f32,
    pub y: f32,
}

/// Mascot manager
pub struct Mascot {
    settings: Arc<RwLock<Settings>>,
    notch_tx: mpsc::Sender<NotchEvent>,
    event_tx: mpsc::Sender<MascotEvent>,
    event_rx: mpsc::Receiver<MascotEvent>,
    
    state: MascotState,
    left_eye: EyePosition,
    right_eye: EyePosition,
    blink_timer: Option<tokio::time::Instant>,
    look_timer: Option<tokio::time::Instant>,
}

#[derive(Debug, Clone)]
pub enum MascotEvent {
    SetState(MascotState),
    Tick,
}

impl Mascot {
    pub fn new(
        settings: Arc<RwLock<Settings>>,
        notch_tx: mpsc::Sender<NotchEvent>,
    ) -> Result<Self> {
        let (event_tx, event_rx) = mpsc::channel(32);
        
        Ok(Self {
            settings,
            notch_tx,
            event_tx,
            event_rx,
            state: MascotState::default(),
            left_eye: EyePosition::default(),
            right_eye: EyePosition::default(),
            blink_timer: None,
            look_timer: None,
        })
    }
    
    pub async fn run(mut self) -> Result<()> {
        info!("Mascot started");
        
        if !self.settings.read().unwrap().show_mascot {
            info!("Mascot disabled in settings");
            return Ok(());
        }
        
        // Animation loop
        let mut animation_interval = interval(Duration::from_millis(50)); // 20 FPS
        let mut blink_interval = interval(Duration::from_secs(3));
        let mut look_interval = interval(Duration::from_secs(5));
        
        loop {
            tokio::select! {
                event = self.event_rx.recv() => {
                    if let Some(event) = event {
                        self.handle_event(event)?;
                    } else {
                        break;
                    }
                }
                _ = animation_interval.tick() => {
                    self.animate().await?;
                }
                _ = blink_interval.tick() => {
                    self.trigger_blink().await?;
                }
                _ = look_interval.tick() => {
                    self.trigger_look().await?;
                }
            }
        }
        
        info!("Mascot stopped");
        Ok(())
    }
    
    fn handle_event(&mut self, event: MascotEvent) -> Result<()> {
        match event {
            MascotEvent::SetState(state) => {
                self.state = state;
            }
            MascotEvent::Tick => {
                // Handled by animation loop
            }
        }
        Ok(())
    }
    
    async fn animate(&mut self) -> Result<()> {
        // Smooth eye movement
        let target_left = self.get_target_eye_position(true);
        let target_right = self.get_target_eye_position(false);
        
        // Lerp towards target
        let lerp_factor = 0.15;
        self.left_eye.x += (target_left.x - self.left_eye.x) * lerp_factor;
        self.left_eye.y += (target_left.y - self.left_eye.y) * lerp_factor;
        self.right_eye.x += (target_right.x - self.right_eye.x) * lerp_factor;
        self.right_eye.y += (target_right.y - self.right_eye.y) * lerp_factor;
        
        // Handle blink animation
        if let Some(blink_start) = self.blink_timer {
            let elapsed = blink_start.elapsed().as_secs_f32();
            if elapsed < 0.15 {
                // Closing
                let progress = elapsed / 0.15;
                self.left_eye.y = 1.0 - progress;
                self.right_eye.y = 1.0 - progress;
            } else if elapsed < 0.3 {
                // Opening
                let progress = (elapsed - 0.15) / 0.15;
                self.left_eye.y = progress;
                self.right_eye.y = progress;
            } else {
                self.blink_timer = None;
            }
        }
        
        // Send eye positions to notch for rendering
        // This would be sent via notch event
        // For now, just log
        debug!("Eyes: L({:.2}, {:.2}) R({:.2}, {:.2})", 
               self.left_eye.x, self.left_eye.y, 
               self.right_eye.x, self.right_eye.y);
        
        Ok(())
    }
    
    async fn trigger_blink(&mut self) -> Result<()> {
        if self.blink_timer.is_none() && self.state != MascotState::Sleeping {
            self.blink_timer = Some(tokio::time::Instant::now());
        }
        Ok(())
    }
    
    async fn trigger_look(&mut self) -> Result<()> {
        if self.look_timer.is_none() && self.state == MascotState::Awake {
            self.look_timer = Some(tokio::time::Instant::now());
            
            // Randomly look left or right
            if rand::random::<bool>() {
                self.state = MascotState::LookingLeft;
            } else {
                self.state = MascotState::LookingRight;
            }
            
            // Reset after a moment
            tokio::time::sleep(Duration::from_millis(2000)).await;
            if self.look_timer.is_some() {
                self.look_timer = None;
                self.state = MascotState::Awake;
            }
        }
        Ok(())
    }
    
    fn get_target_eye_position(&self, is_left: bool) -> EyePosition {
        let base_x = if is_left { -0.3 } else { 0.3 };
        let base_y = 0.0;
        
        match self.state {
            MascotState::Sleeping => EyePosition { x: base_x, y: 1.0 }, // Closed
            MascotState::Awake => EyePosition { x: base_x, y: base_y },
            MascotState::Blinking => EyePosition { x: base_x, y: 1.0 }, // Closed
            MascotState::LookingLeft => EyePosition { x: base_x - 0.2, y: base_y },
            MascotState::LookingRight => EyePosition { x: base_x + 0.2, y: base_y },
            MascotState::Happy => EyePosition { x: base_x, y: base_y - 0.1 }, // Slightly up
            MascotState::Sad => EyePosition { x: base_x, y: base_y + 0.2 }, // Down
        }
    }
    
    /// Draw mascot eyes using Direct2D
    pub fn draw_eyes(
        &self,
        rt: &windows::Win32::Graphics::Direct2D::ID2D1HwndRenderTarget,
        center_x: f32,
        center_y: f32,
        eye_radius: f32,
        dpi_scale: f32,
    ) -> Result<()> {
        use windows::Win32::Graphics::Direct2D::D2D1_ELLIPSE;
        use windows::Win32::Graphics::Direct2D::Common::{D2D_POINT_2F, D2D1_COLOR_F};
        
        let eye_radius = eye_radius * dpi_scale;
        let eye_spacing = eye_radius * 2.5 * dpi_scale;
        
        // Left eye
        let left_center = D2D_POINT_2F {
            x: center_x - eye_spacing + self.left_eye.x * eye_spacing * 0.3,
            y: center_y + self.left_eye.y * eye_spacing * 0.3,
        };
        
        let left_ellipse = D2D1_ELLIPSE {
            point: left_center,
            radiusX: eye_radius,
            radiusY: eye_radius * (1.0 - self.left_eye.y.abs() * 0.8).max(0.1), // Squish on blink
        };
        
        // Right eye
        let right_center = D2D_POINT_2F {
            x: center_x + eye_spacing + self.right_eye.x * eye_spacing * 0.3,
            y: center_y + self.right_eye.y * eye_spacing * 0.3,
        };
        
        let right_ellipse = D2D1_ELLIPSE {
            point: right_center,
            radiusX: eye_radius,
            radiusY: eye_radius * (1.0 - self.right_eye.y.abs() * 0.8).max(0.1),
        };
        
        // Pupils (black)
        let pupil_radius = eye_radius * 0.4;
        let pupil_left = D2D1_ELLIPSE {
            point: D2D_POINT_2F {
                x: left_center.x + self.left_eye.x * pupil_radius * 0.5,
                y: left_center.y + self.left_eye.y * pupil_radius * 0.5,
            },
            radiusX: pupil_radius * (1.0 - self.left_eye.y.abs()).max(0.1),
            radiusY: pupil_radius * (1.0 - self.left_eye.y.abs()).max(0.1),
        };
        
        let pupil_right = D2D1_ELLIPSE {
            point: D2D_POINT_2F {
                x: right_center.x + self.right_eye.x * pupil_radius * 0.5,
                y: right_center.y + self.right_eye.y * pupil_radius * 0.5,
            },
            radiusX: pupil_radius * (1.0 - self.right_eye.y.abs()).max(0.1),
            radiusY: pupil_radius * (1.0 - self.right_eye.y.abs()).max(0.1),
        };
        
        // Highlights (small white dots)
        let highlight_radius = eye_radius * 0.15;
        let hl_left = D2D1_ELLIPSE {
            point: D2D_POINT_2F {
                x: left_center.x - pupil_radius * 0.3,
                y: left_center.y - pupil_radius * 0.3,
            },
            radiusX: highlight_radius,
            radiusY: highlight_radius,
        };
        
        let hl_right = D2D1_ELLIPSE {
            point: D2D_POINT_2F {
                x: right_center.x - pupil_radius * 0.3,
                y: right_center.y - pupil_radius * 0.3,
            },
            radiusX: highlight_radius,
            radiusY: highlight_radius,
        };
        
        unsafe {
            // White eye background
            let white_brush = rt.CreateSolidColorBrush(
                &D2D1_COLOR_F { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
                None,
            )?;
            rt.FillEllipse(&left_ellipse, &white_brush);
            rt.FillEllipse(&right_ellipse, &white_brush);
            
            // Pupils
            let black_brush = rt.CreateSolidColorBrush(
                &D2D1_COLOR_F { r: 0.0, g: 0.0, b: 0.0, a: 1.0 },
                None,
            )?;
            rt.FillEllipse(&pupil_left, &black_brush);
            rt.FillEllipse(&pupil_right, &black_brush);
            
            // Highlights
            let highlight_brush = rt.CreateSolidColorBrush(
                &D2D1_COLOR_F { r: 1.0, g: 1.0, b: 1.0, a: 0.8 },
                None,
            )?;
            rt.FillEllipse(&hl_left, &highlight_brush);
            rt.FillEllipse(&hl_right, &highlight_brush);
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_mascot_state_default() {
        let state = MascotState::default();
        assert_eq!(state, MascotState::Awake);
    }
    
    #[test]
    fn test_eye_position_default() {
        let pos = EyePosition::default();
        assert_eq!(pos.x, 0.0);
        assert_eq!(pos.y, 0.0);
    }
}