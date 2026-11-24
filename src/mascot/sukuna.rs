// src/mascot/sukuna.rs
// Fase 2: Yuji Itadori -> Ryomen Sukuna.
// Detail mengikuti desain manga/anime: tato dahi (garis tengah + dua garis melintang),
// tato pipi, sepasang mata kedua di bawah mata utama, seringai, dan mudra Malevolent Shrine.
use super::core::*;
use windows::Win32::Graphics::Direct2D::Common::{D2D1_COLOR_F, D2D_POINT_2F, D2D_RECT_F};
use windows::Win32::Graphics::Direct2D::{ID2D1Factory, ID2D1HwndRenderTarget, D2D1_ELLIPSE};

pub struct YujiSukuna;

impl YujiSukuna {
    pub fn new() -> Self {
        Self
    }

    /// 0.0 = Yuji, 1.0 = Sukuna penuh
    fn sukuna_factor(t: f32) -> f32 {
        smoothstep(2.4, 3.8, t)
    }

    /// Kemunculan mudra di sisi kanan
    fn shrine_gesture(t: f32) -> f32 {
        if t < 4.2 {
            0.0
        } else if t < 5.4 {
            smoothstep(4.2, 5.4, t)
        } else if t < 9.6 {
            1.0
        } else {
            1.0 - smoothstep(9.6, 10.8, t)
        }
    }

    /// Tato muncul bertahap setelah Sukuna mengambil alih.
    fn mark_factor(s: f32) -> f32 {
        smoothstep(0.15, 0.75, s)
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
        if g > 0.15 {
            Some((1.6 * g, -0.25 * g))
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
        let pulse = 0.86 + 0.14 * (ctx.phase_time * 4.2).sin();

        // Aura dasar: hangat (Yuji) -> crimson pekat (Sukuna)
        draw_soft_radial_glow(
            rt,
            D2D_POINT_2F {
                x: cx,
                y: pill_rect.bottom - 1.0,
            },
            pw * 0.40,
            ph * 0.70,
            lerp(0.95, 0.88, s),
            lerp(0.62, 0.06, s),
            lerp(0.22, 0.28, s),
            (0.26 + 0.18 * s) * pulse * alpha,
        );

        // Aura kutukan di sisi kanan saat Sukuna aktif
        if s > 0.05 {
            draw_soft_radial_glow(
                rt,
                D2D_POINT_2F {
                    x: cx + pw * 0.22,
                    y: pill_rect.bottom - 2.0,
                },
                pw * 0.30,
                ph * 0.68,
                0.52,
                0.08,
                0.72,
                0.32 * s * pulse * alpha,
            );
        }

        // Garis domain (Malevolent Shrine) memudar masuk saat mudra aktif.
        if g > 0.35 {
            let stroke = create_round_stroke(factory);
            let line_a = (g - 0.35) / 0.65 * alpha * 0.28;
            if let (Ok(db), Some(st)) = (
                rt.CreateSolidColorBrush(
                    &D2D1_COLOR_F {
                        r: 0.55,
                        g: 0.04,
                        b: 0.10,
                        a: line_a,
                    },
                    None,
                ),
                stroke.as_ref(),
            ) {
                for i in 0..5 {
                    let k = i as f32;
                    let lx = cx - pw * 0.30 + k * pw * 0.15;
                    let top = pill_rect.top + 2.0 + (k * 1.7).sin() * 1.2;
                    rt.DrawLine(
                        D2D_POINT_2F { x: lx, y: top },
                        D2D_POINT_2F {
                            x: lx + 2.5,
                            y: pill_rect.bottom - 2.0,
                        },
                        &db,
                        0.9,
                        st,
                    );
                }
            }
        }

        // Dismantle / Cleave: dua tebasan bersilang yang berkedip.
        if g > 0.4 {
            let wave = (ctx.phase_time * 15.0).sin();
            if wave > 0.2 {
                let sa = ((wave - 0.2) / 0.8) * g * alpha;
                let stroke = create_round_stroke(factory);
                if let Ok(sb) = rt.CreateSolidColorBrush(
                    &D2D1_COLOR_F {
                        r: 1.0,
                        g: 0.28,
                        b: 0.38,
                        a: sa * 0.9,
                    },
                    None,
                ) {
                    let ox = cx + 38.0 + (ctx.phase_time * 7.0).cos() * 5.0;
                    let oy = pill_rect.top + ph * 0.46;
                    rt.DrawLine(
                        D2D_POINT_2F {
                            x: ox - 13.0,
                            y: oy - 7.0,
                        },
                        D2D_POINT_2F {
                            x: ox + 13.0,
                            y: oy + 6.0,
                        },
                        &sb,
                        1.5,
                        stroke.as_ref(),
                    );
                    rt.DrawLine(
                        D2D_POINT_2F {
                            x: ox + 10.0,
                            y: oy - 8.0,
                        },
                        D2D_POINT_2F {
                            x: ox - 9.0,
                            y: oy + 7.0,
                        },
                        &sb,
                        1.1,
                        stroke.as_ref(),
                    );
                }
            }
        }

        // ── Ambience ────────────────────────────────────────────────────────
        draw_back_rays(
            rt,
            factory,
            cx,
            pill_rect.bottom - 1.0,
            8,
            0.95,
            ph * 1.20,
            ctx.phase_time * 0.55,
            D2D1_COLOR_F {
                r: 0.92,
                g: lerp(0.35, 0.10, s),
                b: 0.20,
                a: 0.085 * alpha,
            },
        );
        draw_soft_radial_glow(
            rt,
            D2D_POINT_2F {
                x: cx,
                y: pill_rect.top + ph * 0.60,
            },
            pw * 0.19,
            ph * 0.58,
            0.85,
            0.10,
            0.18,
            0.18 * alpha,
        );
        draw_rising_particles(
            rt,
            cx,
            pill_rect.bottom - 1.0,
            pw * 0.38,
            ph * 0.9,
            ctx.phase_time,
            9,
            D2D1_COLOR_F {
                r: 0.98,
                g: lerp(0.40, 0.22, s),
                b: 0.30,
                a: 0.28 * alpha,
            },
            0.95,
        );
        draw_soft_radial_glow(
            rt,
            D2D_POINT_2F {
                x: cx,
                y: pill_rect.bottom - 0.5,
            },
            pw * 0.42,
            2.2,
            0.95,
            0.16,
            0.26,
            0.26 * alpha,
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
        let marks = Self::mark_factor(s);
        let face_cx = cx;
        let eye_y = cy + 1.6;

        // Napas halus + rambut terangkat saat Sukuna muncul.
        let breath = (ctx.phase_time * 1.7).sin() * 0.35;
        let hair_lift = s * 1.9 + breath * 0.4;
        let head_y = cy + breath;

        // Kulit Yuji (hangat) -> pucat kemerahan saat Sukuna mengambil alih.
        let skin_col = D2D1_COLOR_F {
            r: lerp(0.98, 0.90, s),
            g: lerp(0.80, 0.58, s),
            b: lerp(0.68, 0.52, s),
            a: alpha,
        };
        let skin_shade = D2D1_COLOR_F {
            r: lerp(0.86, 0.78, s),
            g: lerp(0.66, 0.46, s),
            b: lerp(0.55, 0.40, s),
            a: alpha,
        };
        let pink_col = D2D1_COLOR_F {
            r: 0.95,
            g: 0.62,
            b: 0.72,
            a: alpha,
        };
        let pink_dark = D2D1_COLOR_F {
            r: 0.72,
            g: 0.34,
            b: 0.46,
            a: alpha,
        };
        let undercut_col = D2D1_COLOR_F {
            r: 0.09,
            g: 0.07,
            b: 0.10,
            a: alpha,
        };

        // 1. Dasar kepala + rahang: kulit, bukan warna rambut.
        fill_polygon(
            rt,
            factory,
            &[
                D2D_POINT_2F {
                    x: face_cx - 14.6,
                    y: head_y - 4.0,
                },
                D2D_POINT_2F {
                    x: face_cx + 14.6,
                    y: head_y - 4.0,
                },
                D2D_POINT_2F {
                    x: face_cx + 12.4,
                    y: head_y + 6.6,
                },
                D2D_POINT_2F {
                    x: face_cx,
                    y: head_y + 8.6,
                },
                D2D_POINT_2F {
                    x: face_cx - 12.4,
                    y: head_y + 6.6,
                },
            ],
            skin_col,
        );
        // Bayangan rahang/leher agar wajah tidak flat.
        fill_polygon(
            rt,
            factory,
            &[
                D2D_POINT_2F {
                    x: face_cx - 12.4,
                    y: head_y + 6.6,
                },
                D2D_POINT_2F {
                    x: face_cx + 12.4,
                    y: head_y + 6.6,
                },
                D2D_POINT_2F {
                    x: face_cx,
                    y: head_y + 8.6,
                },
            ],
            skin_shade,
        );

        // 1b. Kerah seragam Jujutsu Tech: leher tinggi gelap + logam kancing.
        let uni_col = D2D1_COLOR_F {
            r: 0.10,
            g: 0.09,
            b: 0.13,
            a: alpha,
        };
        let uni_edge = D2D1_COLOR_F {
            r: 0.24,
            g: 0.22,
            b: 0.30,
            a: alpha,
        };
        fill_polygon(
            rt,
            factory,
            &[
                D2D_POINT_2F {
                    x: face_cx - 8.4,
                    y: head_y + 5.4,
                },
                D2D_POINT_2F {
                    x: face_cx + 8.4,
                    y: head_y + 5.4,
                },
                D2D_POINT_2F {
                    x: face_cx + 20.5,
                    y: head_y + 12.5,
                },
                D2D_POINT_2F {
                    x: face_cx - 20.5,
                    y: head_y + 12.5,
                },
            ],
            uni_col,
        );
        // Garis buka kerah vertikal (khas seragam Jujutsu).
        draw_polyline(
            rt,
            factory,
            &[
                D2D_POINT_2F {
                    x: face_cx,
                    y: head_y + 6.0,
                },
                D2D_POINT_2F {
                    x: face_cx,
                    y: head_y + 12.5,
                },
            ],
            uni_edge,
            1.0,
        );
        if let Ok(bb) = rt.CreateSolidColorBrush(&uni_edge, None) {
            rt.FillEllipse(
                &D2D1_ELLIPSE {
                    point: D2D_POINT_2F {
                        x: face_cx + 2.6,
                        y: head_y + 9.4,
                    },
                    radiusX: 1.1,
                    radiusY: 0.8,
                },
                &bb,
            );
        }

        // 2. Rambut: undercut gelap di bawah, helai pink tersapu ke atas (bezier).
        let _ = fill_smooth_shape(
            rt,
            factory,
            &[
                D2D_POINT_2F {
                    x: face_cx - 16.6,
                    y: head_y + 3.4,
                },
                D2D_POINT_2F {
                    x: face_cx - 15.4,
                    y: head_y - 8.6 - hair_lift * 0.4,
                },
                D2D_POINT_2F {
                    x: face_cx - 7.0,
                    y: head_y - 12.0 - hair_lift * 0.6,
                },
                D2D_POINT_2F {
                    x: face_cx + 0.6,
                    y: head_y - 12.8 - hair_lift * 0.6,
                },
                D2D_POINT_2F {
                    x: face_cx + 8.4,
                    y: head_y - 11.6 - hair_lift * 0.6,
                },
                D2D_POINT_2F {
                    x: face_cx + 15.6,
                    y: head_y - 8.2 - hair_lift * 0.4,
                },
                D2D_POINT_2F {
                    x: face_cx + 16.6,
                    y: head_y + 3.6,
                },
                D2D_POINT_2F {
                    x: face_cx + 10.0,
                    y: head_y - 2.6,
                },
                D2D_POINT_2F {
                    x: face_cx,
                    y: head_y - 5.4,
                },
                D2D_POINT_2F {
                    x: face_cx - 10.0,
                    y: head_y - 2.8,
                },
            ],
            pink_dark,
        );

        // 2b. Helai utama: pangkal gelap di akar, ujung pink menyapu belakang.
        let spikes: [(f32, f32, f32, f32, f32, f32); 9] = [
            (-13.6, -5.6, -20.6, -12.6, 1.6, 5.8),
            (-9.6, -8.2, -13.4, -19.4, 1.2, 5.6),
            (-4.4, -9.4, -5.6, -21.8, -1.0, 6.0),
            (0.8, -9.8, 2.6, -22.6, 1.4, 6.2),
            (5.8, -9.2, 10.4, -20.6, -1.6, 5.8),
            (10.4, -7.8, 16.8, -17.0, 1.2, 5.4),
            (14.2, -5.2, 21.6, -10.6, -1.2, 5.0),
            (15.4, -2.4, 20.4, 3.0, 1.0, 4.0),
            (-15.4, -2.6, -19.6, 2.6, -1.0, 3.8),
        ];
        for (i, &(bx, by, tx, ty, bend, w)) in spikes.iter().enumerate() {
            let lift = if i < 7 { hair_lift } else { hair_lift * 0.2 };
            // Akar gelap
            hair_lock(
                rt,
                factory,
                D2D_POINT_2F {
                    x: face_cx + bx * 0.94,
                    y: head_y + by * 0.94,
                },
                D2D_POINT_2F {
                    x: face_cx + tx * 0.55,
                    y: head_y + ty * 0.55,
                },
                bend * 0.5,
                w * 0.7,
                D2D1_COLOR_F {
                    r: 0.16,
                    g: 0.05,
                    b: 0.10,
                    a: alpha,
                },
                D2D1_COLOR_F { a: 0.0, ..pink_col },
            );
            // Ujung pink
            hair_lock(
                rt,
                factory,
                D2D_POINT_2F {
                    x: face_cx + bx,
                    y: head_y + by - lift * 0.15,
                },
                D2D_POINT_2F {
                    x: face_cx + tx,
                    y: head_y + ty - lift,
                },
                bend,
                w,
                pink_col,
                D2D1_COLOR_F {
                    r: 1.0,
                    g: 0.72,
                    b: 0.80,
                    a: 0.55 * alpha,
                },
            );
        }

        // 2c. Kilau: dua sapuan mulus di massa atas.
        stroke_smooth_curve(
            rt,
            factory,
            &[
                D2D_POINT_2F {
                    x: face_cx - 10.4,
                    y: head_y - 8.2,
                },
                D2D_POINT_2F {
                    x: face_cx - 3.0,
                    y: head_y - 11.6 - hair_lift * 0.4,
                },
                D2D_POINT_2F {
                    x: face_cx + 4.6,
                    y: head_y - 10.4 - hair_lift * 0.3,
                },
            ],
            D2D1_COLOR_F {
                r: 1.0,
                g: 0.78,
                b: 0.85,
                a: 0.70 * alpha,
            },
            0.9,
        );
        stroke_smooth_curve(
            rt,
            factory,
            &[
                D2D_POINT_2F {
                    x: face_cx + 7.4,
                    y: head_y - 8.6,
                },
                D2D_POINT_2F {
                    x: face_cx + 11.8,
                    y: head_y - 6.2,
                },
                D2D_POINT_2F {
                    x: face_cx + 15.2,
                    y: head_y - 3.0,
                },
            ],
            D2D1_COLOR_F {
                r: 1.0,
                g: 0.74,
                b: 0.82,
                a: 0.45 * alpha,
            },
            0.8,
        );

        // 3. Tato Sukuna
        if marks > 0.02 {
            let ink = D2D1_COLOR_F {
                r: 0.04,
                g: 0.02,
                b: 0.05,
                a: alpha * marks,
            };
            let ink_halo = D2D1_COLOR_F {
                r: 0.75,
                g: 0.08,
                b: 0.16,
                a: alpha * marks * 0.30,
            };

            let mark = |pts: &[(f32, f32)], width: f32, halo: bool| {
                let world: Vec<D2D_POINT_2F> = pts
                    .iter()
                    .map(|&(x, y)| D2D_POINT_2F {
                        x: face_cx + x,
                        y: head_y + y,
                    })
                    .collect();
                if halo {
                    draw_polyline(rt, factory, &world, ink_halo, width + 1.6);
                }
                draw_polyline(rt, factory, &world, ink, width);
            };

            // Garis tengah dahi + dua garis melintang di atas mata
            mark(&[(0.0, -13.0), (0.0, -6.4)], 1.1, true);
            mark(&[(-6.4, -6.6), (-1.0, -6.6)], 1.0, true);
            mark(&[(1.0, -6.6), (6.4, -6.6)], 1.0, true);
            mark(&[(-5.6, -5.0), (-1.2, -5.0)], 0.9, true);
            mark(&[(1.2, -5.0), (5.6, -5.0)], 0.9, true);

            // Garis bawah mata utama
            mark(&[(-11.6, 5.9), (-8.0, 6.5)], 1.0, true);
            mark(&[(8.0, 6.5), (11.6, 5.9)], 1.0, true);

            // Tato pipi
            mark(&[(-13.2, -1.6), (-14.6, 3.4)], 0.9, true);
            mark(&[(-10.4, 0.6), (-11.6, 4.6)], 0.8, true);
            mark(&[(13.2, -1.6), (14.6, 3.4)], 0.9, true);
            mark(&[(10.4, 0.6), (11.6, 4.6)], 0.8, true);

            // Bingkai atas & bawah mata kedua
            mark(&[(-12.6, 3.9), (-8.2, 4.4)], 0.8, true);
            mark(&[(8.2, 4.4), (12.6, 3.9)], 0.8, true);
            mark(&[(-12.2, 8.6), (-8.4, 8.2)], 0.8, true);
            mark(&[(8.4, 8.2), (12.2, 8.6)], 0.8, true);

            // Seringai: garis mulut + sudut
            mark(&[(-4.6, 8.0), (0.0, 8.9), (4.6, 8.0)], 1.0, true);
            mark(&[(-4.6, 8.0), (-4.0, 8.8)], 0.8, true);
            mark(&[(4.6, 8.0), (4.0, 8.8)], 0.8, true);
        }

        // 4. Mata utama: sipit tajam khas Yuji (naik ke arah luar), iris cokelat,
        // memerah ke crimson saat Sukuna mengambil alih.
        let outer_col = lerp_color(
            D2D1_COLOR_F {
                r: 0.32,
                g: 0.19,
                b: 0.12,
                a: 1.0,
            },
            D2D1_COLOR_F {
                r: 1.0,
                g: 0.13,
                b: 0.26,
                a: 1.0,
            },
            s,
        );
        let core_col = lerp_color(
            D2D1_COLOR_F {
                r: 0.96,
                g: 0.88,
                b: 0.76,
                a: 1.0,
            },
            D2D1_COLOR_F {
                r: 1.0,
                g: 0.70,
                b: 0.76,
                a: 1.0,
            },
            s,
        );
        draw_capsule_eyes(
            rt,
            factory,
            face_cx,
            eye_y,
            8.2,
            outer_col,
            core_col,
            (1.0, lerp(0.72, 0.10, s), lerp(0.24, 0.28, s)),
            0.25 + s * 0.65 + g * 0.10,
            alpha,
            ctx,
        );
        // Alis tegas Yuji (turun ke tengah, ekspresif) — ikut merah saat Sukuna.
        let brow_col = D2D1_COLOR_F {
            r: lerp(0.55, 0.70, s),
            g: lerp(0.34, 0.10, s),
            b: lerp(0.26, 0.14, s),
            a: alpha,
        };
        for side in [-1.0_f32, 1.0_f32] {
            draw_polyline(
                rt,
                factory,
                &[
                    D2D_POINT_2F {
                        x: face_cx + side * 3.4,
                        y: eye_y - 4.6,
                    },
                    D2D_POINT_2F {
                        x: face_cx + side * 6.6,
                        y: eye_y - 5.5,
                    },
                    D2D_POINT_2F {
                        x: face_cx + side * 9.6,
                        y: eye_y - 4.0,
                    },
                ],
                brow_col,
                1.1,
            );
        }
        // Garis bawah mata khas Yuji (bekas/kelopak bawah) — selalu ada, fungsional.
        let under_col = D2D1_COLOR_F {
            r: 0.55,
            g: 0.30,
            b: 0.24,
            a: alpha * 0.65,
        };
        for side in [-1.0_f32, 1.0_f32] {
            draw_polyline(
                rt,
                factory,
                &[
                    D2D_POINT_2F {
                        x: face_cx + side * 4.2,
                        y: eye_y + 3.0,
                    },
                    D2D_POINT_2F {
                        x: face_cx + side * 7.4,
                        y: eye_y + 3.5,
                    },
                ],
                under_col,
                0.8,
            );
        }
        // Senyum lebar percaya diri khas Yuji (menghilang saat Sukuna datang).
        if s < 0.75 {
            let smile_a = (1.0 - s / 0.75) * alpha;
            draw_polyline(
                rt,
                factory,
                &[
                    D2D_POINT_2F {
                        x: face_cx - 4.2,
                        y: head_y + 3.4,
                    },
                    D2D_POINT_2F {
                        x: face_cx - 1.4,
                        y: head_y + 5.2,
                    },
                    D2D_POINT_2F {
                        x: face_cx + 1.6,
                        y: head_y + 5.2,
                    },
                    D2D_POINT_2F {
                        x: face_cx + 4.4,
                        y: head_y + 3.2,
                    },
                ],
                D2D1_COLOR_F {
                    r: 0.42,
                    g: 0.20,
                    b: 0.18,
                    a: smile_a,
                },
                0.9,
            );
        }

        // 5. Sepasang mata kedua Sukuna di pipi
        let second_open = smoothstep(0.45, 0.95, s) * ctx.open_factor();
        if second_open > 0.04 {
            let socket = D2D1_COLOR_F {
                r: 0.03,
                g: 0.02,
                b: 0.04,
                a: alpha * second_open,
            };
            let iris = D2D1_COLOR_F {
                r: 1.0,
                g: 0.12,
                b: 0.24,
                a: alpha * second_open,
            };
            let spark = D2D1_COLOR_F {
                r: 1.0,
                g: 0.72,
                b: 0.76,
                a: alpha * second_open,
            };

            for side in [-1.0_f32, 1.0_f32] {
                let sx = face_cx + side * 10.4 + ctx.look_x * 0.45;
                let sy = head_y + 6.2 + ctx.look_y * 0.30;

                fill_polygon(
                    rt,
                    factory,
                    &[
                        D2D_POINT_2F {
                            x: sx - 2.6,
                            y: sy + 0.9,
                        },
                        D2D_POINT_2F {
                            x: sx - 1.6,
                            y: sy - 0.9,
                        },
                        D2D_POINT_2F {
                            x: sx + 1.6,
                            y: sy - 0.9,
                        },
                        D2D_POINT_2F {
                            x: sx + 2.6,
                            y: sy + 0.9,
                        },
                    ],
                    socket,
                );
                if let Ok(ib) = rt.CreateSolidColorBrush(&iris, None) {
                    rt.FillEllipse(
                        &D2D1_ELLIPSE {
                            point: D2D_POINT_2F { x: sx, y: sy },
                            radiusX: 1.9,
                            radiusY: 1.15 * second_open.max(0.4),
                        },
                        &ib,
                    );
                }
                if let Ok(sb2) = rt.CreateSolidColorBrush(&spark, None) {
                    rt.FillEllipse(
                        &D2D1_ELLIPSE {
                            point: D2D_POINT_2F { x: sx, y: sy },
                            radiusX: 0.7,
                            radiusY: 0.5 * second_open.max(0.5),
                        },
                        &sb2,
                    );
                }
            }
        }

        // 6. Mudra Malevolent Shrine (dua tangan menyatu)
        if g > 0.04 {
            let drift = (1.0 - g) * 7.0;
            let gx = cx + 38.0;
            let gy = head_y + 2.6 + drift;
            let skin_col = D2D1_COLOR_F {
                r: 0.93,
                g: 0.80,
                b: 0.72,
                a: alpha * g,
            };
            let cuff_col = D2D1_COLOR_F {
                r: 0.07,
                g: 0.05,
                b: 0.09,
                a: alpha * g,
            };
            let energy = D2D1_COLOR_F {
                r: 1.0,
                g: 0.16,
                b: 0.28,
                a: alpha * g * (0.55 + 0.45 * (ctx.phase_time * 5.0).sin().abs()),
            };

            // Ring energi kutukan
            draw_soft_radial_glow(
                rt,
                D2D_POINT_2F { x: gx, y: gy },
                15.0,
                13.0,
                0.9,
                0.08,
                0.24,
                0.42 * g * alpha,
            );

            // Dua telapak menyatu (kiri & kanan)
            for side in [-1.0_f32, 1.0_f32] {
                fill_polygon(
                    rt,
                    factory,
                    &[
                        D2D_POINT_2F {
                            x: gx + side * 0.6,
                            y: gy - 8.8,
                        },
                        D2D_POINT_2F {
                            x: gx + side * 3.4,
                            y: gy - 1.4,
                        },
                        D2D_POINT_2F {
                            x: gx + side * 7.4,
                            y: gy + 5.4,
                        },
                        D2D_POINT_2F {
                            x: gx + side * 4.2,
                            y: gy + 7.2,
                        },
                        D2D_POINT_2F {
                            x: gx + side * 1.2,
                            y: gy + 1.0,
                        },
                    ],
                    skin_col,
                );
                // Lengan berbalut seragam gelap
                fill_polygon(
                    rt,
                    factory,
                    &[
                        D2D_POINT_2F {
                            x: gx + side * 6.2,
                            y: gy + 4.6,
                        },
                        D2D_POINT_2F {
                            x: gx + side * 9.6,
                            y: gy + 7.0,
                        },
                        D2D_POINT_2F {
                            x: gx + side * 11.4,
                            y: gy + 9.2,
                        },
                        D2D_POINT_2F {
                            x: gx + side * 8.0,
                            y: gy + 8.4,
                        },
                    ],
                    cuff_col,
                );
            }

            // Ibu jari & garis jari tengah yang menyilang
            draw_polyline(
                rt,
                factory,
                &[
                    D2D_POINT_2F {
                        x: gx - 2.6,
                        y: gy - 5.6,
                    },
                    D2D_POINT_2F {
                        x: gx,
                        y: gy - 2.0,
                    },
                    D2D_POINT_2F {
                        x: gx + 2.6,
                        y: gy - 5.6,
                    },
                ],
                D2D1_COLOR_F {
                    r: 0.80,
                    g: 0.62,
                    b: 0.54,
                    a: alpha * g,
                },
                0.9,
            );

            // Cincin api kutukan mengelilingi mudra
            if g > 0.6 {
                if let Ok(eb) = rt.CreateSolidColorBrush(&energy, None) {
                    let ring = D2D1_ELLIPSE {
                        point: D2D_POINT_2F { x: gx, y: gy + 1.0 },
                        radiusX: 10.5,
                        radiusY: 11.5,
                    };
                    rt.DrawEllipse(&ring, &eb, 1.1, None);
                    let ring2 = D2D1_ELLIPSE {
                        point: D2D_POINT_2F { x: gx, y: gy + 1.0 },
                        radiusX: 13.0,
                        radiusY: 14.0,
                    };
                    rt.DrawEllipse(&ring2, &eb, 0.6, None);
                }
            }
        }

        // 7. Bayangan torii kuil di belakang mudra saat domain penuh
        if g > 0.85 {
            let ta = (g - 0.85) / 0.15 * alpha * 0.22;
            let tc = D2D1_COLOR_F {
                r: 0.85,
                g: 0.10,
                b: 0.20,
                a: ta,
            };
            let tx = cx + 38.0;
            let ty = head_y - 9.0;
            draw_polyline(
                rt,
                factory,
                &[
                    D2D_POINT_2F {
                        x: tx - 13.0,
                        y: ty,
                    },
                    D2D_POINT_2F {
                        x: tx + 13.0,
                        y: ty,
                    },
                ],
                tc,
                1.4,
            );
            draw_polyline(
                rt,
                factory,
                &[
                    D2D_POINT_2F {
                        x: tx - 10.0,
                        y: ty + 4.0,
                    },
                    D2D_POINT_2F {
                        x: tx + 10.0,
                        y: ty + 4.0,
                    },
                ],
                tc,
                1.0,
            );
            for side in [-1.0_f32, 1.0_f32] {
                draw_polyline(
                    rt,
                    factory,
                    &[
                        D2D_POINT_2F {
                            x: tx + side * 9.0,
                            y: ty,
                        },
                        D2D_POINT_2F {
                            x: tx + side * 9.0,
                            y: ty + 12.0,
                        },
                    ],
                    tc,
                    1.0,
                );
            }
        }

        // 8. Ujung rambut jatuh di sisi saat Sukuna penuh
        if s > 0.3 {
            let drop = D2D1_COLOR_F {
                r: 0.88,
                g: 0.50,
                b: 0.60,
                a: alpha * s,
            };
            let sway = (ctx.phase_time * 2.1).sin() * 0.8;
            for side in [-1.0_f32, 1.0_f32] {
                fill_polygon(
                    rt,
                    factory,
                    &[
                        D2D_POINT_2F {
                            x: face_cx + side * 15.4,
                            y: head_y - 5.0,
                        },
                        D2D_POINT_2F {
                            x: face_cx + side * 17.2 + sway,
                            y: head_y + 2.0,
                        },
                        D2D_POINT_2F {
                            x: face_cx + side * 15.0,
                            y: head_y + 5.5,
                        },
                    ],
                    drop,
                );
            }
        }

        // Glow tambahan di pipi saat tato menyala
        if marks > 0.3 {
            for side in [-1.0_f32, 1.0_f32] {
                draw_soft_radial_glow(
                    rt,
                    D2D_POINT_2F {
                        x: face_cx + side * 12.0,
                        y: head_y + 5.0,
                    },
                    6.0,
                    6.5,
                    0.95,
                    0.10,
                    0.20,
                    0.22 * marks * alpha,
                );
            }
        }
    }
}
