// src/mascot/eren.rs
// Fase 4: Eren Yeager -> Attack Titan.
// Detail mengikuti referensi: rahang tanpa bibir dengan gigi terekspos, otot pipi yang
// menonjol, rambut memanjang, mata hijau menyala, uap panas dari bahu, dan petir emas
// saat transformasi.
use super::core::*;
use windows::Win32::Graphics::Direct2D::Common::{D2D1_COLOR_F, D2D_POINT_2F, D2D_RECT_F};
use windows::Win32::Graphics::Direct2D::{ID2D1Factory, ID2D1HwndRenderTarget, D2D1_ELLIPSE};

pub struct ErenAttackTitan;

impl ErenAttackTitan {
    pub fn new() -> Self {
        Self
    }

    /// 0.0 = Eren manusia, 1.0 = Attack Titan
    fn titan_factor(t: f32) -> f32 {
        smoothstep(2.8, 3.9, t)
    }

    /// Puncak kilatan petir emas saat transformasi
    fn lightning_flash(t: f32) -> f32 {
        if t < 2.7 || t > 4.6 {
            0.0
        } else if t < 3.2 {
            smoothstep(2.7, 3.2, t)
        } else {
            1.0 - smoothstep(3.2, 4.6, t)
        }
    }

    /// Uap titan: kuat tepat setelah transformasi lalu menetap tipis.
    fn steam_factor(t: f32) -> f32 {
        if t < 3.4 {
            0.0
        } else if t < 5.0 {
            smoothstep(3.4, 5.0, t)
        } else {
            0.55 + 0.45 * smoothstep(5.0, 8.0, t)
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
        11.0
    }

    fn override_look_target(&self, ctx: &MascotContext) -> Option<(f32, f32)> {
        let flash = Self::lightning_flash(ctx.local_time);
        if flash > 0.1 {
            Some((1.8 * flash, -0.4 * flash))
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
        let titan = Self::titan_factor(ctx.local_time);
        let flash = Self::lightning_flash(ctx.local_time);
        let steam = Self::steam_factor(ctx.local_time);
        let pw = pill_rect.right - pill_rect.left;
        let ph = pill_rect.bottom - pill_rect.top;
        let cx = (pill_rect.left + pill_rect.right) * 0.5;

        // Dasar: hijau hangat (manusia) -> kuning panas (titan)
        draw_soft_radial_glow(
            rt,
            D2D_POINT_2F {
                x: cx,
                y: pill_rect.bottom - 1.0,
            },
            pw * 0.48,
            ph * 0.72,
            lerp(0.26, 0.96, titan.max(flash)),
            lerp(0.80, 0.78, titan),
            lerp(0.50, 0.26, titan),
            (0.22 + 0.20 * titan + 0.30 * flash) * alpha,
        );

        // Petir transformasi (dan bunga api berkala saat titan)
        let periodic_arc = if titan > 0.5 {
            ((ctx.phase_time * 8.0).sin() - 0.60).max(0.0) * 1.6
        } else {
            0.0
        };
        let bolt_a = flash.max(periodic_arc).clamp(0.0, 1.0) * alpha;

        if bolt_a > 0.05 {
            let glow_col = D2D1_COLOR_F {
                r: 1.0,
                g: 0.78,
                b: 0.14,
                a: bolt_a * 0.42,
            };
            let core_col = D2D1_COLOR_F {
                r: 1.0,
                g: 0.98,
                b: 0.78,
                a: bolt_a,
            };
            let rx = cx + pw * 0.24;
            let ry = pill_rect.top + 2.0;

            draw_lightning_bolt(
                rt,
                factory,
                &[
                    D2D_POINT_2F {
                        x: rx + 13.0,
                        y: ry,
                    },
                    D2D_POINT_2F {
                        x: rx + 4.0,
                        y: ry + 8.0,
                    },
                    D2D_POINT_2F {
                        x: rx + 9.0,
                        y: ry + 12.5,
                    },
                    D2D_POINT_2F {
                        x: rx - 3.0,
                        y: ry + 20.0,
                    },
                    D2D_POINT_2F {
                        x: rx - 13.0,
                        y: pill_rect.bottom - 2.0,
                    },
                ],
                glow_col,
                core_col,
                1.5,
            );
        }

        // Uap titan: kepulan lembut naik dari bahu
        if steam > 0.05 {
            for i in 0..7 {
                let fi = i as f32;
                let life = (ctx.phase_time * (0.36 + fi * 0.06) + fi * 0.19) % 1.0;
                let sx = cx + pw * (0.06 + 0.40 * ((fi * 0.19) % 1.0))
                    + (ctx.phase_time * 1.2 + fi).sin() * 2.4
                    + life * 7.0;
                let sy = pill_rect.bottom - 4.0 - life * (ph * 0.80);
                let sa = (1.0 - life) * life * 3.2 * 0.26 * steam * alpha;
                draw_soft_circle(
                    rt,
                    D2D_POINT_2F { x: sx, y: sy },
                    7.5 + life * 7.0,
                    5.0 + life * 4.5,
                    D2D1_COLOR_F {
                        r: 0.90,
                        g: 0.94,
                        b: 0.92,
                        a: sa,
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
            ctx.phase_time * 0.5,
            D2D1_COLOR_F {
                r: 1.0,
                g: lerp(0.86, 0.70, titan),
                b: lerp(0.44, 0.22, titan),
                a: (0.07 + 0.05 * flash + 0.03 * titan) * alpha,
            },
        );
        draw_soft_radial_glow(
            rt,
            D2D_POINT_2F {
                x: cx,
                y: pill_rect.top + ph * 0.60,
            },
            pw * 0.18,
            ph * 0.58,
            lerp(0.55, 1.0, flash),
            lerp(0.90, 0.94, flash),
            lerp(0.50, 0.52, flash),
            (0.07 + 0.06 * titan) * alpha,
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
                r: 1.0,
                g: 0.86,
                b: 0.44,
                a: (0.26 + 0.10 * titan) * alpha,
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
            1.0,
            0.86,
            0.48,
            (0.24 + 0.10 * flash) * alpha,
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
        let titan = Self::titan_factor(ctx.local_time);
        let flash = Self::lightning_flash(ctx.local_time);
        let face_cx = cx;
        let breath = (ctx.phase_time * 1.6).sin() * 0.35;
        let head_y = cy + breath;
        let eye_y = head_y + 1.9;

        // 1. Rambut: pendek (Eren) memanjang saat titan, ikut tertiup
        let hair_col = D2D1_COLOR_F {
            r: lerp(0.44, 0.38, titan),
            g: lerp(0.31, 0.26, titan),
            b: lerp(0.24, 0.20, titan),
            a: alpha,
        };
        let hair_hi = D2D1_COLOR_F {
            r: 0.34,
            g: 0.24,
            b: 0.20,
            a: alpha * 0.75,
        };
        let hair_grow = titan * 3.0;
        let sway = (ctx.phase_time * 1.9).sin() * (0.6 + titan * 1.4);

        // 1a. Massa rambut: volume penuh di atas tengkorak, turun menutup pelipis.
        let _ = fill_smooth_shape(
            rt,
            factory,
            &[
                D2D_POINT_2F { x: face_cx - 17.8 - hair_grow, y: head_y + 7.0 + hair_grow },
                D2D_POINT_2F { x: face_cx - 19.6 - hair_grow * 0.3, y: head_y - 6.2 },
                D2D_POINT_2F { x: face_cx - 13.8, y: head_y - 16.2 - hair_grow * 0.35 },
                D2D_POINT_2F { x: face_cx - 2.0, y: head_y - 19.4 - hair_grow * 0.4 },
                D2D_POINT_2F { x: face_cx + 9.8, y: head_y - 17.6 - hair_grow * 0.35 },
                D2D_POINT_2F { x: face_cx + 18.4 + hair_grow * 0.3, y: head_y - 7.4 },
                D2D_POINT_2F { x: face_cx + 17.2 + hair_grow, y: head_y + 7.2 + hair_grow },
                D2D_POINT_2F { x: face_cx + 12.0, y: head_y - 4.6 },
                D2D_POINT_2F { x: face_cx + 0.6, y: head_y - 8.2 },
                D2D_POINT_2F { x: face_cx - 12.2, y: head_y - 4.8 },
            ],
            hair_col,
        );

        // 1a2. Rim terang menelusuri siluet rambut: kunci terbaca di ukuran kecil.
        stroke_smooth_curve(
            rt,
            factory,
            &[
                D2D_POINT_2F { x: face_cx - 18.6 - hair_grow * 0.5, y: head_y + 5.4 + hair_grow * 0.8 },
                D2D_POINT_2F { x: face_cx - 19.0 - hair_grow * 0.2, y: head_y - 6.6 },
                D2D_POINT_2F { x: face_cx - 12.6, y: head_y - 16.6 - hair_grow * 0.3 },
                D2D_POINT_2F { x: face_cx - 1.4, y: head_y - 19.8 - hair_grow * 0.4 },
                D2D_POINT_2F { x: face_cx + 10.4, y: head_y - 17.8 - hair_grow * 0.3 },
                D2D_POINT_2F { x: face_cx + 18.8 + hair_grow * 0.2, y: head_y - 7.6 },
                D2D_POINT_2F { x: face_cx + 17.8 + hair_grow * 0.8, y: head_y + 5.6 + hair_grow * 0.8 },
            ],
            D2D1_COLOR_F { r: 0.80, g: 0.62, b: 0.45, a: alpha * 0.80 },
            1.45,
        );

                // 1a3. Spike mahkota: siluet rambut anime, menjulang dari ubun-ubun.
        for (bx, by, tx, ty, bend, w) in [
            (-13.0, -14.0, -17.5, -24.0, -2.0, 4.6),
            (-6.0, -17.0, -8.0, -27.5, 1.5, 5.0),
            (0.5, -18.5, 1.5, -29.0, -1.5, 5.2),
            (7.0, -17.0, 10.5, -26.5, 2.0, 4.8),
            (13.0, -13.5, 18.5, -22.0, 1.0, 4.4),
        ] {
            hair_lock(
                rt,
                factory,
                D2D_POINT_2F { x: face_cx + bx, y: head_y + by },
                D2D_POINT_2F {
                    x: face_cx + tx + sway * 0.5,
                    y: head_y + ty,
                },
                bend,
                w,
                hair_col,
                D2D1_COLOR_F { a: alpha * 0.30, ..hair_hi },
            );
        }

        // 1b. Poni: helai meruncing jatuh menutup dahi, ikut `sway`.
        //     (base_x, base_y, tip_x, tip_y, bend, width)
        let fringe: [(f32, f32, f32, f32, f32, f32); 6] = [
            (-12.6, -10.2, -11.2, -2.0, 1.4, 5.0),
            (-7.6, -12.0, -4.6, -1.4, 1.0, 4.6),
            (-2.2, -12.8, -0.4 + sway, -3.2, -1.2, 4.2),
            (3.0, -12.6, 6.2 + sway * 0.6, -1.8, -1.4, 4.4),
            (8.2, -11.6, 10.4, -2.6, -1.0, 4.6),
            (12.4, -9.4, 14.0 + sway * 0.4, -0.4, -1.2, 4.2),
        ];
        for &(bx, by, tx, ty, bend, w) in &fringe {
            hair_lock(
                rt,
                factory,
                D2D_POINT_2F {
                    x: face_cx + bx,
                    y: head_y + by,
                },
                D2D_POINT_2F {
                    x: face_cx + tx,
                    y: head_y + ty,
                },
                bend,
                w,
                hair_col,
                D2D1_COLOR_F {
                    a: 0.0,
                    ..hair_hi
                },
            );
        }

        // 1c. Samping: dua helai per sisi, memanjang saat titan.
        for side in [-1.0_f32, 1.0_f32] {
            hair_lock(
                rt,
                factory,
                D2D_POINT_2F {
                    x: face_cx + side * 16.0,
                    y: head_y - 4.0,
                },
                D2D_POINT_2F {
                    x: face_cx + side * (17.6 + hair_grow * 0.8),
                    y: head_y + 7.0 + hair_grow * 1.2,
                },
                side * (1.0 + sway * 0.2),
                5.0 + hair_grow * 0.5,
                hair_col,
                D2D1_COLOR_F {
                    a: 0.0,
                    ..hair_hi
                },
            );
        }

        // Attack Titan: rambut panjang menyapu pipi, bukan kubah bulat.
        for side in [-1.0_f32, 1.0_f32] {
            for (bx, by, tx, ty, width) in [
                (12.0, -11.0, 22.0, 11.0, 8.0),
                (15.0, -5.0, 19.0, 18.0, 6.0),
                (10.0, -13.0, 25.0, 3.0, 5.0),
            ] {
                hair_lock(
                    rt, factory,
                    D2D_POINT_2F { x: face_cx + side * bx, y: head_y + by },
                    D2D_POINT_2F {
                        x: face_cx + side * (tx + hair_grow) + sway * 0.4,
                        y: head_y + ty + hair_grow,
                    },
                    side * 5.0, width, hair_col,
                    D2D1_COLOR_F { a: alpha * 0.22, ..hair_hi },
                );
            }
        }

        // 1d. Kilau: satu sapuan mulus di ubun-ubun + pantulan poni kanan.
        stroke_smooth_curve(
            rt,
            factory,
            &[
                D2D_POINT_2F {
                    x: face_cx - 9.6,
                    y: head_y - 9.4,
                },
                D2D_POINT_2F {
                    x: face_cx - 1.0,
                    y: head_y - 12.2,
                },
                D2D_POINT_2F {
                    x: face_cx + 8.6,
                    y: head_y - 9.8,
                },
            ],
            hair_hi,
            1.0,
        );
        stroke_smooth_curve(
            rt,
            factory,
            &[
                D2D_POINT_2F {
                    x: face_cx + 4.6,
                    y: head_y - 12.0,
                },
                D2D_POINT_2F {
                    x: face_cx + 6.4,
                    y: head_y - 7.4,
                },
                D2D_POINT_2F {
                    x: face_cx + 7.8,
                    y: head_y - 3.2,
                },
            ],
            D2D1_COLOR_F {
                r: hair_hi.r,
                g: hair_hi.g,
                b: hair_hi.b,
                a: hair_hi.a * 0.55,
            },
            0.8,
        );

        // 2. Rahang: manusia membulat, titan terbuka dengan gigi
        if titan < 0.35 {
            fill_polygon(
                rt,
                factory,
                &[
                    D2D_POINT_2F {
                        x: face_cx - 13.0,
                        y: head_y + 2.0,
                    },
                    D2D_POINT_2F {
                        x: face_cx + 13.0,
                        y: head_y + 2.0,
                    },
                    D2D_POINT_2F {
                        x: face_cx + 10.0,
                        y: head_y + 8.0,
                    },
                    D2D_POINT_2F {
                        x: face_cx,
                        y: head_y + 9.6,
                    },
                    D2D_POINT_2F {
                        x: face_cx - 10.0,
                        y: head_y + 8.0,
                    },
                ],
                D2D1_COLOR_F {
                    r: 0.10,
                    g: 0.08,
                    b: 0.08,
                    a: alpha,
                },
            );
        }

        if titan > 0.15 {
            let ta = smoothstep(0.15, 0.85, titan) * alpha;

            // Otot pipi & rahang atas (garis vertikal tegas)
            let muscle = D2D1_COLOR_F {
                r: 0.52,
                g: 0.22,
                b: 0.18,
                a: ta * 0.85,
            };
            for side in [-1.0_f32, 1.0_f32] {
                for k in 0..2 {
                    let off = 9.0 + k as f32 * 2.6;
                    draw_polyline(
                        rt,
                        factory,
                        &[
                            D2D_POINT_2F {
                                x: face_cx + side * off,
                                y: head_y + 1.0,
                            },
                            D2D_POINT_2F {
                                x: face_cx + side * (off + 1.4),
                                y: head_y + 6.0,
                            },
                        ],
                        muscle,
                        1.1,
                    );
                }
            }

            // Rahang bawah terbuka: celah gelap + deretan gigi
            let jaw_top = head_y + 6.2;
            fill_polygon(
                rt,
                factory,
                &[
                    D2D_POINT_2F {
                        x: face_cx - 8.6,
                        y: jaw_top,
                    },
                    D2D_POINT_2F {
                        x: face_cx + 8.6,
                        y: jaw_top,
                    },
                    D2D_POINT_2F {
                        x: face_cx + 6.6,
                        y: jaw_top + 4.6,
                    },
                    D2D_POINT_2F {
                        x: face_cx,
                        y: jaw_top + 5.6,
                    },
                    D2D_POINT_2F {
                        x: face_cx - 6.6,
                        y: jaw_top + 4.6,
                    },
                ],
                D2D1_COLOR_F {
                    r: 0.03,
                    g: 0.02,
                    b: 0.02,
                    a: ta,
                },
            );

            // Gigi atas
            if let Ok(tb) = rt.CreateSolidColorBrush(
                &D2D1_COLOR_F {
                    r: 0.94,
                    g: 0.92,
                    b: 0.84,
                    a: ta,
                },
                None,
            ) {
                for k in 0..7 {
                    let kx = face_cx - 7.4 + k as f32 * 2.45;
                    let w = if k % 2 == 0 { 1.05 } else { 0.85 };
                    rt.FillRoundedRectangle(
                        &windows::Win32::Graphics::Direct2D::D2D1_ROUNDED_RECT {
                            rect: D2D_RECT_F {
                                left: kx - w,
                                top: jaw_top + 0.3,
                                right: kx + w,
                                bottom: jaw_top + 2.4,
                            },
                            radiusX: 0.4,
                            radiusY: 0.4,
                        },
                        &tb,
                    );
                }
            }

            // Gigi bawah (baris lebih kecil)
            if let Ok(tb2) = rt.CreateSolidColorBrush(
                &D2D1_COLOR_F {
                    r: 0.82,
                    g: 0.78,
                    b: 0.70,
                    a: ta * 0.9,
                },
                None,
            ) {
                for k in 0..6 {
                    let kx = face_cx - 6.0 + k as f32 * 2.4;
                    rt.FillRoundedRectangle(
                        &windows::Win32::Graphics::Direct2D::D2D1_ROUNDED_RECT {
                            rect: D2D_RECT_F {
                                left: kx - 0.85,
                                top: jaw_top + 3.6,
                                right: kx + 0.85,
                                bottom: jaw_top + 4.8,
                            },
                            radiusX: 0.35,
                            radiusY: 0.35,
                        },
                        &tb2,
                    );
                }
            }
        }

        // 3. Mata hijau menyala
        let outer_col = lerp_color(
            D2D1_COLOR_F {
                r: 0.20,
                g: 0.84,
                b: 0.56,
                a: 1.0,
            },
            D2D1_COLOR_F {
                r: 0.40,
                g: 1.0,
                b: 0.52,
                a: 1.0,
            },
            titan,
        );
        let core_col = lerp_color(
            D2D1_COLOR_F {
                r: 0.80,
                g: 1.0,
                b: 0.90,
                a: 1.0,
            },
            D2D1_COLOR_F {
                r: 1.0,
                g: 1.0,
                b: 0.86,
                a: 1.0,
            },
            titan,
        );
        draw_capsule_eyes(
            rt,
            factory,
            face_cx,
            eye_y,
            9.0,
            outer_col,
            core_col,
            (lerp(0.20, 0.72, titan), 1.0, lerp(0.56, 0.46, titan)),
            titan * 0.70 + flash * 0.25,
            alpha,
            ctx,
        );

        // 4. Titan marks di bawah mata (garis ganda)
        if titan > 0.15 {
            let ma = smoothstep(0.15, 0.85, titan) * alpha;
            if let Ok(mb) = rt.CreateSolidColorBrush(
                &D2D1_COLOR_F {
                    r: 1.0,
                    g: 0.88,
                    b: 0.60,
                    a: ma * 0.80,
                },
                None,
            ) {
                for side in [-1.0_f32, 1.0_f32] {
                    for k in 0..3 {
                        let bx = face_cx + side * (8.8 + k as f32 * 0.9);
                        let by = eye_y + 5.2 + k as f32 * 0.55;
                        rt.DrawLine(
                            D2D_POINT_2F { x: bx, y: by },
                            D2D_POINT_2F {
                                x: bx + side * 1.0,
                                y: by + 3.0,
                            },
                            &mb,
                            1.0,
                            None,
                        );
                    }
                }
            }
        }

        // 5. Glow mata titan + petir kecil saat transformasi
        if titan > 0.3 || flash > 0.2 {
            let eg = titan.max(flash * 0.8);
            for side in [-1.0_f32, 1.0_f32] {
                draw_soft_radial_glow(
                    rt,
                    D2D_POINT_2F {
                        x: face_cx + side * 9.0,
                        y: eye_y,
                    },
                    8.0,
                    8.0,
                    0.55,
                    1.0,
                    0.60,
                    0.30 * eg * alpha,
                );
            }
        }

        // 6. Uap dari bahu (titik kondensasi)
        if titan > 0.25 {
            for side in [-1.0_f32, 1.0_f32] {
                let base_x = face_cx + side * 15.5;
                let t = ctx.phase_time;
                for k in 0..3 {
                    let fk = k as f32;
                    let life = (t * (0.55 + fk * 0.12) + fk * 0.31) % 1.0;
                    draw_soft_circle(
                        rt,
                        D2D_POINT_2F {
                            x: base_x + side * life * 4.0,
                            y: head_y + 3.0 - life * 11.0,
                        },
                        4.0 + life * 5.0,
                        3.0 + life * 3.5,
                        D2D1_COLOR_F {
                            r: 0.92,
                            g: 0.96,
                            b: 0.94,
                            a: (1.0 - life) * life * 1.4 * 0.30 * titan * alpha,
                        },
                    );
                }
            }
        }

        // 7. Cincin hardening tipis mengelilingi kepala saat titan penuh
        if titan > 0.6 {
            let ha = (titan - 0.6) / 0.4 * alpha * 0.20;
            if let Ok(hb) = rt.CreateSolidColorBrush(
                &D2D1_COLOR_F {
                    r: 1.0,
                    g: 0.86,
                    b: 0.50,
                    a: ha,
                },
                None,
            ) {
                rt.DrawEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F {
                            x: face_cx,
                            y: eye_y + 1.0,
                        },
                        radiusX: 24.0,
                        radiusY: 15.5,
                    },
                    &hb,
                    0.8,
                    None,
                );
            }
        }
    }
}
