//! Spring physics and state management for Notch

use crate::mascot::chibi_liebe::ChibiLiebe;
use crate::mascot::eyes::DevilUnionEyes;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotchState {
    Pet,    // Idle mascot: Mata Asta Devil Union Mode
    Idle,   // Closed notch dengan jam/baterai
    Music,  // Expanded music player (+ Chibi Liebe Headbop)
    Lyrics, // Split / wide synced lyrics (Tanpa Chibi Liebe - bersih)
    Pomo,   // Pomodoro focus timer (+ Chibi Liebe FocusGuard)
    Water,  // Water reminder tracker (Tanpa Chibi Liebe)
    Shelf,  // Drag & drop file shelf (+ Chibi Liebe CatchingFiles)
    Todo,   // Task list (Tanpa Chibi Liebe)
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

    pub width: SpringValue,
    pub height: SpringValue,
    pub corner_radius: SpringValue,
    pub content_alpha: SpringValue,

    pub closed_quiet_timer: f32,
    pub hover_grace_timer: f32,

    pub devil_eyes: DevilUnionEyes,
    pub chibi_liebe: ChibiLiebe,
}

impl NotchController {
    pub fn new() -> Self {
        Self {
            state: NotchState::Pet,
            last_expanded_tab: NotchState::Music,
            is_hovered: false,
            minimal_mode: false,
            mascot_enabled: true,
            width: SpringValue::new(168.0, 310.0, 26.0),
            height: SpringValue::new(34.0, 310.0, 26.0),
            corner_radius: SpringValue::new(17.0, 310.0, 26.0),
            content_alpha: SpringValue::new(1.0, 240.0, 28.0),
            closed_quiet_timer: 0.0,
            hover_grace_timer: 0.0,
            devil_eyes: DevilUnionEyes::new(),
            chibi_liebe: ChibiLiebe::new(),
        }
    }

    pub fn target_dimensions_for(state: NotchState, minimal_mode: bool) -> (f32, f32, f32) {
        if minimal_mode && matches!(state, NotchState::Pet | NotchState::Idle) {
            return (120.0, 6.0, 3.0);
        }

        match state {
            NotchState::Pet => (164.0, 34.0, 17.0),
            NotchState::Idle => (186.0, 34.0, 17.0),
            NotchState::Music => (390.0, 148.0, 26.0),
            NotchState::Lyrics => (470.0, 168.0, 28.0),
            NotchState::Pomo => (370.0, 144.0, 26.0),
            NotchState::Water => (380.0, 156.0, 26.0),
            NotchState::Shelf => (420.0, 172.0, 28.0),
            NotchState::Todo => (390.0, 184.0, 28.0),
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
        if cursor_in_pill {
            self.hover_grace_timer = 0.12;
            if !self.is_hovered {
                self.is_hovered = true;
                self.devil_eyes.trigger_wake_surge();
                self.transition_to(self.last_expanded_tab);
            }
        } else if self.is_hovered {
            self.hover_grace_timer -= dt;
            if self.hover_grace_timer <= 0.0 {
                self.is_hovered = false;
                self.closed_quiet_timer = 0.0;
                if self.mascot_enabled {
                    self.transition_to(NotchState::Pet);
                } else {
                    self.transition_to(NotchState::Idle);
                }
            }
        } else if self.state == NotchState::Idle && self.mascot_enabled {
            self.closed_quiet_timer += dt;
            if self.closed_quiet_timer >= 5.0 {
                self.transition_to(NotchState::Pet);
            }
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

    pub fn is_animating(&self) -> bool {
        !self.width.is_settled()
            || !self.height.is_settled()
            || !self.corner_radius.is_settled()
            || !self.content_alpha.is_settled()
            || self.state == NotchState::Pet
            || self.chibi_liebe.is_visible()
    }
}