// src/mascot/bakugo.rs
// Fase 3: Katsuki Bakugo.
// Gaya dikunci ke Asta Devil Union: siluet rambut pirang, topeng mata hitam bergerigi,
// sirip flare hitam ber-outline oranye, mata kapsul merah-oranye, gauntlet granat di
// kanan dengan recoil + ledakan stylized. Tanpa wajah/mulut/hidung.
use super::core::*;
use windows::Win32::Graphics::Direct2D::Common::{D2D1_COLOR_F, D2D_POINT_2F, D2D_RECT_F};
use windows::Win32::Graphics::Direct2D::{ID2D1Factory, ID2D1HwndRenderTarget, D2D1_ELLIPSE};

pub struct KatsukiBakugo;

impl KatsukiBakugo {
    pub fn new() -> Self {
        Self
    }

    /// Pulsa ledakan tiap ~2.6 detik
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
        rt: &ID2D1HwndRenderTarget,
        factory: &ID2D1Factory,
        pill_rect: D2D_RECT_F,
        alpha: f32,
        ctx: &MascotContext,
    ) {
        let blast = Self::blast_pulse(ctx.local_time);
        let pw = pill_rect.right - pill_rect.left;
        let ph = pill_rect.bottom - pill_rect.top;
        let cx = (pill_rect.left + pill_rect.right) * 0.5;

        // Fiery orange-amber bottom glow
        draw_soft_radial_glow(
            rt,
            D2D_POINT_2F {
                x: cx,
                y: pill_rect.bottom - 1.0,
            },
            pw * 0.46,
            ph * 0.70,
            1.0,
            0.42,
            0.08,
            (0.30 + 0.24 * blast) * alpha,
        );

        // Ember kecil melayang naik
        for i in 0..6 {
            let fi = i as f32;
            let life = (ctx.phase_time * (0.9 + fi * 0.13) + fi * 0.37) % 1.0;
            let ex = pill_rect.left
                + pw * (0.16 + 0.68 * ((fi * 0.29 + 0.11) % 1.0))
                + (ctx.phase_time * 3.0 + fi).sin() * 3.0;
            let ey = pill_rect.bottom - 3.0 - life * (ph * 0.80);
            draw_soft_circle(
                rt,
                D2D_POINT_2F { x: ex, y: ey },
                1.3,
                1.3,
                D2D1_COLOR_F {
                    r: 1.0,
                    g: 0.70,
                    b: 0.22,
                    a: (1.0 - life) * alpha * (0.5 + 0.45 * blast),
                },
            );
        }

        // Gelombang kejut tipis saat meledak
        if blast > 0.08 {
            let stroke = create_round_stroke(factory);
            if let (Ok(rb), Some(st)) = (
                rt.CreateSolidColorBrush(
                    &D2D1_COLOR_F {
                        r: 1.0,
                        g: 0.76,
                        b: 0.32,
                        a: blast * alpha * 0.45,
                    },
                    None,
                ),
                stroke.as_ref(),
            ) {
                let bx = cx + 42.0;
                let grow = 1.0 + (1.0 - blast) * 2.2;
                rt.DrawEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F {
                            x: bx,
                            y: pill_rect.top + ph * 0.42,
                        },
                        radiusX: 7.0 * grow,
                        radiusY: 5.5 * grow,
                    },
                    &rb,
                    0.9,
                    st,
                );
            }
        }
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
        let recoil = blast * 1.5;
        let face_cx = cx - recoil * 0.4;
        let eye_y = cy + 2.0;
        let spike_kick = blast * 1.1;

        // 1. Rambut pirang pucat spiky
        let blonde = D2D1_COLOR_F {
            r: 0.92,
            g: 0.87,
            b: 0.72,
            a: alpha,
        };
        let blonde_dim = D2D1_COLOR_F {
            r: 0.72,
            g: 0.66,
            b: 0.50,
            a: alpha,
        };
        fill_polygon(
            rt,
            factory,
            &[
                D2D_POINT_2F {
                    x: face_cx - 18.0 - spike_kick,
                    y: cy - 3.0,
                },
                D2D_POINT_2F {
                    x: face_cx - 17.0,
                    y: cy - 11.5 - spike_kick,
                },
                D2D_POINT_2F {
                    x: face_cx - 10.5,
                    y: cy - 8.8,
                },
                D2D_POINT_2F {
                    x: face_cx - 7.0,
                    y: cy - 14.0 - spike_kick,
                },
                D2D_POINT_2F {
                    x: face_cx - 1.5,
                    y: cy - 9.5,
                },
                D2D_POINT_2F {
                    x: face_cx + 3.5,
                    y: cy - 14.0 - spike_kick,
                },
                D2D_POINT_2F {
                    x: face_cx + 9.0,
                    y: cy - 9.0,
                },
                D2D_POINT_2F {
                    x: face_cx + 16.0,
                    y: cy - 12.0 - spike_kick,
                },
                D2D_POINT_2F {
                    x: face_cx + 18.0,
                    y: cy - 3.0,
                },
            ],
            blonde,
        );
        fill_polygon(
            rt,
            factory,
            &[
                D2D_POINT_2F {
                    x: face_cx - 14.0,
                    y: cy - 3.6,
                },
                D2D_POINT_2F {
                    x: face_cx - 5.0,
                    y: cy - 7.6,
                },
                D2D_POINT_2F {
                    x: face_cx + 6.0,
                    y: cy - 7.4,
                },
                D2D_POINT_2F {
                    x: face_cx + 14.0,
                    y: cy - 3.6,
                },
            ],
            blonde_dim,
        );

        // 2. Sirip flare hitam bergerigi ber-outline oranye (kiri & kanan)
        let flare_dark = D2D1_COLOR_F {
            r: 0.06,
            g: 0.06,
            b: 0.07,
            a: alpha,
        };
        let orange = D2D1_COLOR_F {
            r: 1.0,
            g: 0.42,
            b: 0.10,
            a: alpha,
        };
        for side in [-1.0_f32, 1.0_f32] {
            let pts = [
                D2D_POINT_2F {
                    x: face_cx + side * 12.5,
                    y: cy - 1.5,
                },
                D2D_POINT_2F {
                    x: face_cx + side * 20.5,
                    y: cy - 6.0,
                },
                D2D_POINT_2F {
                    x: face_cx + side * 16.0,
                    y: cy - 0.5,
                },
                D2D_POINT_2F {
                    x: face_cx + side * 21.0,
                    y: cy + 3.6,
                },
                D2D_POINT_2F {
                    x: face_cx + side * 13.0,
                    y: cy + 4.2,
                },
            ];
            fill_polygon(rt, factory, &pts, flare_dark);
            let mut outline = pts.to_vec();
            outline.push(pts[0]);
            draw_polyline(rt, factory, &outline, orange, 1.15);
        }

        // 3. Topeng mata hitam bergerigi
        let mask = D2D1_COLOR_F {
            r: 0.03,
            g: 0.03,
            b: 0.04,
            a: alpha,
        };
        fill_polygon(
            rt,
            factory,
            &[
                D2D_POINT_2F {
                    x: face_cx - 14.5,
                    y: cy - 2.2,
                },
                D2D_POINT_2F {
                    x: face_cx - 6.0,
                    y: cy - 3.4,
                },
                D2D_POINT_2F {
                    x: face_cx,
                    y: cy - 1.4,
                },
                D2D_POINT_2F {
                    x: face_cx + 6.0,
                    y: cy - 3.4,
                },
                D2D_POINT_2F {
                    x: face_cx + 14.5,
                    y: cy - 2.2,
                },
                D2D_POINT_2F {
                    x: face_cx + 13.6,
                    y: cy + 4.4,
                },
                D2D_POINT_2F {
                    x: face_cx + 5.0,
                    y: cy + 5.4,
                },
                D2D_POINT_2F {
                    x: face_cx,
                    y: cy + 4.0,
                },
                D2D_POINT_2F {
                    x: face_cx - 5.0,
                    y: cy + 5.4,
                },
                D2D_POINT_2F {
                    x: face_cx - 13.6,
                    y: cy + 4.4,
                },
            ],
            mask,
        );
        // Gerigi tepi bawah topeng (outline oranye tipis)
        draw_polyline(
            rt,
            factory,
            &[
                D2D_POINT_2F {
                    x: face_cx - 13.6,
                    y: cy + 4.4,
                },
                D2D_POINT_2F {
                    x: face_cx - 5.0,
                    y: cy + 5.4,
                },
                D2D_POINT_2F {
                    x: face_cx,
                    y: cy + 4.0,
                },
                D2D_POINT_2F {
                    x: face_cx + 5.0,
                    y: cy + 5.4,
                },
                D2D_POINT_2F {
                    x: face_cx + 13.6,
                    y: cy + 4.4,
                },
            ],
            D2D1_COLOR_F {
                r: 0.85,
                g: 0.30,
                b: 0.06,
                a: alpha,
            },
            0.8,
        );

        // 4. Mata kapsul merah-oranye garang
        draw_capsule_eyes(
            rt,
            factory,
            face_cx,
            eye_y,
            9.0,
            D2D1_COLOR_F {
                r: 1.0,
                g: 0.23,
                b: 0.08,
                a: 1.0,
            },
            D2D1_COLOR_F {
                r: 1.0,
                g: 0.84,
                b: 0.50,
                a: 1.0,
            },
            (1.0, 0.36, 0.10),
            0.82 + 0.18 * blast,
            alpha,
            ctx,
        );

        // 5. Gestur kanan: gauntlet granat hijau-gelap & silver
        let gx = cx + 37.0 - recoil;
        let gy = cy + 2.4;
        let burst = 0.40 + 0.85 * blast;
        let bx = gx + 9.0;
        let by = gy - 3.0;

        // Ledakan oranye-emas
        draw_soft_radial_glow(
            rt,
            D2D_POINT_2F { x: bx, y: by },
            15.0 * burst,
            12.0 * burst,
            1.0,
            0.54,
            0.12,
            (0.30 + 0.55 * blast) * alpha,
        );
        let burst_b_col = D2D1_COLOR_F {
            r: 1.0,
            g: 0.72,
            b: 0.24,
            a: (0.42 + 0.50 * blast) * alpha,
        };
        {
            let mut star = [D2D_POINT_2F { x: 0.0, y: 0.0 }; 16];
            for (i, pt) in star.iter_mut().enumerate() {
                let ang = (i as f32) * std::f32::consts::PI / 8.0 + ctx.phase_time * 0.5;
                let rad = if i % 2 == 0 {
                    9.5 * burst
                } else {
                    3.8 * burst
                };
                *pt = D2D_POINT_2F {
                    x: bx + ang.cos() * rad,
                    y: by + ang.sin() * rad,
                };
            }
            fill_polygon(rt, factory, &star, burst_b_col);
        }

        // Gauntlet: badan hijau gelap + plat silver + cincin pin
        let gauntlet = D2D1_COLOR_F {
            r: 0.18,
            g: 0.35,
            b: 0.24,
            a: alpha,
        };
        let silver = D2D1_COLOR_F {
            r: 0.78,
            g: 0.81,
            b: 0.84,
            a: alpha,
        };
        fill_polygon(
            rt,
            factory,
            &[
                D2D_POINT_2F {
                    x: gx - 6.0,
                    y: gy + 7.0,
                },
                D2D_POINT_2F {
                    x: gx - 1.5,
                    y: gy + 0.8,
                },
                D2D_POINT_2F {
                    x: gx + 3.6,
                    y: gy - 3.6,
                },
                D2D_POINT_2F {
                    x: gx + 6.0,
                    y: gy + 0.6,
                },
                D2D_POINT_2F {
                    x: gx + 2.0,
                    y: gy + 4.6,
                },
                D2D_POINT_2F {
                    x: gx - 1.8,
                    y: gy + 9.0,
                },
            ],
            gauntlet,
        );
        draw_polyline(
            rt,
            factory,
            &[
                D2D_POINT_2F {
                    x: gx - 2.0,
                    y: gy + 1.8,
                },
                D2D_POINT_2F {
                    x: gx + 3.0,
                    y: gy - 2.0,
                },
            ],
            silver,
            1.1,
        );
        if let Ok(pb) = rt.CreateSolidColorBrush(&silver, None) {
            rt.DrawEllipse(
                &D2D1_ELLIPSE {
                    point: D2D_POINT_2F {
                        x: gx - 3.8,
                        y: gy + 6.0,
                    },
                    radiusX: 1.6,
                    radiusY: 1.6,
                },
                &pb,
                0.8,
                None,
            );
        }
    }
}
