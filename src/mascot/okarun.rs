// src/mascot/okarun.rs
// Fase 5: Okarun (Turbo Granny Mode).
// Bahasa visual sama dengan Asta Devil Union: basis kepala gelap, rambut api berlapis
// (putih-perak + aksen crimson), kacamata bulat tipis, garis yokai, kerah hoodie,
// mata illuminated, speed-slash merah-putih di kanan.
use super::core::*;
use super::vfx;
use windows::Win32::Graphics::Direct2D::Common::{D2D1_COLOR_F, D2D_POINT_2F, D2D_RECT_F};
use windows::Win32::Graphics::Direct2D::{
    ID2D1Factory, ID2D1RenderTarget, D2D1_ELLIPSE, D2D1_ROUNDED_RECT,
};

const SCALE: f32 = 0.78;

pub struct OkarunTurbo;

impl OkarunTurbo {
    pub fn new() -> Self {
        Self
    }

    fn dash_pulse(t: f32) -> f32 {
        let cycle = t % 2.8;
        if cycle < 0.35 {
            smoothstep(0.0, 0.35, cycle)
        } else if cycle < 1.15 {
            1.0 - smoothstep(0.35, 1.15, cycle)
        } else {
            0.0
        }
    }
}

impl Default for OkarunTurbo {
    fn default() -> Self {
        Self::new()
    }
}

impl IdleCharacter for OkarunTurbo {
    fn id(&self) -> &'static str {
        "okarun_turbo_granny"
    }

    fn duration_secs(&self) -> f32 {
        10.0
    }

    fn override_look_target(&self, ctx: &MascotContext) -> Option<(f32, f32)> {
        let dash = Self::dash_pulse(ctx.local_time);
        if dash > 0.1 {
            Some((2.4 * dash, -0.2 * dash))
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
        let dash = Self::dash_pulse(ctx.local_time);
        let pw = pill_rect.right - pill_rect.left;
        let ph = pill_rect.bottom - pill_rect.top;
        let cx = (pill_rect.left + pill_rect.right) * 0.5;
        let cy = (pill_rect.top + pill_rect.bottom) * 0.5;
        let t = ctx.phase_time;

        vfx::draw_ambient_notch_illumination(
            rt,
            pill_rect,
            (0.42, 0.10, 0.14),
            (0.30, 0.04, 0.08),
            (1.00, 0.24, 0.34),
            0.75 + 0.45 * dash,
            alpha,
        );
        vfx::draw_bokeh_motes(rt, pill_rect, 12, t, 58.6, (1.0, 0.34, 0.42), alpha);

        // Dua pita kecepatan melengkung selebar notch.
        let shift = (t * 3.5).sin() * 4.0;
        vfx::draw_tapered_flame_ribbon(
            rt,
            factory,
            D2D_POINT_2F {
                x: pill_rect.left + 8.0,
                y: pill_rect.bottom - 4.0,
            },
            D2D_POINT_2F {
                x: cx - pw * 0.18,
                y: pill_rect.top + 2.0 + shift,
            },
            D2D_POINT_2F {
                x: cx + pw * 0.16,
                y: pill_rect.bottom - 2.0 - shift,
            },
            D2D_POINT_2F {
                x: pill_rect.right - 8.0,
                y: pill_rect.top + 6.0,
            },
            4.6 + 2.2 * dash,
            (0.92, 0.12, 0.22),
            (0.30 + 0.28 * dash) * alpha,
            false,
        );
        vfx::draw_tapered_flame_ribbon(
            rt,
            factory,
            D2D_POINT_2F {
                x: pill_rect.left + pw * 0.16,
                y: pill_rect.bottom - 2.0,
            },
            D2D_POINT_2F {
                x: cx - pw * 0.06,
                y: pill_rect.top + 4.0 - shift * 0.6,
            },
            D2D_POINT_2F {
                x: cx + pw * 0.22,
                y: pill_rect.bottom - 5.0 + shift * 0.6,
            },
            D2D_POINT_2F {
                x: pill_rect.right - 6.0,
                y: pill_rect.top + 4.0,
            },
            3.0 + 1.6 * dash,
            (1.0, 0.96, 0.98),
            (0.42 + 0.24 * dash) * alpha,
            true,
        );

        // Arc mikro di tepi kanan.
        vfx::draw_tapered_anime_lightning(
            rt,
            factory,
            D2D_POINT_2F {
                x: cx + pw * 0.14,
                y: pill_rect.bottom - 6.0,
            },
            D2D_POINT_2F {
                x: pill_rect.right - 12.0,
                y: cy - 4.0,
            },
            t,
            27.9,
            (1.0, 0.18, 0.28),
            (0.28 + 0.32 * dash) * alpha,
            1.15,
            2,
        );
        let _ = ph;
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
        let dash = Self::dash_pulse(ctx.local_time);
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
        let silver_hair = col(0.94, 0.92, 0.90);
        let silver_hi = col(1.00, 0.99, 0.97);
        let crimson = col(0.86, 0.20, 0.24);
        let crimson_hi = col(0.98, 0.42, 0.44);
        let hoodie = col(0.070, 0.072, 0.085);
        let hoodie_hi = col(0.16, 0.17, 0.20);
        let glass = col(0.72, 0.76, 0.82);
        let eye_red = col(1.00, 0.22, 0.18);
        let eye_core = col(1.00, 0.80, 0.68);

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
                r: 0.052,
                g: 0.052,
                b: 0.058,
                a: alpha * 0.92,
            },
        );

        // ── 0. Kerah hoodie + tali serut merah ────────────────────────────
        fill_polygon(
            rt,
            factory,
            &local(&[
                (-27.5, 13.5),
                (-22.0, 4.5),
                (-13.0, 7.4),
                (0.0, 9.6),
                (13.0, 7.4),
                (22.0, 4.5),
                (27.5, 13.5),
                (0.0, 19.5),
            ]),
            hoodie,
        );
        draw_polyline(
            rt,
            factory,
            &local(&[(-22.0, 4.6), (-13.0, 7.5), (0.0, 9.8), (13.0, 7.5), (22.0, 4.6)]),
            D2D1_COLOR_F {
                a: 0.55 * alpha,
                ..hoodie_hi
            },
            1.0,
        );
        for side in [-1.0_f32, 1.0_f32] {
            draw_polyline(
                rt,
                factory,
                &local(&[(side * 3.4, 9.6), (side * 4.2, 14.4)]),
                D2D1_COLOR_F {
                    a: 0.9 * alpha,
                    ..crimson
                },
                1.1,
            );
        }

        // ── 2. Rambut api putih + lapisan crimson ──────────────────────────
        // Lapisan merah di belakang (aksen yokai).
        fill_polygon(
            rt,
            factory,
            &local(&[
                (-24.0, -2.0),
                (-27.5, -9.5),
                (-20.0, -10.5),
                (-23.5, -16.5),
                (-14.5, -14.0),
                (-15.5, -20.5),
                (-7.0, -16.0),
                (-6.0, -22.5),
                (1.0, -17.5),
                (5.0, -21.5),
                (7.0, -15.5),
                (14.5, -18.5),
                (13.0, -12.5),
                (21.0, -14.0),
                (17.5, -8.5),
                (24.5, -6.5),
                (18.0, -1.5),
                (8.0, -4.0),
                (0.0, -1.0),
                (-9.0, -4.0),
                (-18.0, -1.0),
            ]),
            D2D1_COLOR_F {
                a: 0.85 * alpha,
                ..crimson
            },
        );
        // Lapisan putih-perak di atas (flame tipis).
        let white_pts = [
            (-21.0, -3.0),
            (-24.0, -9.0),
            (-18.5, -9.5),
            (-21.0, -14.5),
            (-13.5, -12.0),
            (-13.0, -18.0),
            (-6.5, -13.5),
            (-4.5, -19.5),
            (1.0, -14.5),
            (5.5, -18.5),
            (7.0, -12.5),
            (13.5, -15.5),
            (13.0, -9.5),
            (20.0, -11.5),
            (17.0, -5.5),
            (22.5, -3.0),
            (15.0, 0.5),
            (7.0, -2.8),
            (0.0, 0.2),
            (-8.0, -2.8),
            (-16.0, 0.6),
        ];
        fill_polygon(rt, factory, &local(&white_pts), silver_hair);
        for chunk in [
            [(-19.0, -10.5), (-11.5, -9.0), (-15.5, -5.0)],
            [(-10.0, -14.5), (-2.5, -11.0), (-7.0, -6.5)],
            [(-1.0, -16.5), (3.0, -11.0), (-1.0, -7.0)],
            [(8.0, -14.0), (6.0, -7.5), (1.0, -4.5)],
            [(17.0, -10.0), (14.0, -4.5), (9.0, -3.5)],
        ] {
            fill_polygon(rt, factory, &local(&chunk), silver_hi);
        }
        // Sela rambut merah (streak).
        for side in [-1.0_f32, 1.0_f32] {
            draw_polyline(
                rt,
                factory,
                &local(&[
                    (side * 8.0, -14.5),
                    (side * 10.5, -9.0),
                    (side * 9.0, -4.5),
                ]),
                D2D1_COLOR_F {
                    a: 0.75 * alpha,
                    ..crimson_hi
                },
                1.0,
            );
        }
        draw_polyline(
            rt,
            factory,
            &local(&[
                (-21.0, -3.0),
                (-24.0, -9.0),
                (-18.5, -9.5),
                (-21.0, -14.5),
                (-13.5, -12.0),
                (-13.0, -18.0),
                (-6.5, -13.5),
                (-4.5, -19.5),
                (1.0, -14.5),
                (5.5, -18.5),
                (7.0, -12.5),
                (13.5, -15.5),
                (13.0, -9.5),
                (20.0, -11.5),
                (17.0, -5.5),
                (22.5, -3.0),
            ]),
            D2D1_COLOR_F {
                a: 0.5 * alpha,
                ..silver_hi
            },
            0.9,
        );

        // ── 3. Garis yokai vertikal merah ──────────────────────────────────
        let eye_y = base_cy + 5.0 * s;
        let stroke = create_round_stroke(factory);
        if let Ok(mark_b) = rt.CreateSolidColorBrush(
            &D2D1_COLOR_F {
                r: 0.92,
                g: 0.22,
                b: 0.26,
                a: alpha * 0.9,
            },
            None,
        ) {
            for side in [-1.0_f32, 1.0_f32] {
                let mx = base_cx + side * 9.0;
                rt.DrawLine(
                    D2D_POINT_2F {
                        x: mx,
                        y: eye_y - 10.0,
                    },
                    D2D_POINT_2F {
                        x: mx,
                        y: eye_y + 10.5,
                    },
                    &mark_b,
                    1.6,
                    stroke.as_ref(),
                );
            }
        }

        // ── 4. Mata kapsul merah-oranye ────────────────────────────────────
        draw_capsule_eyes(
            rt,
            factory,
            base_cx,
            eye_y,
            9.0,
            eye_red,
            eye_core,
            (1.0, 0.22, 0.18),
            0.35 + 0.55 * dash,
            alpha,
            ctx,
        );

        // ── 5. Kacamata bulat tipis + jembatan ─────────────────────────────
        if let Ok(glass_b) = rt.CreateSolidColorBrush(
            &D2D1_COLOR_F {
                r: glass.r,
                g: glass.g,
                b: glass.b,
                a: alpha * 0.92,
            },
            None,
        ) {
            for side in [-1.0_f32, 1.0_f32] {
                let gx = base_cx + side * 9.0 + ctx.look_x * 0.4;
                rt.DrawEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F { x: gx, y: eye_y },
                        radiusX: 7.4 * s,
                        radiusY: 7.4 * s,
                    },
                    &glass_b,
                    1.1,
                    stroke.as_ref(),
                );
            }
            rt.DrawLine(
                D2D_POINT_2F {
                    x: base_cx - 2.6 + ctx.look_x * 0.4,
                    y: eye_y,
                },
                D2D_POINT_2F {
                    x: base_cx + 2.6 + ctx.look_x * 0.4,
                    y: eye_y,
                },
                &glass_b,
                1.1,
                stroke.as_ref(),
            );
            // Tangkai kacamata ke pelipis.
            for side in [-1.0_f32, 1.0_f32] {
                rt.DrawLine(
                    D2D_POINT_2F {
                        x: base_cx + side * 16.2 + ctx.look_x * 0.4,
                        y: eye_y - 0.6,
                    },
                    D2D_POINT_2F {
                        x: base_cx + side * 19.5,
                        y: eye_y - 1.8,
                    },
                    &glass_b,
                    1.0,
                    stroke.as_ref(),
                );
            }
        }

        // ── 6. Speed slash merah-putih (kanan) ─────────────────────────────
        let sx = base_cx + 34.0 * s + dash * 4.0;
        let sy = base_cy + 3.6 * s;
        draw_soft_radial_glow(
            rt,
            D2D_POINT_2F { x: sx, y: sy },
            14.0,
            9.0,
            0.96,
            0.18,
            0.24,
            (0.28 + 0.42 * dash) * alpha,
        );
        for k in 0..3 {
            let fk = k as f32;
            let off = (ctx.phase_time * 6.0 + fk * 1.4).sin() * 1.4;
            let len = (5.4 - fk * 1.2).max(1.6) * s;
            draw_polyline(
                rt,
                factory,
                &[
                    D2D_POINT_2F {
                        x: sx - len,
                        y: sy + off,
                    },
                    D2D_POINT_2F {
                        x: sx + len,
                        y: sy - off * 0.6,
                    },
                ],
                D2D1_COLOR_F {
                    r: 0.98,
                    g: 0.96,
                    b: 0.96,
                    a: (0.55 + 0.35 * dash) * alpha,
                },
                1.2,
            );
        }
        draw_lightning_bolt(
            rt,
            factory,
            &[
                D2D_POINT_2F {
                    x: sx - 11.0,
                    y: sy + 3.0,
                },
                D2D_POINT_2F {
                    x: sx - 2.0,
                    y: sy - 2.0,
                },
                D2D_POINT_2F {
                    x: sx + 4.0,
                    y: sy + 1.0,
                },
                D2D_POINT_2F {
                    x: sx + 12.0,
                    y: sy - 4.0,
                },
            ],
            D2D1_COLOR_F {
                r: 0.95,
                g: 0.18,
                b: 0.24,
                a: (0.40 + 0.50 * dash) * alpha,
            },
            D2D1_COLOR_F {
                r: 1.0,
                g: 0.96,
                b: 0.96,
                a: (0.55 + 0.45 * dash) * alpha,
            },
            1.35,
        );
    }
}
