// src/mascot/eyes.rs
use windows::Win32::Graphics::Direct2D::Common::{
    D2D1_COLOR_F, D2D1_FIGURE_BEGIN_FILLED, D2D1_FIGURE_END_CLOSED, D2D1_GRADIENT_STOP,
    D2D_POINT_2F, D2D_RECT_F,
};
use windows::Win32::Graphics::Direct2D::{
    ID2D1Factory, ID2D1HwndRenderTarget, ID2D1PathGeometry, D2D1_ELLIPSE, D2D1_EXTEND_MODE_CLAMP,
    D2D1_GAMMA_2_2, D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES, D2D1_ROUNDED_RECT,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DevilEyePhase {
    Awakening { timer: f32 },
    Active { idle_seconds: f32 },
    Dozing { timer: f32 },
    WakeSurge { timer: f32 },
}

#[derive(Debug, Clone, Copy)]
pub struct AntiMagicSpark {
    pub offset_x: f32,
    pub offset_y: f32,
    pub vx: f32,
    pub vy: f32,
    pub size: f32,
    pub life: f32,
    pub max_life: f32,
    pub kind: u8, // 0 = Obsidian Charcoal, 1 = Cool Slate-Blue, 2 = Muted Rose Ember
}

pub struct DevilUnionEyes {
    pub phase: DevilEyePhase,
    pub time: f32,
    pub look_x: f32,
    pub look_y: f32,
    pub target_look_x: f32,
    pub target_look_y: f32,
    pub openness: f32,
    pub blink_timer: f32,
    pub next_blink_in: f32,
    pub glance_timer: f32,
    pub aura_intensity: f32,
    pub pupil_scale: f32,
    pub sparks: [AntiMagicSpark; 10],
    rng_seed: u32,
}

impl Default for DevilUnionEyes {
    fn default() -> Self {
        Self::new()
    }
}

impl DevilUnionEyes {
    pub fn new() -> Self {
        let mut eyes = Self {
            phase: DevilEyePhase::Awakening { timer: 0.0 },
            time: 0.0,
            look_x: 0.0,
            look_y: 0.0,
            target_look_x: 0.0,
            target_look_y: 0.0,
            openness: 0.05,
            blink_timer: 0.0,
            next_blink_in: 3.2,
            glance_timer: 2.0,
            aura_intensity: 1.1,
            pupil_scale: 1.25,
            sparks: [AntiMagicSpark {
                offset_x: 0.0,
                offset_y: 0.0,
                vx: 0.0,
                vy: 0.0,
                size: 2.0,
                life: 0.0,
                max_life: 1.0,
                kind: 0,
            }; 10],
            rng_seed: 0xA57A_BC02,
        };
        for i in 0..eyes.sparks.len() {
            eyes.respawn_spark(i, true);
        }
        eyes
    }

    fn next_f32(&mut self) -> f32 {
        let mut x = self.rng_seed;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.rng_seed = x;
        (x as f32) / (u32::MAX as f32)
    }

    fn respawn_spark(&mut self, idx: usize, random_life: bool) {
        let side = if idx % 2 == 0 { -1.0 } else { 1.0 };
        let rx = (self.next_f32() - 0.5) * 18.0 + side * 15.0;
        let ry = (self.next_f32() - 0.5) * 7.0;
        let vx = (self.next_f32() - 0.5) * 12.0 + side * 3.5;
        let vy = -9.0 - self.next_f32() * 15.0;
        let max_life = 0.50 + self.next_f32() * 0.65;
        let life = if random_life {
            self.next_f32() * max_life
        } else {
            0.0
        };
        let size = 1.3 + self.next_f32() * 2.0;
        // Mayoritas partikel berwarna Slate/Charcoal netral, hanya 1 dari 5 yang memiliki aksen Muted Rose
        let kind = if idx % 5 == 0 {
            2
        } else if idx % 2 == 0 {
            1
        } else {
            0
        };

        self.sparks[idx] = AntiMagicSpark {
            offset_x: rx,
            offset_y: ry,
            vx,
            vy,
            size,
            life,
            max_life,
            kind,
        };
    }

    pub fn trigger_awakening(&mut self) {
        self.phase = DevilEyePhase::Awakening { timer: 0.0 };
        self.openness = 0.05;
        self.pupil_scale = 1.30;
        self.aura_intensity = 1.25;
    }

    pub fn trigger_wake_surge(&mut self) {
        if matches!(self.phase, DevilEyePhase::Dozing { .. }) {
            self.phase = DevilEyePhase::WakeSurge { timer: 0.0 };
            self.aura_intensity = 1.35;
        }
    }

    pub fn update(&mut self, dt: f32, cursor_offset: Option<(f32, f32)>) {
        let dt = dt.clamp(0.001, 0.05);
        self.time += dt;

        let mut cursor_near = false;
        if let Some((dx, dy)) = cursor_offset {
            let dist = (dx * dx + dy * dy).sqrt();
            if dist < 360.0 {
                cursor_near = true;
                self.target_look_x = (dx / 180.0).clamp(-1.0, 1.0);
                self.target_look_y = (dy / 110.0).clamp(-0.75, 0.85);
                if matches!(self.phase, DevilEyePhase::Dozing { .. }) {
                    self.trigger_wake_surge();
                }
            }
        }

        let target_openness: f32;
        let target_aura: f32;
        let target_pupil: f32;

        let mut phase = self.phase;
        match &mut phase {
            DevilEyePhase::Awakening { timer } => {
                *timer += dt;
                let p = (*timer / 0.45).clamp(0.0, 1.0);
                target_openness = if p < 0.6 { p / 0.6 * 1.10 } else { 1.0 }.min(1.04);
                target_pupil = 1.30 - 0.30 * p;
                target_aura = 1.25 - 0.35 * p;
                if *timer >= 0.45 {
                    phase = DevilEyePhase::Active { idle_seconds: 0.0 };
                }
            }
            DevilEyePhase::Active { idle_seconds } => {
                if cursor_near {
                    *idle_seconds = 0.0;
                    target_aura = 1.05;
                    target_pupil = 0.88;
                } else {
                    *idle_seconds += dt;
                    target_aura = 0.72 + 0.12 * (self.time * 2.8).sin();
                    target_pupil = 1.0 + 0.05 * (self.time * 2.0).cos();

                    self.glance_timer -= dt;
                    if self.glance_timer <= 0.0 {
                        let r1 = self.next_f32();
                        if r1 < 0.38 {
                            self.target_look_x = 0.0;
                            self.target_look_y = 0.0;
                        } else {
                            let r2 = self.next_f32();
                            self.target_look_x = (r2 - 0.5) * 1.45;
                            let r3 = self.next_f32();
                            self.target_look_y = (r3 - 0.5) * 0.55;
                        }
                        let r4 = self.next_f32();
                        self.glance_timer = 2.0 + r4 * 3.0;
                    }

                    if *idle_seconds > 45.0 {
                        phase = DevilEyePhase::Dozing { timer: 0.0 };
                    }
                }
                target_openness = 1.0;
            }
            DevilEyePhase::Dozing { timer } => {
                *timer += dt;
                self.target_look_x = 0.0;
                self.target_look_y = 0.22;
                target_openness = 0.32 + 0.04 * (self.time * 1.4).sin();
                target_aura = 0.40 + 0.10 * (self.time * 1.4).sin();
                target_pupil = 1.04;
            }
            DevilEyePhase::WakeSurge { timer } => {
                *timer += dt;
                target_openness = 1.06;
                target_aura = 1.35;
                target_pupil = 0.80;
                if *timer >= 0.30 {
                    phase = DevilEyePhase::Active { idle_seconds: 0.0 };
                }
            }
        }
        self.phase = phase;

        self.next_blink_in -= dt;
        if self.next_blink_in <= 0.0 {
            self.blink_timer = 0.14;
            self.next_blink_in = 2.8 + self.next_f32() * 3.8;
        }

        let blink_factor = if self.blink_timer > 0.0 {
            self.blink_timer -= dt;
            let norm = (self.blink_timer / 0.14).clamp(0.0, 1.0);
            (norm - 0.5).abs() * 2.0
        } else {
            1.0
        };

        let effective_openness = target_openness * blink_factor;

        let smooth = (14.0 * dt).min(1.0);
        self.look_x += (self.target_look_x - self.look_x) * smooth;
        self.look_y += (self.target_look_y - self.look_y) * smooth;
        self.openness += (effective_openness - self.openness) * (22.0 * dt).min(1.0);
        self.aura_intensity += (target_aura - self.aura_intensity) * (10.0 * dt).min(1.0);
        self.pupil_scale += (target_pupil - self.pupil_scale) * (12.0 * dt).min(1.0);

        let aura_speed_mult = self.aura_intensity.max(0.45);
        for i in 0..self.sparks.len() {
            self.sparks[i].life += dt * aura_speed_mult;
            if self.sparks[i].life >= self.sparks[i].max_life {
                self.respawn_spark(i, false);
            } else {
                self.sparks[i].offset_x += self.sparks[i].vx * dt;
                self.sparks[i].offset_y += self.sparks[i].vy * dt;
            }
        }
    }

    pub fn render(
        &self,
        factory: &ID2D1Factory,
        rt: &ID2D1HwndRenderTarget,
        pill_rect: D2D_RECT_F,
        alpha_mult: f32,
    ) {
        if alpha_mult <= 0.01 {
            return;
        }

        let center_x = (pill_rect.left + pill_rect.right) * 0.5;
        let center_y = (pill_rect.top + pill_rect.bottom) * 0.5;
        let eye_spacing = 17.5;

        unsafe {
            self.draw_anti_magic_sparks(rt, center_x, center_y, alpha_mult);

            for side in [-1.0_f32, 1.0_f32] {
                let base_x = center_x + side * eye_spacing;
                let base_y = center_y + 0.5;
                self.draw_single_devil_eye(factory, rt, base_x, base_y, side, alpha_mult);
            }
        }
    }

    unsafe fn draw_anti_magic_sparks(
        &self,
        rt: &ID2D1HwndRenderTarget,
        cx: f32,
        cy: f32,
        alpha_mult: f32,
    ) {
        for spark in &self.sparks {
            let progress = (spark.life / spark.max_life).clamp(0.0, 1.0);
            let spark_alpha = (1.0 - (2.0 * progress - 1.0).powi(2))
                * self.aura_intensity.clamp(0.2, 1.1)
                * alpha_mult;

            if spark_alpha <= 0.02 {
                continue;
            }

            // Warna partikel dominan Charcoal & Cool Steel-Slate agar netral dengan tema laptop biru/abu-abu
            let color = match spark.kind {
                0 => D2D1_COLOR_F {
                    r: 0.12,
                    g: 0.15,
                    b: 0.22,
                    a: spark_alpha * 0.85,
                },
                1 => D2D1_COLOR_F {
                    r: 0.48,
                    g: 0.56,
                    b: 0.72,
                    a: spark_alpha * 0.60,
                },
                _ => D2D1_COLOR_F {
                    r: 0.68,
                    g: 0.24,
                    b: 0.32,
                    a: spark_alpha * 0.55,
                },
            };

            if let Ok(brush) = rt.CreateSolidColorBrush(&color, None) {
                let x = cx + spark.offset_x;
                let y = cy + spark.offset_y;
                let r = spark.size * (1.0 - 0.4 * progress);
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F { x, y },
                        radiusX: r * 0.8,
                        radiusY: r * 1.4,
                    },
                    &brush,
                );
            }
        }
    }

    unsafe fn draw_single_devil_eye(
        &self,
        factory: &ID2D1Factory,
        rt: &ID2D1HwndRenderTarget,
        base_x: f32,
        base_y: f32,
        side: f32,
        alpha_mult: f32,
    ) {
        let shift_x = self.look_x * 4.2;
        let shift_y = self.look_y * 2.2;
        let ex = base_x + shift_x;
        let ey = base_y + shift_y;

        let eye_rx = 8.2;
        let eye_ry = (6.2 * self.openness).max(0.6);

        // --- LAYER 1: Neutral Smoky Slate-Wine Outer Glow ---
        let glow_stops = [
            D2D1_GRADIENT_STOP {
                position: 0.0,
                color: D2D1_COLOR_F {
                    r: 0.52,
                    g: 0.22,
                    b: 0.30,
                    a: 0.38 * self.aura_intensity * alpha_mult,
                },
            },
            D2D1_GRADIENT_STOP {
                position: 0.55,
                color: D2D1_COLOR_F {
                    r: 0.22,
                    g: 0.28,
                    b: 0.42,
                    a: 0.20 * self.aura_intensity * alpha_mult,
                },
            },
            D2D1_GRADIENT_STOP {
                position: 1.0,
                color: D2D1_COLOR_F {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.0,
                },
            },
        ];

        if let Ok(stop_collection) =
            rt.CreateGradientStopCollection(&glow_stops, D2D1_GAMMA_2_2, D2D1_EXTEND_MODE_CLAMP)
        {
            let props = D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES {
                center: D2D_POINT_2F { x: ex, y: ey },
                gradientOriginOffset: D2D_POINT_2F { x: 0.0, y: 0.0 },
                radiusX: eye_rx * 1.75,
                radiusY: (eye_ry * 1.85).max(3.0),
            };
            if let Ok(glow_brush) = rt.CreateRadialGradientBrush(&props, None, &stop_collection) {
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F { x: ex, y: ey },
                        radiusX: eye_rx * 1.75,
                        radiusY: (eye_ry * 1.85).max(3.0),
                    },
                    &glow_brush,
                );
            }
        }

        // Saat berkedip, tampilkan garis tipis Silver-Rose yang elegan
        if self.openness < 0.16 {
            let slit_color = D2D1_COLOR_F {
                r: 0.68,
                g: 0.32,
                b: 0.40,
                a: 0.85 * alpha_mult,
            };
            if let Ok(brush) = rt.CreateSolidColorBrush(&slit_color, None) {
                let rect = D2D1_ROUNDED_RECT {
                    rect: D2D_RECT_F {
                        left: ex - eye_rx,
                        top: ey - 0.8,
                        right: ex + eye_rx,
                        bottom: ey + 0.8,
                    },
                    radiusX: 0.8,
                    radiusY: 0.8,
                };
                rt.FillRoundedRectangle(&rect, &brush);
            }
            return;
        }

        // --- LAYER 2: Muted Wine-Crimson Iris (Kalem & Tidak Mencolok) ---
        let iris_rx = 6.4;
        let iris_ry = (5.6 * self.openness).max(1.2);

        let iris_stops = [
            D2D1_GRADIENT_STOP {
                position: 0.0,
                color: D2D1_COLOR_F {
                    r: 0.82,
                    g: 0.32,
                    b: 0.38,
                    a: alpha_mult,
                },
            },
            D2D1_GRADIENT_STOP {
                position: 0.68,
                color: D2D1_COLOR_F {
                    r: 0.62,
                    g: 0.16,
                    b: 0.24,
                    a: alpha_mult,
                },
            },
            D2D1_GRADIENT_STOP {
                position: 1.0,
                color: D2D1_COLOR_F {
                    r: 0.24,
                    g: 0.08,
                    b: 0.14,
                    a: alpha_mult,
                },
            },
        ];

        if let Ok(stop_collection) =
            rt.CreateGradientStopCollection(&iris_stops, D2D1_GAMMA_2_2, D2D1_EXTEND_MODE_CLAMP)
        {
            let props = D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES {
                center: D2D_POINT_2F {
                    x: ex,
                    y: ey + iris_ry * 0.15,
                },
                gradientOriginOffset: D2D_POINT_2F { x: 0.0, y: 0.0 },
                radiusX: iris_rx,
                radiusY: iris_ry,
            };
            if let Ok(iris_brush) = rt.CreateRadialGradientBrush(&props, None, &stop_collection) {
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F { x: ex, y: ey },
                        radiusX: iris_rx,
                        radiusY: iris_ry,
                    },
                    &iris_brush,
                );
            }
        }

        // --- LAYER 3: Pupil Tanda Plus ('+') Khas Asta Devil Union Mode ---
        let px = base_x + shift_x * 1.18;
        let py = base_y + shift_y * 1.18;
        let arm_h = 4.6 * self.pupil_scale;
        let arm_v = (4.4 * self.pupil_scale * self.openness).max(1.0);
        let half_thick = 1.15 * self.pupil_scale;

        // Cincin tepi pupil '+' berwarna Cool Silver-Rose tipis agar bentuk '+' terbaca jelas
        if let Ok(inner_ring_geom) = self.create_tapered_plus_geometry(
            factory,
            px,
            py,
            arm_h * 1.14,
            arm_v * 1.14,
            half_thick * 1.22,
        ) {
            let ring_color = D2D1_COLOR_F {
                r: 0.85,
                g: 0.65,
                b: 0.72,
                a: 0.55 * alpha_mult,
            };
            if let Ok(ring_brush) = rt.CreateSolidColorBrush(&ring_color, None) {
                rt.FillGeometry(&inner_ring_geom, &ring_brush, None);
            }
        }

        // Inti pupil '+' Obsidian Black
        if let Ok(plus_geom) =
            self.create_tapered_plus_geometry(factory, px, py, arm_h, arm_v, half_thick)
        {
            let pupil_color = D2D1_COLOR_F {
                r: 0.04,
                g: 0.05,
                b: 0.08,
                a: alpha_mult,
            };
            if let Ok(pupil_brush) = rt.CreateSolidColorBrush(&pupil_color, None) {
                rt.FillGeometry(&plus_geom, &pupil_brush, None);
            }
        }

        // --- LAYER 4: Upper Eyelid Cut (Menyatu dengan warna Obsidian Pill #0B0E14) ---
        self.draw_sharp_upper_eyelid_mask(
            factory,
            rt,
            ex,
            ey,
            iris_rx + 1.5,
            iris_ry,
            side,
            alpha_mult,
        );

        // --- LAYER 5: Cool Ice-Blue / Silver Specular Glint ---
        let glint_color = D2D1_COLOR_F {
            r: 0.88,
            g: 0.92,
            b: 0.98,
            a: 0.72 * alpha_mult,
        };
        if let Ok(glint_brush) = rt.CreateSolidColorBrush(&glint_color, None) {
            rt.FillEllipse(
                &D2D1_ELLIPSE {
                    point: D2D_POINT_2F {
                        x: px - side * 2.1,
                        y: py - iris_ry * 0.35,
                    },
                    radiusX: 1.05,
                    radiusY: 0.75 * self.openness,
                },
                &glint_brush,
            );
        }
    }

    unsafe fn create_tapered_plus_geometry(
        &self,
        factory: &ID2D1Factory,
        cx: f32,
        cy: f32,
        arm_h: f32,
        arm_v: f32,
        half_t: f32,
    ) -> windows::core::Result<ID2D1PathGeometry> {
        let geom = factory.CreatePathGeometry()?;
        let sink = geom.Open()?;
        sink.BeginFigure(
            D2D_POINT_2F {
                x: cx,
                y: cy - arm_v,
            },
            D2D1_FIGURE_BEGIN_FILLED,
        );

        let tip_t = half_t * 0.32;
        let pts = [
            D2D_POINT_2F { x: cx + tip_t, y: cy - arm_v * 0.65 },
            D2D_POINT_2F { x: cx + half_t, y: cy - half_t },
            D2D_POINT_2F { x: cx + arm_h * 0.65, y: cy - tip_t },
            D2D_POINT_2F { x: cx + arm_h, y: cy },
            D2D_POINT_2F { x: cx + arm_h * 0.65, y: cy + tip_t },
            D2D_POINT_2F { x: cx + half_t, y: cy + half_t },
            D2D_POINT_2F { x: cx + tip_t, y: cy + arm_v * 0.65 },
            D2D_POINT_2F { x: cx, y: cy + arm_v },
            D2D_POINT_2F { x: cx - tip_t, y: cy + arm_v * 0.65 },
            D2D_POINT_2F { x: cx - half_t, y: cy + half_t },
            D2D_POINT_2F { x: cx - arm_h * 0.65, y: cy + tip_t },
            D2D_POINT_2F { x: cx - arm_h, y: cy },
            D2D_POINT_2F { x: cx - arm_h * 0.65, y: cy - tip_t },
            D2D_POINT_2F { x: cx - half_t, y: cy - half_t },
            D2D_POINT_2F { x: cx - tip_t, y: cy - arm_v * 0.65 },
        ];

        sink.AddLines(&pts);
        sink.EndFigure(D2D1_FIGURE_END_CLOSED);
        sink.Close()?;
        Ok(geom)
    }

    unsafe fn draw_sharp_upper_eyelid_mask(
        &self,
        factory: &ID2D1Factory,
        rt: &ID2D1HwndRenderTarget,
        ex: f32,
        ey: f32,
        rx: f32,
        ry: f32,
        side: f32,
        alpha_mult: f32,
    ) {
        let outer_x = ex + side * rx;
        let inner_x = ex - side * rx;
        let outer_y = ey - ry * 0.92;
        let inner_y = ey - ry * 0.42;

        if let Ok(geom) = factory.CreatePathGeometry() {
            if let Ok(sink) = geom.Open() {
                sink.BeginFigure(
                    D2D_POINT_2F { x: outer_x, y: outer_y },
                    D2D1_FIGURE_BEGIN_FILLED,
                );
                sink.AddLines(&[
                    D2D_POINT_2F { x: inner_x, y: inner_y },
                    D2D_POINT_2F { x: inner_x, y: ey - ry - 4.0 },
                    D2D_POINT_2F { x: outer_x, y: ey - ry - 4.0 },
                ]);
                sink.EndFigure(D2D1_FIGURE_END_CLOSED);
                let _ = sink.Close();

                // Warna sama persis dengan Obsidian Slate Pill (#0B0E14)
                let mask_color = D2D1_COLOR_F {
                    r: 0.043,
                    g: 0.055,
                    b: 0.078,
                    a: alpha_mult,
                };
                if let Ok(brush) = rt.CreateSolidColorBrush(&mask_color, None) {
                    rt.FillGeometry(&geom, &brush, None);
                }
            }
        }
    }
}
