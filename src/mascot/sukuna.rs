// src/mascot/sukuna.rs
// Fase 2: Yuji Itadori -> Ryomen Sukuna.
// Bahasa visual sama dengan Asta Devil Union: basis kepala gelap, rambut berlapis dengan
// highlight chunk + outline, aksesoris kepala, kerah pakaian, mata illuminated, objek
// ikonik di kanan.
use super::core::*;
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
        let sk = Self::sukuna_factor(ctx.local_time);
        let g = Self::shrine_gesture(ctx.local_time);
        let pw = pill_rect.right - pill_rect.left;
        let ph = pill_rect.bottom - pill_rect.top;
        let cx = (pill_rect.left + pill_rect.right) * 0.5;
        let t = ctx.phase_time;

        // Glow inti: cyan (Yuji) -> merah (Sukuna).
        draw_soft_radial_glow(
            rt,
            D2D_POINT_2F {
                x: cx,
                y: pill_rect.top + ph * 0.50,
            },
            pw * 0.30,
            ph * 0.52,
            lerp(0.20, 0.95, sk),
            lerp(0.80, 0.14, sk),
            lerp(1.00, 0.22, sk),
            (0.20 + 0.14 * sk) * alpha,
        );

        // Gradien bawah: cyan -> ungu-magenta.
        draw_bottom_gradient(
            rt,
            pill_rect,
            D2D1_COLOR_F {
                r: lerp(0.20, 0.62, sk),
                g: lerp(0.78, 0.16, sk),
                b: lerp(1.00, 0.92, sk),
                a: alpha,
            },
            ph * 0.46,
            (0.55 + 0.25 * sk) * alpha,
        );

        // Asap ungu-magenta di kiri & kanan kepala.
        let smoke = D2D1_COLOR_F {
            r: lerp(0.34, 0.72, sk),
            g: lerp(0.72, 0.18, sk),
            b: lerp(1.00, 0.94, sk),
            a: alpha,
        };
        for side in [-1.0_f32, 1.0_f32] {
            draw_smoke_wisp(
                rt,
                pill_rect,
                cx + side * pw * 0.24,
                0.86,
                side,
                smoke,
                (0.22 + 0.08 * sk) * alpha,
                t,
                side * 0.31 + 0.17,
            );
        }

        // Wisp cursed energy tipis di depan (mengalir naik).
        for k in 0..3 {
            let fk = k as f32;
            let life = ((t * (0.20 + fk * 0.03)) + (t * 0.08 + fk).sin() * 0.05 + fk * 0.23) % 1.0;
            let a = (life * (1.0 - life) * 3.2).min(1.0) * 0.26 * alpha;
            let bx = cx + pw * (0.14 + 0.24 * ((fk * 0.37) % 1.0));
            let by = pill_rect.bottom - 3.0;
            let bow = (t * 0.6 + fk * 1.6).sin() * 4.4 + (t * 0.22 + fk).cos() * 2.2;
            draw_tapered_streak(
                rt,
                factory,
                D2D_POINT_2F { x: bx, y: by },
                D2D_POINT_2F {
                    x: bx + bow,
                    y: by - ph * (0.40 + 0.30 * life),
                },
                bow * 0.7,
                2.2,
                D2D1_COLOR_F {
                    r: lerp(0.30, 0.98, sk),
                    g: lerp(0.88, 0.26, sk),
                    b: lerp(1.00, 0.34, sk),
                    a,
                },
            );
        }

        // Dismantle: busur putih bertepi merah, melintang naik ke kanan.
        for k in 0..3 {
            let fk = k as f32;
            let phase = (t * 0.34 + fk * 0.29) % 1.0;
            let a = (phase * (1.0 - phase) * 4.0).min(1.0) * 0.60 * alpha;
            if a <= 0.01 {
                continue;
            }
            let ox = pill_rect.left + pw * (0.42 + 0.10 * fk);
            let oy = pill_rect.top + ph * (0.72 - 0.20 * fk);
            let len = pw * 0.16;
            let arc = [
                D2D_POINT_2F {
                    x: ox - len * 0.35,
                    y: oy + len * 0.40,
                },
                D2D_POINT_2F {
                    x: ox + len * 0.40,
                    y: oy - len * 0.10,
                },
                D2D_POINT_2F {
                    x: ox + len,
                    y: oy - len * 0.62,
                },
            ];
            draw_soft_streak(
                rt,
                factory,
                &arc,
                D2D1_COLOR_F {
                    r: 1.0,
                    g: 0.26,
                    b: 0.34,
                    a: a * 0.55,
                },
                1.9,
            );
            draw_soft_streak(
                rt,
                factory,
                &arc,
                D2D1_COLOR_F {
                    r: 0.99,
                    g: 0.95,
                    b: 0.96,
                    a,
                },
                1.0,
            );
        }

        // Flame ribbon: crimson + ungu, mengalir menyilang.
        draw_smoke_wisp(
            rt,
            pill_rect,
            cx - pw * 0.20,
            0.80,
            -1.0,
            D2D1_COLOR_F {
                r: 0.95,
                g: 0.16,
                b: 0.26,
                a: alpha,
            },
            (0.18 + 0.10 * sk) * alpha,
            t * 1.15,
            0.29,
        );
        draw_smoke_wisp(
            rt,
            pill_rect,
            cx + pw * 0.22,
            0.72,
            1.0,
            D2D1_COLOR_F {
                r: 0.64,
                g: 0.18,
                b: 0.92,
                a: alpha,
            },
            (0.16 + 0.12 * sk) * alpha,
            t * 0.9,
            0.53,
        );

        // Kilatan Dismantle/Cleave saat hand sign aktif.
        if g > 0.4 {
            let wave = (t * 13.0).sin();
            if wave > 0.35 {
                let sa = ((wave - 0.35) / 0.65) * g * alpha;
                let ox = cx + 30.0;
                let oy = pill_rect.top + ph * 0.40;
                for k in 0..2 {
                    let dx = k as f32 * 5.0;
                    draw_soft_streak(
                        rt,
                        factory,
                        &[
                            D2D_POINT_2F {
                                x: ox - 9.0 + dx,
                                y: oy - 6.0,
                            },
                            D2D_POINT_2F {
                                x: ox + 1.0 + dx,
                                y: oy - 0.8,
                            },
                            D2D_POINT_2F {
                                x: ox + 9.0 + dx,
                                y: oy + 5.0,
                            },
                        ],
                        D2D1_COLOR_F {
                            r: 1.0,
                            g: 0.50,
                            b: 0.56,
                            a: sa * 0.9,
                        },
                        1.1,
                    );
                }
            }
        }
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

        // ── 1. Basis kepala ────────────────────────────────────────────────
        // Wajah: loop mulus dengan dagu & tulang pipi (bukan kotak yang di-resize).
        let skin = lerp_color(col(0.91, 0.79, 0.70), col(0.84, 0.72, 0.70), sk);
        let skin_shade = col(0.60, 0.46, 0.42);
        draw_smooth_face(
            rt,
            factory,
            &local(&[
                (0.0, -10.6),
                (10.2, -9.2),
                (17.0, -5.4),
                (20.0, 0.6),
                (17.2, 7.2),
                (10.4, 12.0),
                (3.6, 14.2),
                (-3.6, 14.2),
                (-10.4, 12.0),
                (-17.2, 7.2),
                (-20.0, 0.6),
                (-17.0, -5.4),
                (-10.2, -9.2),
            ]),
            skin,
            skin_shade,
            alpha,
        );
        let rim = lerp_color(cyan, crimson, sk);
        draw_soft_radial_glow(
            rt,
            at(-19.0, 1.5),
            9.0,
            12.0,
            rim.r,
            rim.g,
            rim.b,
            0.16 * alpha,
        );
        draw_soft_radial_glow(
            rt,
            at(19.0, 1.5),
            9.0,
            12.0,
            rim.r,
            rim.g,
            rim.b,
            0.16 * alpha,
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
            if let Ok(b) = rt.CreateSolidColorBrush(
                &D2D1_COLOR_F {
                    r: 0.02,
                    g: 0.01,
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
                                left: ex - 0.85,
                                top: eye_y - 2.6,
                                right: ex + 0.85,
                                bottom: eye_y + 2.6,
                            },
                            radiusX: 0.85,
                            radiusY: 1.2,
                        },
                        &b,
                    );
                }
            }
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
            let stroke = create_round_stroke(factory);
            if let Ok(rb) = rt.CreateSolidColorBrush(
                &D2D1_COLOR_F {
                    r: 1.0,
                    g: 0.18,
                    b: 0.30,
                    a: 0.42 * gg,
                },
                None,
            ) {
                rt.DrawEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F { x: sx, y: sy },
                        radiusX: 12.0 * s,
                        radiusY: 12.0 * s,
                    },
                    &rb,
                    0.9,
                    stroke.as_ref(),
                );
            }
        }
    }
}
