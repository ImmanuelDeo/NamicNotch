// src/mascot/asta.rs
// Fase 1: Asta (Devil Union) — detail penuh: rambut berlapis, tanduk domba,
// ikat kepala Black Bulls, pupil '+' anti-magic Liebe, pedang Demon-Slayer,
// grimoire Black Bulls melayang, kerah mantel, bara anti-magic.
use super::core::*;
use super::vfx;
use windows::Win32::Graphics::Direct2D::Common::{D2D1_COLOR_F, D2D_POINT_2F, D2D_RECT_F};
use windows::Win32::Graphics::Direct2D::{
    ID2D1Factory, ID2D1RenderTarget, D2D1_ELLIPSE, D2D1_ROUNDED_RECT,
};

const SCALE: f32 = 0.78;

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

    // ── AURA / ILUMINASI LATAR ─────────────────────────────────────────────
    unsafe fn draw_aura(
        &self,
        rt: &ID2D1RenderTarget,
        factory: &ID2D1Factory,
        pill_rect: D2D_RECT_F,
        alpha: f32,
        ctx: &MascotContext,
    ) {
        let (grab, pull) = Self::sword_choreography(ctx.local_time);
        let pw = pill_rect.right - pill_rect.left;
        let ph = pill_rect.bottom - pill_rect.top;
        let cx = (pill_rect.left + pill_rect.right) * 0.5;
        let t = ctx.phase_time;
        let boost = 1.0 + pull * 0.45 + grab * 0.15;

        vfx::draw_ambient_notch_illumination(
            rt,
            pill_rect,
            (0.34, 0.03, 0.07),
            (0.22, 0.02, 0.05),
            (1.00, 0.18, 0.28),
            0.85 + 0.35 * grab,
            alpha,
        );
        vfx::draw_bokeh_motes(rt, pill_rect, 10, t, 12.4, (1.0, 0.30, 0.36), alpha);

        // 1. Pendar dasar crimson dari bawah notch.
        draw_soft_radial_glow(
            rt,
            D2D_POINT_2F {
                x: cx,
                y: pill_rect.bottom + 1.0,
            },
            pw * 0.52,
            ph * 0.82,
            0.90,
            0.09,
            0.20,
            (0.34 * alpha * boost).min(0.58),
        );

        // 2. Berkas sinar di belakang kepala (iluminasi environment).
        draw_back_rays(
            rt,
            factory,
            cx,
            pill_rect.bottom - 1.0,
            9,
            1.05,
            ph * 1.25,
            t * 0.55,
            D2D1_COLOR_F {
                r: 0.95,
                g: 0.13,
                b: 0.24,
                a: 0.085 * alpha * boost,
            },
        );

        // 3. Halo inti di belakang kepala.
        draw_soft_radial_glow(
            rt,
            D2D_POINT_2F {
                x: cx,
                y: pill_rect.top + ph * 0.62,
            },
            pw * 0.20,
            ph * 0.62,
            1.0,
            0.28,
            0.36,
            (0.20 * alpha * boost).min(0.34),
        );

        // 4. Kilat crimson acak + lonjakan saat pedang ditarik.
        let flash = ((t * 12.0).sin() + (t * 21.0).cos()) * 0.5;
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

            for (i, ax) in [-pw * 0.27, pw * 0.25].iter().enumerate() {
                let shift = (t * 14.0 + i as f32 * 2.1).sin() * 3.5;
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

        // 5. Bara anti-magic naik dari dasar notch.
        draw_rising_particles(
            rt,
            cx,
            pill_rect.bottom - 1.0,
            pw * 0.40,
            ph * 0.95,
            t,
            11,
            D2D1_COLOR_F {
                r: 1.0,
                g: 0.24,
                b: 0.34,
                a: 0.42 * alpha,
            },
            1.05,
        );

        // 5b. Crimson thunder + asap anti-magic (referensi efek Asta).
        for k in 0..2 {
            let fk = k as f32;
            let y = pill_rect.top + ph * (0.20 + 0.30 * fk);
            let pulse = 0.40 + 0.40 * (t * (1.3 + fk * 0.4) + fk).sin().abs();
            vfx::draw_tapered_anime_lightning(
                rt,
                factory,
                D2D_POINT_2F {
                    x: pill_rect.left + pw * 0.04,
                    y,
                },
                D2D_POINT_2F {
                    x: pill_rect.left + pw * 0.58,
                    y: y + ph * 0.12,
                },
                t,
                fk * 3.1,
                (1.0, 0.22, 0.40),
                0.55 * alpha * pulse,
                1.1,
                3,
            );
        }
        draw_smoke_wisp(
            rt,
            pill_rect,
            cx + pw * 0.26,
            0.72,
            1.0,
            D2D1_COLOR_F {
                r: 0.42,
                g: 0.05,
                b: 0.12,
                a: alpha,
            },
            0.15 * alpha,
            t,
            0.41,
        );

        // 6. Garis cahaya di bibir bawah notch (refleksi lantai).
        draw_soft_radial_glow(
            rt,
            D2D_POINT_2F {
                x: cx,
                y: pill_rect.bottom - 0.5,
            },
            pw * 0.45,
            2.4,
            1.0,
            0.55,
            0.60,
            (0.30 * alpha).min(0.40),
        );
    }

    // ── KARAKTER ───────────────────────────────────────────────────────────
    unsafe fn draw_character(
        &self,
        rt: &ID2D1RenderTarget,
        factory: &ID2D1Factory,
        cx: f32,
        cy: f32,
        alpha: f32,
        ctx: &MascotContext,
    ) {
        let (grab, pull) = Self::sword_choreography(ctx.local_time);
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
        let dark = col(0.060, 0.064, 0.082);
        let hair = col(0.120, 0.130, 0.160);
        let hair_hi = col(0.330, 0.360, 0.415);
        let red = col(1.000, 0.140, 0.260);
        let red_core = col(1.000, 0.820, 0.880);
        let red_deep = col(0.520, 0.050, 0.120);
        let gold = col(0.980, 0.820, 0.280);

        // ── 0. Kerah mantel Black Bulls + bahu ─────────────────────────────
        fill_polygon(
            rt,
            factory,
            &local(&[
                (-26.0, 12.5),
                (-21.0, 4.0),
                (-12.0, 6.5),
                (0.0, 9.0),
                (12.0, 6.5),
                (21.0, 4.0),
                (26.0, 12.5),
                (0.0, 17.5),
            ]),
            black,
        );
        draw_polyline(
            rt,
            factory,
            &local(&[
                (-21.0, 4.0),
                (-12.0, 6.5),
                (0.0, 9.0),
                (12.0, 6.5),
                (21.0, 4.0),
            ]),
            D2D1_COLOR_F {
                a: 0.55 * alpha,
                ..red_deep
            },
            1.1,
        );
        draw_polyline(
            rt,
            factory,
            &local(&[(-25.0, 12.0), (-20.5, 4.4)]),
            D2D1_COLOR_F {
                a: 0.70 * alpha,
                ..red
            },
            1.2,
        );
        draw_polyline(
            rt,
            factory,
            &local(&[(25.0, 12.0), (20.5, 4.4)]),
            D2D1_COLOR_F {
                a: 0.70 * alpha,
                ..red
            },
            1.2,
        );

        // ── 1. Dasar kepala / visor hitam pekat ─────────────────────────────
        if let Ok(b) = rt.CreateSolidColorBrush(&black, None) {
            rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: D2D_RECT_F {
                        left: base_cx - 21.0 * s,
                        top: base_cy - 9.0 * s,
                        right: base_cx + 21.0 * s,
                        bottom: base_cy + 13.0 * s,
                    },
                    radiusX: 10.0 * s,
                    radiusY: 10.0 * s,
                },
                &b,
            );
        }
        // Rim light crimson di sisi luar kepala.
        draw_soft_radial_glow(rt, at(-20.0, 2.0), 9.0, 12.0, 0.95, 0.12, 0.22, 0.16 * alpha);
        draw_soft_radial_glow(rt, at(20.0, 2.0), 9.0, 12.0, 0.95, 0.12, 0.22, 0.16 * alpha);

        // ── 2. Rambut spiky berlapis + cowlick ──────────────────────────────
        let hair_pts = [
            (-23.0, -3.0),
            (-28.0, -8.0),
            (-20.0, -9.0),
            (-25.0, -15.0),
            (-14.0, -12.0),
            (-16.0, -19.0),
            (-6.0, -15.0),
            (-2.0, -22.0),
            (2.0, -18.0),
            (1.0, -14.0),
            (10.0, -19.0),
            (10.0, -14.0),
            (21.0, -17.0),
            (18.0, -11.0),
            (26.0, -12.0),
            (22.0, -6.0),
            (25.0, -3.0),
            (16.0, 0.0),
            (8.0, -3.0),
            (0.0, 0.0),
            (-9.0, -3.0),
            (-18.0, 0.0),
        ];
        fill_polygon(rt, factory, &local(&hair_pts), hair);
        // Highlight volume → rambut rapi berlapis, bukan blob.
        for chunk in [
            [(-23.0, -13.0), (-12.0, -10.0), (-17.0, -6.0)],
            [(-13.0, -17.0), (-4.0, -12.0), (-9.0, -7.0)],
            [(-1.0, -20.0), (3.0, -14.0), (0.0, -9.0)],
            [(10.0, -17.0), (7.0, -8.0), (1.0, -5.0)],
            [(20.0, -15.0), (16.0, -7.0), (10.0, -5.0)],
            [(-24.0, -7.0), (-17.0, -4.0), (-20.0, -2.0)],
        ] {
            fill_polygon(rt, factory, &local(&chunk), hair_hi);
        }
        // Siluet luar rambut.
        draw_polyline(
            rt,
            factory,
            &local(&[
                (-23.0, -3.0),
                (-28.0, -8.0),
                (-20.0, -9.0),
                (-25.0, -15.0),
                (-14.0, -12.0),
                (-16.0, -19.0),
                (-6.0, -15.0),
                (-2.0, -22.0),
                (2.0, -18.0),
                (10.0, -19.0),
                (21.0, -17.0),
                (26.0, -12.0),
                (25.0, -3.0),
            ]),
            D2D1_COLOR_F {
                a: 0.60 * alpha,
                ..hair_hi
            },
            0.9,
        );

        // ── 3. Tanduk domba Devil Union ─────────────────────────────────────
        for side in [-1.0_f32, 1.0_f32] {
            let hx = base_cx + side * 20.0 * s;
            let hy = base_cy - 8.5 * s;
            if let Ok(b) = rt.CreateSolidColorBrush(&dark, None) {
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F { x: hx, y: hy },
                        radiusX: 5.6 * s,
                        radiusY: 5.8 * s,
                    },
                    &b,
                );
            }
            if let Ok(hb) = rt.CreateSolidColorBrush(&hair_hi, None) {
                rt.DrawEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F { x: hx, y: hy },
                        radiusX: 5.6 * s,
                        radiusY: 5.8 * s,
                    },
                    &hb,
                    1.0,
                    None,
                );
            }
            draw_polyline(
                rt,
                factory,
                &[
                    D2D_POINT_2F {
                        x: hx - side * 2.8 * s,
                        y: hy + 2.8 * s,
                    },
                    D2D_POINT_2F {
                        x: hx + side * 2.4 * s,
                        y: hy + 1.8 * s,
                    },
                    D2D_POINT_2F {
                        x: hx + side * 2.8 * s,
                        y: hy - 1.8 * s,
                    },
                    D2D_POINT_2F { x: hx, y: hy - 2.8 * s },
                    D2D_POINT_2F { x: hx - side * s, y: hy },
                ],
                hair,
                2.0,
            );
            for j in 0..3 {
                let y = hy - 2.8 * s + j as f32 * 2.2 * s;
                draw_polyline(
                    rt,
                    factory,
                    &[
                        D2D_POINT_2F {
                            x: hx + side * 3.9 * s,
                            y,
                        },
                        D2D_POINT_2F {
                            x: hx + side * 5.3 * s,
                            y: y - 0.6 * s,
                        },
                    ],
                    D2D1_COLOR_F {
                        a: 0.85 * alpha,
                        ..red
                    },
                    0.75,
                );
            }
        }

        // ── 4. Ikat kepala: pita + salib + emblem emas + mahkota ────────────
        fill_polygon(
            rt,
            factory,
            &local(&[
                (-19.0, -7.0),
                (-9.0, -6.0),
                (0.0, -7.0),
                (19.0, -7.0),
                (20.0, -2.0),
                (9.0, -1.0),
                (0.0, -2.0),
                (-20.0, -2.0),
            ]),
            dark,
        );
        // Salib merah Black Bulls (kiri).
        draw_polyline(rt, factory, &local(&[(-16.0, -5.7), (-10.0, -2.8)]), red, 1.5);
        draw_polyline(rt, factory, &local(&[(-12.0, -6.0), (-14.0, -2.0)]), red, 1.5);
        // Emblem emas (kanan).
        draw_polyline(
            rt,
            factory,
            &local(&[
                (10.0, -6.0),
                (10.5, -3.5),
                (13.0, -2.5),
                (15.5, -3.5),
                (16.0, -6.0),
            ]),
            gold,
            1.1,
        );
        fill_polygon(
            rt,
            factory,
            &local(&[(11.0, -4.8), (15.0, -4.8), (14.0, -1.6), (12.0, -1.6)]),
            gold,
        );
        // Poni perak tengah.
        fill_polygon(
            rt,
            factory,
            &local(&[(-3.0, -3.0), (0.0, 1.0), (3.0, -3.0)]),
            hair_hi,
        );
        // Mahkota zigzag crimson Devil Union.
        draw_polyline(
            rt,
            factory,
            &local(&[
                (-18.0, -4.5),
                (-10.0, -3.5),
                (-5.0, -5.0),
                (0.0, -9.0),
                (5.0, -5.0),
                (10.0, -3.5),
                (18.0, -4.5),
            ]),
            red,
            1.6,
        );
        draw_polyline(
            rt,
            factory,
            &local(&[(-4.0, -7.0), (0.0, -2.0), (4.0, -7.0)]),
            red,
            1.6,
        );
        draw_polyline(
            rt,
            factory,
            &local(&[(-3.0, -7.0), (0.0, -3.0), (3.0, -7.0)]),
            red_core,
            0.8,
        );

        // ── 5. Mata capsule baku (tanpa pupil '+' / tanda silang) ──────────
        let grab_now = grab;
        draw_capsule_eyes(
            rt,
            factory,
            base_cx,
            base_cy + 5.0 * s,
            9.2,
            red,
            D2D1_COLOR_F {
                r: 1.0,
                g: 0.72,
                b: 0.78,
                a: alpha,
            },
            (1.0, 0.14, 0.24),
            0.15 + 0.75 * grab_now,
            alpha,
            ctx,
        );

        // ── 6. Grimoire Black Bulls melayang (kiri) ─────────────────────────
        let gb = ctx.phase_time * 1.5;
        let bob = gb.sin() * 1.1;
        let tilt = gb.cos() * 0.10;
        let gx = base_cx - 34.0 * s;
        let gy = base_cy + 2.2 * s + bob;

        draw_soft_radial_glow(
            rt,
            D2D_POINT_2F { x: gx, y: gy },
            12.0,
            12.0,
            1.0,
            0.20,
            0.30,
            0.30 * alpha,
        );

        let cover = |ox: f32, oy: f32| D2D_POINT_2F {
            x: gx + (ox * tilt.cos() - oy * tilt.sin()) * s,
            y: gy + (ox * tilt.sin() + oy * tilt.cos()) * s,
        };
        fill_polygon(
            rt,
            factory,
            &[
                cover(-6.5, -8.0),
                cover(6.5, -8.0),
                cover(6.5, 8.0),
                cover(-6.5, 8.0),
            ],
            col(0.055, 0.060, 0.080),
        );
        // Halaman (kilau krem di tepi kanan).
        fill_polygon(
            rt,
            factory,
            &[
                cover(4.6, -7.4),
                cover(6.5, -7.4),
                cover(6.5, 7.4),
                cover(4.6, 7.4),
            ],
            col(0.78, 0.74, 0.60),
        );
        draw_polyline(
            rt,
            factory,
            &[
                cover(-6.5, -8.0),
                cover(6.5, -8.0),
                cover(6.5, 8.0),
                cover(-6.5, 8.0),
                cover(-6.5, -8.0),
            ],
            D2D1_COLOR_F {
                a: 0.85 * alpha,
                ..red_deep
            },
            1.0,
        );
        // Clover 5 daun emas.
        let clover_c = cover(-0.8, 0.0);
        for ang in [0.0_f32, 0.6283, 1.2566, 1.8849, 2.5132] {
            fill_polygon(
                rt,
                factory,
                &[
                    D2D_POINT_2F {
                        x: clover_c.x + ang.cos() * 1.1 * s,
                        y: clover_c.y + ang.sin() * 1.1 * s,
                    },
                    D2D_POINT_2F {
                        x: clover_c.x + (ang + 0.45).cos() * 3.1 * s,
                        y: clover_c.y + (ang + 0.45).sin() * 3.1 * s,
                    },
                    D2D_POINT_2F {
                        x: clover_c.x + (ang + 0.9).cos() * 1.1 * s,
                        y: clover_c.y + (ang + 0.9).sin() * 1.1 * s,
                    },
                ],
                gold,
            );
        }
        draw_soft_circle(rt, clover_c, 1.0 * s, 1.0 * s, red_deep);

        // ── 7. Pedang Demon-Slayer + tangan menggenggam (kanan) ─────────────
        let float_y = (ctx.phase_time * 2.8).sin() * 1.6 * (1.0 - grab);
        let sx = base_cx + 31.0 * s - pull * 3.0 * s;
        let sy = base_cy + 4.5 * s + float_y + pull * 3.5 * s;
        let sw = |pts: &[(f32, f32)]| -> Vec<D2D_POINT_2F> {
            pts.iter()
                .map(|&(x, y)| D2D_POINT_2F {
                    x: sx + x * s,
                    y: sy + y * s,
                })
                .collect()
        };

        draw_soft_radial_glow(
            rt,
            D2D_POINT_2F {
                x: sx + 6.0 * s,
                y: sy - 8.0 * s,
            },
            16.0,
            16.0,
            1.0,
            0.12,
            0.24,
            (0.30 + pull * 0.30) * alpha,
        );

        fill_polygon(
            rt,
            factory,
            &sw(&[
                (-3.0, -3.0),
                (10.0, -22.0),
                (19.0, -25.0),
                (21.0, -17.0),
                (5.0, 1.0),
            ]),
            dark,
        );
        draw_polyline(
            rt,
            factory,
            &sw(&[
                (-3.0, -3.0),
                (10.0, -22.0),
                (19.0, -25.0),
                (21.0, -17.0),
                (5.0, 1.0),
                (-3.0, -3.0),
            ]),
            hair_hi,
            1.0,
        );
        // Retakan cahaya anti-magic di bilah.
        for (colr, w) in [(red, 1.4_f32), (red_core, 0.7)] {
            draw_polyline(
                rt,
                factory,
                &sw(&[
                    (2.0, -3.0),
                    (8.0, -10.0),
                    (6.0, -11.0),
                    (13.0, -15.0),
                    (12.0, -19.0),
                    (17.0, -22.0),
                ]),
                colr,
                w,
            );
        }
        draw_polyline(
            rt,
            factory,
            &sw(&[(8.0, -10.0), (13.0, -10.0), (16.0, -15.0)]),
            red,
            1.0,
        );

        // Crossguard + gagang + pommel.
        draw_polyline(rt, factory, &sw(&[(-6.0, -4.0), (7.0, 4.0)]), hair, 3.0);
        draw_polyline(rt, factory, &sw(&[(-1.0, 1.0), (-6.0, 8.0)]), hair, 3.2);
        for j in 0..3 {
            let k = j as f32;
            draw_polyline(
                rt,
                factory,
                &sw(&[(-2.0 - k, 2.0 + k * 1.5), (-0.5 - k, 3.0 + k * 1.5)]),
                hair_hi,
                0.8,
            );
        }
        if let Ok(b) = rt.CreateSolidColorBrush(&hair, None) {
            rt.DrawEllipse(
                &D2D1_ELLIPSE {
                    point: D2D_POINT_2F {
                        x: sx - 7.0 * s,
                        y: sy + 9.0 * s,
                    },
                    radiusX: 2.2 * s,
                    radiusY: 2.2 * s,
                },
                &b,
                1.4,
                None,
            );
        }

        // Tangan meraih & menggenggam.
        if grab > 0.02 {
            let hx = lerp(base_cx + 19.0 * s, sx - 3.0 * s, grab);
            let hy = lerp(base_cy + 10.0 * s, sy + 4.5 * s, grab);
            let hand = [
                D2D_POINT_2F {
                    x: hx - 3.2 * s,
                    y: hy + 3.2 * s,
                },
                D2D_POINT_2F {
                    x: hx - 4.2 * s,
                    y: hy,
                },
                D2D_POINT_2F {
                    x: hx - 3.2 * s,
                    y: hy - 3.2 * s,
                },
                D2D_POINT_2F {
                    x: hx,
                    y: hy - 3.8 * s,
                },
                D2D_POINT_2F {
                    x: hx + 2.2 * s,
                    y: hy - 1.6 * s,
                },
                D2D_POINT_2F {
                    x: hx + 4.2 * s,
                    y: hy - 2.2 * s,
                },
                D2D_POINT_2F {
                    x: hx + 4.2 * s,
                    y: hy + 1.2 * s,
                },
                D2D_POINT_2F {
                    x: hx + 1.6 * s,
                    y: hy + 3.8 * s,
                },
            ];
            draw_soft_radial_glow(
                rt,
                D2D_POINT_2F { x: hx, y: hy },
                10.0,
                10.0,
                1.0,
                0.20,
                0.32,
                0.55 * alpha * grab,
            );
            fill_polygon(rt, factory, &hand, black);
            draw_polyline(
                rt,
                factory,
                &hand,
                D2D1_COLOR_F {
                    a: alpha * grab,
                    ..red
                },
                1.5,
            );
            draw_polyline(
                rt,
                factory,
                &[
                    D2D_POINT_2F {
                        x: hx - 2.0 * s,
                        y: hy - 1.0 * s,
                    },
                    D2D_POINT_2F {
                        x: hx + 1.0 * s,
                        y: hy,
                    },
                    D2D_POINT_2F {
                        x: hx + 1.5 * s,
                        y: hy + 1.5 * s,
                    },
                ],
                D2D1_COLOR_F {
                    a: alpha * grab,
                    ..red_core
                },
                1.1,
            );
        }

        // ── 8. Rune anti-magic melayang ─────────────────────────────────────
        for (i, (bx, by, sp)) in [(-24.0_f32, -4.0_f32, 0.9_f32), (16.0, -12.0, 1.3)]
            .iter()
            .enumerate()
        {
            let drift = (ctx.phase_time * sp + i as f32 * 1.7).sin();
            let px = base_cx + (bx + drift * 1.6) * s;
            let py = base_cy + (by + drift * 1.2) * s;
            let a = (0.35 + 0.35 * drift.abs()) * alpha;
            let arm = 1.5 * s;
            for pts in [
                [
                    D2D_POINT_2F { x: px - arm, y: py },
                    D2D_POINT_2F { x: px + arm, y: py },
                ],
                [
                    D2D_POINT_2F {
                        x: px,
                        y: py - arm,
                    },
                    D2D_POINT_2F {
                        x: px,
                        y: py + arm,
                    },
                ],
            ] {
                draw_polyline(rt, factory, &pts, D2D1_COLOR_F { a, ..red }, 0.9);
            }
        }
    }
}
