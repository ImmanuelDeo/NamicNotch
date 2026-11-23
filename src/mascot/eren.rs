// src/mascot/eren.rs
// Fase 4: Eren Yeager → Attack Titan — petir emas transformasi, titan marks, uap.
use super::core::*;
use windows::Win32::Graphics::Direct2D::Common::{D2D1_COLOR_F, D2D_POINT_2F, D2D_RECT_F};
use windows::Win32::Graphics::Direct2D::{ID2D1Factory, ID2D1HwndRenderTarget};

pub struct ErenAttackTitan;

impl ErenAttackTitan {
    pub fn new() -> Self {
        Self
    }

    /// 0.0 = Eren manusia, 1.0 = Attack Titan
    fn titan_factor(t: f32) -> f32 {
        smoothstep(2.8, 3.8, t)
    }

    /// Puncak kilatan petir emas saat transformasi
    fn lightning_flash(t: f32) -> f32 {
        if t < 2.7 || t > 4.6 {
            0.0
        } else if t < 3.2 {
            smoothstep(2.7, 3.2, t)
        } else {
            1.0 - smoothstep(3.2, 4.6, t)
        }
    }
}

impl Default for ErenAttackTitan {
    fn default() -> Self {
        Self::new()
    }
}

impl IdleCharacter for ErenAttackTitan {
    fn id(&self) -> &'static str {
        "eren_attack_titan"
    }

    fn duration_secs(&self) -> f32 {
        11.0
    }

    fn override_look_target(&self, ctx: &MascotContext) -> Option<(f32, f32)> {
        let flash = Self::lightning_flash(ctx.local_time);
        if flash > 0.1 {
            Some((1.8 * flash, -0.4 * flash))
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
        let titan = Self::titan_factor(ctx.local_time);
        let flash = Self::lightning_flash(ctx.local_time);
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
            lerp(0.18, 0.96, flash.max(titan * 0.35)),
            lerp(0.84, 0.80, flash),
            lerp(0.52, 0.24, flash),
            (0.24 + 0.22 * titan + 0.32 * flash) * alpha,
        );

        let periodic_arc = if titan > 0.5 {
            ((ctx.phase_time * 9.0).sin() - 0.55).max(0.0) * 1.4
        } else {
            0.0
        };
        let bolt_a = flash.max(periodic_arc).clamp(0.0, 1.0) * alpha;

        if bolt_a > 0.05 {
            let glow_col = D2D1_COLOR_F {
                r: 1.0,
                g: 0.78,
                b: 0.14,
                a: bolt_a * 0.42,
            };
            let core_col = D2D1_COLOR_F {
                r: 1.0,
                g: 0.98,
                b: 0.76,
                a: bolt_a,
            };
            let rx = cx + pw * 0.24;
            let ry = pill_rect.top + 3.0;

            draw_lightning_bolt(
                rt,
                factory,
                &[
                    D2D_POINT_2F {
                        x: rx + 12.0,
                        y: ry,
                    },
                    D2D_POINT_2F {
                        x: rx + 3.0,
                        y: ry + 8.0,
                    },
                    D2D_POINT_2F {
                        x: rx + 8.5,
                        y: ry + 12.0,
                    },
                    D2D_POINT_2F {
                        x: rx - 4.0,
                        y: ry + 20.0,
                    },
                    D2D_POINT_2F {
                        x: rx - 14.0,
                        y: pill_rect.bottom - 2.0,
                    },
                ],
                glow_col,
                core_col,
                1.45,
            );
        }

        // Uap titan mengepul setelah transformasi
        if titan > 0.2 {
            let steam_base = titan * alpha;
            for i in 0..6 {
                let fi = i as f32;
                let life = (ctx.phase_time * (0.45 + fi * 0.08) + fi * 0.21) % 1.0;
                let sx = cx + pw * (0.08 + 0.34 * ((fi * 0.23) % 1.0)) + life * 10.0;
                let sy = pill_rect.bottom - 3.0 - life * (ph * 0.75);
                let sa = (1.0 - life) * life * 4.0 * 0.22 * steam_base;
                draw_soft_circle(
                    rt,
                    D2D_POINT_2F { x: sx, y: sy },
                    8.0 + life * 6.0,
                    5.5 + life * 4.0,
                    D2D1_COLOR_F {
                        r: 0.86,
                        g: 0.92,
                        b: 0.90,
                        a: sa,
                    },
                );
            }
        }

        // ── Ambience: berkas sinar, halo, bara, refleksi dasar ──────────────
        draw_back_rays(
            rt,
            factory,
            cx,
            pill_rect.bottom - 1.0,
            8,
            0.95,
            ph * 1.20,
            ctx.phase_time * 0.5,
            D2D1_COLOR_F {
                r: 1.0,
                g: 0.72,
                b: 0.20,
                a: (0.075 + 0.05 * flash + 0.03 * titan) * alpha,
            },
        );
        draw_soft_radial_glow(
            rt,
            D2D_POINT_2F {
                x: cx,
                y: pill_rect.top + ph * 0.60,
            },
            pw * 0.18,
            ph * 0.58,
            lerp(0.55, 1.0, flash),
            lerp(0.85, 0.95, flash),
            lerp(0.45, 0.55, flash),
            (0.14 + 0.10 * titan) * alpha,
        );
        draw_rising_particles(
            rt,
            cx,
            pill_rect.bottom - 1.0,
            pw * 0.38,
            ph * 0.9,
            ctx.phase_time,
            9,
            D2D1_COLOR_F {
                r: 1.0,
                g: 0.86,
                b: 0.42,
                a: (0.28 + 0.10 * titan) * alpha,
            },
            0.95,
        );
        draw_soft_radial_glow(
            rt,
            D2D_POINT_2F {
                x: cx,
                y: pill_rect.bottom - 0.5,
            },
            pw * 0.42,
            2.2,
            1.0,
            0.86,
            0.48,
            (0.26 + 0.10 * flash) * alpha,
        );
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
        let titan = Self::titan_factor(ctx.local_time);
        let flash = Self::lightning_flash(ctx.local_time);
        let face_cx = cx;
        let eye_y = cy + 2.2;

        let hair_grow = titan * 2.4;
        let hair_col = D2D1_COLOR_F {
            r: 0.22,
            g: 0.15,
            b: 0.12,
            a: alpha,
        };
        fill_polygon(
            rt,
            factory,
            &[
                D2D_POINT_2F {
                    x: face_cx - 16.5 - hair_grow,
                    y: cy + 5.0 + hair_grow,
                },
                D2D_POINT_2F {
                    x: face_cx - 17.5,
                    y: cy - 6.0,
                },
                D2D_POINT_2F {
                    x: face_cx - 11.0,
                    y: cy - 13.0,
                },
                D2D_POINT_2F {
                    x: face_cx,
                    y: cy - 14.0,
                },
                D2D_POINT_2F {
                    x: face_cx + 11.0,
                    y: cy - 13.0,
                },
                D2D_POINT_2F {
                    x: face_cx + 17.5,
                    y: cy - 6.0,
                },
                D2D_POINT_2F {
                    x: face_cx + 16.5 + hair_grow,
                    y: cy + 5.0 + hair_grow,
                },
                D2D_POINT_2F {
                    x: face_cx + 12.0,
                    y: cy - 3.5,
                },
                D2D_POINT_2F {
                    x: face_cx,
                    y: cy - 6.5,
                },
                D2D_POINT_2F {
                    x: face_cx - 12.0,
                    y: cy - 3.5,
                },
            ],
            hair_col,
        );

        let outer_col = lerp_color(
            D2D1_COLOR_F {
                r: 0.16,
                g: 0.84,
                b: 0.56,
                a: 1.0,
            },
            D2D1_COLOR_F {
                r: 0.14,
                g: 1.0,
                b: 0.62,
                a: 1.0,
            },
            titan,
        );
        let core_col = lerp_color(
            D2D1_COLOR_F {
                r: 0.78,
                g: 1.0,
                b: 0.88,
                a: 1.0,
            },
            D2D1_COLOR_F {
                r: 0.90,
                g: 1.0,
                b: 0.94,
                a: 1.0,
            },
            titan,
        );

        draw_capsule_eyes(
            rt,
            factory,
            face_cx,
            eye_y,
            9.0,
            outer_col,
            core_col,
            (0.16, 1.0, 0.62),
            titan * 0.72 + flash * 0.2,
            alpha,
            ctx,
        );

        // Titan marks di bawah kedua mata
        if titan > 0.15 {
            let ma = smoothstep(0.15, 0.85, titan) * alpha;
            let stroke = create_round_stroke(factory);
            if let Ok(mb) = rt.CreateSolidColorBrush(
                &D2D1_COLOR_F {
                    r: 1.0,
                    g: 0.90,
                    b: 0.62,
                    a: ma * 0.85,
                },
                None,
            ) {
                for side in [-1.0_f32, 1.0_f32] {
                    let base_x = face_cx + side * 9.5;
                    let base_y = eye_y + 5.6;
                    for k in -1..=1 {
                        let kx = base_x + (k as f32) * 2.1;
                        rt.DrawLine(
                            D2D_POINT_2F { x: kx, y: base_y },
                            D2D_POINT_2F {
                                x: kx + side * 0.8,
                                y: base_y + 2.8,
                            },
                            &mb,
                            1.1,
                            stroke.as_ref(),
                        );
                    }
                }
            }
        }

        // ── Aksesori pendukung: pecahan hardening & rune Eldia melayang ─────
        let amber = D2D1_COLOR_F {
            r: 1.0,
            g: 0.82,
            b: 0.34,
            a: alpha,
        };
        for (i, (bx, by, sc)) in [
            (-20.0_f32, -7.0_f32, 1.0_f32),
            (19.0, -9.0, 0.8),
            (-23.0, 6.0, 0.6),
            (22.0, 5.0, 0.9),
        ]
        .iter()
        .enumerate()
        {
            let drift = (ctx.phase_time * (1.1 + i as f32 * 0.17) + i as f32).sin();
            let px = face_cx + bx + drift * 1.4;
            let py = eye_y + by + drift * 1.8;
            let w = 2.2 * sc;
            let h = 3.4 * sc;
            fill_polygon(
                rt,
                factory,
                &[
                    D2D_POINT_2F { x: px, y: py - h },
                    D2D_POINT_2F { x: px + w, y: py },
                    D2D_POINT_2F { x: px, y: py + h },
                    D2D_POINT_2F { x: px - w, y: py },
                ],
                D2D1_COLOR_F {
                    a: (0.35 + 0.25 * drift.abs()) * alpha,
                    ..amber
                },
            );
        }
        // Busur rune Eldia di belakang kepala.
        let arc_r = 26.0;
        let arc: Vec<D2D_POINT_2F> = (0..9)
            .map(|k| {
                let a = std::f32::consts::PI * (0.18 + 0.64 * k as f32 / 8.0);
                D2D_POINT_2F {
                    x: face_cx - a.cos() * arc_r,
                    y: eye_y - 2.0 - a.sin() * arc_r * 0.55,
                }
            })
            .collect();
        draw_polyline(
            rt,
            factory,
            &arc,
            D2D1_COLOR_F {
                a: (0.22 + 0.12 * titan) * alpha,
                ..amber
            },
            1.0,
        );
    }
}
