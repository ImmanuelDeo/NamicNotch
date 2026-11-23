// src/mascot/bakugo.rs
// Fase 3: Katsuki Bakugo — topeng bergerigi, gauntlet, ledakan nitrogliserin.
use super::core::*;
use windows::Win32::Graphics::Direct2D::Common::{D2D1_COLOR_F, D2D_POINT_2F, D2D_RECT_F};
use windows::Win32::Graphics::Direct2D::{ID2D1Factory, ID2D1HwndRenderTarget};

pub struct KatsukiBakugo;

impl KatsukiBakugo {
    pub fn new() -> Self {
        Self
    }

    /// Pulsa ledakan berulang tiap ~2.6 detik
    fn blast_pulse(t: f32) -> f32 {
        let cycle = t % 2.6;
        if cycle < 0.25 {
            smoothstep(0.0, 0.25, cycle)
        } else if cycle < 0.95 {
            1.0 - smoothstep(0.25, 0.95, cycle)
        } else {
            0.0
        }
    }
}

impl Default for KatsukiBakugo {
    fn default() -> Self {
        Self::new()
    }
}

impl IdleCharacter for KatsukiBakugo {
    fn id(&self) -> &'static str {
        "katsuki_bakugo"
    }

    fn duration_secs(&self) -> f32 {
        10.0
    }

    fn override_look_target(&self, ctx: &MascotContext) -> Option<(f32, f32)> {
        let blast = Self::blast_pulse(ctx.local_time);
        if blast > 0.1 {
            Some((2.2 * blast, -0.3 * blast))
        } else {
            None
        }
    }

    unsafe fn draw_aura(
        &self,
        rt: &ID2D1HwndRenderTarget,
        _factory: &ID2D1Factory,
        pill_rect: D2D_RECT_F,
        alpha: f32,
        ctx: &MascotContext,
    ) {
        let blast = Self::blast_pulse(ctx.local_time);
        let pw = pill_rect.right - pill_rect.left;
        let ph = pill_rect.bottom - pill_rect.top;
        let cx = (pill_rect.left + pill_rect.right) * 0.5;

        draw_soft_radial_glow(
            rt,
            D2D_POINT_2F {
                x: cx,
                y: pill_rect.bottom - 1.0,
            },
            pw * 0.48,
            ph * 0.72,
            1.0,
            0.42,
            0.08,
            (0.30 + 0.24 * blast) * alpha,
        );

        // Bara api melayang naik
        for i in 0..7 {
            let fi = i as f32;
            let life = (ctx.phase_time * (0.9 + fi * 0.13) + fi * 0.37) % 1.0;
            let ex = pill_rect.left
                + pw * (0.14 + 0.72 * ((fi * 0.29 + 0.11) % 1.0))
                + (ctx.phase_time * 3.0 + fi).sin() * 3.5;
            let ey = pill_rect.bottom - 3.0 - life * (ph * 0.85);
            let ea = (1.0 - life) * alpha * (0.55 + 0.45 * blast);
            draw_soft_circle(
                rt,
                D2D_POINT_2F { x: ex, y: ey },
                1.4,
                1.4,
                D2D1_COLOR_F {
                    r: 1.0,
                    g: 0.72,
                    b: 0.22,
                    a: ea,
                },
            );
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
        let blast = Self::blast_pulse(ctx.local_time);
        let recoil = blast * 1.4;
        let face_cx = cx - recoil * 0.5;
        let eye_y = cy + 2.4;

        // Rambut spiky pirang abu-abu
        let blonde = D2D1_COLOR_F {
            r: 0.95,
            g: 0.88,
            b: 0.68,
            a: alpha,
        };
        fill_polygon(
            rt,
            factory,
            &[
                D2D_POINT_2F {
                    x: face_cx - 19.0,
                    y: cy - 3.0,
                },
                D2D_POINT_2F {
                    x: face_cx - 17.5,
                    y: cy - 12.0,
                },
                D2D_POINT_2F {
                    x: face_cx - 11.0,
                    y: cy - 9.0,
                },
                D2D_POINT_2F {
                    x: face_cx - 7.5,
                    y: cy - 14.5,
                },
                D2D_POINT_2F {
                    x: face_cx - 1.5,
                    y: cy - 9.5,
                },
                D2D_POINT_2F {
                    x: face_cx + 3.5,
                    y: cy - 14.5,
                },
                D2D_POINT_2F {
                    x: face_cx + 9.0,
                    y: cy - 9.5,
                },
                D2D_POINT_2F {
                    x: face_cx + 16.5,
                    y: cy - 12.5,
                },
                D2D_POINT_2F {
                    x: face_cx + 19.0,
                    y: cy - 3.0,
                },
            ],
            blonde,
        );

        // Flare oranye samping + band topeng hitam
        let mask_col = D2D1_COLOR_F {
            r: 0.06,
            g: 0.06,
            b: 0.08,
            a: alpha,
        };
        let flare_col = D2D1_COLOR_F {
            r: 1.0,
            g: 0.42,
            b: 0.12,
            a: alpha,
        };

        for side in [-1.0_f32, 1.0_f32] {
            fill_polygon(
                rt,
                factory,
                &[
                    D2D_POINT_2F {
                        x: face_cx + side * 13.0,
                        y: cy - 1.0,
                    },
                    D2D_POINT_2F {
                        x: face_cx + side * 20.5,
                        y: cy - 5.5,
                    },
                    D2D_POINT_2F {
                        x: face_cx + side * 16.5,
                        y: cy + 0.5,
                    },
                    D2D_POINT_2F {
                        x: face_cx + side * 20.0,
                        y: cy + 4.5,
                    },
                    D2D_POINT_2F {
                        x: face_cx + side * 13.0,
                        y: cy + 4.0,
                    },
                ],
                flare_col,
            );
        }

        fill_polygon(
            rt,
            factory,
            &[
                D2D_POINT_2F {
                    x: face_cx - 15.0,
                    y: cy - 3.0,
                },
                D2D_POINT_2F {
                    x: face_cx,
                    y: cy - 0.5,
                },
                D2D_POINT_2F {
                    x: face_cx + 15.0,
                    y: cy - 3.0,
                },
                D2D_POINT_2F {
                    x: face_cx + 14.0,
                    y: cy + 6.5,
                },
                D2D_POINT_2F {
                    x: face_cx,
                    y: cy + 4.5,
                },
                D2D_POINT_2F {
                    x: face_cx - 14.0,
                    y: cy + 6.5,
                },
            ],
            mask_col,
        );

        // Mata kapsul oranye-merah garang
        draw_capsule_eyes(
            rt,
            factory,
            face_cx,
            eye_y,
            9.0,
            D2D1_COLOR_F {
                r: 1.0,
                g: 0.34,
                b: 0.10,
                a: 1.0,
            },
            D2D1_COLOR_F {
                r: 1.0,
                g: 0.84,
                b: 0.48,
                a: 1.0,
            },
            (1.0, 0.38, 0.10),
            0.82 + 0.18 * blast,
            alpha,
            ctx,
        );

        // Gestur kanan: gauntlet hitam + ledakan
        let gx = cx + 38.0 - recoil;
        let gy = cy + 2.5;
        let burst_scale = 0.45 + 0.85 * blast;
        let bx = gx + 8.5;
        let by = gy - 2.5;

        draw_soft_radial_glow(
            rt,
            D2D_POINT_2F { x: bx, y: by },
            15.0 * burst_scale,
            13.0 * burst_scale,
            1.0,
            0.54,
            0.12,
            (0.35 + 0.55 * blast) * alpha,
        );

        let mut star_pts = [D2D_POINT_2F { x: 0.0, y: 0.0 }; 16];
        for (i, pt) in star_pts.iter_mut().enumerate() {
            let angle = (i as f32) * std::f32::consts::PI / 8.0 + ctx.phase_time * 0.6;
            let rad = if i % 2 == 0 {
                10.5 * burst_scale
            } else {
                4.2 * burst_scale
            };
            *pt = D2D_POINT_2F {
                x: bx + angle.cos() * rad,
                y: by + angle.sin() * rad,
            };
        }
        fill_polygon(
            rt,
            factory,
            &star_pts,
            D2D1_COLOR_F {
                r: 1.0,
                g: 0.64,
                b: 0.18,
                a: (0.45 + 0.55 * blast) * alpha,
            },
        );
        draw_soft_circle(
            rt,
            D2D_POINT_2F { x: bx, y: by },
            3.8 * burst_scale,
            3.8 * burst_scale,
            D2D1_COLOR_F {
                r: 1.0,
                g: 0.96,
                b: 0.82,
                a: (0.5 + 0.5 * blast) * alpha,
            },
        );

        fill_polygon(
            rt,
            factory,
            &[
                D2D_POINT_2F {
                    x: gx - 6.0,
                    y: gy + 7.0,
                },
                D2D_POINT_2F {
                    x: gx - 1.5,
                    y: gy + 1.0,
                },
                D2D_POINT_2F {
                    x: gx + 3.5,
                    y: gy - 3.5,
                },
                D2D_POINT_2F {
                    x: gx + 6.0,
                    y: gy + 0.5,
                },
                D2D_POINT_2F {
                    x: gx + 2.0,
                    y: gy + 4.5,
                },
                D2D_POINT_2F {
                    x: gx - 2.0,
                    y: gy + 9.0,
                },
            ],
            D2D1_COLOR_F {
                r: 0.07,
                g: 0.07,
                b: 0.09,
                a: alpha,
            },
        );
    }
}
