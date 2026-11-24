// src/mascot/eren.rs
// Fase 4: Eren Yeager (man-bun) -> Attack Titan.
// Patokan visual: referensi 4A/4B (bun di kanan-atas yang jelas terlihat, poni di tengah
// dahi; titan: rambut shaggy melebar, mulut bergerigi, garis gelap di pipi, petir + uap).
// Efek memakai kurva mulus (draw_soft_streak / draw_tapered_streak), bukan partikel kaku.
use super::core::*;
use windows::Win32::Graphics::Direct2D::Common::{D2D1_COLOR_F, D2D_POINT_2F, D2D_RECT_F};
use windows::Win32::Graphics::Direct2D::{
    ID2D1Factory, ID2D1HwndRenderTarget, D2D1_ELLIPSE, D2D1_ROUNDED_RECT,
};

const SCALE: f32 = 0.78;

pub struct ErenAttackTitan;

impl ErenAttackTitan {
    pub fn new() -> Self {
        Self
    }

    fn titan_factor(t: f32) -> f32 {
        smoothstep(2.8, 4.0, t)
    }

    fn lightning_flash(t: f32) -> f32 {
        if t < 2.8 || t > 4.8 {
            0.0
        } else if t < 3.3 {
            smoothstep(2.8, 3.3, t)
        } else {
            1.0 - smoothstep(3.3, 4.8, t)
        }
    }

    /// Petir susulan halus setelah transformasi (denyut lembut, bukan kedip kaku).
    fn lightning_idle(t: f32, titan: f32) -> f32 {
        if titan <= 0.5 || t < 4.8 {
            return 0.0;
        }
        let cycle = (t - 4.8) % 3.6;
        if cycle < 0.5 {
            smoothstep(0.0, 0.5, cycle) * 0.55
        } else if cycle < 1.6 {
            (1.0 - smoothstep(0.5, 1.6, cycle)) * 0.55
        } else {
            0.0
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
            Some((1.6 * flash, -0.35 * flash))
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
        let idle_bolt = Self::lightning_idle(ctx.local_time, titan);
        let pw = pill_rect.right - pill_rect.left;
        let ph = pill_rect.bottom - pill_rect.top;
        let cx = (pill_rect.left + pill_rect.right) * 0.5;
        let t = ctx.phase_time;

        // Aura dasar: teal tenang -> emerald-gold, dua lapis supaya lembut.
        draw_soft_radial_glow(
            rt,
            D2D_POINT_2F {
                x: cx,
                y: pill_rect.bottom - 1.0,
            },
            pw * 0.47,
            ph * 0.72,
            lerp(0.16, 0.90, titan.max(flash)),
            lerp(0.75, 0.78, titan),
            lerp(0.56, 0.32, titan),
            (0.24 + 0.16 * titan + 0.22 * flash) * alpha,
        );
        draw_soft_radial_glow(
            rt,
            D2D_POINT_2F {
                x: cx + pw * 0.16,
                y: pill_rect.bottom - 6.0,
            },
            pw * 0.30,
            ph * 0.54,
            1.0,
            0.95,
            0.84,
            (0.06 + 0.20 * titan) * alpha,
        );

        // Uap titan: pita mulus yang mengalir naik dan memudar (bukan titik-titik).
        if titan > 0.15 {
            for k in 0..4 {
                let fk = k as f32;
                let speed = 0.34 + fk * 0.05;
                let life = (t * speed + fk * 0.27) % 1.0;
                let a = (life * (1.0 - life) * 3.4).min(1.0) * titan * alpha * 0.42;
                let bx = cx - pw * 0.06 + pw * 0.30 * ((fk * 0.31) % 1.0);
                let by = pill_rect.bottom - 3.0;
                let bow = (life * 5.0 + fk).sin() * 5.0;
                draw_tapered_streak(
                    rt,
                    factory,
                    D2D_POINT_2F { x: bx, y: by },
                    D2D_POINT_2F {
                        x: bx + bow,
                        y: by - ph * (0.42 + 0.34 * life),
                    },
                    bow * 0.8,
                    2.6,
                    D2D1_COLOR_F {
                        r: 0.90,
                        g: 0.95,
                        b: 0.93,
                        a,
                    },
                );
            }
        }

        // Petir: kurva mulus berlapis + cabang, memudar halus.
        let bolt = (flash + idle_bolt).clamp(0.0, 1.0) * alpha;
        if bolt > 0.04 {
            let bx = cx + pw * 0.23;
            let top = pill_rect.top + 1.0;
            let bottom = pill_rect.bottom - 2.0;
            let sway = (t * 2.1).sin() * 2.6;
            let main = [
                D2D_POINT_2F {
                    x: bx + 9.0,
                    y: top,
                },
                D2D_POINT_2F {
                    x: bx + 2.0,
                    y: top + (bottom - top) * 0.26 + sway,
                },
                D2D_POINT_2F {
                    x: bx + 7.0,
                    y: top + (bottom - top) * 0.52 - sway * 0.6,
                },
                D2D_POINT_2F {
                    x: bx - 3.0,
                    y: top + (bottom - top) * 0.78,
                },
                D2D_POINT_2F {
                    x: bx - 11.0,
                    y: bottom,
                },
            ];
            let glow = D2D1_COLOR_F {
                r: 1.0,
                g: 0.90,
                b: 0.42,
                a: bolt * 0.9,
            };
            draw_soft_streak(rt, factory, &main, glow, 1.5);
            // Cabang kecil.
            let branch = [
                D2D_POINT_2F {
                    x: bx + 3.0,
                    y: top + (bottom - top) * 0.42,
                },
                D2D_POINT_2F {
                    x: bx + 12.0,
                    y: top + (bottom - top) * 0.52,
                },
                D2D_POINT_2F {
                    x: bx + 9.0,
                    y: top + (bottom - top) * 0.66,
                },
            ];
            draw_soft_streak(
                rt,
                factory,
                &branch,
                D2D1_COLOR_F {
                    r: 1.0,
                    g: 0.97,
                    b: 0.82,
                    a: bolt * 0.55,
                },
                0.9,
            );
        }

        // Mote ambient: melayang lambat dengan alfa sinusoidal (halus, tidak berkedip).
        for i in 0..5 {
            let fi = i as f32;
            let phase = (t * 0.22 + fi * 0.19) % 1.0;
            let a = (phase * std::f32::consts::PI).sin() * 0.22 * alpha;
            let mx = cx + (t * 0.5 + fi * 2.1).sin() * pw * 0.30;
            let my = pill_rect.bottom - 8.0 - phase * ph * 0.72;
            draw_soft_circle(
                rt,
                D2D_POINT_2F { x: mx, y: my },
                1.5,
                1.5,
                D2D1_COLOR_F {
                    r: 1.0,
                    g: lerp(0.88, 0.74, titan),
                    b: lerp(0.56, 0.30, titan),
                    a,
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
        let titan = Self::titan_factor(ctx.local_time);
        let s = SCALE;
        let base_cx = cx;
        let base_cy = cy + 2.0;
        let local = |pts: &[(f32, f32)]| -> Vec<D2D_POINT_2F> {
            pts.iter()
                .map(|&(x, y)| D2D_POINT_2F {
                    x: base_cx + x * s,
                    y: base_cy + y * s,
                })
                .collect()
        };
        let at = |x: f32, y: f32| D2D_POINT_2F {
            x: base_cx + x * s,
            y: base_cy + y * s,
        };
        let col = |r: f32, g: f32, b: f32| D2D1_COLOR_F { r, g, b, a: alpha };

        let black = col(0.018, 0.018, 0.024);
        let hair = col(0.21, 0.14, 0.11);
        let hair_hi = col(0.35, 0.25, 0.20);
        let hair_deep = col(0.11, 0.07, 0.06);
        let cloak = col(0.16, 0.26, 0.19);
        let cloak_hi = col(0.26, 0.38, 0.28);
        let teal = col(0.20, 0.86, 0.62);
        let teal_core = col(0.80, 1.00, 0.92);
        let teeth = col(0.90, 0.88, 0.82);
        let sway = (ctx.phase_time * 1.6).sin();

        // ── 0. Kerah cloak tipis (fase manusia minimal, titan lebih tegas) ─
        fill_polygon(
            rt,
            factory,
            &local(&[
                (-26.0, 13.5),
                (-21.0, 5.0),
                (-12.5, 7.4),
                (0.0, 9.4),
                (12.5, 7.4),
                (21.0, 5.0),
                (26.0, 13.5),
                (0.0, 19.0),
            ]),
            cloak,
        );
        draw_polyline(
            rt,
            factory,
            &local(&[(-21.0, 5.2), (-12.5, 7.6), (0.0, 9.6), (12.5, 7.6), (21.0, 5.2)]),
            D2D1_COLOR_F {
                a: 0.45 * alpha,
                ..cloak_hi
            },
            0.9,
        );

        // ── 1. Basis kepala ────────────────────────────────────────────────
        if let Ok(b) = rt.CreateSolidColorBrush(&black, None) {
            rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: D2D_RECT_F {
                        left: base_cx - 20.5 * s,
                        top: base_cy - 8.5 * s,
                        right: base_cx + 20.5 * s,
                        bottom: base_cy + 13.5 * s,
                    },
                    radiusX: 10.0 * s,
                    radiusY: 10.0 * s,
                },
                &b,
            );
        }
        draw_soft_radial_glow(rt, at(-19.0, 1.5), 9.0, 12.0, teal.r, teal.g, teal.b, 0.15 * alpha);
        draw_soft_radial_glow(rt, at(19.0, 1.5), 9.0, 12.0, teal.r, teal.g, teal.b, 0.15 * alpha);

        // ── 2. Rambut: swept-back rapi -> shaggy melebar ───────────────────
        let spread = titan * 3.6;
        let hair_pts = [
            (-21.5 - spread, -1.8),
            (-24.5 - spread, -7.4),
            (-19.0 - spread * 0.6, -8.6),
            (-22.5 - spread * 0.7, -14.6),
            (-14.0, -13.2),
            (-12.5, -19.4),
            (-4.5, -16.2),
            (0.0, -20.6),
            (5.5, -16.0),
            (13.5, -18.6),
            (14.5, -12.6),
            (21.5 + spread * 0.7, -14.0),
            (19.0 + spread * 0.6, -8.0),
            (24.5 + spread, -6.6),
            (21.0 + spread, -1.2),
            (16.0, 1.4),
            (8.0, -2.2),
            (0.0, 0.6),
            (-8.0, -2.2),
            (-16.5, 1.4),
        ];
        fill_polygon(rt, factory, &local(&hair_pts), hair);
        // Helai shaggy menjuntai di kedua sisi saat titan.
        if titan > 0.15 {
            for side in [-1.0_f32, 1.0_f32] {
                for k in 0..2 {
                    let fk = k as f32;
                    let x0 = side * (16.0 + fk * 3.0);
                    fill_polygon(
                        rt,
                        factory,
                        &local(&[
                            (x0, -6.0 - fk * 1.6),
                            (x0 + side * (6.0 + spread * 0.6), 1.5 + fk * 2.0 + sway * 0.4),
                            (x0 + side * (2.0 + spread * 0.4), 8.5 + fk * 1.5),
                            (x0 - side * 2.5, 2.0),
                        ]),
                        D2D1_COLOR_F {
                            a: alpha * titan,
                            ..hair
                        },
                    );
                }
            }
        }
        for chunk in [
            [(-20.0, -11.8), (-11.5, -9.8), (-16.5, -5.4)],
            [(-9.5, -15.8), (-2.0, -11.8), (-7.0, -6.8)],
            [(1.0, -17.4), (5.0, -11.6), (0.0, -7.2)],
            [(10.0, -16.0), (8.0, -7.8), (2.5, -4.6)],
            [(19.0, -12.6), (16.0, -5.8), (10.0, -4.2)],
        ] {
            fill_polygon(rt, factory, &local(&chunk), hair_hi);
        }
        draw_polyline(
            rt,
            factory,
            &local(&[
                (-21.5 - spread, -1.8),
                (-24.5 - spread, -7.4),
                (-19.0 - spread * 0.6, -8.6),
                (-22.5 - spread * 0.7, -14.6),
                (-14.0, -13.2),
                (-12.5, -19.4),
                (-4.5, -16.2),
                (0.0, -20.6),
                (5.5, -16.0),
                (13.5, -18.6),
                (14.5, -12.6),
                (21.5 + spread * 0.7, -14.0),
                (19.0 + spread * 0.6, -8.0),
                (24.5 + spread, -6.6),
                (21.0 + spread, -1.2),
            ]),
            D2D1_COLOR_F {
                a: 0.5 * alpha,
                ..hair_hi
            },
            0.9,
        );

        // ── 3. Man-bun jelas di kanan-atas + poni tengah dahi ──────────────
        let human = (1.0 - titan).max(0.0);
        if human > 0.03 {
            let bun = at(10.5, -17.5);
            let br = 5.0 * s * (0.72 + 0.28 * human);
            if let Ok(b) = rt.CreateSolidColorBrush(
                &D2D1_COLOR_F {
                    a: alpha * human,
                    ..hair_deep
                },
                None,
            ) {
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: bun,
                        radiusX: br,
                        radiusY: br * 0.88,
                    },
                    &b,
                );
            }
            // Highlight bun supaya terbaca sebagai ikatan rambut.
            if let Ok(b) = rt.CreateSolidColorBrush(
                &D2D1_COLOR_F {
                    a: 0.75 * alpha * human,
                    ..hair_hi
                },
                None,
            ) {
                rt.DrawEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F {
                            x: bun.x - br * 0.18,
                            y: bun.y - br * 0.22,
                        },
                        radiusX: br * 0.60,
                        radiusY: br * 0.52,
                    },
                    &b,
                    0.9,
                    None,
                );
            }
            // Ikatan rambut.
            draw_soft_streak(
                rt,
                factory,
                &[
                    D2D_POINT_2F {
                        x: bun.x - br * 0.9,
                        y: bun.y + br * 0.55,
                    },
                    D2D_POINT_2F {
                        x: bun.x + br * 0.1,
                        y: bun.y + br * 1.05,
                    },
                    D2D_POINT_2F {
                        x: bun.x + br * 0.95,
                        y: bun.y + br * 0.5,
                    },
                ],
                D2D1_COLOR_F {
                    a: 0.85 * alpha * human,
                    ..hair_hi
                },
                1.0,
            );

            // Poni: beberapa helai jatuh di dahi, satu di antara alis.
            for strand in [
                [(-10.0, -12.4), (-7.0, -8.0), (-9.4, -3.6)],
                [(-2.0, -14.2), (0.4, -9.0), (-0.6, -4.2)],
                [(6.0, -12.8), (5.2, -8.4), (7.4, -4.6)],
            ] {
                fill_polygon(
                    rt,
                    factory,
                    &local(&strand),
                    D2D1_COLOR_F {
                        a: alpha * human,
                        ..hair_deep
                    },
                );
            }
        }

        // ── 4. Titan: garis gelap pipi + celah mata + mulut bergerigi ──────
        let eye_y = base_cy + 5.0 * s;
        if titan > 0.12 {
            let ta = smoothstep(0.12, 0.85, titan) * alpha;
            if let Ok(b) = rt.CreateSolidColorBrush(
                &D2D1_COLOR_F {
                    r: 0.035,
                    g: 0.030,
                    b: 0.030,
                    a: ta * 0.8,
                },
                None,
            ) {
                for side in [-1.0_f32, 1.0_f32] {
                    rt.FillEllipse(
                        &D2D1_ELLIPSE {
                            point: D2D_POINT_2F {
                                x: base_cx + side * 9.0 * s + ctx.look_x * 0.6,
                                y: eye_y,
                            },
                            radiusX: 6.4 * s,
                            radiusY: 7.2 * s,
                        },
                        &b,
                    );
                }
            }
            // Streak gelap vertikal dari mata ke rahang.
            for side in [-1.0_f32, 1.0_f32] {
                for k in 0..2 {
                    let x0 = base_cx + side * (6.6 + k as f32 * 3.4) * s;
                    draw_soft_streak(
                        rt,
                        factory,
                        &[
                            D2D_POINT_2F {
                                x: x0,
                                y: eye_y + 3.0 * s,
                            },
                            D2D_POINT_2F {
                                x: x0 + side * 0.8,
                                y: eye_y + 7.0 * s,
                            },
                            D2D_POINT_2F {
                                x: x0 + side * 1.6,
                                y: eye_y + 11.0 * s,
                            },
                        ],
                        D2D1_COLOR_F {
                            r: 0.05,
                            g: 0.04,
                            b: 0.04,
                            a: ta * 0.75,
                        },
                        1.1,
                    );
                }
            }
            // Mulut bergerigi (grimace titan).
            let my = base_cy + 12.2 * s;
            let mouth = [
                (-8.6, 0.0),
                (-4.3, 1.5),
                (0.0, 2.2),
                (4.3, 1.5),
                (8.6, 0.0),
            ];
            let mut jaw = Vec::new();
            for &(x, y) in mouth.iter() {
                jaw.push(D2D_POINT_2F {
                    x: base_cx + x * s,
                    y: my + y * s,
                });
            }
            fill_polygon(
                rt,
                factory,
                &[
                    D2D_POINT_2F {
                        x: base_cx - 9.4 * s,
                        y: my - 1.2 * s,
                    },
                    D2D_POINT_2F {
                        x: base_cx - 4.6 * s,
                        y: my + 1.4 * s,
                    },
                    D2D_POINT_2F {
                        x: base_cx,
                        y: my + 2.3 * s,
                    },
                    D2D_POINT_2F {
                        x: base_cx + 4.6 * s,
                        y: my + 1.4 * s,
                    },
                    D2D_POINT_2F {
                        x: base_cx + 9.4 * s,
                        y: my - 1.2 * s,
                    },
                    D2D_POINT_2F {
                        x: base_cx + 8.6 * s,
                        y: my + 4.4 * s,
                    },
                    D2D_POINT_2F {
                        x: base_cx,
                        y: my + 6.2 * s,
                    },
                    D2D_POINT_2F {
                        x: base_cx - 8.6 * s,
                        y: my + 4.4 * s,
                    },
                ],
                D2D1_COLOR_F {
                    r: 0.10,
                    g: 0.07,
                    b: 0.07,
                    a: ta,
                },
            );
            // Gigi: deretan blok pucat.
            if let Ok(b) = rt.CreateSolidColorBrush(
                &D2D1_COLOR_F {
                    a: ta * 0.95,
                    ..teeth
                },
                None,
            ) {
                for k in 0..7 {
                    let fk = k as f32;
                    let tx = base_cx + (-7.2 + fk * 2.4) * s;
                    let top = my + (1.9 - (fk - 3.0).abs() * 0.28) * s;
                    rt.FillRoundedRectangle(
                        &D2D1_ROUNDED_RECT {
                            rect: D2D_RECT_F {
                                left: tx - 0.9 * s,
                                top,
                                right: tx + 0.9 * s,
                                bottom: top + (1.5 + (fk % 2.0) * 0.5) * s,
                            },
                            radiusX: 0.5 * s,
                            radiusY: 0.6 * s,
                        },
                        &b,
                    );
                }
            }
            let _ = jaw;
        }

        // ── 5. Mata teal-green menyala (kedua mode) ────────────────────────
        draw_capsule_eyes(
            rt,
            factory,
            base_cx,
            eye_y,
            9.0,
            teal,
            teal_core,
            (0.20, 0.86, 0.62),
            0.30 + 0.42 * titan,
            alpha,
            ctx,
        );
        // Celah gelap vertikal menembus mata saat titan.
        if titan > 0.4 {
            let a = alpha * smoothstep(0.4, 0.9, titan) * ctx.open_factor();
            if let Ok(b) = rt.CreateSolidColorBrush(
                &D2D1_COLOR_F {
                    r: 0.03,
                    g: 0.03,
                    b: 0.03,
                    a,
                },
                None,
            ) {
                for side in [-1.0_f32, 1.0_f32] {
                    let ex = base_cx + side * 9.0 + ctx.look_x * 0.65;
                    rt.FillRoundedRectangle(
                        &D2D1_ROUNDED_RECT {
                            rect: D2D_RECT_F {
                                left: ex - 0.7,
                                top: eye_y - 3.4,
                                right: ex + 0.7,
                                bottom: eye_y + 3.4,
                            },
                            radiusX: 0.7,
                            radiusY: 1.1,
                        },
                        &b,
                    );
                }
            }
        }
    }
}
