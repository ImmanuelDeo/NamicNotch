// src/mascot/bakugo.rs
// Fase 3: Katsuki Bakugo.
// Detail kostum hero mengikuti referensi: masker mata hitam bertepi oranye, granat pelipis
// dengan cincin pin, pelindung dagu metal, gauntlet granat di lengan kanan, dan ledakan
// nitrogliserin berlapis (inti, gelombang kejut, serpihan, asap).
use super::core::*;
use windows::Win32::Graphics::Direct2D::Common::{D2D1_COLOR_F, D2D_POINT_2F, D2D_RECT_F};
use windows::Win32::Graphics::Direct2D::{
    ID2D1Factory, ID2D1HwndRenderTarget, D2D1_ELLIPSE, D2D1_ROUNDED_RECT,
};

pub struct KatsukiBakugo;

impl KatsukiBakugo {
    pub fn new() -> Self {
        Self
    }

    /// Pulsa ledakan berulang tiap ~2.6 detik
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

    /// Sisa asap setelah ledakan mereda
    fn smoke_factor(t: f32) -> f32 {
        let cycle = t % 2.6;
        if cycle < 0.5 {
            0.0
        } else if cycle < 1.6 {
            smoothstep(0.5, 1.6, cycle) * (1.0 - smoothstep(1.2, 1.6, cycle))
                + (1.0 - smoothstep(0.5, 1.6, cycle)) * 0.7
        } else {
            (1.0 - smoothstep(1.6, 2.4, cycle)).max(0.0) * 0.45
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
        rt: &ID2D1HwndRenderTarget,
        factory: &ID2D1Factory,
        pill_rect: D2D_RECT_F,
        alpha: f32,
        ctx: &MascotContext,
    ) {
        let blast = Self::blast_pulse(ctx.local_time);
        let smoke = Self::smoke_factor(ctx.local_time);
        let pw = pill_rect.right - pill_rect.left;
        let ph = pill_rect.bottom - pill_rect.top;
        let cx = (pill_rect.left + pill_rect.right) * 0.5;

        // Panas dasar
        draw_soft_radial_glow(
            rt,
            D2D_POINT_2F {
                x: cx,
                y: pill_rect.bottom - 1.0,
            },
            pw * 0.48,
            ph * 0.72,
            1.0,
            0.40,
            0.06,
            (0.28 + 0.26 * blast) * alpha,
        );

        // Gelombang kejut: cincin tipis yang melebar lalu hilang
        if blast > 0.05 {
            let stroke = create_round_stroke(factory);
            if let (Ok(ring_b), Some(st)) = (
                rt.CreateSolidColorBrush(
                    &D2D1_COLOR_F {
                        r: 1.0,
                        g: 0.78,
                        b: 0.34,
                        a: blast * alpha * 0.55,
                    },
                    None,
                ),
                stroke.as_ref(),
            ) {
                let bx = cx + 46.0;
                for k in 0..2 {
                    let grow = 1.0 + k as f32 * 0.55 + (1.0 - blast) * 2.4;
                    rt.DrawEllipse(
                        &D2D1_ELLIPSE {
                            point: D2D_POINT_2F {
                                x: bx,
                                y: pill_rect.top + ph * 0.42,
                            },
                            radiusX: 8.0 * grow,
                            radiusY: 6.5 * grow,
                        },
                        &ring_b,
                        (1.0 - k as f32 * 0.35).max(0.3) * blast,
                        st,
                    );
                }
            }
        }

        // Bara naik
        for i in 0..7 {
            let fi = i as f32;
            let life = (ctx.phase_time * (0.9 + fi * 0.13) + fi * 0.37) % 1.0;
            let ex = pill_rect.left
                + pw * (0.14 + 0.72 * ((fi * 0.29 + 0.11) % 1.0))
                + (ctx.phase_time * 3.0 + fi).sin() * 3.5;
            let ey = pill_rect.bottom - 3.0 - life * (ph * 0.85);
            draw_soft_circle(
                rt,
                D2D_POINT_2F { x: ex, y: ey },
                1.4,
                1.4,
                D2D1_COLOR_F {
                    r: 1.0,
                    g: 0.70,
                    b: 0.20,
                    a: (1.0 - life) * alpha * (0.55 + 0.45 * blast),
                },
            );
        }

        // Asap hangat yang mengendap
        if smoke > 0.02 {
            for i in 0..5 {
                let fi = i as f32;
                let life = (ctx.phase_time * 0.32 + fi * 0.21) % 1.0;
                let sx = cx + 30.0 + (fi * 3.1).sin() * 9.0 + life * 6.0;
                let sy = pill_rect.top + ph * 0.30 - life * ph * 0.55;
                draw_soft_circle(
                    rt,
                    D2D_POINT_2F { x: sx, y: sy },
                    9.0 + life * 7.0,
                    6.0 + life * 4.0,
                    D2D1_COLOR_F {
                        r: 0.52,
                        g: 0.34,
                        b: 0.22,
                        a: (1.0 - life) * smoke * alpha * 0.22,
                    },
                );
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
            ctx.phase_time * 0.6,
            D2D1_COLOR_F {
                r: 1.0,
                g: 0.52,
                b: 0.10,
                a: (0.07 + 0.06 * blast) * alpha,
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
            1.0,
            0.62,
            0.18,
            0.17 * alpha,
        );
        draw_rising_particles(
            rt,
            cx,
            pill_rect.bottom - 1.0,
            pw * 0.38,
            ph * 0.9,
            ctx.phase_time,
            10,
            D2D1_COLOR_F {
                r: 1.0,
                g: 0.66,
                b: 0.24,
                a: 0.28 * alpha,
            },
            0.9,
        );
        draw_soft_radial_glow(
            rt,
            D2D_POINT_2F {
                x: cx,
                y: pill_rect.bottom - 0.5,
            },
            pw * 0.42,
            2.2,
            1.0,
            0.66,
            0.24,
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
        let blast = Self::blast_pulse(ctx.local_time);
        let recoil = blast * 1.6;
        let face_cx = cx - recoil * 0.45;
        let head_y = cy + (ctx.phase_time * 1.9).sin() * 0.3;
        let eye_y = head_y + 2.2;

        let spike_kick = blast * 1.2;

        // 1. Rambut: massa halus + helai meruncing (bezier, gaya anime)
        let blonde = D2D1_COLOR_F {
            r: 0.96,
            g: 0.90,
            b: 0.70,
            a: alpha,
        };
        let blonde_dark = D2D1_COLOR_F {
            r: 0.66,
            g: 0.56,
            b: 0.36,
            a: alpha,
        };
        let blonde_hi = D2D1_COLOR_F {
            r: 1.0,
            g: 0.97,
            b: 0.86,
            a: 0.80 * alpha,
        };
        let rim_col = D2D1_COLOR_F {
            r: 0.80,
            g: 0.62,
            b: 0.30,
            a: 0.55 * alpha,
        };

        // 1a. Massa dasar di belakang helai (siluet tengkorak, mulus).
        let _ = fill_smooth_shape(
            rt,
            factory,
            &[
                D2D_POINT_2F {
                    x: face_cx - 18.2,
                    y: head_y - 1.5,
                },
                D2D_POINT_2F {
                    x: face_cx - 16.0,
                    y: head_y - 9.5 - spike_kick * 0.5,
                },
                D2D_POINT_2F {
                    x: face_cx - 8.0,
                    y: head_y - 12.5 - spike_kick * 0.7,
                },
                D2D_POINT_2F {
                    x: face_cx + 1.0,
                    y: head_y - 13.5 - spike_kick * 0.8,
                },
                D2D_POINT_2F {
                    x: face_cx + 9.5,
                    y: head_y - 12.0 - spike_kick * 0.7,
                },
                D2D_POINT_2F {
                    x: face_cx + 16.8,
                    y: head_y - 8.5 - spike_kick * 0.5,
                },
                D2D_POINT_2F {
                    x: face_cx + 18.4,
                    y: head_y - 1.0,
                },
                D2D_POINT_2F {
                    x: face_cx + 13.0,
                    y: head_y - 3.5,
                },
                D2D_POINT_2F {
                    x: face_cx + 3.0,
                    y: head_y - 6.0,
                },
                D2D_POINT_2F {
                    x: face_cx - 8.0,
                    y: head_y - 5.5,
                },
                D2D_POINT_2F {
                    x: face_cx - 14.5,
                    y: head_y - 3.0,
                },
            ],
            blonde_dark,
        );

        // 1b. Helai asimetris: pangkal lebar di kulit kepala, ujung melengkung keluar.
        //     (base_x, base_y, tip_x, tip_y, bend, width)
        let locks: [(f32, f32, f32, f32, f32, f32); 10] = [
            (-16.6, -3.0, -23.6, -13.4, 1.9, 6.4),
            (-13.2, -5.6, -17.4, -19.6, -1.2, 6.0),
            (-8.6, -7.0, -9.4, -22.2, 1.6, 6.6),
            (-3.4, -7.8, 1.6, -22.8, -1.8, 6.8),
            (2.6, -7.8, 9.4, -20.8, 2.0, 6.2),
            (8.4, -7.0, 16.4, -17.2, -1.4, 6.0),
            (13.4, -5.2, 22.4, -11.4, 1.3, 5.6),
            (16.6, -2.6, 21.6, 4.2, 1.0, 4.2),
            (-16.8, -2.2, -20.8, 3.6, -1.0, 4.0),
            (5.6, -7.6, 12.2, -25.2, 1.2, 4.6),
        ];
        for (i, &(bx, by, tx, ty, bend, w)) in locks.iter().enumerate() {
            let lift = if i < 7 { spike_kick } else { spike_kick * 0.2 };
            hair_lock(
                rt,
                factory,
                D2D_POINT_2F {
                    x: face_cx + bx,
                    y: head_y + by,
                },
                D2D_POINT_2F {
                    x: face_cx + tx - recoil * 0.25,
                    y: head_y + ty - lift,
                },
                bend,
                w,
                blonde,
                rim_col,
            );
        }

        // 1c. Kilau tepi: garis mulus menyusuri tiga helai depan.
        for &(bx, by, tx, ty, bend, _) in &locks[..3] {
            let ox = face_cx + bx;
            let oy = head_y + by;
            let ex = face_cx + tx;
            let ey = head_y + ty - spike_kick;
            stroke_smooth_curve(
                rt,
                factory,
                &[
                    D2D_POINT_2F {
                        x: ox + bend * 0.5,
                        y: oy,
                    },
                    D2D_POINT_2F {
                        x: (ox + ex) * 0.5 + bend * 0.9,
                        y: (oy + ey) * 0.5,
                    },
                    D2D_POINT_2F {
                        x: ex,
                        y: ey + 1.4,
                    },
                ],
                blonde_hi,
                0.9,
            );
        }
        // Halo tipis di ubun-ubun biar rambut terpisah dari latar.
        draw_soft_radial_glow(
            rt,
            D2D_POINT_2F {
                x: face_cx + 1.0,
                y: head_y - 11.0,
            },
            2.0,
            16.0,
            1.0,
            0.82,
            0.45,
            0.10 * alpha,
        );

        // 2. Dasar wajah
        fill_polygon(
            rt,
            factory,
            &[
                D2D_POINT_2F {
                    x: face_cx - 15.5,
                    y: head_y - 4.0,
                },
                D2D_POINT_2F {
                    x: face_cx + 15.5,
                    y: head_y - 4.0,
                },
                D2D_POINT_2F {
                    x: face_cx + 13.0,
                    y: head_y + 6.0,
                },
                D2D_POINT_2F {
                    x: face_cx,
                    y: head_y + 8.5,
                },
                D2D_POINT_2F {
                    x: face_cx - 13.0,
                    y: head_y + 6.0,
                },
            ],
            D2D1_COLOR_F {
                r: 0.09,
                g: 0.08,
                b: 0.09,
                a: alpha,
            },
        );

        let mask_col = D2D1_COLOR_F {
            r: 0.04,
            g: 0.04,
            b: 0.05,
            a: alpha,
        };
        let orange = D2D1_COLOR_F {
            r: 1.0,
            g: 0.44,
            b: 0.09,
            a: alpha,
        };
        let orange_dim = D2D1_COLOR_F {
            r: 0.72,
            g: 0.26,
            b: 0.05,
            a: alpha,
        };
        let steel = D2D1_COLOR_F {
            r: 0.66,
            g: 0.68,
            b: 0.72,
            a: alpha,
        };

        // 3. Masker mata hitam bertepi oranye (mulai dari pelipis ke pelipis)
        fill_polygon(
            rt,
            factory,
            &[
                D2D_POINT_2F {
                    x: face_cx - 15.4,
                    y: head_y - 1.6,
                },
                D2D_POINT_2F {
                    x: face_cx - 8.0,
                    y: head_y - 2.6,
                },
                D2D_POINT_2F {
                    x: face_cx,
                    y: head_y - 1.2,
                },
                D2D_POINT_2F {
                    x: face_cx + 8.0,
                    y: head_y - 2.6,
                },
                D2D_POINT_2F {
                    x: face_cx + 15.4,
                    y: head_y - 1.6,
                },
                D2D_POINT_2F {
                    x: face_cx + 14.6,
                    y: head_y + 4.6,
                },
                D2D_POINT_2F {
                    x: face_cx + 7.0,
                    y: head_y + 5.6,
                },
                D2D_POINT_2F {
                    x: face_cx,
                    y: head_y + 4.4,
                },
                D2D_POINT_2F {
                    x: face_cx - 7.0,
                    y: head_y + 5.6,
                },
                D2D_POINT_2F {
                    x: face_cx - 14.6,
                    y: head_y + 4.6,
                },
            ],
            mask_col,
        );
        // Trim oranye tepi atas masker
        draw_polyline(
            rt,
            factory,
            &[
                D2D_POINT_2F {
                    x: face_cx - 15.0,
                    y: head_y - 1.8,
                },
                D2D_POINT_2F {
                    x: face_cx,
                    y: head_y - 1.2,
                },
                D2D_POINT_2F {
                    x: face_cx + 15.0,
                    y: head_y - 1.8,
                },
            ],
            orange_dim,
            0.9,
        );

        // 4. Granat pelipis + cincin pin
        for side in [-1.0_f32, 1.0_f32] {
            let gx2 = face_cx + side * 16.4;
            let gy2 = head_y + 1.0;
            if let Ok(gb) = rt.CreateSolidColorBrush(&orange, None) {
                rt.FillRoundedRectangle(
                    &D2D1_ROUNDED_RECT {
                        rect: D2D_RECT_F {
                            left: gx2 - 3.0,
                            top: gy2 - 4.2,
                            right: gx2 + 3.0,
                            bottom: gy2 + 4.2,
                        },
                        radiusX: 2.2,
                        radiusY: 2.2,
                    },
                    &gb,
                );
            }
            if let Ok(dark_b) = rt.CreateSolidColorBrush(&mask_col, None) {
                for k in 0..2 {
                    let ky = gy2 - 1.2 + k as f32 * 2.2;
                    rt.DrawLine(
                        D2D_POINT_2F {
                            x: gx2 - 2.0,
                            y: ky,
                        },
                        D2D_POINT_2F {
                            x: gx2 + 2.0,
                            y: ky,
                        },
                        &dark_b,
                        0.6,
                        None,
                    );
                }
            }
            if let Ok(pin_b) = rt.CreateSolidColorBrush(&steel, None) {
                rt.DrawEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F {
                            x: gx2 + side * 3.4,
                            y: gy2 - 3.0,
                        },
                        radiusX: 1.5,
                        radiusY: 1.5,
                    },
                    &pin_b,
                    0.8,
                    None,
                );
            }
        }

        // 5. Mata merah menyala di balik masker
        draw_capsule_eyes(
            rt,
            factory,
            face_cx,
            eye_y,
            9.0,
            D2D1_COLOR_F {
                r: 0.94,
                g: 0.12,
                b: 0.16,
                a: 1.0,
            },
            D2D1_COLOR_F {
                r: 1.0,
                g: 0.74,
                b: 0.72,
                a: 1.0,
            },
            (1.0, 0.20, 0.16),
            0.80 + 0.18 * blast,
            alpha,
            ctx,
        );

        // 6. Pelindung dagu metal dengan ventilasi
        if let Ok(jb) = rt.CreateSolidColorBrush(&steel, None) {
            for k in 0..3 {
                let kx = face_cx - 4.0 + k as f32 * 4.0;
                rt.DrawLine(
                    D2D_POINT_2F {
                        x: kx,
                        y: head_y + 6.2,
                    },
                    D2D_POINT_2F {
                        x: kx,
                        y: head_y + 8.0,
                    },
                    &jb,
                    0.7,
                    None,
                );
            }
        }
        draw_polyline(
            rt,
            factory,
            &[
                D2D_POINT_2F {
                    x: face_cx - 6.0,
                    y: head_y + 6.0,
                },
                D2D_POINT_2F {
                    x: face_cx,
                    y: head_y + 8.6,
                },
                D2D_POINT_2F {
                    x: face_cx + 6.0,
                    y: head_y + 6.0,
                },
            ],
            steel,
            1.0,
        );

        // 7. Gauntlet granat + ledakan di lengan kanan
        let gx = cx + 35.0 - recoil;
        let gy = head_y + 2.2;
        let burst_scale = 0.40 + 0.90 * blast;
        let bx = gx + 11.0;
        let by = gy - 3.0;

        // Ledakan: glow besar
        draw_soft_radial_glow(
            rt,
            D2D_POINT_2F { x: bx, y: by },
            17.0 * burst_scale,
            14.0 * burst_scale,
            1.0,
            0.50,
            0.10,
            (0.30 + 0.60 * blast) * alpha,
        );

        // Serpihan radial
        for i in 0..8 {
            let fi = i as f32;
            let ang = fi * std::f32::consts::PI / 4.0 + ctx.phase_time * 0.8;
            let len = (5.0 + 9.5 * blast) * (0.7 + 0.3 * (fi * 2.1).sin());
            draw_polyline(
                rt,
                factory,
                &[
                    D2D_POINT_2F {
                        x: bx + ang.cos() * (3.0 + blast * 2.0),
                        y: by + ang.sin() * (3.0 + blast * 2.0),
                    },
                    D2D_POINT_2F {
                        x: bx + ang.cos() * len,
                        y: by + ang.sin() * len,
                    },
                ],
                D2D1_COLOR_F {
                    r: 1.0,
                    g: 0.78,
                    b: 0.30,
                    a: (0.25 + 0.65 * blast) * alpha,
                },
                1.1,
            );
        }

        // Bintang ledakan
        let mut star_pts = [D2D_POINT_2F { x: 0.0, y: 0.0 }; 16];
        for (i, pt) in star_pts.iter_mut().enumerate() {
            let angle = (i as f32) * std::f32::consts::PI / 8.0 + ctx.phase_time * 0.6;
            let rad = if i % 2 == 0 {
                11.0 * burst_scale
            } else {
                4.4 * burst_scale
            };
            *pt = D2D_POINT_2F {
                x: bx + angle.cos() * rad,
                y: by + angle.sin() * rad,
            };
        }
        fill_polygon(
            rt,
            factory,
            &star_pts,
            D2D1_COLOR_F {
                r: 1.0,
                g: 0.62,
                b: 0.16,
                a: (0.40 + 0.60 * blast) * alpha,
            },
        );
        draw_soft_circle(
            rt,
            D2D_POINT_2F { x: bx, y: by },
            3.6 * burst_scale,
            3.6 * burst_scale,
            D2D1_COLOR_F {
                r: 1.0,
                g: 0.97,
                b: 0.86,
                a: (0.45 + 0.55 * blast) * alpha,
            },
        );

        // Badan gauntlet: panel gelap + aksen oranye
        fill_polygon(
            rt,
            factory,
            &[
                D2D_POINT_2F {
                    x: gx - 6.5,
                    y: gy + 7.5,
                },
                D2D_POINT_2F {
                    x: gx - 1.5,
                    y: gy + 1.0,
                },
                D2D_POINT_2F {
                    x: gx + 4.0,
                    y: gy - 3.8,
                },
                D2D_POINT_2F {
                    x: gx + 6.5,
                    y: gy + 0.8,
                },
                D2D_POINT_2F {
                    x: gx + 2.2,
                    y: gy + 5.0,
                },
                D2D_POINT_2F {
                    x: gx - 2.0,
                    y: gy + 9.5,
                },
            ],
            D2D1_COLOR_F {
                r: 0.06,
                g: 0.06,
                b: 0.08,
                a: alpha,
            },
        );
        draw_polyline(
            rt,
            factory,
            &[
                D2D_POINT_2F {
                    x: gx - 2.2,
                    y: gy + 2.0,
                },
                D2D_POINT_2F {
                    x: gx + 3.2,
                    y: gy - 2.2,
                },
            ],
            orange,
            1.3,
        );
        if let Ok(vb) = rt.CreateSolidColorBrush(&orange_dim, None) {
            for k in 0..3 {
                let ky = gy + 3.0 + k as f32 * 1.8;
                rt.DrawLine(
                    D2D_POINT_2F {
                        x: gx - 3.6 + k as f32 * 0.4,
                        y: ky,
                    },
                    D2D_POINT_2F {
                        x: gx + 1.8 + k as f32 * 0.4,
                        y: ky,
                    },
                    &vb,
                    0.6,
                    None,
                );
            }
        }
        // Cincin pin granat
        if let Ok(pb) = rt.CreateSolidColorBrush(&steel, None) {
            rt.DrawEllipse(
                &D2D1_ELLIPSE {
                    point: D2D_POINT_2F {
                        x: gx - 4.2,
                        y: gy + 6.4,
                    },
                    radiusX: 1.7,
                    radiusY: 1.7,
                },
                &pb,
                0.8,
                None,
            );
        }
    }
}
