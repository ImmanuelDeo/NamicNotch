// src/mascot/asta.rs
// Fase 1: Asta (Devil Union) — detail penuh: rambut berlapis, tanduk domba,
// ikat kepala Black Bulls, pupil '+' anti-magic Liebe, pedang Demon-Slayer,
// grimoire Black Bulls melayang, kerah mantel, bara anti-magic.
use super::atlas;
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
            (0.30, 0.03, 0.07),
            (0.20, 0.02, 0.05),
            (1.00, 0.18, 0.28),
            0.80,
            alpha,
        );
        draw_soft_radial_glow(
            rt,
            D2D_POINT_2F { x: cx, y: cy },
            pw * 0.46 * breathe,
            ph * 1.00 * breathe,
            0.92,
            0.10,
            0.20,
            (0.34 * alpha).min(0.34),
        );
        draw_soft_radial_glow(
            rt,
            D2D_POINT_2F { x: cx, y: pill_rect.bottom + 2.0 },
            pw * 0.58,
            ph * 0.88,
            0.922,
            0.102,
            0.202,
            (0.342 * alpha).min(0.30),
        );
        // Partikel lembut, ukuran kecil, melayang pelan.
        vfx::draw_bokeh_motes(rt, pill_rect, 9, t, 12.4, (1.0, 0.32, 0.38), alpha * 0.95);
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

        // ── 1. Kepala HD (rambut, tanduk, ikat kepala) ──────────────────
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

        // Kepala HD digambar di atas mata: salib ikat kepala duduk di atas mata.
        atlas::draw_head(rt, atlas::Head::Asta, base_cx, base_cy - 7.0 * s, 104.0 * s, alpha);

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
