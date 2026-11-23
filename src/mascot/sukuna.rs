// src/mascot/sukuna.rs
// Fase 2: Yuji Itadori → Ryomen Sukuna — tato, empat mata, hand sign Malevolent Shrine.
use super::core::*;
use windows::Win32::Graphics::Direct2D::Common::{D2D1_COLOR_F, D2D_POINT_2F, D2D_RECT_F};
use windows::Win32::Graphics::Direct2D::{ID2D1Factory, ID2D1HwndRenderTarget, D2D1_ROUNDED_RECT};

pub struct YujiSukuna;

impl YujiSukuna {
    pub fn new() -> Self {
        Self
    }

    /// 0.0 = Yuji, 1.0 = Sukuna penuh
    fn sukuna_factor(t: f32) -> f32 {
        smoothstep(2.8, 4.2, t)
    }

    /// Kemunculan hand sign di sisi kanan
    fn shrine_gesture(t: f32) -> f32 {
        if t < 4.0 {
            0.0
        } else if t < 5.2 {
            smoothstep(4.0, 5.2, t)
        } else if t < 9.4 {
            1.0
        } else {
            1.0 - smoothstep(9.4, 10.6, t)
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
        if g > 0.15 {
            Some((1.5 * g, -0.3 * g))
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
        let pulse = 0.85 + 0.15 * (ctx.phase_time * 4.2).sin();

        draw_soft_radial_glow(
            rt,
            D2D_POINT_2F {
                x: cx,
                y: pill_rect.bottom - 1.0,
            },
            pw * 0.40,
            ph * 0.70,
            lerp(0.95, 0.86, s),
            lerp(0.65, 0.08, s),
            lerp(0.22, 0.26, s),
            (0.28 + 0.16 * s) * pulse * alpha,
        );

        if s > 0.05 {
            draw_soft_radial_glow(
                rt,
                D2D_POINT_2F {
                    x: cx + pw * 0.22,
                    y: pill_rect.bottom - 2.0,
                },
                pw * 0.32,
                ph * 0.68,
                0.48,
                0.14,
                0.76,
                0.34 * s * pulse * alpha,
            );
        }

        // Dismantle / Cleave: garis tebasan cepat saat Shrine aktif
        if g > 0.4 {
            let slash_wave = (ctx.phase_time * 16.0).sin();
            if slash_wave > 0.25 {
                let sa = ((slash_wave - 0.25) / 0.75) * g * alpha;
                let stroke = create_round_stroke(factory);
                if let Ok(sb) = rt.CreateSolidColorBrush(
                    &D2D1_COLOR_F {
                        r: 1.0,
                        g: 0.32,
                        b: 0.42,
                        a: sa * 0.85,
                    },
                    None,
                ) {
                    let ox = cx + 40.0 + (ctx.phase_time * 9.0).cos() * 6.0;
                    let oy = pill_rect.top + ph * 0.48;
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
                        1.4,
                        stroke.as_ref(),
                    );
                    rt.DrawLine(
                        D2D_POINT_2F {
                            x: ox + 9.0,
                            y: oy - 7.0,
                        },
                        D2D_POINT_2F {
                            x: ox - 8.0,
                            y: oy + 6.0,
                        },
                        &sb,
                        1.1,
                        stroke.as_ref(),
                    );
                }
            }
        }

        // ── Ambience: aura kutukan, berkas sinar, bara, refleksi dasar ──────
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
                g: 0.15,
                b: 0.22,
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
            0.12,
            0.20,
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
                g: 0.28,
                b: 0.32,
                a: 0.30 * alpha,
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
            0.18,
            0.28,
            0.27 * alpha,
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

        let hair_lift = s * 1.8;
        let pink_col = D2D1_COLOR_F {
            r: 0.94,
            g: 0.56,
            b: 0.66,
            a: alpha,
        };
        let undercut_col = D2D1_COLOR_F {
            r: 0.10,
            g: 0.08,
            b: 0.12,
            a: alpha,
        };

        fill_polygon(
            rt,
            factory,
            &[
                D2D_POINT_2F {
                    x: face_cx - 16.0,
                    y: cy + 3.0,
                },
                D2D_POINT_2F {
                    x: face_cx - 15.0,
                    y: cy - 6.0,
                },
                D2D_POINT_2F {
                    x: face_cx + 15.0,
                    y: cy - 6.0,
                },
                D2D_POINT_2F {
                    x: face_cx + 16.0,
                    y: cy + 3.0,
                },
            ],
            undercut_col,
        );

        fill_polygon(
            rt,
            factory,
            &[
                D2D_POINT_2F {
                    x: face_cx - 16.5,
                    y: cy - 4.5,
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
                    x: face_cx + 16.5,
                    y: cy - 4.5,
                },
            ],
            pink_col,
        );

        // Tato Sukuna di dahi
        if s > 0.05 {
            let mark_col = D2D1_COLOR_F {
                r: 0.05,
                g: 0.03,
                b: 0.05,
                a: alpha * s,
            };
            fill_polygon(
                rt,
                factory,
                &[
                    D2D_POINT_2F {
                        x: face_cx - 3.0,
                        y: cy - 7.5,
                    },
                    D2D_POINT_2F {
                        x: face_cx,
                        y: cy - 4.0,
                    },
                    D2D_POINT_2F {
                        x: face_cx + 3.0,
                        y: cy - 7.5,
                    },
                    D2D_POINT_2F {
                        x: face_cx,
                        y: cy - 5.6,
                    },
                ],
                mark_col,
            );
        }

        // Mata utama: amber hangat (Yuji) → crimson (Sukuna)
        let outer_col = lerp_color(
            D2D1_COLOR_F {
                r: 1.0,
                g: 0.74,
                b: 0.28,
                a: 1.0,
            },
            D2D1_COLOR_F {
                r: 1.0,
                g: 0.14,
                b: 0.28,
                a: 1.0,
            },
            s,
        );
        let core_col = lerp_color(
            D2D1_COLOR_F {
                r: 1.0,
                g: 0.94,
                b: 0.76,
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
        let glow_rgb = (1.0, lerp(0.72, 0.12, s), lerp(0.24, 0.26, s));

        draw_capsule_eyes(
            rt,
            factory,
            face_cx,
            eye_y,
            9.0,
            outer_col,
            core_col,
            glow_rgb,
            s * 0.78,
            alpha,
            ctx,
        );

        // Sepasang mata kecil Sukuna di bawah mata utama
        if s > 0.2 {
            let sub_a = smoothstep(0.2, 0.9, s) * alpha * ctx.open_factor();
            if let Ok(sub_b) = rt.CreateSolidColorBrush(
                &D2D1_COLOR_F {
                    r: 1.0,
                    g: 0.22,
                    b: 0.34,
                    a: sub_a,
                },
                None,
            ) {
                for side in [-1.0_f32, 1.0_f32] {
                    let sx = face_cx + side * 10.6 + ctx.look_x * 0.5;
                    let sy = eye_y + 5.8 + ctx.look_y * 0.35;
                    rt.FillRoundedRectangle(
                        &D2D1_ROUNDED_RECT {
                            rect: D2D_RECT_F {
                                left: sx - 2.3,
                                top: sy - 1.0 * sub_a,
                                right: sx + 2.3,
                                bottom: sy + 1.0 * sub_a,
                            },
                            radiusX: 1.0,
                            radiusY: 1.0,
                        },
                        &sub_b,
                    );
                }
            }
        }

        // Gestur kanan: hand sign Malevolent Shrine
        if g > 0.05 {
            let gx = cx + 40.0;
            let gy = cy + 2.0 + (1.0 - g) * 6.0;
            let skin_col = D2D1_COLOR_F {
                r: 0.92,
                g: 0.80,
                b: 0.72,
                a: alpha * g,
            };
            let cuff_col = D2D1_COLOR_F {
                r: 0.08,
                g: 0.06,
                b: 0.10,
                a: alpha * g,
            };

            draw_soft_radial_glow(
                rt,
                D2D_POINT_2F { x: gx, y: gy },
                14.0,
                11.0,
                0.85,
                0.12,
                0.35,
                0.35 * g * alpha,
            );

            fill_polygon(
                rt,
                factory,
                &[
                    D2D_POINT_2F {
                        x: gx,
                        y: gy - 8.5,
                    },
                    D2D_POINT_2F {
                        x: gx - 2.0,
                        y: gy - 1.0,
                    },
                    D2D_POINT_2F {
                        x: gx - 6.8,
                        y: gy + 6.5,
                    },
                    D2D_POINT_2F {
                        x: gx - 9.2,
                        y: gy + 4.8,
                    },
                    D2D_POINT_2F {
                        x: gx - 4.2,
                        y: gy - 2.0,
                    },
                ],
                skin_col,
            );
            fill_polygon(
                rt,
                factory,
                &[
                    D2D_POINT_2F {
                        x: gx,
                        y: gy - 8.5,
                    },
                    D2D_POINT_2F {
                        x: gx + 4.2,
                        y: gy - 2.0,
                    },
                    D2D_POINT_2F {
                        x: gx + 9.2,
                        y: gy + 4.8,
                    },
                    D2D_POINT_2F {
                        x: gx + 6.8,
                        y: gy + 6.5,
                    },
                    D2D_POINT_2F {
                        x: gx + 2.0,
                        y: gy - 1.0,
                    },
                ],
                skin_col,
            );
            fill_polygon(
                rt,
                factory,
                &[
                    D2D_POINT_2F {
                        x: gx - 9.5,
                        y: gy + 4.8,
                    },
                    D2D_POINT_2F {
                        x: gx - 6.5,
                        y: gy + 7.2,
                    },
                    D2D_POINT_2F {
                        x: gx - 7.8,
                        y: gy + 8.5,
                    },
                    D2D_POINT_2F {
                        x: gx - 10.8,
                        y: gy + 6.1,
                    },
                ],
                cuff_col,
            );
            fill_polygon(
                rt,
                factory,
                &[
                    D2D_POINT_2F {
                        x: gx + 9.5,
                        y: gy + 4.8,
                    },
                    D2D_POINT_2F {
                        x: gx + 10.8,
                        y: gy + 6.1,
                    },
                    D2D_POINT_2F {
                        x: gx + 7.8,
                        y: gy + 8.5,
                    },
                    D2D_POINT_2F {
                        x: gx + 6.5,
                        y: gy + 7.2,
                    },
                ],
                cuff_col,
            );
        }
    }
}
