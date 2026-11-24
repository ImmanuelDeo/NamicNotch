// src/mascot/eren.rs
// Fase 4: Eren Yeager (man-bun) -> Attack Titan.
// Gaya dikunci ke Asta Devil Union: siluet rambut + mata kapsul illuminated + petir emas
// transformasi + uap putih, satu gestur di kanan. Tanpa wajah penuh, hidung, atau mulut.
use super::core::*;
use windows::Win32::Graphics::Direct2D::Common::{D2D1_COLOR_F, D2D_POINT_2F, D2D_RECT_F};
use windows::Win32::Graphics::Direct2D::{ID2D1Factory, ID2D1HwndRenderTarget, D2D1_ELLIPSE};

pub struct ErenAttackTitan;

impl ErenAttackTitan {
    pub fn new() -> Self {
        Self
    }

    /// 0.0 = Eren manusia, 1.0 = Attack Titan (transisi 3.0s)
    fn titan_factor(t: f32) -> f32 {
        smoothstep(3.0, 4.2, t)
    }

    /// Kilatan petir emas saat transformasi
    fn lightning_flash(t: f32) -> f32 {
        if t < 3.0 || t > 4.8 {
            0.0
        } else if t < 3.5 {
            smoothstep(3.0, 3.5, t)
        } else {
            1.0 - smoothstep(3.5, 4.8, t)
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
        let pw = pill_rect.right - pill_rect.left;
        let ph = pill_rect.bottom - pill_rect.top;
        let cx = (pill_rect.left + pill_rect.right) * 0.5;

        // Teal tenang (Eren) -> emerald-gold (Titan)
        draw_soft_radial_glow(
            rt,
            D2D_POINT_2F {
                x: cx,
                y: pill_rect.bottom - 1.0,
            },
            pw * 0.46,
            ph * 0.70,
            lerp(0.18, 0.94, titan.max(flash)),
            lerp(0.77, 0.82, titan),
            lerp(0.57, 0.30, titan),
            (0.24 + 0.20 * titan + 0.26 * flash) * alpha,
        );

        // Petir emas transformasi
        let periodic = if titan > 0.6 {
            ((ctx.phase_time * 7.5).sin() - 0.62).max(0.0) * 1.6
        } else {
            0.0
        };
        let bolt = flash.max(periodic).clamp(0.0, 1.0) * alpha;
        if bolt > 0.05 {
            let rx = cx + pw * 0.22;
            let ry = pill_rect.top + 2.0;
            draw_lightning_bolt(
                rt,
                factory,
                &[
                    D2D_POINT_2F {
                        x: rx + 12.0,
                        y: ry,
                    },
                    D2D_POINT_2F {
                        x: rx + 3.0,
                        y: ry + 8.0,
                    },
                    D2D_POINT_2F {
                        x: rx + 8.0,
                        y: ry + 12.0,
                    },
                    D2D_POINT_2F {
                        x: rx - 3.0,
                        y: ry + 19.0,
                    },
                    D2D_POINT_2F {
                        x: rx - 12.0,
                        y: pill_rect.bottom - 2.0,
                    },
                ],
                D2D1_COLOR_F {
                    r: 1.0,
                    g: 0.82,
                    b: 0.18,
                    a: bolt * 0.42,
                },
                D2D1_COLOR_F {
                    r: 1.0,
                    g: 0.98,
                    b: 0.78,
                    a: bolt,
                },
                1.45,
            );
        }

        // Uap putih titan
        if titan > 0.2 {
            for i in 0..5 {
                let fi = i as f32;
                let life = (ctx.phase_time * (0.38 + fi * 0.07) + fi * 0.23) % 1.0;
                let sx = cx + pw * (0.08 + 0.34 * ((fi * 0.23) % 1.0))
                    + (ctx.phase_time * 1.1 + fi).sin() * 2.2
                    + life * 6.0;
                let sy = pill_rect.bottom - 4.0 - life * (ph * 0.78);
                draw_soft_circle(
                    rt,
                    D2D_POINT_2F { x: sx, y: sy },
                    7.0 + life * 6.0,
                    4.8 + life * 4.0,
                    D2D1_COLOR_F {
                        r: 0.90,
                        g: 0.95,
                        b: 0.93,
                        a: (1.0 - life) * life * 3.0 * 0.24 * titan * alpha,
                    },
                );
            }
        }

        // Partikel ringan
        draw_rising_particles(
            rt,
            cx,
            pill_rect.bottom - 1.0,
            pw * 0.34,
            ph * 0.82,
            ctx.phase_time,
            7,
            D2D1_COLOR_F {
                r: 1.0,
                g: lerp(0.86, 0.70, titan),
                b: lerp(0.50, 0.26, titan),
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
        let titan = Self::titan_factor(ctx.local_time);
        let flash = Self::lightning_flash(ctx.local_time);
        let face_cx = cx;
        let eye_y = cy + 1.8;
        let breath = (ctx.phase_time * 1.5).sin() * 0.3;
        let sway = (ctx.phase_time * 1.8).sin();

        // 1. Rambut: espresso rapi ke belakang -> gondrong shaggy
        let hair = D2D1_COLOR_F {
            r: lerp(0.21, 0.16, titan),
            g: lerp(0.14, 0.11, titan),
            b: lerp(0.11, 0.09, titan),
            a: alpha,
        };
        let hair_hi = D2D1_COLOR_F {
            r: 0.34,
            g: 0.24,
            b: 0.19,
            a: alpha * 0.8,
        };
        let grow = titan * 2.8;

        fill_polygon(
            rt,
            factory,
            &[
                D2D_POINT_2F {
                    x: face_cx - 16.5 - grow,
                    y: cy + 4.5 + grow,
                },
                D2D_POINT_2F {
                    x: face_cx - 17.0,
                    y: cy - 6.5,
                },
                D2D_POINT_2F {
                    x: face_cx - 10.5,
                    y: cy - 12.5,
                },
                D2D_POINT_2F {
                    x: face_cx,
                    y: cy - 13.5,
                },
                D2D_POINT_2F {
                    x: face_cx + 10.5,
                    y: cy - 12.5,
                },
                D2D_POINT_2F {
                    x: face_cx + 17.0,
                    y: cy - 6.5,
                },
                D2D_POINT_2F {
                    x: face_cx + 16.5 + grow,
                    y: cy + 4.5 + grow,
                },
                D2D_POINT_2F {
                    x: face_cx + 11.0,
                    y: cy - 3.2,
                },
                D2D_POINT_2F {
                    x: face_cx,
                    y: cy - 5.8,
                },
                D2D_POINT_2F {
                    x: face_cx - 11.0,
                    y: cy - 3.2,
                },
            ],
            hair,
        );

        // 2. Man-bun kecil di belakang atas (fase manusia) -> hilang saat titan
        if titan < 0.6 {
            let bun = D2D1_COLOR_F {
                r: 0.20,
                g: 0.13,
                b: 0.10,
                a: alpha * (1.0 - titan / 0.6),
            };
            if let Ok(bb) = rt.CreateSolidColorBrush(&bun, None) {
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F {
                            x: face_cx + 2.0,
                            y: cy - 13.0,
                        },
                        radiusX: 4.6,
                        radiusY: 3.6,
                    },
                    &bb,
                );
            }
        }

        // 3. Dua poni melengkung di tengah dahi (man-bun) -> terurai saat titan
        let bang_a = (1.0 - titan).max(0.0);
        if bang_a > 0.03 {
            for side in [-1.0_f32, 1.0_f32] {
                fill_polygon(
                    rt,
                    factory,
                    &[
                        D2D_POINT_2F {
                            x: face_cx + side * 1.0,
                            y: cy - 12.0,
                        },
                        D2D_POINT_2F {
                            x: face_cx + side * 5.2,
                            y: cy - 11.0,
                        },
                        D2D_POINT_2F {
                            x: face_cx + side * 2.2 + sway * 0.4,
                            y: cy - 4.6,
                        },
                    ],
                    D2D1_COLOR_F {
                        r: 0.24,
                        g: 0.16,
                        b: 0.12,
                        a: alpha * bang_a,
                    },
                );
            }
        }

        // 4. Rambut shaggy menjuntai di kiri-kanan saat titan
        if titan > 0.2 {
            for side in [-1.0_f32, 1.0_f32] {
                fill_polygon(
                    rt,
                    factory,
                    &[
                        D2D_POINT_2F {
                            x: face_cx + side * 15.0,
                            y: cy - 6.0,
                        },
                        D2D_POINT_2F {
                            x: face_cx + side * 18.5 + sway,
                            y: cy + 2.0,
                        },
                        D2D_POINT_2F {
                            x: face_cx + side * 16.0,
                            y: cy + 7.0,
                        },
                    ],
                    D2D1_COLOR_F {
                        r: 0.19,
                        g: 0.13,
                        b: 0.11,
                        a: alpha * titan,
                    },
                );
            }
        }
        // Aksen helai depan
        fill_polygon(
            rt,
            factory,
            &[
                D2D_POINT_2F {
                    x: face_cx - 12.0,
                    y: cy - 9.5,
                },
                D2D_POINT_2F {
                    x: face_cx - 3.0,
                    y: cy - 12.0,
                },
                D2D_POINT_2F {
                    x: face_cx - 6.0,
                    y: cy - 6.5,
                },
            ],
            hair_hi,
        );

        // 5. Bayangan soket mata gelap (titan) + titan marks emas tipis
        if titan > 0.15 {
            let ta = smoothstep(0.15, 0.85, titan) * alpha;
            if let Ok(socket) = rt.CreateSolidColorBrush(
                &D2D1_COLOR_F {
                    r: 0.04,
                    g: 0.03,
                    b: 0.03,
                    a: ta * 0.85,
                },
                None,
            ) {
                for side in [-1.0_f32, 1.0_f32] {
                    rt.FillEllipse(
                        &D2D1_ELLIPSE {
                            point: D2D_POINT_2F {
                                x: face_cx + side * 9.0,
                                y: eye_y,
                            },
                            radiusX: 6.2,
                            radiusY: 7.0,
                        },
                        &socket,
                    );
                }
            }
            if let Ok(mark) = rt.CreateSolidColorBrush(
                &D2D1_COLOR_F {
                    r: 1.0,
                    g: 0.84,
                    b: 0.42,
                    a: ta * 0.75,
                },
                None,
            ) {
                for side in [-1.0_f32, 1.0_f32] {
                    for k in 0..3 {
                        let bx = face_cx + side * (8.6 + k as f32 * 1.1);
                        let by = eye_y + 5.4 + k as f32 * 0.5;
                        rt.DrawLine(
                            D2D_POINT_2F { x: bx, y: by },
                            D2D_POINT_2F {
                                x: bx + side * 0.9,
                                y: by + 2.6,
                            },
                            &mark,
                            0.9,
                            None,
                        );
                    }
                }
            }
        }

        // 6. Mata kapsul: hijau-teal tenang -> emerald intens
        let outer = lerp_color(
            D2D1_COLOR_F {
                r: 0.18,
                g: 0.77,
                b: 0.57,
                a: 1.0,
            },
            D2D1_COLOR_F {
                r: 0.12,
                g: 1.0,
                b: 0.55,
                a: 1.0,
            },
            titan,
        );
        let core = lerp_color(
            D2D1_COLOR_F {
                r: 0.77,
                g: 1.0,
                b: 0.91,
                a: 1.0,
            },
            D2D1_COLOR_F {
                r: 0.90,
                g: 1.0,
                b: 0.95,
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
            outer,
            core,
            (lerp(0.18, 0.12, titan), 1.0, lerp(0.57, 0.55, titan)),
            titan * 0.75 + flash * 0.2,
            alpha,
            ctx,
        );

        // 7. Gestur/efek kanan: petir emas mereda jadi uap
        let gx = cx + 38.0;
        let gy = cy + 2.0 + breath * 0.4;
        if flash > 0.1 {
            draw_soft_radial_glow(
                rt,
                D2D_POINT_2F { x: gx, y: gy },
                12.0,
                12.0,
                1.0,
                0.86,
                0.30,
                0.55 * flash * alpha,
            );
        }
        if titan > 0.3 {
            let t = ctx.phase_time;
            for k in 0..3 {
                let fk = k as f32;
                let life = (t * (0.5 + fk * 0.12) + fk * 0.31) % 1.0;
                draw_soft_circle(
                    rt,
                    D2D_POINT_2F {
                        x: gx + life * 3.5,
                        y: gy + 4.0 - life * 12.0,
                    },
                    3.6 + life * 5.0,
                    2.8 + life * 3.2,
                    D2D1_COLOR_F {
                        r: 0.92,
                        g: 0.96,
                        b: 0.94,
                        a: (1.0 - life) * life * 1.3 * 0.32 * titan * alpha,
                    },
                );
            }
        }
    }
}
