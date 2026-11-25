// src/mascot/sukuna.rs
// Fase 2: Yuji Itadori -> Ryomen Sukuna.
// Bahasa visual sama dengan Asta Devil Union: basis kepala gelap, rambut berlapis dengan
// highlight chunk + outline, aksesoris kepala, kerah pakaian, mata illuminated, objek
// ikonik di kanan.
use super::core::*;
use super::vfx;
use windows::Win32::Graphics::Direct2D::Common::{D2D1_COLOR_F, D2D_POINT_2F, D2D_RECT_F};
use windows::Win32::Graphics::Direct2D::{
    ID2D1Factory, ID2D1RenderTarget, D2D1_ELLIPSE, D2D1_ROUNDED_RECT,
};

const SCALE: f32 = 0.78;

pub struct YujiSukuna;

impl YujiSukuna {
    pub fn new() -> Self {
        Self
    }

    fn sukuna_factor(t: f32) -> f32 {
        smoothstep(3.0, 4.2, t)
    }

    fn shrine_gesture(t: f32) -> f32 {
        if t < 4.0 {
            0.0
        } else if t < 5.2 {
            smoothstep(4.0, 5.2, t)
        } else {
            1.0
        }
    }
}

impl Default for YujiSukuna {
    fn default() -> Self {
        Self::new()
    }
}

impl IdleCharacter for YujiSukuna {
    fn id(&self) -> &'static str {
        "yuji_to_sukuna"
    }

    fn duration_secs(&self) -> f32 {
        11.0
    }

    fn override_look_target(&self, ctx: &MascotContext) -> Option<(f32, f32)> {
        let g = Self::shrine_gesture(ctx.local_time);
        if g > 0.1 {
            Some((1.8 * g, -0.25 * g))
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
        let pw = pill_rect.right - pill_rect.left;
        let ph = pill_rect.bottom - pill_rect.top;
        let cx = (pill_rect.left + pill_rect.right) * 0.5;
        let cy = pill_rect.top + ph * 0.58;
        let t = ctx.phase_time;
        let breathe = 1.0 + 0.05 * (t * 0.55).sin();

        // Calm illumination: satu gradien radial besar, tanpa garis/laser.
        vfx::draw_ambient_notch_illumination(
            rt,
            pill_rect,
            (0.28, 0.04, 0.10),
            (0.16, 0.03, 0.08),
            (0.85, 0.14, 0.34),
            0.78,
            alpha,
        );
        draw_soft_radial_glow(
            rt,
            D2D_POINT_2F { x: cx, y: cy },
            pw * 0.46 * breathe,
            ph * 1.00 * breathe,
            0.82,
            0.08,
            0.26,
            (0.32 * alpha).min(0.34),
        );
        draw_soft_radial_glow(
            rt,
            D2D_POINT_2F { x: cx, y: pill_rect.bottom + 2.0 },
            pw * 0.58,
            ph * 0.88,
            0.822,
            0.082,
            0.262,
            (0.322 * alpha).min(0.30),
        );
        // Partikel lembut, ukuran kecil, melayang pelan.
        vfx::draw_bokeh_motes(rt, pill_rect, 9, t, 31.7, (0.95, 0.28, 0.44), alpha * 0.95);
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
        let sk = Self::sukuna_factor(ctx.local_time);
        let g = Self::shrine_gesture(ctx.local_time);
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

        let black = col(0.020, 0.020, 0.026);
        let hair_pink = lerp_color(col(0.90, 0.58, 0.60), col(0.78, 0.42, 0.46), sk);
        let hair_hi = lerp_color(col(0.99, 0.78, 0.78), col(0.93, 0.62, 0.65), sk);
        let hair_deep = col(0.42, 0.20, 0.24);
        let cyan = col(0.35, 0.93, 1.00);
        let crimson = col(1.00, 0.14, 0.26);
        let crimson_core = col(1.00, 0.72, 0.78);
        let amber = col(0.94, 0.62, 0.24);
        let amber_core = col(1.00, 0.91, 0.72);
        let cloth_white = col(0.86, 0.84, 0.80);
        let cloth_dark = col(0.075, 0.070, 0.090);

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
                r: 0.088,
                g: 0.084,
                b: 0.098,
                a: alpha * 0.92,
            },
        );

        // ── 0. Kerah + bahu ────────────────────────────────────────────────
        fill_polygon(
            rt,
            factory,
            &local(&[
                (-27.0, 13.0),
                (-22.0, 4.5),
                (-13.0, 7.0),
                (0.0, 9.5),
                (13.0, 7.0),
                (22.0, 4.5),
                (27.0, 13.0),
                (0.0, 18.5),
            ]),
            cloth_dark,
        );
        fill_polygon(
            rt,
            factory,
            &local(&[
                (-11.0, 6.6),
                (0.0, 9.2),
                (11.0, 6.6),
                (9.0, 9.6),
                (0.0, 12.2),
                (-9.0, 9.6),
            ]),
            lerp_color(
                D2D1_COLOR_F {
                    r: 0.78,
                    g: 0.10,
                    b: 0.18,
                    a: alpha * 0.55,
                },
                cloth_white,
                sk,
            ),
        );
        draw_polyline(
            rt,
            factory,
            &local(&[
                (-22.0, 4.6),
                (-13.0, 7.1),
                (0.0, 9.6),
                (13.0, 7.1),
                (22.0, 4.6),
            ]),
            if sk > 0.5 { cloth_white } else { crimson },
            1.0,
        );

        // ── 2. Rambut berlapis ─────────────────────────────────────────────
        let hair_pts = [
            (-23.0, -2.5),
            (-27.5, -8.0),
            (-21.0, -8.8),
            (-26.0, -14.5),
            (-17.5, -12.2),
            (-19.5, -19.0),
            (-11.0, -14.6),
            (-11.0, -20.5),
            (-4.5, -15.6),
            (-2.0, -22.0),
            (2.0, -17.6),
            (6.0, -21.0),
            (7.5, -15.4),
            (14.0, -19.4),
            (13.0, -13.6),
            (20.5, -16.6),
            (17.5, -10.6),
            (25.5, -11.6),
            (21.0, -6.4),
            (26.0, -3.0),
            (17.0, 0.2),
            (8.0, -3.2),
            (0.0, -0.4),
            (-9.0, -3.4),
            (-18.0, 0.0),
        ];
        fill_polygon(rt, factory, &local(&hair_pts), hair_pink);
        for chunk in [
            [(-22.0, -12.0), (-13.5, -9.6), (-18.5, -5.6)],
            [(-12.0, -16.6), (-3.6, -12.4), (-8.6, -7.4)],
            [(-1.0, -19.4), (3.2, -13.6), (-0.6, -8.8)],
            [(8.6, -17.0), (6.4, -8.6), (0.8, -5.4)],
            [(19.4, -14.0), (15.2, -7.0), (9.4, -5.2)],
            [(-23.4, -6.6), (-17.0, -3.8), (-19.6, -1.8)],
        ] {
            fill_polygon(rt, factory, &local(&chunk), hair_hi);
        }
        for side in [-1.0_f32, 1.0_f32] {
            fill_polygon(
                rt,
                factory,
                &local(&[
                    (side * 24.6, -3.4),
                    (side * 15.4, -2.6),
                    (side * 19.4, -9.2),
                ]),
                hair_deep,
            );
        }
        draw_polyline(
            rt,
            factory,
            &local(&[
                (-23.0, -2.5),
                (-27.5, -8.0),
                (-21.0, -8.8),
                (-26.0, -14.5),
                (-17.5, -12.2),
                (-19.5, -19.0),
                (-11.0, -14.6),
                (-11.0, -20.5),
                (-4.5, -15.6),
                (-2.0, -22.0),
                (2.0, -17.6),
                (6.0, -21.0),
                (7.5, -15.4),
                (14.0, -19.4),
                (13.0, -13.6),
                (20.5, -16.6),
                (17.5, -10.6),
                (25.5, -11.6),
                (21.0, -6.4),
                (26.0, -3.0),
            ]),
            D2D1_COLOR_F {
                a: 0.55 * alpha,
                ..hair_hi
            },
            0.9,
        );

        // ── 3. Tato Sukuna ─────────────────────────────────────────────────
        if sk > 0.04 {
            let ink = D2D1_COLOR_F {
                a: alpha * sk,
                ..black
            };
            let ink_glow = D2D1_COLOR_F {
                r: 0.85,
                g: 0.10,
                b: 0.20,
                a: 0.30 * alpha * sk,
            };
            let mark = |pts: &[(f32, f32)], w: f32| {
                draw_polyline(rt, factory, &local(pts), ink_glow, w + 1.6);
                draw_polyline(rt, factory, &local(pts), ink, w);
            };
            mark(&[(0.0, -12.6), (0.0, -5.4)], 1.3);
            mark(&[(-4.6, -12.2), (-1.5, -6.4)], 1.0);
            mark(&[(4.6, -12.2), (1.5, -6.4)], 1.0);
            mark(&[(-16.0, 2.6), (-13.4, 6.6)], 0.9);
            mark(&[(16.0, 2.6), (13.4, 6.6)], 0.9);
            mark(&[(-9.6, 8.6), (-6.4, 10.4)], 0.8);
            mark(&[(9.6, 8.6), (6.4, 10.4)], 0.8);
        }

        // ── 4. Mata ────────────────────────────────────────────────────────
        let eye_y = base_cy + 5.0 * s;
        let (outer, core_c, glow) = if sk < 0.5 {
            (amber, amber_core, (0.94, 0.62, 0.24))
        } else {
            (crimson, crimson_core, (1.0, 0.14, 0.26))
        };
        draw_capsule_eyes(
            rt, factory, base_cx, eye_y, 9.0, outer, core_c, glow, 0.78 * sk, alpha, ctx,
        );

        if sk > 0.35 {
            let a = alpha * smoothstep(0.35, 0.9, sk) * ctx.open_factor();
            // Pupil garis hitam dihapus: mata Sukuna murni glow crimson.
            if let Ok(b) = rt.CreateSolidColorBrush(
                &D2D1_COLOR_F {
                    r: 1.0,
                    g: 0.20,
                    b: 0.30,
                    a,
                },
                None,
            ) {
                for side in [-1.0_f32, 1.0_f32] {
                    let ex = base_cx + side * 10.6 + ctx.look_x * 0.5;
                    rt.FillEllipse(
                        &D2D1_ELLIPSE {
                            point: D2D_POINT_2F {
                                x: ex,
                                y: eye_y + 7.4,
                            },
                            radiusX: 2.6,
                            radiusY: 1.1,
                        },
                        &b,
                    );
                }
            }
        }

        // ── 5. Gestur kanan ────────────────────────────────────────────────
        let float_y = (ctx.phase_time * 2.4).sin() * 1.2 * (1.0 - g);
        let sx = base_cx + 34.0 * s;
        let sy = base_cy + 3.4 * s + float_y;

        if g < 0.5 {
            let ca = (1.0 - g * 2.0) * alpha;
            let pulse = 0.55 + 0.35 * (ctx.phase_time * 6.0).sin().abs();
            draw_soft_radial_glow(
                rt,
                D2D_POINT_2F { x: sx, y: sy },
                13.0,
                11.0,
                0.20,
                0.90,
                1.0,
                pulse * ca,
            );
            fill_polygon(
                rt,
                factory,
                &[
                    D2D_POINT_2F {
                        x: sx - 5.2 * s,
                        y: sy + 5.4 * s,
                    },
                    D2D_POINT_2F {
                        x: sx - 4.4 * s,
                        y: sy - 4.0 * s,
                    },
                    D2D_POINT_2F {
                        x: sx + 1.2 * s,
                        y: sy - 5.6 * s,
                    },
                    D2D_POINT_2F {
                        x: sx + 5.4 * s,
                        y: sy - 0.6 * s,
                    },
                    D2D_POINT_2F {
                        x: sx + 3.6 * s,
                        y: sy + 5.2 * s,
                    },
                ],
                D2D1_COLOR_F {
                    r: 0.10,
                    g: 0.09,
                    b: 0.12,
                    a: ca,
                },
            );
            draw_polyline(
                rt,
                factory,
                &[
                    D2D_POINT_2F {
                        x: sx - 3.6 * s,
                        y: sy - 4.6 * s,
                    },
                    D2D_POINT_2F {
                        x: sx - 1.4 * s,
                        y: sy - 10.0 * s,
                    },
                    D2D_POINT_2F {
                        x: sx + 1.8 * s,
                        y: sy - 4.4 * s,
                    },
                    D2D_POINT_2F {
                        x: sx + 4.4 * s,
                        y: sy - 8.6 * s,
                    },
                ],
                D2D1_COLOR_F {
                    r: 0.42,
                    g: 0.95,
                    b: 1.0,
                    a: ca,
                },
                1.3,
            );
            for k in 0..3 {
                let fk = k as f32;
                let ph = (ctx.phase_time * 1.6 + fk * 0.33) % 1.0;
                let a = (ph * (1.0 - ph) * 3.6).min(1.0) * ca * 0.85;
                let bx = sx + (fk - 1.0) * 4.6;
                draw_tapered_streak(
                    rt,
                    factory,
                    D2D_POINT_2F {
                        x: bx,
                        y: sy - 2.0,
                    },
                    D2D_POINT_2F {
                        x: bx + (fk - 1.0) * 2.2,
                        y: sy - 11.0 - ph * 4.0,
                    },
                    (fk - 1.0) * 1.6,
                    1.8,
                    D2D1_COLOR_F {
                        r: 0.55,
                        g: 0.96,
                        b: 1.0,
                        a,
                    },
                );
            }
        } else {
            let gg = ((g - 0.5) * 2.0) * alpha;
            draw_soft_radial_glow(
                rt,
                D2D_POINT_2F { x: sx, y: sy },
                14.0,
                12.0,
                0.85,
                0.10,
                0.22,
                0.35 * gg,
            );
            for side in [-1.0_f32, 1.0_f32] {
                fill_polygon(
                    rt,
                    factory,
                    &[
                        D2D_POINT_2F {
                            x: sx + side * 1.0 * s,
                            y: sy - 9.0 * s,
                        },
                        D2D_POINT_2F {
                            x: sx + side * 4.2 * s,
                            y: sy - 1.6 * s,
                        },
                        D2D_POINT_2F {
                            x: sx + side * 7.4 * s,
                            y: sy + 5.4 * s,
                        },
                        D2D_POINT_2F {
                            x: sx + side * 4.0 * s,
                            y: sy + 7.0 * s,
                        },
                        D2D_POINT_2F {
                            x: sx + side * 1.4 * s,
                            y: sy + 0.8 * s,
                        },
                    ],
                    D2D1_COLOR_F {
                        r: 0.86,
                        g: 0.72,
                        b: 0.66,
                        a: gg,
                    },
                );
                draw_polyline(
                    rt,
                    factory,
                    &[
                        D2D_POINT_2F {
                            x: sx + side * 1.0 * s,
                            y: sy - 9.0 * s,
                        },
                        D2D_POINT_2F {
                            x: sx + side * 4.2 * s,
                            y: sy - 1.6 * s,
                        },
                        D2D_POINT_2F {
                            x: sx + side * 7.4 * s,
                            y: sy + 5.4 * s,
                        },
                    ],
                    D2D1_COLOR_F {
                        r: 1.0,
                        g: 0.30,
                        b: 0.38,
                        a: 0.55 * gg,
                    },
                    0.9,
                );
            }
            // Pengganti ring keras: glow lembut + dua busur meruncing.
            draw_soft_radial_glow(
                rt,
                D2D_POINT_2F { x: sx, y: sy },
                15.0 * s,
                14.0 * s,
                1.0,
                0.20,
                0.28,
                0.30 * gg,
            );
            for k in 0..2 {
                let fk = k as f32;
                let a0 = -1.9 + fk * 3.1;
                let a1 = a0 + 1.9;
                let r = 11.0 * s;
                vfx::draw_tapered_flame_ribbon(
                    rt,
                    factory,
                    D2D_POINT_2F {
                        x: sx + a0.cos() * r,
                        y: sy + a0.sin() * r,
                    },
                    D2D_POINT_2F {
                        x: sx + ((a0 + a1) * 0.5).cos() * r * 1.04,
                        y: sy + ((a0 + a1) * 0.5).sin() * r * 1.04,
                    },
                    D2D_POINT_2F {
                        x: sx + ((a0 + a1) * 0.5).cos() * r * 1.05,
                        y: sy + ((a0 + a1) * 0.5).sin() * r * 1.05,
                    },
                    D2D_POINT_2F {
                        x: sx + a1.cos() * r,
                        y: sy + a1.sin() * r,
                    },
                    2.2 * s,
                    (1.0, 0.22, 0.32),
                    0.40 * gg,
                    false,
                );
            }
        }
    }
}
