// src/mascot/eren.rs
// Fase 4: Eren Yeager (man-bun) -> Attack Titan.
// Manusia: rambut cokelat tersisir ke belakang, man-bun jelas di kanan-atas, tiga poni.
// Titan: tengkorak lebih lebar + rahang menjorok, kulit tanah liat gelap, surai penuh
// menutup sisi wajah, mulut bergerigi atas-bawah, garis titan di pipi, mata menyempit
// dengan celah vertikal, uap keluar dari bahu.
// Semua efek memakai kurva mulus berlapis + kabut lembut, bukan partikel kaku.
use super::core::*;
use windows::Win32::Graphics::Direct2D::Common::{D2D1_COLOR_F, D2D_POINT_2F, D2D_RECT_F};
use windows::Win32::Graphics::Direct2D::{
    ID2D1Factory, ID2D1HwndRenderTarget, D2D1_ELLIPSE, D2D1_ROUNDED_RECT,
};

const SCALE: f32 = 0.78;

pub struct ErenAttackTitan;

impl ErenAttackTitan {
    pub fn new() -> Self {
        Self
    }

    fn titan_factor(t: f32) -> f32 {
        smoothstep(2.6, 3.9, t)
    }

    fn lightning_flash(t: f32) -> f32 {
        if t < 2.6 || t > 4.6 {
            0.0
        } else if t < 3.2 {
            smoothstep(2.6, 3.2, t)
        } else {
            1.0 - smoothstep(3.2, 4.6, t)
        }
    }

    fn lightning_idle(t: f32, titan: f32) -> f32 {
        if titan <= 0.5 || t < 4.6 {
            return 0.0;
        }
        let cycle = (t - 4.6) % 4.2;
        if cycle < 0.7 {
            smoothstep(0.0, 0.7, cycle) * 0.45
        } else if cycle < 1.9 {
            (1.0 - smoothstep(0.7, 1.9, cycle)) * 0.45
        } else {
            0.0
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
        11.5
    }

    fn override_look_target(&self, ctx: &MascotContext) -> Option<(f32, f32)> {
        let flash = Self::lightning_flash(ctx.local_time);
        if flash > 0.1 {
            Some((1.6 * flash, -0.35 * flash))
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
        let idle_bolt = Self::lightning_idle(ctx.local_time, titan);
        let pw = pill_rect.right - pill_rect.left;
        let ph = pill_rect.bottom - pill_rect.top;
        let cx = (pill_rect.left + pill_rect.right) * 0.5;
        let t = ctx.phase_time;

        // ── Bloom dasar ────────────────────────────────────────────────────
        draw_soft_radial_glow(
            rt,
            D2D_POINT_2F {
                x: cx,
                y: pill_rect.bottom - 1.0,
            },
            pw * 0.47,
            ph * 0.74,
            lerp(0.16, 0.86, titan.max(flash)),
            lerp(0.75, 0.74, titan),
            lerp(0.56, 0.34, titan),
            (0.22 + 0.14 * titan + 0.22 * flash) * alpha,
        );
        draw_soft_radial_glow(
            rt,
            D2D_POINT_2F {
                x: cx + pw * 0.14,
                y: pill_rect.bottom - 8.0,
            },
            pw * 0.32,
            ph * 0.50,
            1.0,
            0.95,
            0.86,
            (0.05 + 0.18 * titan) * alpha,
        );

        // ── Kabut uap: blob lembut melayang (bukan titik) ──────────────────
        if titan > 0.1 {
            for k in 0..6 {
                let fk = k as f32;
                // tiga harmonisa -> gerak tidak berulang terasa
                let life = ((t * (0.16 + fk * 0.022))
                    + (t * 0.07 + fk).sin() * 0.06
                    + fk * 0.17)
                    % 1.0;
                let a = (life * (1.0 - life) * 3.6).min(1.0) * titan * alpha * 0.30;
                let bx = cx - pw * 0.24 + pw * 0.46 * ((fk * 0.37 + 0.08) % 1.0);
                let drift = (t * 0.55 + fk * 1.7).sin() * 4.0
                    + (t * 0.19 + fk).cos() * 2.4;
                let by = pill_rect.bottom - 4.0 - life * ph * 0.86;
                draw_soft_circle(
                    rt,
                    D2D_POINT_2F {
                        x: bx + drift,
                        y: by,
                    },
                    6.0 + life * 9.0,
                    4.5 + life * 7.0,
                    D2D1_COLOR_F {
                        r: 0.92,
                        g: 0.96,
                        b: 0.94,
                        a,
                    },
                );
            }
            // Anyaman streak tipis di atas kabut supaya ada arah gerak.
            for k in 0..3 {
                let fk = k as f32;
                let phase = (t * 0.20 + fk * 0.31) % 1.0;
                let a = (phase * (1.0 - phase) * 3.2).min(1.0) * titan * alpha * 0.34;
                let bx = cx - pw * 0.10 + pw * 0.26 * ((fk * 0.41) % 1.0);
                let bow = (t * 0.6 + fk * 1.9).sin() * 6.0;
                draw_tapered_streak(
                    rt,
                    factory,
                    D2D_POINT_2F {
                        x: bx,
                        y: pill_rect.bottom - 3.0,
                    },
                    D2D_POINT_2F {
                        x: bx + bow,
                        y: pill_rect.bottom - 3.0 - ph * (0.45 + 0.32 * phase),
                    },
                    bow * 0.8,
                    2.8,
                    D2D1_COLOR_F {
                        r: 0.90,
                        g: 0.94,
                        b: 0.92,
                        a,
                    },
                );
            }
        }

        // ── Petir: kurva mulus berlapis + cabang ───────────────────────────
        let bolt = (flash + idle_bolt).clamp(0.0, 1.0) * alpha;
        if bolt > 0.04 {
            let bx = cx + pw * 0.23;
            let top = pill_rect.top + 1.0;
            let bottom = pill_rect.bottom - 2.0;
            let span = bottom - top;
            let sway = (t * 1.7).sin() * 2.4 + (t * 0.7).cos() * 1.4;
            let main = [
                D2D_POINT_2F {
                    x: bx + 9.0,
                    y: top,
                },
                D2D_POINT_2F {
                    x: bx + 2.0,
                    y: top + span * 0.26 + sway,
                },
                D2D_POINT_2F {
                    x: bx + 7.0,
                    y: top + span * 0.52 - sway * 0.6,
                },
                D2D_POINT_2F {
                    x: bx - 3.0,
                    y: top + span * 0.78,
                },
                D2D_POINT_2F {
                    x: bx - 11.0,
                    y: bottom,
                },
            ];
            draw_soft_streak(
                rt,
                factory,
                &main,
                D2D1_COLOR_F {
                    r: 1.0,
                    g: 0.90,
                    b: 0.42,
                    a: bolt * 0.9,
                },
                1.5,
            );
            draw_soft_streak(
                rt,
                factory,
                &[
                    D2D_POINT_2F {
                        x: bx + 3.0,
                        y: top + span * 0.42,
                    },
                    D2D_POINT_2F {
                        x: bx + 12.0,
                        y: top + span * 0.52,
                    },
                    D2D_POINT_2F {
                        x: bx + 9.0,
                        y: top + span * 0.66,
                    },
                ],
                D2D1_COLOR_F {
                    r: 1.0,
                    g: 0.97,
                    b: 0.82,
                    a: bolt * 0.55,
                },
                0.9,
            );
        }

        // ── Mote ambient: alfa sinusoidal, gerak dua sumbu ─────────────────
        for i in 0..5 {
            let fi = i as f32;
            let phase = (t * 0.14 + fi * 0.19) % 1.0;
            let a = (phase * std::f32::consts::PI).sin() * 0.18 * alpha;
            let mx = cx + (t * 0.42 + fi * 2.1).sin() * pw * 0.28;
            let my = pill_rect.bottom - 9.0 - phase * ph * 0.74;
            draw_soft_circle(
                rt,
                D2D_POINT_2F { x: mx, y: my },
                1.6,
                1.6,
                D2D1_COLOR_F {
                    r: 1.0,
                    g: lerp(0.88, 0.74, titan),
                    b: lerp(0.56, 0.32, titan),
                    a,
                },
            );
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
        let titan = Self::titan_factor(ctx.local_time);
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

        // Kulit titan: hitam notch (manusia) -> tanah liat gelap hangat (titan).
        let skin = lerp_color(col(0.018, 0.018, 0.024), col(0.135, 0.098, 0.082), titan);
        let skin_shadow = lerp_color(col(0.012, 0.012, 0.016), col(0.075, 0.052, 0.045), titan);
        let hair = col(0.21, 0.14, 0.11);
        let hair_hi = col(0.36, 0.26, 0.21);
        let hair_deep = col(0.10, 0.065, 0.055);
        let cloak = col(0.16, 0.26, 0.19);
        let cloak_hi = col(0.26, 0.38, 0.28);
        let teal = col(0.20, 0.86, 0.62);
        let teal_core = col(0.80, 1.00, 0.92);
        let teeth = col(0.88, 0.85, 0.78);
        let tongue = col(0.26, 0.075, 0.075);
        let sway = (ctx.phase_time * 1.5).sin();
        let breath = (ctx.phase_time * 1.2).sin() * 0.5;

        // ── 0. Bahu / kerah: tipis (manusia) -> trapesius lebar (titan) ────
        let shoulder = lerp(26.0, 32.0, titan);
        fill_polygon(
            rt,
            factory,
            &local(&[
                (-shoulder, 14.0),
                (-shoulder * 0.80, 4.0),
                (-12.5, 7.6),
                (0.0, 9.6),
                (12.5, 7.6),
                (shoulder * 0.80, 4.0),
                (shoulder, 14.0),
                (0.0, 20.0),
            ]),
            lerp_color(cloak, skin_shadow, titan),
        );
        // Trap menonjol khas titan.
        if titan > 0.05 {
            for side in [-1.0_f32, 1.0_f32] {
                fill_polygon(
                    rt,
                    factory,
                    &local(&[
                        (side * 20.0, 5.0),
                        (side * 12.0, 8.0),
                        (side * 8.0, 12.0),
                        (side * 17.0, 13.0),
                    ]),
                    D2D1_COLOR_F {
                        a: alpha * titan,
                        ..skin
                    },
                );
            }
        }
        draw_polyline(
            rt,
            factory,
            &local(&[
                (-shoulder * 0.80, 4.2),
                (-12.5, 7.8),
                (0.0, 9.8),
                (12.5, 7.8),
                (shoulder * 0.80, 4.2),
            ]),
            lerp_color(cloak_hi, skin_hi_fallback(hair_hi), titan),
            0.9,
        );

        // ── 1. Basis kepala: bulat (manusia) -> tengkorak titan ────────────
        let head_half = 20.5 + 2.2 * titan;
        let jaw_drop = 15.0 + 3.6 * titan;
        if let Ok(b) = rt.CreateSolidColorBrush(&skin, None) {
            rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: D2D_RECT_F {
                        left: base_cx - head_half * s,
                        top: base_cy - (8.5 + 1.2 * titan) * s,
                        right: base_cx + head_half * s,
                        bottom: base_cy + jaw_drop * s,
                    },
                    radiusX: (10.0 - 3.0 * titan) * s,
                    radiusY: (10.0 - 2.0 * titan) * s,
                },
                &b,
            );
        }
        // Rahang menjorok (titan): lebar bawah + dagu.
        if titan > 0.05 {
            fill_polygon(
                rt,
                factory,
                &local(&[
                    (-head_half * 0.94, 6.0),
                    (-14.0, 12.4),
                    (-7.0, 16.4),
                    (0.0, 17.6),
                    (7.0, 16.4),
                    (14.0, 12.4),
                    (head_half * 0.94, 6.0),
                    (head_half * 0.86, 13.6),
                    (0.0, 19.4),
                    (-head_half * 0.86, 13.6),
                ]),
                D2D1_COLOR_F {
                    a: alpha * titan,
                    ..skin
                },
            );
        }
        // Telinga.
        for side in [-1.0_f32, 1.0_f32] {
            let ear = at(side * (head_half * 0.99), 3.0 + titan * 1.2);
            if let Ok(b) = rt.CreateSolidColorBrush(
                &D2D1_COLOR_F {
                    a: alpha * (0.85 + 0.15 * titan),
                    ..skin
                },
                None,
            ) {
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: ear,
                        radiusX: (2.6 + 0.9 * titan) * s,
                        radiusY: (3.6 + 1.0 * titan) * s,
                    },
                    &b,
                );
            }
        }
        // Rim light.
        draw_soft_radial_glow(rt, at(-19.0, 1.5), 9.0, 12.0, teal.r, teal.g, teal.b, 0.15 * alpha);
        draw_soft_radial_glow(rt, at(19.0, 1.5), 9.0, 12.0, teal.r, teal.g, teal.b, 0.15 * alpha);

        // ── 2. Rambut: tersisir (manusia) -> surai titan menutup sisi ──────
        let spread = titan * 2.4;
        let hair_pts = [
            (-21.5 - spread, -1.8),
            (-24.5 - spread, -7.4),
            (-19.0 - spread * 0.6, -8.6),
            (-22.0 - spread * 0.7, -14.6),
            (-14.0, -13.2),
            (-12.5, -19.4),
            (-4.5, -16.2),
            (0.0, -20.6),
            (5.5, -16.0),
            (13.5, -18.6),
            (14.5, -12.6),
            (21.5 + spread * 0.7, -14.0),
            (19.0 + spread * 0.6, -8.0),
            (24.5 + spread, -6.6),
            (21.0 + spread, -1.2),
            (16.0, 1.4),
            (8.0, -2.2),
            (0.0, 0.6),
            (-8.0, -2.2),
            (-16.5, 1.4),
        ];
        fill_polygon(rt, factory, &local(&hair_pts), hair);
        // Surai titan: helai turun melewati rahang di kiri & kanan.
        if titan > 0.12 {
            for side in [-1.0_f32, 1.0_f32] {
                for k in 0..3 {
                    let fk = k as f32;
                    let x0 = side * (15.0 + fk * 3.2);
                    let drop = 6.0 + fk * 3.4 + titan * 4.0;
                    fill_polygon(
                        rt,
                        factory,
                        &local(&[
                            (x0, -7.0 - fk * 1.4),
                            (x0 + side * (5.6 + fk * 1.2), 0.5 + drop * 0.4 + sway * 0.5),
                            (x0 + side * (1.8 + fk), drop + sway * 0.8),
                            (x0 - side * 2.6, 1.0),
                        ]),
                        D2D1_COLOR_F {
                            a: alpha * titan,
                            ..if k % 2 == 0 { hair } else { hair_deep }
                        },
                    );
                }
            }
        }
        for chunk in [
            [(-20.0, -11.8), (-11.5, -9.8), (-16.5, -5.4)],
            [(-9.5, -15.8), (-2.0, -11.8), (-7.0, -6.8)],
            [(1.0, -17.4), (5.0, -11.6), (0.0, -7.2)],
            [(10.0, -16.0), (8.0, -7.8), (2.5, -4.6)],
            [(19.0, -12.6), (16.0, -5.8), (10.0, -4.2)],
        ] {
            fill_polygon(rt, factory, &local(&chunk), hair_hi);
        }
        draw_polyline(
            rt,
            factory,
            &local(&[
                (-21.5 - spread, -1.8),
                (-24.5 - spread, -7.4),
                (-19.0 - spread * 0.6, -8.6),
                (-22.0 - spread * 0.7, -14.6),
                (-14.0, -13.2),
                (-12.5, -19.4),
                (-4.5, -16.2),
                (0.0, -20.6),
                (5.5, -16.0),
                (13.5, -18.6),
                (14.5, -12.6),
                (21.5 + spread * 0.7, -14.0),
                (19.0 + spread * 0.6, -8.0),
                (24.5 + spread, -6.6),
                (21.0 + spread, -1.2),
            ]),
            D2D1_COLOR_F {
                a: 0.5 * alpha,
                ..hair_hi
            },
            0.9,
        );

        // ── 3. Man-bun + poni (manusia) ────────────────────────────────────
        let human = (1.0 - titan).max(0.0);
        if human > 0.03 {
            let bun = at(10.5, -17.5);
            let br = 5.0 * s * (0.72 + 0.28 * human);
            if let Ok(b) = rt.CreateSolidColorBrush(
                &D2D1_COLOR_F {
                    a: alpha * human,
                    ..hair_deep
                },
                None,
            ) {
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: bun,
                        radiusX: br,
                        radiusY: br * 0.88,
                    },
                    &b,
                );
            }
            if let Ok(b) = rt.CreateSolidColorBrush(
                &D2D1_COLOR_F {
                    a: 0.75 * alpha * human,
                    ..hair_hi
                },
                None,
            ) {
                rt.DrawEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F {
                            x: bun.x - br * 0.18,
                            y: bun.y - br * 0.22,
                        },
                        radiusX: br * 0.60,
                        radiusY: br * 0.52,
                    },
                    &b,
                    0.9,
                    None,
                );
            }
            draw_soft_streak(
                rt,
                factory,
                &[
                    D2D_POINT_2F {
                        x: bun.x - br * 0.9,
                        y: bun.y + br * 0.55,
                    },
                    D2D_POINT_2F {
                        x: bun.x + br * 0.1,
                        y: bun.y + br * 1.05,
                    },
                    D2D_POINT_2F {
                        x: bun.x + br * 0.95,
                        y: bun.y + br * 0.5,
                    },
                ],
                D2D1_COLOR_F {
                    a: 0.85 * alpha * human,
                    ..hair_hi
                },
                1.0,
            );
            for strand in [
                [(-10.0, -12.4), (-7.0, -8.0), (-9.4, -3.6)],
                [(-2.0, -14.2), (0.4, -9.0), (-0.6, -4.2)],
                [(6.0, -12.8), (5.2, -8.4), (7.4, -4.6)],
            ] {
                fill_polygon(
                    rt,
                    factory,
                    &local(&strand),
                    D2D1_COLOR_F {
                        a: alpha * human,
                        ..hair_deep
                    },
                );
            }
        }

        // ── 4. Wajah titan: alis, pipi, mulut bergerigi, garis titan ───────
        let eye_y = base_cy + (5.0 + 0.8 * titan) * s;
        if titan > 0.10 {
            let ta = smoothstep(0.10, 0.8, titan) * alpha;

            // Alis tebal menjorok.
            for side in [-1.0_f32, 1.0_f32] {
                fill_polygon(
                    rt,
                    factory,
                    &local(&[
                        (side * 4.5, 0.8 + 0.6 * titan),
                        (side * 14.5, 1.6 + titan),
                        (side * 13.6, 3.4 + titan),
                        (side * 4.6, 2.6),
                    ]),
                    D2D1_COLOR_F {
                        a: ta * 0.75,
                        ..hair_deep
                    },
                );
            }

            // Soket mata dalam.
            if let Ok(b) = rt.CreateSolidColorBrush(
                &D2D1_COLOR_F {
                    r: 0.030,
                    g: 0.024,
                    b: 0.024,
                    a: ta * 0.85,
                },
                None,
            ) {
                for side in [-1.0_f32, 1.0_f32] {
                    rt.FillEllipse(
                        &D2D1_ELLIPSE {
                            point: D2D_POINT_2F {
                                x: base_cx + side * 9.0 * s + ctx.look_x * 0.6,
                                y: eye_y,
                            },
                            radiusX: 6.6 * s,
                            radiusY: 7.6 * s,
                        },
                        &b,
                    );
                }
            }

            // Garis titan: streak gelap di pipi + garis otot rahang.
            for side in [-1.0_f32, 1.0_f32] {
                for k in 0..2 {
                    let x0 = base_cx + side * (6.4 + k as f32 * 3.6) * s;
                    draw_soft_streak(
                        rt,
                        factory,
                        &[
                            D2D_POINT_2F {
                                x: x0,
                                y: eye_y + 3.2 * s,
                            },
                            D2D_POINT_2F {
                                x: x0 + side * 1.0,
                                y: eye_y + 7.6 * s,
                            },
                            D2D_POINT_2F {
                                x: x0 + side * 2.2,
                                y: eye_y + 12.4 * s,
                            },
                        ],
                        D2D1_COLOR_F {
                            r: 0.09,
                            g: 0.06,
                            b: 0.055,
                            a: ta * 0.80,
                        },
                        1.2,
                    );
                }
                // Garis urat rahang.
                draw_soft_streak(
                    rt,
                    factory,
                    &[
                        D2D_POINT_2F {
                            x: base_cx + side * 17.0 * s,
                            y: (base_cy + 8.0 * s),
                        },
                        D2D_POINT_2F {
                            x: base_cx + side * 13.0 * s,
                            y: (base_cy + 12.0 * s),
                        },
                        D2D_POINT_2F {
                            x: base_cx + side * 8.0 * s,
                            y: (base_cy + 14.5 * s),
                        },
                    ],
                    D2D1_COLOR_F {
                        r: 0.10,
                        g: 0.07,
                        b: 0.06,
                        a: ta * 0.5,
                    },
                    0.9,
                );
            }

            // Mulut: rongga + gigi atas & bawah + lidah.
            let my = base_cy + (12.6 + 0.8 * titan) * s + breath * 0.4;
            let mouth_w = 10.2 + 1.4 * titan;
            fill_polygon(
                rt,
                factory,
                &[
                    D2D_POINT_2F {
                        x: base_cx - mouth_w * s,
                        y: my - 1.6 * s,
                    },
                    D2D_POINT_2F {
                        x: base_cx - 5.0 * s,
                        y: my - 0.2 * s,
                    },
                    D2D_POINT_2F {
                        x: base_cx,
                        y: my + 0.4 * s,
                    },
                    D2D_POINT_2F {
                        x: base_cx + 5.0 * s,
                        y: my - 0.2 * s,
                    },
                    D2D_POINT_2F {
                        x: base_cx + mouth_w * s,
                        y: my - 1.6 * s,
                    },
                    D2D_POINT_2F {
                        x: base_cx + (mouth_w - 0.6) * s,
                        y: my + 5.2 * s,
                    },
                    D2D_POINT_2F {
                        x: base_cx,
                        y: my + 7.2 * s,
                    },
                    D2D_POINT_2F {
                        x: base_cx - (mouth_w - 0.6) * s,
                        y: my + 5.2 * s,
                    },
                ],
                D2D1_COLOR_F {
                    r: 0.085,
                    g: 0.055,
                    b: 0.055,
                    a: ta,
                },
            );
            if let Ok(b) = rt.CreateSolidColorBrush(
                &D2D1_COLOR_F {
                    a: ta * 0.75,
                    ..tongue
                },
                None,
            ) {
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F {
                            x: base_cx,
                            y: my + 4.6 * s,
                        },
                        radiusX: 5.4 * s,
                        radiusY: 2.0 * s,
                    },
                    &b,
                );
            }
            // Gigi atas (turun) & bawah (naik).
            if let Ok(b) = rt.CreateSolidColorBrush(
                &D2D1_COLOR_F {
                    a: ta * 0.95,
                    ..teeth
                },
                None,
            ) {
                for k in 0..7 {
                    let fk = k as f32;
                    let tx = base_cx + (-7.4 + fk * 2.46) * s;
                    let curve = (fk - 3.0).abs() * 0.24;
                    rt.FillRoundedRectangle(
                        &D2D1_ROUNDED_RECT {
                            rect: D2D_RECT_F {
                                left: tx - 0.95 * s,
                                top: my + (0.1 + curve) * s,
                                right: tx + 0.95 * s,
                                bottom: my + (2.0 + curve + (fk % 2.0) * 0.5) * s,
                            },
                            radiusX: 0.5 * s,
                            radiusY: 0.6 * s,
                        },
                        &b,
                    );
                    rt.FillRoundedRectangle(
                        &D2D1_ROUNDED_RECT {
                            rect: D2D_RECT_F {
                                left: tx - 0.85 * s,
                                top: my + (5.6 + curve) * s,
                                right: tx + 0.85 * s,
                                bottom: my + (7.0 + curve - (fk % 2.0) * 0.4) * s,
                            },
                            radiusX: 0.5 * s,
                            radiusY: 0.5 * s,
                        },
                        &b,
                    );
                }
            }
        }

        // ── 5. Mata teal-green (manusia) -> emerald tajam (titan) ──────────
        let (outer, core_c, glow) = if titan < 0.5 {
            (teal, teal_core, (0.20, 0.86, 0.62))
        } else {
            (
                col(0.16, 1.00, 0.62),
                col(0.88, 1.00, 0.96),
                (0.16, 1.0, 0.62),
            )
        };
        draw_capsule_eyes(
            rt,
            factory,
            base_cx,
            eye_y,
            9.0,
            outer,
            core_c,
            glow,
            0.28 + 0.52 * titan,
            alpha,
            ctx,
        );
        // Celah vertikal menembus mata (khas titan).
        if titan > 0.35 {
            let a = alpha * smoothstep(0.35, 0.9, titan) * ctx.open_factor();
            if let Ok(b) = rt.CreateSolidColorBrush(
                &D2D1_COLOR_F {
                    r: 0.03,
                    g: 0.03,
                    b: 0.03,
                    a,
                },
                None,
            ) {
                for side in [-1.0_f32, 1.0_f32] {
                    let ex = base_cx + side * 9.0 + ctx.look_x * 0.65;
                    rt.FillRoundedRectangle(
                        &D2D1_ROUNDED_RECT {
                            rect: D2D_RECT_F {
                                left: ex - 0.75,
                                top: eye_y - 3.6,
                                right: ex + 0.75,
                                bottom: eye_y + 3.6,
                            },
                            radiusX: 0.75,
                            radiusY: 1.1,
                        },
                        &b,
                    );
                }
            }
        }
    }
}

/// Highlight netral untuk transisi kerah -> kulit.
fn skin_hi_fallback(c: D2D1_COLOR_F) -> D2D1_COLOR_F {
    D2D1_COLOR_F {
        r: c.r * 0.8,
        g: c.g * 0.7,
        b: c.b * 0.7,
        a: c.a,
    }
}
