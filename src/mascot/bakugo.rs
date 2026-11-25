// src/mascot/bakugo.rs
// Fase 3: Katsuki Bakugo.
// Bahasa visual sama dengan Asta Devil Union: basis kepala gelap, rambut pirang berlapis,
// topeng mata hitam bergerigi, sirip flare hitam ber-outline oranye, kerah kostum +
// pelat dada, gauntlet granat dengan ledakan di kanan.
use super::atlas;
use super::core::*;
use super::vfx;
use windows::Win32::Graphics::Direct2D::Common::{D2D1_COLOR_F, D2D_POINT_2F, D2D_RECT_F};
use windows::Win32::Graphics::Direct2D::{
    ID2D1Factory, ID2D1RenderTarget, D2D1_ELLIPSE, D2D1_ROUNDED_RECT,
};

const SCALE: f32 = 0.78;

pub struct KatsukiBakugo;

impl KatsukiBakugo {
    pub fn new() -> Self {
        Self
    }

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
            (0.30, 0.16, 0.03),
            (0.20, 0.10, 0.02),
            (1.00, 0.60, 0.14),
            0.82,
            alpha,
        );
        draw_soft_radial_glow(
            rt,
            D2D_POINT_2F { x: cx, y: cy },
            pw * 0.46 * breathe,
            ph * 1.00 * breathe,
            0.95,
            0.55,
            0.12,
            (0.32 * alpha).min(0.34),
        );
        draw_soft_radial_glow(
            rt,
            D2D_POINT_2F { x: cx, y: pill_rect.bottom + 2.0 },
            pw * 0.58,
            ph * 0.88,
            0.952,
            0.552,
            0.122,
            (0.322 * alpha).min(0.30),
        );
        // Partikel lembut, ukuran kecil, melayang pelan.
        vfx::draw_bokeh_motes(rt, pill_rect, 10, t, 7.9, (1.0, 0.72, 0.28), alpha * 0.95);
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
        let blast = Self::blast_pulse(ctx.local_time);
        let s = SCALE;
        let base_cx = cx - blast * 0.6;
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
        let hair = col(0.92, 0.87, 0.72);
        let hair_hi = col(1.00, 0.96, 0.82);
        let hair_deep = col(0.66, 0.60, 0.44);
        let orange = col(1.00, 0.42, 0.10);
        let orange_hot = col(1.00, 0.72, 0.24);
        let costume = col(0.075, 0.080, 0.095);
        let costume_hi = col(0.18, 0.19, 0.22);
        let silver = col(0.78, 0.81, 0.84);
        let green = col(0.18, 0.35, 0.24);

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
                r: 0.088,
                g: 0.084,
                b: 0.094,
                a: alpha * 0.92,
            },
        );

        // ── 0. Kerah kostum + pelat dada + granat leher ───────────────────
        fill_polygon(
            rt,
            factory,
            &local(&[
                (-27.0, 13.5),
                (-22.0, 4.5),
                (-13.5, 7.0),
                (0.0, 9.0),
                (13.5, 7.0),
                (22.0, 4.5),
                (27.0, 13.5),
                (0.0, 19.0),
            ]),
            costume,
        );
        // Pelat dada bersudut (kostum musim panas).
        fill_polygon(
            rt,
            factory,
            &local(&[
                (-12.0, 10.0),
                (-6.0, 9.2),
                (0.0, 11.0),
                (6.0, 9.2),
                (12.0, 10.0),
                (9.0, 15.5),
                (0.0, 17.5),
                (-9.0, 15.5),
            ]),
            costume_hi,
        );
        // Garis X oranye di tengah dada.
        draw_polyline(
            rt,
            factory,
            &local(&[(-7.5, 10.6), (7.5, 15.4)]),
            D2D1_COLOR_F {
                a: 0.85 * alpha,
                ..orange
            },
            1.6,
        );
        draw_polyline(
            rt,
            factory,
            &local(&[(7.5, 10.6), (-7.5, 15.4)]),
            D2D1_COLOR_F {
                a: 0.85 * alpha,
                ..orange
            },
            1.6,
        );
        // Granat pengapian di sisi leher.
        for side in [-1.0_f32, 1.0_f32] {
            let gx = at(side * 15.0, 6.4);
            if let Ok(b) = rt.CreateSolidColorBrush(&silver, None) {
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: gx,
                        radiusX: 2.3 * s,
                        radiusY: 2.3 * s,
                    },
                    &b,
                );
            }
            if let Ok(b) = rt.CreateSolidColorBrush(&orange, None) {
                rt.DrawEllipse(
                    &D2D1_ELLIPSE {
                        point: gx,
                        radiusX: 2.3 * s,
                        radiusY: 2.3 * s,
                    },
                    &b,
                    0.9,
                    None,
                );
            }
        }

        // ── 2. Kepala HD (hair + mask) ──────────────────────────────────
        // ── 4. Mata merah-oranye garang ────────────────────────────────────
        let eye_y = base_cy + 5.0 * s;
        draw_capsule_eyes(
            rt,
            factory,
            base_cx,
            eye_y,
            9.0,
            col(1.00, 0.23, 0.08),
            col(1.00, 0.84, 0.50),
            (1.0, 0.30, 0.10),
            0.85 + 0.15 * blast,
            alpha,
            ctx,
        );

        // Kepala HD digambar di atas mata: lubang topeng membiarkan mata menyala.
        atlas::draw_head(rt, atlas::Head::Bakugo, base_cx, base_cy - 9.0 * s, 96.0 * s, alpha);

        // ── 5. Gauntlet granat + ledakan (kanan) ───────────────────────────
        let recoil = blast * 2.2;
        let gx = base_cx + 34.0 * s - recoil;
        let gy = base_cy + 4.0 * s;
        let burst = 0.45 + 0.9 * blast;

        fill_polygon(
            rt,
            factory,
            &[
                D2D_POINT_2F {
                    x: gx - 7.0 * s,
                    y: gy + 7.5 * s,
                },
                D2D_POINT_2F {
                    x: gx - 2.0 * s,
                    y: gy + 0.5 * s,
                },
                D2D_POINT_2F {
                    x: gx + 3.5 * s,
                    y: gy - 4.2 * s,
                },
                D2D_POINT_2F {
                    x: gx + 6.4 * s,
                    y: gy + 0.8 * s,
                },
                D2D_POINT_2F {
                    x: gx + 2.4 * s,
                    y: gy + 4.8 * s,
                },
                D2D_POINT_2F {
                    x: gx - 1.6 * s,
                    y: gy + 9.4 * s,
                },
            ],
            green,
        );
        draw_polyline(
            rt,
            factory,
            &[
                D2D_POINT_2F {
                    x: gx - 2.4 * s,
                    y: gy + 1.8 * s,
                },
                D2D_POINT_2F {
                    x: gx + 3.2 * s,
                    y: gy - 2.4 * s,
                },
            ],
            silver,
            1.1,
        );
        if let Ok(b) = rt.CreateSolidColorBrush(&silver, None) {
            rt.DrawEllipse(
                &D2D1_ELLIPSE {
                    point: D2D_POINT_2F {
                        x: gx - 4.2 * s,
                        y: gy + 6.2 * s,
                    },
                    radiusX: 1.7 * s,
                    radiusY: 1.7 * s,
                },
                &b,
                0.8,
                None,
            );
        }

        // Ledakan stylized oranye-emas.
        let bx = gx + 8.0 * s;
        let by = gy - 3.0 * s;
        draw_soft_radial_glow(
            rt,
            D2D_POINT_2F { x: bx, y: by },
            14.0 * burst,
            12.0 * burst,
            1.0,
            0.54,
            0.12,
            (0.30 + 0.55 * blast) * alpha,
        );
        let mut star = [D2D_POINT_2F { x: 0.0, y: 0.0 }; 16];
        for (i, pt) in star.iter_mut().enumerate() {
            let ang = (i as f32) * std::f32::consts::PI / 8.0 + ctx.phase_time * 0.5;
            let rad = if i % 2 == 0 {
                9.5 * burst * s
            } else {
                3.8 * burst * s
            };
            *pt = D2D_POINT_2F {
                x: bx + ang.cos() * rad,
                y: by + ang.sin() * rad,
            };
        }
        draw_soft_radial_glow(
            rt,
            D2D_POINT_2F { x: bx, y: by },
            12.0 * burst,
            12.0 * burst,
            1.0,
            0.92,
            0.62,
            (0.16 + 0.30 * blast) * alpha,
        );
        fill_polygon(
            rt,
            factory,
            &star,
            D2D1_COLOR_F {
                r: 1.0,
                g: 0.72,
                b: 0.24,
                a: (0.34 + 0.44 * blast) * alpha,
            },
        );
        fill_polygon(
            rt,
            factory,
            &star[..8],
            D2D1_COLOR_F {
                r: 1.0,
                g: 0.96,
                b: 0.78,
                a: (0.30 + 0.55 * blast) * alpha,
            },
        );
    }
}
