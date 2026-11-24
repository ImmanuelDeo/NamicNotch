// src/mascot/eren.rs
// Fase 4: Eren Yeager (man-bun) -> Attack Titan.
// Bahasa visual sama dengan Asta Devil Union: basis kepala gelap, rambut berlapis,
// kerah cloak Survey Corps, mata illuminated, petir emas -> uap putih di kanan.
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
        if t < 2.8 || t > 4.6 {
            0.0
        } else if t < 3.3 {
            smoothstep(2.8, 3.3, t)
        } else {
            1.0 - smoothstep(3.3, 4.6, t)
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
            pw * 0.46,
            ph * 0.70,
            lerp(0.18, 0.92, titan.max(flash)),
            lerp(0.77, 0.78, titan),
            lerp(0.57, 0.30, titan),
            (0.26 + 0.18 * titan + 0.26 * flash) * alpha,
        );
        draw_soft_radial_glow(
            rt,
            D2D_POINT_2F {
                x: cx + pw * 0.22,
                y: pill_rect.bottom - 2.0,
            },
            pw * 0.24,
            ph * 0.62,
            1.0,
            0.96,
            0.86,
            (0.10 + 0.24 * titan) * alpha,
        );

        let periodic = if titan > 0.6 {
            ((ctx.phase_time * 7.5).sin() - 0.62).max(0.0) * 1.6
        } else {
            0.0
        };
        let bolt = flash.max(periodic).clamp(0.0, 1.0) * alpha;
        if bolt > 0.05 {
            let rx = cx + pw * 0.22;
            let ry = pill_rect.top + 2.0;
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
                        x: rx + 8.0,
                        y: ry + 12.0,
                    },
                    D2D_POINT_2F {
                        x: rx - 3.0,
                        y: ry + 19.0,
                    },
                    D2D_POINT_2F {
                        x: rx - 12.0,
                        y: pill_rect.bottom - 2.0,
                    },
                ],
                D2D1_COLOR_F {
                    r: 1.0,
                    g: 0.82,
                    b: 0.18,
                    a: bolt * 0.42,
                },
                D2D1_COLOR_F {
                    r: 1.0,
                    g: 0.98,
                    b: 0.78,
                    a: bolt,
                },
                1.45,
            );
        }

        if titan > 0.2 {
            for i in 0..5 {
                let fi = i as f32;
                let life = (ctx.phase_time * (0.38 + fi * 0.07) + fi * 0.23) % 1.0;
                let sx = cx + pw * (0.08 + 0.34 * ((fi * 0.23) % 1.0))
                    + (ctx.phase_time * 1.1 + fi).sin() * 2.2
                    + life * 6.0;
                let sy = pill_rect.bottom - 4.0 - life * (ph * 0.78);
                draw_soft_circle(
                    rt,
                    D2D_POINT_2F { x: sx, y: sy },
                    7.0 + life * 6.0,
                    4.8 + life * 4.0,
                    D2D1_COLOR_F {
                        r: 0.90,
                        g: 0.95,
                        b: 0.93,
                        a: (1.0 - life) * life * 0.7 * titan * alpha,
                    },
                );
            }
        }

        draw_rising_particles(
            rt,
            cx,
            pill_rect.bottom - 1.0,
            pw * 0.34,
            ph * 0.82,
            ctx.phase_time,
            7,
            D2D1_COLOR_F {
                r: 1.0,
                g: lerp(0.86, 0.70, titan),
                b: lerp(0.50, 0.26, titan),
                a: 0.22 * alpha,
            },
            0.9,
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
        let hair_hi = col(0.34, 0.24, 0.19);
        let hair_deep = col(0.12, 0.08, 0.07);
        let cloak = col(0.16, 0.26, 0.19);
        let cloak_hi = col(0.26, 0.38, 0.28);
        let shirt = col(0.84, 0.82, 0.76);
        let gold = col(1.00, 0.86, 0.32);
        let emerald = col(0.12, 1.00, 0.55);
        let emerald_core = col(0.90, 1.00, 0.95);
        let teal = col(0.18, 0.77, 0.57);
        let teal_core = col(0.77, 1.00, 0.91);
        let sway = (ctx.phase_time * 1.8).sin();

        // ── 0. Kerah cloak + krawat putih ──────────────────────────────────
        fill_polygon(
            rt,
            factory,
            &local(&[
                (-28.0, 13.5),
                (-22.5, 4.5),
                (-13.0, 7.2),
                (0.0, 9.4),
                (13.0, 7.2),
                (22.5, 4.5),
                (28.0, 13.5),
                (0.0, 19.5),
            ]),
            cloak,
        );
        fill_polygon(
            rt,
            factory,
            &local(&[
                (-9.0, 8.2),
                (0.0, 10.4),
                (9.0, 8.2),
                (6.0, 13.0),
                (0.0, 15.6),
                (-6.0, 13.0),
            ]),
            shirt,
        );
        // Krawat cravat.
        fill_polygon(
            rt,
            factory,
            &local(&[(-3.2, 9.4), (3.2, 9.4), (1.6, 13.4), (0.0, 15.2), (-1.6, 13.4)]),
            cloak_hi,
        );
        draw_polyline(
            rt,
            factory,
            &local(&[(-22.5, 4.6), (-13.0, 7.3), (0.0, 9.6), (13.0, 7.3), (22.5, 4.6)]),
            D2D1_COLOR_F {
                a: 0.5 * alpha,
                ..cloak_hi
            },
            1.0,
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
        let rim = lerp_color(teal, emerald, titan);
        draw_soft_radial_glow(rt, at(-19.0, 1.5), 9.0, 12.0, rim.r, rim.g, rim.b, 0.16 * alpha);
        draw_soft_radial_glow(rt, at(19.0, 1.5), 9.0, 12.0, rim.r, rim.g, rim.b, 0.16 * alpha);

        // ── 2. Rambut: swept-back + man-bun -> shaggy titan ────────────────
        let grow = titan * 2.6;
        let hair_pts = [
            (-22.0, -2.0),
            (-24.5, -8.0),
            (-19.5, -9.0),
            (-22.5, -15.0),
            (-14.5, -13.5),
            (-13.0, -19.5),
            (-5.0, -16.5),
            (0.0, -21.0),
            (5.5, -16.5),
            (13.5, -19.0),
            (14.5, -13.0),
            (21.5, -14.5),
            (19.0, -8.4),
            (23.5, -7.0),
            (21.0, -1.5),
            (16.0, 1.0),
            (8.0, -2.4),
            (0.0, 0.4),
            (-8.0, -2.4),
            (-16.5, 1.0),
        ];
        fill_polygon(rt, factory, &local(&hair_pts), hair);
        // Helai shaggy menjuntai saat titan.
        if titan > 0.2 {
            for side in [-1.0_f32, 1.0_f32] {
                fill_polygon(
                    rt,
                    factory,
                    &local(&[
                        (side * (17.0 + grow), -6.0),
                        (side * (21.0 + grow) + sway, 3.0),
                        (side * (17.5 + grow), 8.5),
                        (side * 13.0, 2.0),
                    ]),
                    D2D1_COLOR_F {
                        a: alpha * titan,
                        ..hair
                    },
                );
            }
        }
        for chunk in [
            [(-21.0, -12.0), (-12.0, -10.0), (-17.0, -5.5)],
            [(-10.0, -16.0), (-2.0, -12.0), (-7.0, -7.0)],
            [(1.0, -18.0), (5.0, -12.0), (0.0, -7.5)],
            [(10.0, -16.5), (8.0, -8.0), (2.5, -4.8)],
            [(19.0, -13.0), (16.0, -6.0), (10.0, -4.4)],
        ] {
            fill_polygon(rt, factory, &local(&chunk), hair_hi);
        }
        draw_polyline(
            rt,
            factory,
            &local(&[
                (-22.0, -2.0),
                (-24.5, -8.0),
                (-19.5, -9.0),
                (-22.5, -15.0),
                (-14.5, -13.5),
                (-13.0, -19.5),
                (-5.0, -16.5),
                (0.0, -21.0),
                (5.5, -16.5),
                (13.5, -19.0),
                (14.5, -13.0),
                (21.5, -14.5),
                (19.0, -8.4),
                (23.5, -7.0),
                (21.0, -1.5),
            ]),
            D2D1_COLOR_F {
                a: 0.5 * alpha,
                ..hair_hi
            },
            0.9,
        );

        // Man-bun + dua poni depan (fase manusia).
        let bun_a = (1.0 - titan).max(0.0);
        if bun_a > 0.04 {
            if let Ok(b) = rt.CreateSolidColorBrush(
                &D2D1_COLOR_F {
                    a: alpha * bun_a,
                    ..hair_hi
                },
                None,
            ) {
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: at(2.5, -18.5),
                        radiusX: 4.8 * s,
                        radiusY: 3.8 * s,
                    },
                    &b,
                );
            }
            for side in [-1.0_f32, 1.0_f32] {
                fill_polygon(
                    rt,
                    factory,
                    &local(&[
                        (side * 1.2, -14.0),
                        (side * 6.4, -12.6),
                        (side * 3.0 + sway * 0.4, -4.8),
                    ]),
                    D2D1_COLOR_F {
                        a: alpha * bun_a,
                        ..hair_dark_tone(hair)
                    },
                );
            }
        }

        // ── 3. Titan marks: soket gelap + garis emas ───────────────────────
        let eye_y = base_cy + 5.0 * s;
        if titan > 0.12 {
            let ta = smoothstep(0.12, 0.85, titan) * alpha;
            if let Ok(b) = rt.CreateSolidColorBrush(
                &D2D1_COLOR_F {
                    r: 0.035,
                    g: 0.030,
                    b: 0.030,
                    a: ta * 0.85,
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
                            radiusX: 6.6 * s,
                            radiusY: 7.4 * s,
                        },
                        &b,
                    );
                }
            }
            for side in [-1.0_f32, 1.0_f32] {
                for k in 0..3 {
                    let bx = base_cx + side * (8.2 + k as f32 * 1.2) * s;
                    let by = eye_y + 5.6 * s + k as f32 * 0.6;
                    draw_polyline(
                        rt,
                        factory,
                        &[
                            D2D_POINT_2F { x: bx, y: by },
                            D2D_POINT_2F {
                                x: bx + side * 1.0 * s,
                                y: by + 3.0 * s,
                            },
                        ],
                        D2D1_COLOR_F {
                            r: 1.0,
                            g: 0.84,
                            b: 0.42,
                            a: ta * 0.8,
                        },
                        0.9,
                    );
                }
            }
        }

        // ── 4. Mata: teal tenang -> emerald intens ─────────────────────────
        let (outer, core_c, glow) = if titan < 0.5 {
            (teal, teal_core, (0.18, 0.77, 0.57))
        } else {
            (emerald, emerald_core, (0.12, 1.0, 0.55))
        };
        draw_capsule_eyes(
            rt,
            factory,
            base_cx,
            eye_y,
            9.0,
            outer,
            core_c,
            glow,
            titan * 0.75 + flash * 0.2,
            alpha,
            ctx,
        );

        // ── 5. Petir emas / uap (kanan) ────────────────────────────────────
        let gx = base_cx + 34.0 * s;
        let gy = base_cy + 3.6 * s;
        if flash > 0.08 {
            draw_soft_radial_glow(
                rt,
                D2D_POINT_2F { x: gx, y: gy },
                14.0,
                13.0,
                1.0,
                0.86,
                0.30,
                0.55 * flash * alpha,
            );
            draw_lightning_bolt(
                rt,
                factory,
                &[
                    D2D_POINT_2F {
                        x: gx + 4.0 * s,
                        y: gy - 12.0 * s,
                    },
                    D2D_POINT_2F {
                        x: gx - 1.5 * s,
                        y: gy - 4.0 * s,
                    },
                    D2D_POINT_2F {
                        x: gx + 3.0 * s,
                        y: gy - 1.0 * s,
                    },
                    D2D_POINT_2F {
                        x: gx - 3.0 * s,
                        y: gy + 9.0 * s,
                    },
                ],
                D2D1_COLOR_F {
                    r: 1.0,
                    g: 0.84,
                    b: 0.20,
                    a: 0.45 * flash * alpha,
                },
                D2D1_COLOR_F {
                    r: 1.0,
                    g: 0.98,
                    b: 0.80,
                    a: flash * alpha,
                },
                1.5,
            );
        }
        if titan > 0.25 {
            let t = ctx.phase_time;
            for k in 0..3 {
                let fk = k as f32;
                let life = (t * (0.5 + fk * 0.12) + fk * 0.31) % 1.0;
                draw_soft_circle(
                    rt,
                    D2D_POINT_2F {
                        x: gx + life * 4.0,
                        y: gy + 5.0 - life * 14.0,
                    },
                    3.8 + life * 5.2,
                    3.0 + life * 3.4,
                    D2D1_COLOR_F {
                        r: 0.92,
                        g: 0.96,
                        b: 0.94,
                        a: (1.0 - life) * life * 0.5 * titan * alpha,
                    },
                );
            }
        }
    }
}

/// Versi lebih gelap untuk helai poni.
fn hair_dark_tone(c: D2D1_COLOR_F) -> D2D1_COLOR_F {
    D2D1_COLOR_F {
        r: c.r * 0.72,
        g: c.g * 0.72,
        b: c.b * 0.72,
        a: c.a,
    }
}
