// src/mascot/sukuna.rs
// Fase 2: Yuji Itadori -> Ryomen Sukuna.
// Gaya dikunci ke Asta Devil Union: hanya siluet rambut + mata kapsul illuminated +
// satu gestur di kanan + aura bawah lembut. Tanpa wajah, hidung, atau mulut.
use super::core::*;
use windows::Win32::Graphics::Direct2D::Common::{D2D1_COLOR_F, D2D_POINT_2F, D2D_RECT_F};
use windows::Win32::Graphics::Direct2D::{ID2D1Factory, ID2D1HwndRenderTarget, D2D1_ELLIPSE};

pub struct YujiSukuna;

impl YujiSukuna {
    pub fn new() -> Self {
        Self
    }

    /// 0.0 = Yuji, 1.0 = Sukuna penuh (transisi 3.2s)
    fn sukuna_factor(t: f32) -> f32 {
        smoothstep(3.2, 4.4, t)
    }

    /// Gestur kanan: fist Divergent Fist (Yuji) -> hand sign Shrine (Sukuna)
    fn shrine_gesture(t: f32) -> f32 {
        if t < 4.2 {
            0.0
        } else if t < 5.4 {
            smoothstep(4.2, 5.4, t)
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
        rt: &ID2D1HwndRenderTarget,
        factory: &ID2D1Factory,
        pill_rect: D2D_RECT_F,
        alpha: f32,
        ctx: &MascotContext,
    ) {
        let s = Self::sukuna_factor(ctx.local_time);
        let g = Self::shrine_gesture(ctx.local_time);
        let pw = pill_rect.right - pill_rect.left;
        let ph = pill_rect.bottom - pill_rect.top;
        let cx = (pill_rect.left + pill_rect.right) * 0.5;
        let pulse = 0.88 + 0.12 * (ctx.phase_time * 4.0).sin();

        // Cyan Divergent Fist (Yuji) memudar ke crimson-violet (Sukuna)
        draw_soft_radial_glow(
            rt,
            D2D_POINT_2F {
                x: cx,
                y: pill_rect.bottom - 1.0,
            },
            pw * 0.40,
            ph * 0.68,
            lerp(0.0, 0.92, s),
            lerp(0.83, 0.10, s),
            lerp(1.0, 0.34, s),
            (0.30 - 0.10 * s) * pulse * alpha,
        );

        if s > 0.05 {
            draw_soft_radial_glow(
                rt,
                D2D_POINT_2F {
                    x: cx + pw * 0.20,
                    y: pill_rect.bottom - 2.0,
                },
                pw * 0.28,
                ph * 0.60,
                0.55,
                0.10,
                0.70,
                0.28 * s * pulse * alpha,
            );
        }

        // Kilatan tebasan Dismantle/Cleave saat hand sign aktif
        if g > 0.4 {
            let wave = (ctx.phase_time * 13.0).sin();
            if wave > 0.35 {
                let sa = ((wave - 0.35) / 0.65) * g * alpha;
                let stroke = create_round_stroke(factory);
                if let Ok(sb) = rt.CreateSolidColorBrush(
                    &D2D1_COLOR_F {
                        r: 1.0,
                        g: 0.34,
                        b: 0.42,
                        a: sa * 0.85,
                    },
                    None,
                ) {
                    let ox = cx + 40.0;
                    let oy = pill_rect.top + ph * 0.45;
                    rt.DrawLine(
                        D2D_POINT_2F {
                            x: ox - 11.0,
                            y: oy - 6.0,
                        },
                        D2D_POINT_2F {
                            x: ox + 11.0,
                            y: oy + 5.0,
                        },
                        &sb,
                        1.3,
                        stroke.as_ref(),
                    );
                }
            }
        }

        // Partikel ringan khas karakter
        draw_rising_particles(
            rt,
            cx,
            pill_rect.bottom - 1.0,
            pw * 0.34,
            ph * 0.78,
            ctx.phase_time,
            7,
            D2D1_COLOR_F {
                r: lerp(0.25, 0.98, s),
                g: lerp(0.82, 0.26, s),
                b: lerp(1.0, 0.34, s),
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
        let s = Self::sukuna_factor(ctx.local_time);
        let g = Self::shrine_gesture(ctx.local_time);
        let face_cx = cx;
        let eye_y = cy + 1.8;

        let breath = (ctx.phase_time * 1.6).sin() * 0.3;
        let hair_lift = s * 1.2 + breath * 0.2;

        // 1. Siluet rambut pink salmon dengan spike
        let pink = D2D1_COLOR_F {
            r: 0.88,
            g: 0.60,
            b: 0.61,
            a: alpha,
        };
        let pink_dark = D2D1_COLOR_F {
            r: 0.62,
            g: 0.36,
            b: 0.39,
            a: alpha,
        };
        fill_polygon(
            rt,
            factory,
            &[
                D2D_POINT_2F {
                    x: face_cx - 17.0,
                    y: cy - 3.0,
                },
                D2D_POINT_2F {
                    x: face_cx - 15.0,
                    y: cy - 11.5 - hair_lift,
                },
                D2D_POINT_2F {
                    x: face_cx - 9.0,
                    y: cy - 8.5 - hair_lift * 0.5,
                },
                D2D_POINT_2F {
                    x: face_cx - 4.5,
                    y: cy - 13.5 - hair_lift,
                },
                D2D_POINT_2F {
                    x: face_cx,
                    y: cy - 9.5 - hair_lift * 0.4,
                },
                D2D_POINT_2F {
                    x: face_cx + 4.5,
                    y: cy - 13.5 - hair_lift,
                },
                D2D_POINT_2F {
                    x: face_cx + 9.0,
                    y: cy - 8.5 - hair_lift * 0.5,
                },
                D2D_POINT_2F {
                    x: face_cx + 15.0,
                    y: cy - 11.5 - hair_lift,
                },
                D2D_POINT_2F {
                    x: face_cx + 17.0,
                    y: cy - 3.0,
                },
            ],
            pink,
        );
        // Undercut gelap di pelipis
        for side in [-1.0_f32, 1.0_f32] {
            fill_polygon(
                rt,
                factory,
                &[
                    D2D_POINT_2F {
                        x: face_cx + side * 17.0,
                        y: cy - 3.2,
                    },
                    D2D_POINT_2F {
                        x: face_cx + side * 10.5,
                        y: cy - 3.6,
                    },
                    D2D_POINT_2F {
                        x: face_cx + side * 14.0,
                        y: cy - 9.5 - hair_lift * 0.6,
                    },
                ],
                pink_dark,
            );
        }

        // 2. Aksesoris Sukuna: trisula dahi + garis pipi tipis
        if s > 0.05 {
            let ink = D2D1_COLOR_F {
                r: 0.05,
                g: 0.03,
                b: 0.06,
                a: alpha * s,
            };
            let ink_halo = D2D1_COLOR_F {
                r: 0.72,
                g: 0.08,
                b: 0.18,
                a: alpha * s * 0.35,
            };
            let mark = |pts: &[(f32, f32)], w: f32| {
                let world: Vec<D2D_POINT_2F> = pts
                    .iter()
                    .map(|&(x, y)| D2D_POINT_2F {
                        x: face_cx + x,
                        y: cy + y,
                    })
                    .collect();
                draw_polyline(rt, factory, &world, ink_halo, w + 1.4);
                draw_polyline(rt, factory, &world, ink, w);
            };
            mark(&[(0.0, -11.5), (0.0, -5.2)], 1.1);
            mark(&[(-3.4, -11.0), (-1.2, -6.6)], 0.9);
            mark(&[(3.4, -11.0), (1.2, -6.6)], 0.9);
            mark(&[(-14.0, 0.4), (-12.2, 3.8)], 0.8);
            mark(&[(14.0, 0.4), (12.2, 3.8)], 0.8);
        }

        // 3. Mata utama: amber hangat -> crimson menyala
        let outer = lerp_color(
            D2D1_COLOR_F {
                r: 0.91,
                g: 0.60,
                b: 0.24,
                a: 1.0,
            },
            D2D1_COLOR_F {
                r: 1.0,
                g: 0.14,
                b: 0.26,
                a: 1.0,
            },
            s,
        );
        let core = lerp_color(
            D2D1_COLOR_F {
                r: 1.0,
                g: 0.91,
                b: 0.71,
                a: 1.0,
            },
            D2D1_COLOR_F {
                r: 1.0,
                g: 0.72,
                b: 0.78,
                a: 1.0,
            },
            s,
        );
        draw_capsule_eyes(
            rt,
            factory,
            face_cx,
            eye_y,
            9.0,
            outer,
            core,
            (1.0, lerp(0.62, 0.14, s), lerp(0.26, 0.28, s)),
            0.78 * s,
            alpha,
            ctx,
        );

        // 4. Sepasang mata kecil Sukuna di bawah mata utama
        let second = smoothstep(0.55, 1.0, s);
        if second > 0.04 {
            let a = alpha * second * ctx.open_factor();
            if let Ok(b) = rt.CreateSolidColorBrush(
                &D2D1_COLOR_F {
                    r: 1.0,
                    g: 0.18,
                    b: 0.30,
                    a,
                },
                None,
            ) {
                for side in [-1.0_f32, 1.0_f32] {
                    let sx = face_cx + side * 10.2 + ctx.look_x * 0.4;
                    let sy = eye_y + 5.6 + ctx.look_y * 0.3;
                    rt.FillEllipse(
                        &D2D1_ELLIPSE {
                            point: D2D_POINT_2F { x: sx, y: sy },
                            radiusX: 2.3,
                            radiusY: 1.0,
                        },
                        &b,
                    );
                }
            }
        }

        // 5. Gestur kanan: fist cyan (Yuji) -> hand sign Shrine (Sukuna)
        let gx = cx + 37.0;
        let gy = cy + 2.2;

        if g < 0.5 {
            // Divergent Fist: kepalan kecil berbalut api kutukan cyan
            let ca = 1.0 - g * 2.0;
            let cy_glow = 0.45 + 0.35 * (ctx.phase_time * 6.0).sin().abs();
            draw_soft_radial_glow(
                rt,
                D2D_POINT_2F { x: gx, y: gy },
                12.0,
                10.0,
                0.0,
                0.83,
                1.0,
                cy_glow * ca * alpha,
            );
            if let Ok(fist) = rt.CreateSolidColorBrush(
                &D2D1_COLOR_F {
                    r: 0.10,
                    g: 0.09,
                    b: 0.12,
                    a: alpha * ca,
                },
                None,
            ) {
                rt.FillRoundedRectangle(
                    &windows::Win32::Graphics::Direct2D::D2D1_ROUNDED_RECT {
                        rect: D2D_RECT_F {
                            left: gx - 4.6,
                            top: gy - 4.4,
                            right: gx + 4.6,
                            bottom: gy + 4.4,
                        },
                        radiusX: 3.2,
                        radiusY: 3.2,
                    },
                    &fist,
                );
            }
            draw_polyline(
                rt,
                factory,
                &[
                    D2D_POINT_2F {
                        x: gx - 3.4,
                        y: gy - 5.2,
                    },
                    D2D_POINT_2F {
                        x: gx - 1.2,
                        y: gy - 9.4,
                    },
                    D2D_POINT_2F {
                        x: gx + 1.6,
                        y: gy - 5.0,
                    },
                ],
                D2D1_COLOR_F {
                    r: 0.35,
                    g: 0.93,
                    b: 1.0,
                    a: ca * alpha,
                },
                1.2,
            );
        } else {
            // Hand sign Malevolent Shrine: dua telapak menyatu
            let gg = (g - 0.5) * 2.0;
            let skin = D2D1_COLOR_F {
                r: 0.92,
                g: 0.79,
                b: 0.71,
                a: alpha * gg,
            };
            draw_soft_radial_glow(
                rt,
                D2D_POINT_2F { x: gx, y: gy },
                13.0,
                11.0,
                0.80,
                0.10,
                0.22,
                0.34 * gg * alpha,
            );
            for side in [-1.0_f32, 1.0_f32] {
                fill_polygon(
                    rt,
                    factory,
                    &[
                        D2D_POINT_2F {
                            x: gx + side * 0.8,
                            y: gy - 8.0,
                        },
                        D2D_POINT_2F {
                            x: gx + side * 3.6,
                            y: gy - 1.2,
                        },
                        D2D_POINT_2F {
                            x: gx + side * 6.8,
                            y: gy + 5.0,
                        },
                        D2D_POINT_2F {
                            x: gx + side * 3.8,
                            y: gy + 6.4,
                        },
                        D2D_POINT_2F {
                            x: gx + side * 1.2,
                            y: gy + 0.8,
                        },
                    ],
                    skin,
                );
            }
        }
    }
}
