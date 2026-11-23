// src/mascot/asta.rs
// Fase 1: Asta (Devil Union) — pedang melayang, tangan meraih & menarik, tatapan serius.
use super::core::*;
use windows::Win32::Graphics::Direct2D::Common::{D2D1_COLOR_F, D2D_POINT_2F, D2D_RECT_F};
use windows::Win32::Graphics::Direct2D::{ID2D1Factory, ID2D1HwndRenderTarget, D2D1_ROUNDED_RECT};

pub struct AstaDevilUnion;

impl AstaDevilUnion {
    pub fn new() -> Self {
        Self
    }

    /// (grab_reach, sword_pull) dalam durasi 10 detik.
    fn sword_choreography(t: f32) -> (f32, f32) {
        if t < 1.8 {
            (0.0, 0.0)
        } else if t < 3.6 {
            (smoothstep(1.8, 3.6, t), 0.0)
        } else if t < 7.2 {
            (1.0, smoothstep(3.6, 5.4, t))
        } else {
            let p = 1.0 - smoothstep(7.2, 9.2, t);
            (p, p)
        }
    }
}

impl Default for AstaDevilUnion {
    fn default() -> Self {
        Self::new()
    }
}

impl IdleCharacter for AstaDevilUnion {
    fn id(&self) -> &'static str {
        "asta_devil_union"
    }

    fn duration_secs(&self) -> f32 {
        10.0
    }

    fn override_look_target(&self, ctx: &MascotContext) -> Option<(f32, f32)> {
        let (grab, pull) = Self::sword_choreography(ctx.local_time);
        if grab > 0.1 {
            Some((2.8 * grab - 1.2 * pull, -0.2 * grab))
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
        let (_, pull) = Self::sword_choreography(ctx.local_time);
        let pw = pill_rect.right - pill_rect.left;
        let ph = pill_rect.bottom - pill_rect.top;
        let cx = (pill_rect.left + pill_rect.right) * 0.5;

        draw_soft_radial_glow(
            rt,
            D2D_POINT_2F {
                x: cx,
                y: pill_rect.bottom - 1.0,
            },
            pw * 0.46,
            ph * 0.72,
            0.88,
            0.10,
            0.22,
            (0.34 * alpha * (1.0 + pull * 0.45)).min(0.55),
        );

        let flash = ((ctx.phase_time * 12.0).sin() + (ctx.phase_time * 21.0).cos()) * 0.5;
        if flash > -0.1 || pull > 0.25 {
            let spark_a = (((flash + 0.25) * 0.85).max(pull * 0.85)).clamp(0.18, 0.9) * alpha;
            let glow_col = D2D1_COLOR_F {
                r: 0.94,
                g: 0.12,
                b: 0.22,
                a: spark_a * 0.35,
            };
            let core_col = D2D1_COLOR_F {
                r: 1.0,
                g: 0.45,
                b: 0.50,
                a: spark_a,
            };

            for (i, ax) in [-pw * 0.26, pw * 0.24].iter().enumerate() {
                let shift = (ctx.phase_time * 14.0 + i as f32 * 2.1).sin() * 3.5;
                let bx = cx + ax + shift;
                let by = pill_rect.top + 5.0;
                draw_lightning_bolt(
                    rt,
                    factory,
                    &[
                        D2D_POINT_2F {
                            x: bx - 5.0,
                            y: by + 2.0,
                        },
                        D2D_POINT_2F { x: bx, y: by + 8.0 },
                        D2D_POINT_2F {
                            x: bx - 3.5,
                            y: by + 12.0,
                        },
                        D2D_POINT_2F {
                            x: bx + 3.5,
                            y: by + 19.0,
                        },
                    ],
                    glow_col,
                    core_col,
                    1.15,
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
        let (grab, pull) = Self::sword_choreography(ctx.local_time);
        let face_cx = cx;
        let eye_y = cy + 2.2;

        // 1. Rambut hitam obsidian + 4 tanduk
        let hair_col = D2D1_COLOR_F {
            r: 0.08,
            g: 0.08,
            b: 0.11,
            a: alpha,
        };
        let horn_col = D2D1_COLOR_F {
            r: 0.04,
            g: 0.04,
            b: 0.06,
            a: alpha,
        };
        fill_polygon(
            rt,
            factory,
            &[
                D2D_POINT_2F {
                    x: face_cx - 17.0,
                    y: cy - 3.0,
                },
                D2D_POINT_2F {
                    x: face_cx - 14.0,
                    y: cy - 12.5,
                },
                D2D_POINT_2F {
                    x: face_cx - 7.0,
                    y: cy - 9.0,
                },
                D2D_POINT_2F {
                    x: face_cx,
                    y: cy - 14.5,
                },
                D2D_POINT_2F {
                    x: face_cx + 7.0,
                    y: cy - 9.0,
                },
                D2D_POINT_2F {
                    x: face_cx + 14.0,
                    y: cy - 12.5,
                },
                D2D_POINT_2F {
                    x: face_cx + 17.0,
                    y: cy - 3.0,
                },
            ],
            hair_col,
        );

        for side in [-1.0_f32, 1.0_f32] {
            fill_polygon(
                rt,
                factory,
                &[
                    D2D_POINT_2F {
                        x: face_cx + side * 12.0,
                        y: cy - 6.0,
                    },
                    D2D_POINT_2F {
                        x: face_cx + side * 18.5,
                        y: cy - 14.0,
                    },
                    D2D_POINT_2F {
                        x: face_cx + side * 15.5,
                        y: cy - 4.5,
                    },
                ],
                horn_col,
            );
            fill_polygon(
                rt,
                factory,
                &[
                    D2D_POINT_2F {
                        x: face_cx + side * 6.0,
                        y: cy - 8.5,
                    },
                    D2D_POINT_2F {
                        x: face_cx + side * 9.5,
                        y: cy - 14.5,
                    },
                    D2D_POINT_2F {
                        x: face_cx + side * 9.0,
                        y: cy - 7.5,
                    },
                ],
                horn_col,
            );
        }

        // 2. Headband crimson + salib tengah
        let crown_col = D2D1_COLOR_F {
            r: 0.92,
            g: 0.14,
            b: 0.28,
            a: alpha,
        };
        if let Ok(crown_b) = rt.CreateSolidColorBrush(&crown_col, None) {
            rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: D2D_RECT_F {
                        left: face_cx - 14.0,
                        top: cy - 8.2,
                        right: face_cx + 14.0,
                        bottom: cy - 5.2,
                    },
                    radiusX: 1.5,
                    radiusY: 1.5,
                },
                &crown_b,
            );
            fill_polygon(
                rt,
                factory,
                &[
                    D2D_POINT_2F {
                        x: face_cx,
                        y: cy - 11.2,
                    },
                    D2D_POINT_2F {
                        x: face_cx + 3.2,
                        y: cy - 6.7,
                    },
                    D2D_POINT_2F {
                        x: face_cx,
                        y: cy - 2.2,
                    },
                    D2D_POINT_2F {
                        x: face_cx - 3.2,
                        y: cy - 6.7,
                    },
                ],
                crown_col,
            );
        }

        // 3. Mata kapsul crimson menyala, menyipit saat meraih pedang
        draw_capsule_eyes(
            rt,
            factory,
            face_cx,
            eye_y,
            9.2,
            D2D1_COLOR_F {
                r: 1.0,
                g: 0.14,
                b: 0.26,
                a: 1.0,
            },
            D2D1_COLOR_F {
                r: 1.0,
                g: 0.72,
                b: 0.76,
                a: 1.0,
            },
            (1.0, 0.14, 0.26),
            grab,
            alpha,
            ctx,
        );

        // 4. Gestur kanan: pedang Demon-Slayer melayang + tangan meraih
        let float_y = (ctx.phase_time * 2.8).sin() * 2.2 * (1.0 - pull * 0.65);
        let sx = face_cx + 40.0 - pull * 10.0;
        let sy = cy + 0.5 + float_y;

        draw_soft_radial_glow(
            rt,
            D2D_POINT_2F { x: sx, y: sy },
            16.0,
            10.0,
            0.95,
            0.12,
            0.24,
            (0.28 + pull * 0.35) * alpha,
        );

        let blade_col = D2D1_COLOR_F {
            r: 0.14,
            g: 0.13,
            b: 0.17,
            a: alpha,
        };
        let edge_col = D2D1_COLOR_F {
            r: 0.85,
            g: 0.20,
            b: 0.30,
            a: (0.65 + pull * 0.35) * alpha,
        };
        fill_polygon(
            rt,
            factory,
            &[
                D2D_POINT_2F {
                    x: sx - 10.0,
                    y: sy + 6.0,
                },
                D2D_POINT_2F {
                    x: sx + 8.0,
                    y: sy - 9.0,
                },
                D2D_POINT_2F {
                    x: sx + 12.5,
                    y: sy - 8.5,
                },
                D2D_POINT_2F {
                    x: sx + 11.5,
                    y: sy - 4.5,
                },
                D2D_POINT_2F {
                    x: sx - 6.5,
                    y: sy + 9.0,
                },
            ],
            blade_col,
        );
        fill_polygon(
            rt,
            factory,
            &[
                D2D_POINT_2F {
                    x: sx - 12.5,
                    y: sy + 3.5,
                },
                D2D_POINT_2F {
                    x: sx - 5.0,
                    y: sy + 10.5,
                },
                D2D_POINT_2F {
                    x: sx - 7.0,
                    y: sy + 12.0,
                },
                D2D_POINT_2F {
                    x: sx - 14.0,
                    y: sy + 5.2,
                },
            ],
            edge_col,
        );

        if grab > 0.05 {
            let hx = lerp(face_cx + 22.0, sx - 10.5, grab);
            let hy = lerp(cy + 12.0, sy + 8.5, grab);
            draw_soft_circle(
                rt,
                D2D_POINT_2F { x: hx, y: hy },
                3.4,
                3.0,
                D2D1_COLOR_F {
                    r: 0.12,
                    g: 0.10,
                    b: 0.14,
                    a: alpha * grab,
                },
            );
            draw_soft_circle(
                rt,
                D2D_POINT_2F { x: hx, y: hy },
                4.8,
                4.2,
                D2D1_COLOR_F {
                    r: 0.95,
                    g: 0.18,
                    b: 0.30,
                    a: 0.25 * alpha * grab,
                },
            );
        }
    }
}
