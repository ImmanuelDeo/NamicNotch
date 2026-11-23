// src/mascot/mod.rs
// MascotDirector: rotasi 4 karakter idle + transisi fade, dengan alias kompatibel
// `DevilUnionEyes` agar `src/notch/` tidak perlu diubah.
pub mod asta;
pub mod bakugo;
pub mod chibi_liebe;
pub mod core;
pub mod eren;
pub mod sukuna;

use self::core::{IdleCharacter, MascotContext};
use windows::Win32::Graphics::Direct2D::Common::D2D_RECT_F;
use windows::Win32::Graphics::Direct2D::{
    ID2D1Factory, ID2D1HwndRenderTarget, D2D1_ANTIALIAS_MODE_PER_PRIMITIVE,
};

pub struct MascotDirector {
    roster: Vec<Box<dyn IdleCharacter>>,
    current_idx: usize,
    phase_time: f32,
    local_time: f32,
    blink_timer: f32,
    blink_progress: f32,
    look_x: f32,
    look_y: f32,
    target_look_x: f32,
    target_look_y: f32,
    flare_intensity: f32,
}

impl MascotDirector {
    pub fn new() -> Self {
        Self {
            // Tambah / hapus / ubah urutan karakter cukup di sini.
            roster: vec![
                Box::new(asta::AstaDevilUnion::new()),
                Box::new(sukuna::YujiSukuna::new()),
                Box::new(bakugo::KatsukiBakugo::new()),
                Box::new(eren::ErenAttackTitan::new()),
            ],
            current_idx: 0,
            phase_time: 0.0,
            local_time: 0.0,
            blink_timer: 2.8,
            blink_progress: 0.0,
            look_x: 0.0,
            look_y: 0.0,
            target_look_x: 0.0,
            target_look_y: 0.0,
            flare_intensity: 1.0,
        }
    }

    pub fn trigger_awakening(&mut self) {
        self.flare_intensity = 1.8;
        self.blink_progress = 1.0;
    }

    pub fn trigger_wake_surge(&mut self) {
        self.flare_intensity = 2.4;
    }

    #[allow(dead_code)]
    pub fn next_character(&mut self) {
        if !self.roster.is_empty() {
            self.current_idx = (self.current_idx + 1) % self.roster.len();
            self.local_time = 0.0;
            self.blink_progress = 0.95;
        }
    }

    fn current_context(&self) -> MascotContext {
        let duration = self
            .roster
            .get(self.current_idx)
            .map(|c| c.duration_secs())
            .unwrap_or(10.0);

        MascotContext {
            phase_time: self.phase_time,
            local_time: self.local_time,
            duration,
            blink_progress: self.blink_progress,
            look_x: self.look_x,
            look_y: self.look_y,
            flare_intensity: self.flare_intensity,
        }
    }

    /// Amplop fade-in / fade-out 0.45 detik saat ganti karakter.
    fn transition_alpha(&self, duration: f32) -> f32 {
        let fade = 0.45_f32;
        if self.local_time < fade {
            (self.local_time / fade).clamp(0.0, 1.0)
        } else if self.local_time > duration - fade {
            ((duration - self.local_time) / fade).clamp(0.0, 1.0)
        } else {
            1.0
        }
    }

    pub fn update(&mut self, dt: f32, cursor_offset: Option<(f32, f32)>) {
        if self.roster.is_empty() {
            return;
        }
        self.phase_time += dt;
        self.local_time += dt;

        let duration = self.roster[self.current_idx].duration_secs();
        if self.local_time >= duration {
            self.local_time = 0.0;
            self.current_idx = (self.current_idx + 1) % self.roster.len();
            self.blink_progress = 0.92;
        }

        let ctx = self.current_context();
        if let Some((tx, ty)) = self.roster[self.current_idx].override_look_target(&ctx) {
            self.target_look_x = tx;
            self.target_look_y = ty;
        } else if let Some((dx, dy)) = cursor_offset {
            self.target_look_x = (dx / 160.0).clamp(-1.0, 1.0) * 2.5;
            self.target_look_y = (dy / 90.0).clamp(-1.0, 1.0) * 1.2;
        } else {
            self.target_look_x = (self.phase_time * 1.0).sin() * 1.5;
            self.target_look_y = (self.phase_time * 0.7).cos() * 0.45;
        }

        let follow = (dt * 10.0).min(1.0);
        self.look_x += (self.target_look_x - self.look_x) * follow;
        self.look_y += (self.target_look_y - self.look_y) * follow;

        if self.flare_intensity > 1.0 {
            self.flare_intensity = (self.flare_intensity - dt * 2.2).max(1.0);
        }

        self.blink_timer -= dt;
        if self.blink_timer <= 0.0 {
            self.blink_progress += dt * 15.0;
            if self.blink_progress >= 1.0 {
                self.blink_progress = 0.0;
                self.blink_timer = 2.6 + (self.phase_time.sin().abs() * 2.2);
            }
        }
    }

    pub unsafe fn draw_idle_aura_background(
        &self,
        rt: &ID2D1HwndRenderTarget,
        factory: &ID2D1Factory,
        pill_rect: D2D_RECT_F,
        alpha: f32,
    ) {
        if self.roster.is_empty() {
            return;
        }
        rt.SetAntialiasMode(D2D1_ANTIALIAS_MODE_PER_PRIMITIVE);
        let ctx = self.current_context();
        let eff_alpha = alpha * self.transition_alpha(ctx.duration);
        self.roster[self.current_idx].draw_aura(rt, factory, pill_rect, eff_alpha, &ctx);
    }

    pub unsafe fn draw(
        &self,
        rt: &ID2D1HwndRenderTarget,
        factory: &ID2D1Factory,
        cx: f32,
        cy: f32,
        alpha: f32,
    ) {
        if self.roster.is_empty() {
            return;
        }
        rt.SetAntialiasMode(D2D1_ANTIALIAS_MODE_PER_PRIMITIVE);
        let ctx = self.current_context();
        let eff_alpha = alpha * self.transition_alpha(ctx.duration);
        self.roster[self.current_idx].draw_character(rt, factory, cx, cy, eff_alpha, &ctx);
    }
}

impl Default for MascotDirector {
    fn default() -> Self {
        Self::new()
    }
}

/// Alias drop-in untuk kode lama di `src/notch/`.
pub type DevilUnionEyes = MascotDirector;
