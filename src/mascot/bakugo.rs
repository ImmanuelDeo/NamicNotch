// src/mascot/bakugo.rs
// Fase 3: Katsuki Bakugo.
// Bahasa visual sama dengan Asta Devil Union: basis kepala gelap, rambut pirang berlapis,
// topeng mata hitam bergerigi, sirip flare hitam ber-outline oranye, kerah kostum +
// pelat dada, gauntlet granat dengan ledakan di kanan.
use super::core::*;
use windows::Win32::Graphics::Direct2D::Common::{D2D1_COLOR_F, D2D_POINT_2F, D2D_RECT_F};
use windows::Win32::Graphics::Direct2D::{
    ID2D1Factory, ID2D1RenderTarget, D2D1_ELLIPSE, D2D1_ROUNDED_RECT,
};

const SCALE: f32 = 0.78;

pub struct KatsukiBakugo;

impl KatsukiBakugo {
    pub fn new() -> Self {
        Self
    }

    fn blast_pulse(t: f32) -> f32 {
        let cycle = t % 2.6;
        if cycle < 0.22 {
            smoothstep(0.0, 0.22, cycle)
        } else if cycle < 0.95 {
            1.0 - smoothstep(0.22, 0.95, cycle)
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
        rt: &ID2D1RenderTarget,
        factory: &ID2D1Factory,
        pill_rect: D2D_RECT_F,
        alpha: f32,
        ctx: &MascotContext,
    ) {
        let blast = Self::blast_pulse(ctx.local_time);
        let pw = pill_rect.right - pill_rect.left;
        let ph = pill_rect.bottom - pill_rect.top;
        let cx = (pill_rect.left + pill_rect.right) * 0.5;
        let t = ctx.phase_time;

        // Glow amber di belakang kepala.
        draw_soft_radial_glow(
            rt,
            D2D_POINT_2F {
                x: cx,
                y: pill_rect.top + ph * 0.50,
            },
            pw * 0.32,
            ph * 0.54,
            1.0,
            0.55,
            0.16,
            (0.22 + 0.12 * blast) * alpha,
        );
        // Gradien bawah oranye.
        draw_bottom_gradient(
            rt,
            pill_rect,
            D2D1_COLOR_F {
                r: 1.0,
                g: 0.46,
                b: 0.10,
                a: alpha,
            },
            ph * 0.44,
            (0.60 + 0.20 * blast) * alpha,
        );

        // Ember melayang: ukuran & alfa bervariasi, gerak dua harmonisa.
        for i in 0..12 {
            let fi = i as f32;
            let seed = (fi * 0.371).fract();
            let life = ((t * (0.10 + 0.06 * seed)) + seed * 3.1) % 1.0;
            let a = (life * (1.0 - life) * 4.0).min(1.0) * alpha * (0.20 + 0.35 * seed);
            let ex = pill_rect.left + pw * (0.10 + 0.80 * ((fi * 0.271 + 0.07) % 1.0))
                + (t * 0.35 + fi * 1.7).sin() * 5.0;
            let ey = pill_rect.bottom - 4.0 - life * ph * 0.90;
            let size = 0.7 + seed * 1.5;
            draw_soft_circle(
                rt,
                D2D_POINT_2F { x: ex, y: ey },
                size,
                size,
                D2D1_COLOR_F {
                    r: 1.0,
                    g: 0.66 + 0.20 * seed,
                    b: 0.22,
                    a,
                },
            );
        }

        // Impact starburst: inti putih-panas + sinar oranye.
        let bx = cx + pw * 0.24;
        let by = pill_rect.top + ph * 0.44;
        let burst = 0.55 + 0.85 * blast;
        draw_soft_radial_glow(
            rt,
            D2D_POINT_2F { x: bx, y: by },
            17.0 * burst,
            15.0 * burst,
            1.0,
            0.62,
            0.24,
            (0.18 + 0.30 * blast) * alpha,
        );
        draw_soft_radial_glow(
            rt,
            D2D_POINT_2F { x: bx, y: by },
            7.0 * burst,
            6.0 * burst,
            1.0,
            0.97,
            0.86,
            (0.24 + 0.40 * blast) * alpha,
        );
        for k in 0..10 {
            let fk = k as f32;
            let ang = -1.9 + fk * 0.42 + (t * 0.3).sin() * 0.06;
            let inner = 4.0 * burst;
            let outer = (11.0 + (fk % 3.0) * 4.0) * burst;
            draw_soft_streak(
                rt,
                factory,
                &[
                    D2D_POINT_2F {
                        x: bx + ang.cos() * inner,
                        y: by + ang.sin() * inner,
                    },
                    D2D_POINT_2F {
                        x: bx + ang.cos() * (inner + outer) * 0.5,
                        y: by + ang.sin() * (inner + outer) * 0.5,
                    },
                    D2D_POINT_2F {
                        x: bx + ang.cos() * outer,
                        y: by + ang.sin() * outer,
                    },
                ],
                D2D1_COLOR_F {
                    r: 1.0,
                    g: 0.72 - 0.10 * (fk % 2.0),
                    b: 0.24,
                    a: (0.30 + 0.45 * blast) * alpha,
                },
                1.5,
            );
        }
        // Kepulan asap gelap di kiri & kanan ledakan.
        for side in [-1.0_f32, 1.0_f32] {
            draw_smoke_wisp(
                rt,
                pill_rect,
                bx + side * pw * 0.13,
                0.55,
                side,
                D2D1_COLOR_F {
                    r: 0.26,
                    g: 0.17,
                    b: 0.12,
                    a: alpha,
                },
                (0.14 + 0.10 * blast) * alpha,
                t * 0.85 + side,
                0.37 + side * 0.2,
            );
        }

        // Gelombang kejut lembut saat meledak.
        if blast > 0.08 {
            let stroke = create_round_stroke(factory);
            if let Ok(rb) = rt.CreateSolidColorBrush(
                &D2D1_COLOR_F {
                    r: 1.0,
                    g: 0.78,
                    b: 0.36,
                    a: blast * alpha * 0.32,
                },
                None,
            ) {
                let bx = cx + 28.0;
                let grow = 1.0 + (1.0 - blast) * 2.4;
                for k in 0..2 {
                    let fk = k as f32;
                    rt.DrawEllipse(
                        &D2D1_ELLIPSE {
                            point: D2D_POINT_2F {
                                x: bx,
                                y: pill_rect.top + ph * 0.42,
                            },
                            radiusX: (6.0 + fk * 4.0) * grow,
                            radiusY: (4.6 + fk * 3.0) * grow,
                        },
                        &rb,
                        0.8 - 0.2 * fk,
                        stroke.as_ref(),
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
        let blast = Self::blast_pulse(ctx.local_time);
        let s = SCALE;
        let base_cx = cx - blast * 0.6;
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
        let hair = col(0.92, 0.87, 0.72);
        let hair_hi = col(1.00, 0.96, 0.82);
        let hair_deep = col(0.66, 0.60, 0.44);
        let orange = col(1.00, 0.42, 0.10);
        let orange_hot = col(1.00, 0.72, 0.24);
        let costume = col(0.075, 0.080, 0.095);
        let costume_hi = col(0.18, 0.19, 0.22);
        let silver = col(0.78, 0.81, 0.84);
        let green = col(0.18, 0.35, 0.24);

        // ── 0. Kerah kostum + pelat dada + granat leher ───────────────────
        fill_polygon(
            rt,
            factory,
            &local(&[
                (-27.0, 13.5),
                (-22.0, 4.5),
                (-13.5, 7.0),
                (0.0, 9.0),
                (13.5, 7.0),
                (22.0, 4.5),
                (27.0, 13.5),
                (0.0, 19.0),
            ]),
            costume,
        );
        // Pelat dada bersudut (kostum musim panas).
        fill_polygon(
            rt,
            factory,
            &local(&[
                (-12.0, 10.0),
                (-6.0, 9.2),
                (0.0, 11.0),
                (6.0, 9.2),
                (12.0, 10.0),
                (9.0, 15.5),
                (0.0, 17.5),
                (-9.0, 15.5),
            ]),
            costume_hi,
        );
        // Garis X oranye di tengah dada.
        draw_polyline(
            rt,
            factory,
            &local(&[(-7.5, 10.6), (7.5, 15.4)]),
            D2D1_COLOR_F {
                a: 0.85 * alpha,
                ..orange
            },
            1.6,
        );
        draw_polyline(
            rt,
            factory,
            &local(&[(7.5, 10.6), (-7.5, 15.4)]),
            D2D1_COLOR_F {
                a: 0.85 * alpha,
                ..orange
            },
            1.6,
        );
        // Granat pengapian di sisi leher.
        for side in [-1.0_f32, 1.0_f32] {
            let gx = at(side * 15.0, 6.4);
            if let Ok(b) = rt.CreateSolidColorBrush(&silver, None) {
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: gx,
                        radiusX: 2.3 * s,
                        radiusY: 2.3 * s,
                    },
                    &b,
                );
            }
            if let Ok(b) = rt.CreateSolidColorBrush(&orange, None) {
                rt.DrawEllipse(
                    &D2D1_ELLIPSE {
                        point: gx,
                        radiusX: 2.3 * s,
                        radiusY: 2.3 * s,
                    },
                    &b,
                    0.9,
                    None,
                );
            }
        }

        // ── 1. Basis kepala ────────────────────────────────────────────────
        // Wajah: bentuk organik dengan dagu tegas.
        draw_smooth_face(
            rt,
            factory,
            &local(&[
                (0.0, -10.8),
                (10.6, -9.4),
                (17.4, -5.6),
                (20.2, 0.4),
                (17.6, 7.0),
                (11.0, 11.6),
                (4.0, 13.8),
                (-4.0, 13.8),
                (-11.0, 11.6),
                (-17.6, 7.0),
                (-20.2, 0.4),
                (-17.4, -5.6),
                (-10.6, -9.4),
            ]),
            col(0.94, 0.82, 0.74),
            col(0.64, 0.50, 0.44),
            alpha,
        );
        draw_soft_radial_glow(rt, at(-19.0, 1.5), 9.0, 12.0, 1.0, 0.45, 0.12, 0.16 * alpha);
        draw_soft_radial_glow(rt, at(19.0, 1.5), 9.0, 12.0, 1.0, 0.45, 0.12, 0.16 * alpha);

        // ── 2. Rambut pirang berlapis + sirip flare ────────────────────────
        let hair_pts = [
            (-23.5, -2.4),
            (-28.5, -7.4),
            (-22.5, -8.6),
            (-28.0, -14.0),
            (-19.0, -12.0),
            (-21.5, -19.5),
            (-12.5, -15.0),
            (-13.0, -21.5),
            (-5.0, -16.0),
            (-2.5, -22.5),
            (2.0, -17.8),
            (7.0, -21.4),
            (8.0, -15.6),
            (15.0, -19.6),
            (13.5, -13.4),
            (21.5, -16.4),
            (18.0, -10.4),
            (26.0, -11.0),
            (21.5, -6.0),
            (26.5, -2.6),
            (17.5, 0.4),
            (8.5, -3.0),
            (0.0, -0.2),
            (-9.0, -3.2),
            (-18.5, 0.2),
        ];
        fill_polygon(rt, factory, &local(&hair_pts), hair);
        for chunk in [
            [(-22.5, -11.6), (-14.0, -9.4), (-19.0, -5.4)],
            [(-13.0, -16.8), (-4.0, -12.6), (-9.0, -7.2)],
            [(-1.5, -19.6), (3.4, -13.8), (-0.4, -8.6)],
            [(9.0, -17.2), (6.6, -8.4), (1.0, -5.2)],
            [(19.8, -13.8), (15.4, -6.8), (9.6, -5.0)],
            [(-24.0, -6.4), (-17.4, -3.6), (-20.0, -1.6)],
        ] {
            fill_polygon(rt, factory, &local(&chunk), hair_hi);
        }
        // Bayangan akar rambut.
        fill_polygon(
            rt,
            factory,
            &local(&[
                (-16.0, -3.4),
                (-5.5, -7.8),
                (5.5, -7.8),
                (16.0, -3.4),
                (0.0, -1.2),
            ]),
            hair_deep,
        );
        draw_polyline(
            rt,
            factory,
            &local(&[
                (-23.5, -2.4),
                (-28.5, -7.4),
                (-22.5, -8.6),
                (-28.0, -14.0),
                (-19.0, -12.0),
                (-21.5, -19.5),
                (-12.5, -15.0),
                (-13.0, -21.5),
                (-5.0, -16.0),
                (-2.5, -22.5),
                (2.0, -17.8),
                (7.0, -21.4),
                (8.0, -15.6),
                (15.0, -19.6),
                (13.5, -13.4),
                (21.5, -16.4),
                (18.0, -10.4),
                (26.0, -11.0),
                (21.5, -6.0),
                (26.5, -2.6),
            ]),
            D2D1_COLOR_F {
                a: 0.55 * alpha,
                ..hair_hi
            },
            0.9,
        );

        // Sirip flare hitam bergerigi ber-outline oranye.
        for side in [-1.0_f32, 1.0_f32] {
            let fin = [
                (side * 14.0, -3.0),
                (side * 24.0, -8.0),
                (side * 19.0, -1.5),
                (side * 25.5, 2.0),
                (side * 17.5, 4.4),
                (side * 20.0, 8.0),
                (side * 12.5, 5.0),
            ];
            fill_polygon(rt, factory, &local(&fin), black);
            let mut outline = fin.to_vec();
            outline.push(fin[0]);
            draw_polyline(
                rt,
                factory,
                &local(&outline),
                D2D1_COLOR_F {
                    a: 0.95 * alpha,
                    ..orange
                },
                1.2,
            );
        }

        // ── 3. Topeng mata hitam bergerigi ─────────────────────────────────
        fill_polygon(
            rt,
            factory,
            &local(&[
                (-17.0, -2.6),
                (-7.0, -3.8),
                (0.0, -1.8),
                (7.0, -3.8),
                (17.0, -2.6),
                (15.5, 5.2),
                (6.0, 6.4),
                (0.0, 4.6),
                (-6.0, 6.4),
                (-15.5, 5.2),
            ]),
            black,
        );
        // Gerigi tepi topeng.
        let mut teeth = Vec::new();
        for k in 0..7 {
            let fk = k as f32;
            let x = -14.0 + fk * 4.6;
            teeth.push((x, 5.4));
            teeth.push((x + 2.3, 3.4));
        }
        draw_polyline(
            rt,
            factory,
            &local(&teeth),
            D2D1_COLOR_F {
                a: 0.55 * alpha,
                ..orange_hot
            },
            0.8,
        );
        // Aksen oranye tepi atas topeng.
        draw_polyline(
            rt,
            factory,
            &local(&[(-16.4, -2.4), (-7.0, -3.6), (0.0, -1.6), (7.0, -3.6), (16.4, -2.4)]),
            D2D1_COLOR_F {
                a: 0.45 * alpha,
                ..orange
            },
            0.9,
        );

        // ── 4. Mata merah-oranye garang ────────────────────────────────────
        let eye_y = base_cy + 5.0 * s;
        draw_capsule_eyes(
            rt,
            factory,
            base_cx,
            eye_y,
            9.0,
            col(1.00, 0.23, 0.08),
            col(1.00, 0.84, 0.50),
            (1.0, 0.30, 0.10),
            0.85 + 0.15 * blast,
            alpha,
            ctx,
        );

        // ── 5. Gauntlet granat + ledakan (kanan) ───────────────────────────
        let recoil = blast * 2.2;
        let gx = base_cx + 34.0 * s - recoil;
        let gy = base_cy + 4.0 * s;
        let burst = 0.45 + 0.9 * blast;

        fill_polygon(
            rt,
            factory,
            &[
                D2D_POINT_2F {
                    x: gx - 7.0 * s,
                    y: gy + 7.5 * s,
                },
                D2D_POINT_2F {
                    x: gx - 2.0 * s,
                    y: gy + 0.5 * s,
                },
                D2D_POINT_2F {
                    x: gx + 3.5 * s,
                    y: gy - 4.2 * s,
                },
                D2D_POINT_2F {
                    x: gx + 6.4 * s,
                    y: gy + 0.8 * s,
                },
                D2D_POINT_2F {
                    x: gx + 2.4 * s,
                    y: gy + 4.8 * s,
                },
                D2D_POINT_2F {
                    x: gx - 1.6 * s,
                    y: gy + 9.4 * s,
                },
            ],
            green,
        );
        draw_polyline(
            rt,
            factory,
            &[
                D2D_POINT_2F {
                    x: gx - 2.4 * s,
                    y: gy + 1.8 * s,
                },
                D2D_POINT_2F {
                    x: gx + 3.2 * s,
                    y: gy - 2.4 * s,
                },
            ],
            silver,
            1.1,
        );
        if let Ok(b) = rt.CreateSolidColorBrush(&silver, None) {
            rt.DrawEllipse(
                &D2D1_ELLIPSE {
                    point: D2D_POINT_2F {
                        x: gx - 4.2 * s,
                        y: gy + 6.2 * s,
                    },
                    radiusX: 1.7 * s,
                    radiusY: 1.7 * s,
                },
                &b,
                0.8,
                None,
            );
        }

        // Ledakan stylized oranye-emas.
        let bx = gx + 8.0 * s;
        let by = gy - 3.0 * s;
        draw_soft_radial_glow(
            rt,
            D2D_POINT_2F { x: bx, y: by },
            14.0 * burst,
            12.0 * burst,
            1.0,
            0.54,
            0.12,
            (0.30 + 0.55 * blast) * alpha,
        );
        let mut star = [D2D_POINT_2F { x: 0.0, y: 0.0 }; 16];
        for (i, pt) in star.iter_mut().enumerate() {
            let ang = (i as f32) * std::f32::consts::PI / 8.0 + ctx.phase_time * 0.5;
            let rad = if i % 2 == 0 {
                9.5 * burst * s
            } else {
                3.8 * burst * s
            };
            *pt = D2D_POINT_2F {
                x: bx + ang.cos() * rad,
                y: by + ang.sin() * rad,
            };
        }
        draw_soft_radial_glow(
            rt,
            D2D_POINT_2F { x: bx, y: by },
            12.0 * burst,
            12.0 * burst,
            1.0,
            0.92,
            0.62,
            (0.16 + 0.30 * blast) * alpha,
        );
        fill_polygon(
            rt,
            factory,
            &star,
            D2D1_COLOR_F {
                r: 1.0,
                g: 0.72,
                b: 0.24,
                a: (0.34 + 0.44 * blast) * alpha,
            },
        );
        fill_polygon(
            rt,
            factory,
            &star[..8],
            D2D1_COLOR_F {
                r: 1.0,
                g: 0.96,
                b: 0.78,
                a: (0.30 + 0.55 * blast) * alpha,
            },
        );
    }
}
