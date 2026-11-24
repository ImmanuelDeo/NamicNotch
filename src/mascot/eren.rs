// src/mascot/eren.rs
// Fase 4: Eren Yeager (man-bun) -> Attack Titan / Founding Titan (S4).
// Patokan: kartu referensi 4A/4B. Manusia = siluet cokelat, bun kanan-atas, 3 poni, mata
// teal menyala. Titan = tengkorak memanjang ala Founding Titan, surai panjang menjuntai
// kedua sisi, satu baris gigi bergerigi tanpa bibir, mata hijau terang memanjang vertikal,
// garis titan tipis di bawah mata.
// Environment: asap abu di kanan, petir emas bercabang, gradien hijau-kuning bawah.
use super::core::*;
use super::vfx;
use windows::Win32::Graphics::Direct2D::Common::{D2D1_COLOR_F, D2D_POINT_2F, D2D_RECT_F};
use windows::Win32::Graphics::Direct2D::{ID2D1Factory, ID2D1RenderTarget, D2D1_ELLIPSE};

const SCALE: f32 = 0.78;

pub struct ErenAttackTitan;

impl ErenAttackTitan {
    pub fn new() -> Self {
        Self
    }

    fn titan_factor(t: f32) -> f32 {
        smoothstep(2.6, 3.9, t)
    }

    fn lightning_flash(t: f32) -> f32 {
        if t < 2.6 || t > 4.8 {
            0.0
        } else if t < 3.2 {
            smoothstep(2.6, 3.2, t)
        } else {
            1.0 - smoothstep(3.2, 4.8, t)
        }
    }

    fn lightning_idle(t: f32, titan: f32) -> f32 {
        if titan <= 0.5 || t < 4.8 {
            return 0.0;
        }
        let cycle = (t - 4.8) % 3.8;
        if cycle < 0.8 {
            smoothstep(0.0, 0.8, cycle) * 0.9
        } else if cycle < 2.2 {
            (1.0 - smoothstep(0.8, 2.2, cycle)) * 0.9
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
        11.5
    }

    fn override_look_target(&self, ctx: &MascotContext) -> Option<(f32, f32)> {
        let flash = Self::lightning_flash(ctx.local_time);
        if flash > 0.1 {
            Some((1.5 * flash, -0.3 * flash))
        } else {
            None
        }
    }

    unsafe fn draw_aura(
        &self,
        rt: &ID2D1RenderTarget,
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
        let cy = (pill_rect.top + pill_rect.bottom) * 0.5;
        let t = ctx.phase_time;

        vfx::draw_ambient_notch_illumination(
            rt,
            pill_rect,
            (lerp(0.10, 0.46, titan), lerp(0.30, 0.42, titan), lerp(0.28, 0.12, titan)),
            (lerp(0.06, 0.24, titan), lerp(0.20, 0.30, titan), lerp(0.12, 0.10, titan)),
            (lerp(0.30, 0.92, titan), lerp(0.90, 0.94, titan), lerp(0.60, 0.44, titan)),
            0.80 + 0.35 * titan + 0.20 * flash,
            alpha,
        );
        vfx::draw_bokeh_motes(
            rt,
            pill_rect,
            10,
            t,
            44.2,
            (lerp(0.60, 0.92, titan), lerp(0.96, 0.98, titan), lerp(0.80, 0.72, titan)),
            alpha,
        );

        if titan > 0.08 {
            vfx::draw_feathered_titan_steam(
                rt,
                pill_rect,
                cx,
                18,
                t,
                7.3,
                (0.74, 0.96, 0.88),
                0.55 * titan * alpha,
            );
        }

        let bolt = (flash + idle_bolt).clamp(0.0, 1.0);
        if bolt > 0.03 {
            vfx::draw_tapered_anime_lightning(
                rt,
                factory,
                D2D_POINT_2F {
                    x: cx + pw * 0.20,
                    y: pill_rect.top + 1.0,
                },
                D2D_POINT_2F {
                    x: cx + pw * 0.16,
                    y: pill_rect.bottom - 2.0,
                },
                t,
                5.9,
                (1.0, 0.84, 0.30),
                bolt * alpha * 0.95,
                1.35,
                4,
            );
        }
        let _ = ph;
        let _ = cy;
    }

    unsafe fn draw_character(
        &self,
        rt: &ID2D1RenderTarget,
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

        // Siluet: cokelat gelap flat (tanpa shading) sesuai referensi.
        let silhouette = lerp_color(col(0.145, 0.105, 0.092), col(0.115, 0.083, 0.072), titan);
        let hair = col(0.20, 0.14, 0.115);
        let hair_hi = col(0.33, 0.235, 0.19);
        let hair_deep = col(0.10, 0.068, 0.058);
        let teal = col(0.20, 0.86, 0.62);
        let teal_core = col(0.80, 1.00, 0.92);
        let green = col(0.30, 1.00, 0.58);
        let green_core = col(0.92, 1.00, 0.86);
        let sway = (ctx.phase_time * 1.3).sin();

        // Basis kepala: hampir menyatu dengan latar obsidian, hanya cukup
        // untuk memisahkan siluet dari background (tanpa warna kulit).
        fill_smooth_shape(
            rt,
            factory,
            &local(&[
                (0.0, -9.6),
                (9.6, -8.4),
                (15.8, -4.8),
                (18.6, 0.8),
                (16.0, 6.8),
                (9.6, 11.4),
                (3.4, 13.4),
                (-3.4, 13.4),
                (-9.6, 11.4),
                (-16.0, 6.8),
                (-18.6, 0.8),
                (-15.8, -4.8),
                (-9.6, -8.4),
            ]),
            D2D1_COLOR_F {
                r: 0.066,
                g: 0.058,
                b: 0.052,
                a: alpha * 0.92,
            },
        );

        // ── 0. Bahu: tipis (manusia) -> lebar (titan) ──────────────────────
        let shoulder = lerp(26.0, 31.0, titan);
        fill_polygon(
            rt,
            factory,
            &local(&[
                (-shoulder, 14.5),
                (-shoulder * 0.80, 4.5),
                (-12.5, 7.8),
                (0.0, 9.8),
                (12.5, 7.8),
                (shoulder * 0.80, 4.5),
                (shoulder, 14.5),
                (0.0, 20.0),
            ]),
            silhouette,
        );
        draw_polyline(
            rt,
            factory,
            &local(&[
                (-shoulder * 0.80, 4.7),
                (-12.5, 8.0),
                (0.0, 10.0),
                (12.5, 8.0),
                (shoulder * 0.80, 4.7),
            ]),
            D2D1_COLOR_F {
                a: 0.35 * alpha,
                ..hair_hi
            },
            0.9,
        );

        // Ujung telinga runcing khas Attack Titan.
        if titan > 0.2 {
            let ear_a = smoothstep(0.2, 0.85, titan) * alpha;
            let ear_col = D2D1_COLOR_F {
                r: 0.20,
                g: 0.15,
                b: 0.12,
                a: ear_a,
            };
            for side in [-1.0_f32, 1.0_f32] {
                fill_polygon(
                    rt,
                    factory,
                    &local(&[
                        (side * 14.0, -2.5),
                        (side * 22.0, -7.0),
                        (side * 15.5, 3.5),
                    ]),
                    ear_col,
                );
            }
        }

        // ── 2. Rambut ──────────────────────────────────────────────────────
        let spread = titan * 2.0;
        let hair_pts = [
            (-21.0 - spread, -1.6),
            (-24.0 - spread, -7.2),
            (-18.5 - spread * 0.6, -8.4),
            (-21.5 - spread * 0.7, -14.4),
            (-13.5, -13.0),
            (-12.0, -19.2),
            (-4.5, -16.0),
            (0.0, -20.4),
            (5.5, -15.8),
            (13.5, -18.4),
            (14.5, -12.4),
            (21.5 + spread * 0.7, -13.8),
            (19.0 + spread * 0.6, -7.8),
            (24.0 + spread, -6.4),
            (21.0 + spread, -1.0),
            (16.0, 1.6),
            (8.0, -2.0),
            (0.0, 0.8),
            (-8.0, -2.0),
            (-16.5, 1.6),
        ];
        fill_polygon(rt, factory, &local(&hair_pts), hair);

        // Surai panjang founding titan: turun melewati rahang sampai bahu.
        if titan > 0.10 {
            for side in [-1.0_f32, 1.0_f32] {
                for k in 0..4 {
                    let fk = k as f32;
                    let x0 = side * (13.0 + fk * 2.4);
                    let drop = 10.0 + fk * 4.2 + titan * 4.0;
                    fill_polygon(
                        rt,
                        factory,
                        &local(&[
                            (x0, -8.0 - fk * 1.2),
                            (x0 + side * (7.5 + fk * 1.8), -1.0 + drop * 0.32 + sway * 0.6),
                            (x0 + side * (3.4 + fk * 1.4), drop * 0.68 + sway * 0.9),
                            (x0 + side * (1.0 + fk * 1.0), drop + sway),
                            (x0 - side * 3.4, 1.0),
                        ]),
                        D2D1_COLOR_F {
                            a: alpha * titan,
                            ..if k % 2 == 0 { hair } else { hair_deep }
                        },
                    );
                }
            }
        }
        for chunk in [
            [(-19.5, -11.6), (-11.0, -9.6), (-16.0, -5.2)],
            [(-9.0, -15.6), (-1.5, -11.6), (-6.5, -6.6)],
            [(1.0, -17.2), (5.0, -11.4), (0.0, -7.0)],
            [(10.0, -15.8), (8.0, -7.6), (2.5, -4.4)],
            [(19.0, -12.4), (16.0, -5.6), (10.0, -4.0)],
        ] {
            fill_polygon(rt, factory, &local(&chunk), hair_hi);
        }
        draw_polyline(
            rt,
            factory,
            &local(&[
                (-21.0 - spread, -1.6),
                (-24.0 - spread, -7.2),
                (-18.5 - spread * 0.6, -8.4),
                (-21.5 - spread * 0.7, -14.4),
                (-13.5, -13.0),
                (-12.0, -19.2),
                (-4.5, -16.0),
                (0.0, -20.4),
                (5.5, -15.8),
                (13.5, -18.4),
                (14.5, -12.4),
                (21.5 + spread * 0.7, -13.8),
                (19.0 + spread * 0.6, -7.8),
                (24.0 + spread, -6.4),
                (21.0 + spread, -1.0),
            ]),
            D2D1_COLOR_F {
                a: 0.45 * alpha,
                ..hair_hi
            },
            0.9,
        );

        // ── 3. Man-bun + poni (manusia) ────────────────────────────────────
        let human = (1.0 - titan).max(0.0);
        if human > 0.03 {
            let bun = at(10.5, -17.0);
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
            if let Ok(b) = rt.CreateSolidColorBrush(
                &D2D1_COLOR_F {
                    a: 0.7 * alpha * human,
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
                        radiusX: br * 0.58,
                        radiusY: br * 0.50,
                    },
                    &b,
                    0.9,
                    None,
                );
            }
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
                    a: 0.8 * alpha * human,
                    ..hair_hi
                },
                1.0,
            );
            for strand in [
                [(-10.0, -12.2), (-7.0, -7.8), (-9.4, -3.4)],
                [(-2.0, -14.0), (0.4, -8.8), (-0.6, -4.0)],
                [(6.0, -12.6), (5.2, -8.2), (7.4, -4.4)],
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

        // ── 4. Wajah titan: garis titan + satu baris gigi ──────────────────
        let eye_y = base_cy + (5.0 + 0.6 * titan) * s;
        if titan > 0.10 {
            let ta = smoothstep(0.10, 0.8, titan) * alpha;

            // Garis titan: streak tipis merah muda di bawah mata.
            for side in [-1.0_f32, 1.0_f32] {
                for k in 0..2 {
                    let x0 = base_cx + side * (6.2 + k as f32 * 3.4) * s;
                    draw_soft_streak(
                        rt,
                        factory,
                        &[
                            D2D_POINT_2F {
                                x: x0,
                                y: eye_y + 4.0 * s,
                            },
                            D2D_POINT_2F {
                                x: x0 + side * 1.2,
                                y: eye_y + 8.0 * s,
                            },
                            D2D_POINT_2F {
                                x: x0 + side * 2.4,
                                y: eye_y + 12.0 * s,
                            },
                        ],
                        D2D1_COLOR_F {
                            r: 1.0,
                            g: 0.80,
                            b: 0.36,
                            a: ta * 0.55,
                        },
                        0.9,
                    );
                }
            }
        }

        // ── 5. Mata: teal kecil (manusia) -> hijau besar memanjang (titan) ──
        let (outer, core_c, glow, spacing) = if titan < 0.5 {
            (teal, teal_core, (0.20, 0.86, 0.62), 9.0)
        } else {
            (green, green_core, (0.30, 1.0, 0.58), 9.8)
        };
        draw_capsule_eyes(
            rt,
            factory,
            base_cx,
            eye_y,
            spacing,
            outer,
            core_c,
            glow,
            0.24 + 0.30 * titan,
            alpha,
            ctx,
        );
        // Mata titan sedikit dipanjangkan vertikal.
        if titan > 0.3 {
            let a = alpha * smoothstep(0.3, 0.9, titan) * ctx.open_factor();
            if let Ok(b) = rt.CreateSolidColorBrush(
                &D2D1_COLOR_F {
                    a: a * 0.55,
                    ..green
                },
                None,
            ) {
                for side in [-1.0_f32, 1.0_f32] {
                    let ex = base_cx + side * spacing + ctx.look_x * 0.65;
                    rt.FillRoundedRectangle(
                        &windows::Win32::Graphics::Direct2D::D2D1_ROUNDED_RECT {
                            rect: D2D_RECT_F {
                                left: ex - 2.6,
                                top: eye_y - 7.4,
                                right: ex + 2.6,
                                bottom: eye_y + 7.4,
                            },
                            radiusX: 2.6,
                            radiusY: 3.4,
                        },
                        &b,
                    );
                }
            }
        }
    }
}
