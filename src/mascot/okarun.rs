// src/mascot/okarun.rs
// Fase 5: Okarun (Turbo Granny Mode).
// Gaya dikunci ke Asta Devil Union: siluet rambut api putih-perak dengan aksen crimson,
// kacamata bulat tipis, garis yokai vertikal, mata kapsul merah-oranye, speed dash di kanan.
use super::core::*;
use windows::Win32::Graphics::Direct2D::Common::{D2D1_COLOR_F, D2D_POINT_2F, D2D_RECT_F};
use windows::Win32::Graphics::Direct2D::{ID2D1Factory, ID2D1HwndRenderTarget, D2D1_ELLIPSE};

pub struct OkarunTurbo;

impl OkarunTurbo {
    pub fn new() -> Self {
        Self
    }

    /// Pulsa speed-dash Turbo Granny berulang setiap ~2.8 detik
    fn dash_pulse(t: f32) -> f32 {
        let cycle = t % 2.8;
        if cycle < 0.35 {
            smoothstep(0.0, 0.35, cycle)
        } else if cycle < 1.15 {
            1.0 - smoothstep(0.35, 1.15, cycle)
        } else {
            0.0
        }
    }
}

impl Default for OkarunTurbo {
    fn default() -> Self {
        Self::new()
    }
}

impl IdleCharacter for OkarunTurbo {
    fn id(&self) -> &'static str {
        "okarun_turbo_granny"
    }

    fn duration_secs(&self) -> f32 {
        10.0
    }

    fn override_look_target(&self, ctx: &MascotContext) -> Option<(f32, f32)> {
        let dash = Self::dash_pulse(ctx.local_time);
        if dash > 0.1 {
            Some((2.4 * dash, -0.2 * dash))
        } else {
            None
        }
    }

    unsafe fn draw_aura(
        &self,
        rt: &ID2D1HwndRenderTarget,
        factory: &ID2D1Factory,
        pill_rect: D2D_RECT_F,
        alpha: f32,
        ctx: &MascotContext,
    ) {
        let dash = Self::dash_pulse(ctx.local_time);
        let pw = pill_rect.right - pill_rect.left;
        let ph = pill_rect.bottom - pill_rect.top;
        let cx = (pill_rect.left + pill_rect.right) * 0.5;

        // Crimson-red & silver-white electric speed glow
        draw_soft_radial_glow(
            rt,
            D2D_POINT_2F {
                x: cx - pw * 0.10,
                y: pill_rect.bottom - 1.0,
            },
            pw * 0.44,
            ph * 0.70,
            0.94,
            0.16,
            0.22,
            (0.28 + 0.24 * dash) * alpha,
        );
        draw_soft_radial_glow(
            rt,
            D2D_POINT_2F {
                x: cx + pw * 0.20,
                y: pill_rect.bottom - 2.0,
            },
            pw * 0.30,
            ph * 0.58,
            0.94,
            0.92,
            0.96,
            (0.14 + 0.22 * dash) * alpha,
        );

        // Horizontal speed-lines melesat cepat di background
        let stroke = create_round_stroke(factory);
        if let (Ok(red_b), Ok(white_b)) = (
            rt.CreateSolidColorBrush(
                &D2D1_COLOR_F {
                    r: 0.96,
                    g: 0.22,
                    b: 0.28,
                    a: (0.25 + 0.55 * dash) * alpha,
                },
                None,
            ),
            rt.CreateSolidColorBrush(
                &D2D1_COLOR_F {
                    r: 0.98,
                    g: 0.96,
                    b: 0.98,
                    a: (0.20 + 0.50 * dash) * alpha,
                },
                None,
            ),
        ) {
            for i in 0..4 {
                let fi = i as f32;
                let progress = (ctx.phase_time * (2.6 + fi * 0.5) + fi * 0.31) % 1.0;
                let lx = pill_rect.left + progress * (pw - 24.0);
                let ly = pill_rect.top + 6.0 + fi * (ph * 0.21);
                let len = 14.0 + dash * 16.0;
                let brush = if i % 2 == 0 { &red_b } else { &white_b };
                rt.DrawLine(
                    D2D_POINT_2F { x: lx, y: ly },
                    D2D_POINT_2F {
                        x: lx + len,
                        y: ly,
                    },
                    brush,
                    1.15,
                    stroke.as_ref(),
                );
            }
        }
    }

    unsafe fn draw_character(
        &self,
        rt: &ID2D1HwndRenderTarget,
        factory: &ID2D1Factory,
        cx: f32,
        cy: f32,
        alpha: f32,
        ctx: &MascotContext,
    ) {
        let dash = Self::dash_pulse(ctx.local_time);
        let face_cx = cx - 4.0;
        let eye_y = cy + 2.2;

        // 1. Rambut api putih-perak dengan aksen merah di bawahnya
        fill_polygon(
            rt,
            factory,
            &[
                D2D_POINT_2F {
                    x: face_cx - 18.0,
                    y: cy - 1.0,
                },
                D2D_POINT_2F {
                    x: face_cx - 16.5,
                    y: cy - 12.5,
                },
                D2D_POINT_2F {
                    x: face_cx - 7.0,
                    y: cy - 10.0,
                },
                D2D_POINT_2F {
                    x: face_cx,
                    y: cy - 15.5,
                },
                D2D_POINT_2F {
                    x: face_cx + 7.0,
                    y: cy - 10.0,
                },
                D2D_POINT_2F {
                    x: face_cx + 16.5,
                    y: cy - 12.5,
                },
                D2D_POINT_2F {
                    x: face_cx + 18.0,
                    y: cy - 1.0,
                },
            ],
            D2D1_COLOR_F {
                r: 0.86,
                g: 0.20,
                b: 0.24,
                a: alpha,
            },
        );
        fill_polygon(
            rt,
            factory,
            &[
                D2D_POINT_2F {
                    x: face_cx - 16.5,
                    y: cy - 3.5,
                },
                D2D_POINT_2F {
                    x: face_cx - 14.5,
                    y: cy - 12.0,
                },
                D2D_POINT_2F {
                    x: face_cx - 8.0,
                    y: cy - 8.5,
                },
                D2D_POINT_2F {
                    x: face_cx - 1.5,
                    y: cy - 14.5,
                },
                D2D_POINT_2F {
                    x: face_cx + 3.5,
                    y: cy - 9.0,
                },
                D2D_POINT_2F {
                    x: face_cx + 13.5,
                    y: cy - 12.0,
                },
                D2D_POINT_2F {
                    x: face_cx + 16.5,
                    y: cy - 3.5,
                },
                D2D_POINT_2F {
                    x: face_cx + 8.0,
                    y: cy - 6.0,
                },
                D2D_POINT_2F {
                    x: face_cx,
                    y: cy - 3.0,
                },
                D2D_POINT_2F {
                    x: face_cx - 8.0,
                    y: cy - 6.0,
                },
            ],
            D2D1_COLOR_F {
                r: 0.94,
                g: 0.92,
                b: 0.90,
                a: alpha,
            },
        );

        // 2. Garis yokai vertikal merah di atas & bawah mata
        let stroke = create_round_stroke(factory);
        if let Ok(mark_b) = rt.CreateSolidColorBrush(
            &D2D1_COLOR_F {
                r: 0.92,
                g: 0.22,
                b: 0.26,
                a: alpha * 0.9,
            },
            None,
        ) {
            for side in [-1.0_f32, 1.0_f32] {
                let mx = face_cx + side * 9.0;
                rt.DrawLine(
                    D2D_POINT_2F {
                        x: mx,
                        y: eye_y - 8.5,
                    },
                    D2D_POINT_2F {
                        x: mx,
                        y: eye_y + 9.0,
                    },
                    &mark_b,
                    2.0,
                    stroke.as_ref(),
                );
            }
        }

        // 3. Mata kapsul merah-oranye menyala
        draw_capsule_eyes(
            rt,
            factory,
            face_cx,
            eye_y,
            9.0,
            D2D1_COLOR_F {
                r: 1.0,
                g: 0.22,
                b: 0.18,
                a: 1.0,
            },
            D2D1_COLOR_F {
                r: 1.0,
                g: 0.80,
                b: 0.68,
                a: 1.0,
            },
            (1.0, 0.22, 0.18),
            0.35 + 0.55 * dash,
            alpha,
            ctx,
        );

        // 4. Kacamata bulat tipis silver
        if let Ok(glass_b) = rt.CreateSolidColorBrush(
            &D2D1_COLOR_F {
                r: 0.72,
                g: 0.76,
                b: 0.82,
                a: alpha * 0.92,
            },
            None,
        ) {
            for side in [-1.0_f32, 1.0_f32] {
                let gx = face_cx + side * 9.0 + ctx.look_x * 0.3;
                rt.DrawEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F { x: gx, y: eye_y },
                        radiusX: 6.4,
                        radiusY: 6.4,
                    },
                    &glass_b,
                    1.25,
                    stroke.as_ref(),
                );
            }
            rt.DrawLine(
                D2D_POINT_2F {
                    x: face_cx - 2.6 + ctx.look_x * 0.3,
                    y: eye_y,
                },
                D2D_POINT_2F {
                    x: face_cx + 2.6 + ctx.look_x * 0.3,
                    y: eye_y,
                },
                &glass_b,
                1.25,
                stroke.as_ref(),
            );
        }

        // 5. Gestur kanan: speed slash merah-putih
        let sx = cx + 36.0 + dash * 6.0;
        let sy = cy + 1.5;
        draw_soft_radial_glow(
            rt,
            D2D_POINT_2F { x: sx, y: sy },
            14.0,
            8.0,
            0.96,
            0.18,
            0.24,
            (0.28 + 0.42 * dash) * alpha,
        );
        draw_lightning_bolt(
            rt,
            factory,
            &[
                D2D_POINT_2F {
                    x: sx - 12.0,
                    y: sy + 3.0,
                },
                D2D_POINT_2F {
                    x: sx - 2.0,
                    y: sy - 2.0,
                },
                D2D_POINT_2F {
                    x: sx + 4.0,
                    y: sy + 1.0,
                },
                D2D_POINT_2F {
                    x: sx + 13.0,
                    y: sy - 4.0,
                },
            ],
            D2D1_COLOR_F {
                r: 0.95,
                g: 0.18,
                b: 0.24,
                a: (0.40 + 0.50 * dash) * alpha,
            },
            D2D1_COLOR_F {
                r: 1.0,
                g: 0.96,
                b: 0.96,
                a: (0.55 + 0.45 * dash) * alpha,
            },
            1.35,
        );
    }
}
