// src/mascot/eyes.rs
use windows::Win32::Graphics::Direct2D::Common::{
    D2D1_BEZIER_SEGMENT, D2D1_COLOR_F, D2D1_FIGURE_BEGIN_FILLED, D2D1_FIGURE_BEGIN_HOLLOW,
    D2D1_FIGURE_END_CLOSED, D2D1_FIGURE_END_OPEN, D2D1_GRADIENT_STOP, D2D_POINT_2F, D2D_RECT_F,
};
use windows::Win32::Graphics::Direct2D::{
    ID2D1Factory, ID2D1HwndRenderTarget, D2D1_ANTIALIAS_MODE_PER_PRIMITIVE, D2D1_CAP_STYLE_ROUND,
    D2D1_ELLIPSE, D2D1_EXTEND_MODE_CLAMP, D2D1_GAMMA_2_2, D2D1_LINE_JOIN_ROUND,
    D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES, D2D1_ROUNDED_RECT, D2D1_STROKE_STYLE_PROPERTIES,
};

pub struct DevilUnionEyes {
    pub phase_time: f32,
    pub blink_timer: f32,
    pub blink_progress: f32, // 0.0 = terbuka, 1.0 = tertutup
    pub look_x: f32,
    pub look_y: f32,
    pub target_look_x: f32,
    pub target_look_y: f32,
    pub flare_intensity: f32,

    pub idle_cycle_timer: f32,
    pub is_cheer_mode: bool,
    pub mode_aura_timer: f32,
    pub sword_float_phase: f32,
    pub grab_reach: f32, // 0.0..1.0: Tangan muncul meraih gagang pedang
    pub sword_pull: f32, // 0.0..1.0: Tangan menarik pedang mendekat dan tetap menggenggam
}

impl DevilUnionEyes {
    pub fn new() -> Self {
        Self {
            phase_time: 0.0,
            blink_timer: 2.8,
            blink_progress: 0.0,
            look_x: 0.0,
            look_y: 0.0,
            target_look_x: 0.0,
            target_look_y: 0.0,
            flare_intensity: 1.0,
            idle_cycle_timer: 0.0,
            is_cheer_mode: false,
            mode_aura_timer: 0.0,
            sword_float_phase: 0.0,
            grab_reach: 0.0,
            sword_pull: 0.0,
        }
    }

    pub fn trigger_awakening(&mut self) {
        self.flare_intensity = 1.8;
        self.blink_progress = 1.0;
        self.mode_aura_timer = 0.0;
    }

    pub fn trigger_wake_surge(&mut self) {
        self.flare_intensity = 2.4;
    }

    pub fn current_aura_envelope(&self) -> f32 {
        let dur = 5.2_f32;
        if self.mode_aura_timer >= dur {
            0.22
        } else if self.mode_aura_timer < 0.6 {
            0.22 + 0.78 * (self.mode_aura_timer / 0.6).clamp(0.0, 1.0)
        } else if self.mode_aura_timer > dur - 1.2 {
            0.22 + 0.78 * ((dur - self.mode_aura_timer) / 1.2).clamp(0.0, 1.0)
        } else {
            1.0
        }
    }

    pub fn update(&mut self, dt: f32, cursor_offset: Option<(f32, f32)>) {
        self.phase_time += dt;
        self.idle_cycle_timer += dt;
        self.mode_aura_timer += dt;
        self.sword_float_phase += dt * 2.6;

        let cycle = self.idle_cycle_timer % 15.0;
        let next_cheer = cycle >= 7.5;
        if next_cheer != self.is_cheer_mode {
            self.is_cheer_mode = next_cheer;
            self.mode_aura_timer = 0.0;
            self.blink_progress = 0.90;
        }

        // Koreografi Tangan Mengambil & Menarik Pedang (Genggaman Menetap Sampai Transisi Happy):
        // 0.0 .. 1.4: Pedang mengambang santai di sebelah kanan
        // 1.4 .. 2.8: Tangan muncul dan meraih gagang pedang (grab_reach: 0 -> 1)
        // 2.8 .. 7.5: Tangan menarik pedang mendekat dan TETAP MENGGENGGAMNYA sampai transisi Happy mode!
        let t = cycle.clamp(0.0, 7.5);
        if t < 1.4 {
            self.grab_reach = 0.0;
            self.sword_pull = 0.0;
        } else if t < 2.8 {
            let p = ((t - 1.4) / 1.4).clamp(0.0, 1.0);
            self.grab_reach = p * p * (3.0 - 2.0 * p);
            self.sword_pull = 0.0;
        } else {
            self.grab_reach = 1.0;
            let p = ((t - 2.8) / 1.2).clamp(0.0, 1.0);
            self.sword_pull = p * p * (3.0 - 2.0 * p);
        }

        if !self.is_cheer_mode && self.grab_reach > 0.1 {
            let look_target = 2.4 * self.grab_reach - 1.0 * self.sword_pull;
            self.target_look_x = look_target;
            self.target_look_y = -0.2 * self.grab_reach;
        } else if let Some((dx, dy)) = cursor_offset {
            self.target_look_x = (dx / 160.0).clamp(-1.0, 1.0) * 2.5;
            self.target_look_y = (dy / 90.0).clamp(-1.0, 1.0) * 1.2;
        } else {
            self.target_look_x = (self.phase_time * 1.0).sin() * 1.6;
            self.target_look_y = (self.phase_time * 0.7).cos() * 0.5;
        }

        let follow = (dt * 10.0).min(1.0);
        self.look_x += (self.target_look_x - self.look_x) * follow;
        self.look_y += (self.target_look_y - self.look_y) * follow;

        if self.blink_progress > 0.0 {
            self.blink_progress = (self.blink_progress - dt * 4.0).max(0.0);
        } else {
            self.blink_timer -= dt;
            if self.blink_timer <= 0.0 {
                self.blink_progress = 1.0;
                let seed = (self.phase_time * 100.0) as u32;
                self.blink_timer = 2.2 + (seed % 28) as f32 * 0.1;
            }
        }
    }

    unsafe fn draw_soft_radial_glow(
        &self,
        rt: &ID2D1HwndRenderTarget,
        center: D2D_POINT_2F,
        radius_x: f32,
        radius_y: f32,
        r: f32,
        g: f32,
        b: f32,
        peak_alpha: f32,
    ) {
        if peak_alpha <= 0.005 {
            return;
        }
        let stops = [
            D2D1_GRADIENT_STOP {
                position: 0.0,
                color: D2D1_COLOR_F {
                    r,
                    g,
                    b,
                    a: peak_alpha,
                },
            },
            D2D1_GRADIENT_STOP {
                position: 0.42,
                color: D2D1_COLOR_F {
                    r,
                    g,
                    b,
                    a: peak_alpha * 0.46,
                },
            },
            D2D1_GRADIENT_STOP {
                position: 0.76,
                color: D2D1_COLOR_F {
                    r: r * 0.7,
                    g: g * 0.7,
                    b: b * 0.7,
                    a: peak_alpha * 0.14,
                },
            },
            D2D1_GRADIENT_STOP {
                position: 1.0,
                color: D2D1_COLOR_F {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.0,
                },
            },
        ];

        if let Ok(stop_collection) =
            rt.CreateGradientStopCollection(&stops, D2D1_GAMMA_2_2, D2D1_EXTEND_MODE_CLAMP)
        {
            let props = D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES {
                center,
                gradientOriginOffset: D2D_POINT_2F { x: 0.0, y: 0.0 },
                radiusX: radius_x,
                radiusY: radius_y,
            };
            if let Ok(rad_brush) = rt.CreateRadialGradientBrush(&props, None, &stop_collection) {
                rt.FillRectangle(
                    &D2D_RECT_F {
                        left: center.x - radius_x,
                        top: center.y - radius_y,
                        right: center.x + radius_x,
                        bottom: center.y + radius_y,
                    },
                    &rad_brush,
                );
            }
        }
    }

    pub unsafe fn draw_idle_aura_background(
        &self,
        rt: &ID2D1HwndRenderTarget,
        factory: &ID2D1Factory,
        pill_rect: D2D_RECT_F,
        alpha: f32,
    ) {
        rt.SetAntialiasMode(D2D1_ANTIALIAS_MODE_PER_PRIMITIVE);

        let env = self.current_aura_envelope() * alpha;
        let pw = pill_rect.right - pill_rect.left;
        let ph = pill_rect.bottom - pill_rect.top;
        let cx = (pill_rect.left + pill_rect.right) * 0.5;

        if !self.is_cheer_mode {
            let pull_boost = 1.0 + self.sword_pull * 0.50;
            self.draw_soft_radial_glow(
                rt,
                D2D_POINT_2F {
                    x: cx,
                    y: pill_rect.bottom - 2.0,
                },
                pw * 0.46,
                ph * 0.70,
                0.95,
                0.12,
                0.24,
                (0.36 * env * pull_boost).min(0.55),
            );

            let flash_wave =
                ((self.phase_time * 12.0).sin() + (self.phase_time * 21.0).cos()) * 0.5;
            if flash_wave > -0.15 || self.sword_pull > 0.3 {
                let spark_alpha =
                    (((flash_wave + 0.25) * 0.9).max(self.sword_pull * 0.85)).clamp(0.2, 0.92)
                        * env;
                let glow_col = D2D1_COLOR_F {
                    r: 0.98,
                    g: 0.14,
                    b: 0.26,
                    a: spark_alpha * 0.38,
                };
                let core_col = D2D1_COLOR_F {
                    r: 1.0,
                    g: 0.70,
                    b: 0.78,
                    a: spark_alpha,
                };

                if let (Ok(glow_b), Ok(core_b)) = (
                    rt.CreateSolidColorBrush(&glow_col, None),
                    rt.CreateSolidColorBrush(&core_col, None),
                ) {
                    let anchors = [-pw * 0.26, pw * 0.22];
                    for (i, ax) in anchors.iter().enumerate() {
                        let shift = (self.phase_time * 14.0 + i as f32 * 2.3).sin() * 4.0;
                        let bx = cx + ax + shift;
                        let by = pill_rect.top + 5.0;

                        if let Ok(path) = factory.CreatePathGeometry() {
                            if let Ok(sink) = path.Open() {
                                sink.BeginFigure(
                                    D2D_POINT_2F {
                                        x: bx - 6.0,
                                        y: by + 2.0,
                                    },
                                    D2D1_FIGURE_BEGIN_HOLLOW,
                                );
                                sink.AddLines(&[
                                    D2D_POINT_2F {
                                        x: bx - 1.0,
                                        y: by + 9.0,
                                    },
                                    D2D_POINT_2F {
                                        x: bx - 4.0,
                                        y: by + 10.0,
                                    },
                                    D2D_POINT_2F {
                                        x: bx + 3.0,
                                        y: by + 19.0,
                                    },
                                ]);
                                sink.EndFigure(D2D1_FIGURE_END_OPEN);
                                let _ = sink.Close();

                                rt.DrawGeometry(&path, &glow_b, 3.4, None);
                                rt.DrawGeometry(&path, &core_b, 1.4, None);
                            }
                        }
                    }
                }
            }
        } else {
            self.draw_soft_radial_glow(
                rt,
                D2D_POINT_2F {
                    x: cx,
                    y: pill_rect.bottom - 2.0,
                },
                pw * 0.46,
                ph * 0.70,
                0.20,
                0.92,
                0.56,
                0.38 * env,
            );

            let part_col = D2D1_COLOR_F {
                r: 0.40,
                g: 1.0,
                b: 0.70,
                a: 0.65 * env,
            };
            if let Ok(pb) = rt.CreateSolidColorBrush(&part_col, None) {
                let p_offsets = [
                    (-pw * 0.24_f32, 2.0_f32, 1.2_f32),
                    (-pw * 0.12_f32, 4.2_f32, 1.6_f32),
                    (pw * 0.14_f32, 1.5_f32, 1.1_f32),
                    (pw * 0.26_f32, 3.8_f32, 1.5_f32),
                ];
                for (ox, speed, size) in &p_offsets {
                    let float_y = (self.phase_time * speed * 8.0) % (ph * 0.85);
                    let px = cx + ox;
                    let py = (pill_rect.bottom - 4.0) - float_y;

                    rt.DrawLine(
                        D2D_POINT_2F {
                            x: px - size * 2.0,
                            y: py,
                        },
                        D2D_POINT_2F {
                            x: px + size * 2.0,
                            y: py,
                        },
                        &pb,
                        1.2,
                        None,
                    );
                    rt.DrawLine(
                        D2D_POINT_2F {
                            x: px,
                            y: py - size * 2.0,
                        },
                        D2D_POINT_2F {
                            x: px,
                            y: py + size * 2.0,
                        },
                        &pb,
                        1.2,
                        None,
                    );
                }
            }
        }
    }

    /// Gambar maskot Asta Chibi:
    /// Menggunakan bentuk desain persis sebelumnya (robot visor, layered hair, horns, headband, sword, hand)
    /// yang disesuaikan agar CENTER di notch, FULLSIZE memenuhi tinggi notch (~30.8 DIPs dari 34 DIPs),
    /// dan dilengkapi efek ILUMINASI HIGHLIGHT menyala terang!
    pub unsafe fn draw(
        &self,
        rt: &ID2D1HwndRenderTarget,
        factory: &ID2D1Factory,
        center_x: f32,
        center_y: f32,
        alpha: f32,
    ) {
        if alpha <= 0.02 {
            return;
        }
        rt.SetAntialiasMode(D2D1_ANTIALIAS_MODE_PER_PRIMITIVE);

        let color = |r, g, b| D2D1_COLOR_F { r, g, b, a: alpha };
        let Ok(black) = rt.CreateSolidColorBrush(&color(0.02, 0.02, 0.025), None) else { return; };
        let Ok(hair) = rt.CreateSolidColorBrush(
            &if self.is_cheer_mode {
                color(0.72, 0.74, 0.78)
            } else {
                color(0.12, 0.13, 0.16)
            },
            None,
        ) else { return; };
        let Ok(highlight) = rt.CreateSolidColorBrush(
            &if self.is_cheer_mode {
                color(0.92, 0.94, 0.96)
            } else {
                color(0.32, 0.35, 0.40)
            },
            None,
        ) else { return; };
        let Ok(dark) = rt.CreateSolidColorBrush(&color(0.06, 0.065, 0.08), None) else { return; };
        let Ok(red) = rt.CreateSolidColorBrush(&color(1.0, 0.14, 0.26), None) else { return; };
        let Ok(red_core) = rt.CreateSolidColorBrush(&color(1.0, 0.82, 0.88), None) else { return; };
        let Ok(mint) = rt.CreateSolidColorBrush(&color(0.32, 0.98, 0.58), None) else { return; };
        let Ok(mint_core) = rt.CreateSolidColorBrush(&color(0.88, 1.0, 0.93), None) else { return; };
        let Ok(mint_glow) = rt.CreateSolidColorBrush(
            &D2D1_COLOR_F {
                r: 0.24,
                g: 0.96,
                b: 0.52,
                a: alpha * 0.48,
            },
            None,
        ) else { return; };
        let Ok(gold) = rt.CreateSolidColorBrush(&color(0.98, 0.82, 0.28), None) else { return; };

        let stroke = factory
            .CreateStrokeStyle(
                &D2D1_STROKE_STYLE_PROPERTIES {
                    startCap: D2D1_CAP_STYLE_ROUND,
                    endCap: D2D1_CAP_STYLE_ROUND,
                    dashCap: D2D1_CAP_STYLE_ROUND,
                    lineJoin: D2D1_LINE_JOIN_ROUND,
                    miterLimit: 10.0,
                    ..Default::default()
                },
                None,
            )
            .ok();

        let p = |x, y| D2D_POINT_2F { x, y };
        let poly = |points: &[(f32, f32)],
                    brush: &windows::Win32::Graphics::Direct2D::ID2D1SolidColorBrush| {
            if let Ok(path) = factory.CreatePathGeometry() {
                if let Ok(sink) = path.Open() {
                    sink.BeginFigure(p(points[0].0, points[0].1), D2D1_FIGURE_BEGIN_FILLED);
                    for &(x, y) in &points[1..] {
                        sink.AddLine(p(x, y));
                    }
                    sink.EndFigure(D2D1_FIGURE_END_CLOSED);
                    if sink.Close().is_ok() {
                        rt.FillGeometry(&path, brush, None);
                    }
                }
            }
        };
        let line = |points: &[(f32, f32)],
                    brush: &windows::Win32::Graphics::Direct2D::ID2D1SolidColorBrush,
                    width| {
            if let Ok(path) = factory.CreatePathGeometry() {
                if let Ok(sink) = path.Open() {
                    sink.BeginFigure(p(points[0].0, points[0].1), D2D1_FIGURE_BEGIN_HOLLOW);
                    for &(x, y) in &points[1..] {
                        sink.AddLine(p(x, y));
                    }
                    sink.EndFigure(D2D1_FIGURE_END_OPEN);
                    if sink.Close().is_ok() {
                        rt.DrawGeometry(&path, brush, width, stroke.as_ref());
                    }
                }
            }
        };

        // Layout dan Sizing:
        // Wajah dan kepala TEPAT DI TENGAH (center_x), tidak miring ke kiri!
        // Faktor skala s = 0.88 membuat seluruh maskot (rambut, tanduk, mata, pedang)
        // fit to notch setinggi 30.8 DIPs (dalam notch 34 DIPs), terisi penuh (fullsize) tanpa terpotong!
        let s = 0.88_f32;
        let cx = center_x;
        let cy = center_y + 2.0;

        let local = |points: &[(f32, f32)]| {
            points
                .iter()
                .map(|&(x, y)| (cx + x * s, cy + y * s))
                .collect::<Vec<_>>()
        };

        // 1. Dasar Kepala Robot Peeker (Hitam Pekat)
        rt.FillRoundedRectangle(
            &D2D1_ROUNDED_RECT {
                rect: D2D_RECT_F {
                    left: cx - 21.0 * s,
                    top: cy - 9.0 * s,
                    right: cx + 21.0 * s,
                    bottom: cy + 13.0 * s,
                },
                radiusX: 10.0 * s,
                radiusY: 10.0 * s,
            },
            &black,
        );

        // 2. Rambut Spiky Berlapis (Layered Spikes & Cowlick)
        poly(
            &local(&[
                (-23., -3.),
                (-28., -8.),
                (-20., -9.),
                (-25., -15.),
                (-14., -12.),
                (-16., -19.),
                (-6., -15.),
                (-2., -22.),
                (2., -18.),
                (1., -14.),
                (10., -19.),
                (10., -14.),
                (21., -17.),
                (18., -11.),
                (26., -12.),
                (22., -6.),
                (25., -3.),
                (16., 0.),
                (8., -3.),
                (0., 0.),
                (-9., -3.),
                (-18., 0.),
            ]),
            &hair,
        );

        // Highlight layer rambut (Memberi tekstur volume 3D)
        for points in [
            vec![(-23., -13.), (-12., -10.), (-17., -6.)],
            vec![(-13., -17.), (-4., -12.), (-9., -7.)],
            vec![(-1., -20.), (3., -14.), (0., -9.)],
            vec![(10., -17.), (7., -8.), (1., -5.)],
            vec![(20., -15.), (16., -7.), (10., -5.)],
            vec![(-24., -7.), (-17., -4.), (-20., -2.)],
        ] {
            poly(&local(&points), &highlight);
        }

        // 3. Aksesori Tanduk Devil Union (Ram Horns)
        if !self.is_cheer_mode {
            for side in [-1.0_f32, 1.0_f32] {
                let hx = cx + side * 20.0 * s;
                let hy = cy - 8.5 * s;
                let horn = D2D1_ELLIPSE {
                    point: p(hx, hy),
                    radiusX: 5.6 * s,
                    radiusY: 5.8 * s,
                };
                rt.FillEllipse(&horn, &dark);
                rt.DrawEllipse(&horn, &highlight, 1.0, None);
                line(
                    &[
                        (hx - side * 2.8 * s, hy + 2.8 * s),
                        (hx + side * 2.4 * s, hy + 1.8 * s),
                        (hx + side * 2.8 * s, hy - 1.8 * s),
                        (hx, hy - 2.8 * s),
                        (hx - side * s, hy),
                    ],
                    &hair,
                    2.0,
                );
                // Iluminasi crimson highlight tipis pada lekukan tanduk
                for j in 0..3 {
                    let y = hy - 2.8 * s + j as f32 * 2.2 * s;
                    line(
                        &[(hx + side * 3.8 * s, y), (hx + side * 5.2 * s, y - 0.6 * s)],
                        &red,
                        0.75,
                    );
                }
            }
        }

        // 4. Ikat Kepala (Headband)
        poly(
            &local(&[
                (-19., -7.),
                (-9., -6.),
                (0., -7.),
                (19., -7.),
                (20., -2.),
                (9., -1.),
                (0., -2.),
                (-20., -2.),
            ]),
            &dark,
        );

        if self.is_cheer_mode {
            // Silang merah regu Black Bulls di kiri
            line(&local(&[(-16., -5.7), (-10., -2.8)]), &red, 1.5);
            line(&local(&[(-12., -6.), (-14., -2.)]), &red, 1.5);
            // Emblem emas Black Bulls di kanan
            line(
                &local(&[(10., -6.), (10.5, -3.5), (13., -2.5), (15.5, -3.5), (16., -6.)]),
                &gold,
                1.1,
            );
            poly(&local(&[(11., -4.8), (15., -4.8), (14., -1.6), (12., -1.6)]), &gold);
            // Poni perak tengah
            poly(&local(&[(-3., -3.), (0., 1.), (3., -3.)]), &highlight);
        } else {
            // Mahkota garis zigzag merah Devil Union
            line(
                &local(&[
                    (-18., -4.5),
                    (-10., -3.5),
                    (-5., -5.),
                    (0., -9.),
                    (5., -5.),
                    (10., -3.5),
                    (18., -4.5),
                ]),
                &red,
                1.6,
            );
            line(&local(&[(-4., -7.), (0., -2.), (4., -7.)]), &red, 1.6);
            line(&local(&[(-3., -7.), (0., -3.), (3., -7.)]), &red_core, 0.8);
        }

        // 5. MATA BERSINAR (ILLUMINATED EYES)
        let ey = cy + 5.0 * s + self.look_y * 0.45;
        for side in [-1.0_f32, 1.0_f32] {
            let ex = cx + side * 9.2 * s + self.look_x * 0.35;

            if self.is_cheer_mode {
                // Efek Iluminasi Hijau Mint Emerald
                self.draw_soft_radial_glow(rt, p(ex, ey), 13.0, 13.0, 0.24, 0.96, 0.55, 0.65 * alpha);

                // Mata Lengkung Robotik Happy Arch (^  ^) dengan Core Putih-Mint Terang Menyala
                line(
                    &[(ex - 4.6 * s, ey + 2.4 * s), (ex, ey - 2.6 * s), (ex + 4.6 * s, ey + 2.4 * s)],
                    &mint_glow,
                    5.2,
                );
                line(
                    &[(ex - 4.6 * s, ey + 2.4 * s), (ex, ey - 2.6 * s), (ex + 4.6 * s, ey + 2.4 * s)],
                    &mint,
                    3.0,
                );
                line(
                    &[(ex - 4.6 * s, ey + 2.4 * s), (ex, ey - 2.6 * s), (ex + 4.6 * s, ey + 2.4 * s)],
                    &mint_core,
                    1.4,
                );
            } else {
                // Efek Iluminasi Merah Crimson Menyala
                self.draw_soft_radial_glow(rt, p(ex, ey), 13.0, 14.0, 1.0, 0.12, 0.25, 0.65 * alpha);

                // Mata Kapsul Devil Union Asta (Persis Anime Black Clover):
                // Iris gelap obsidian dengan limbal ring merah menyala + Pupil '+' (Liebe's Anti-Magic Cross)
                let serious_squash = 1.0 - 0.18 * self.grab_reach;
                let open = (1.0 - self.blink_progress).clamp(0.15, 1.0);
                let ry = 5.8 * s * serious_squash * open;
                let rx = 3.6 * s;

                // Dasar Iris Gelap Obsidian
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: p(ex, ey),
                        radiusX: rx,
                        radiusY: ry,
                    },
                    &dark,
                );

                // Ring Luar Crimson Menyala
                rt.DrawEllipse(
                    &D2D1_ELLIPSE {
                        point: p(ex, ey),
                        radiusX: rx,
                        radiusY: ry,
                    },
                    &red,
                    2.0,
                    None,
                );
                // Core Highlight Rim
                rt.DrawEllipse(
                    &D2D1_ELLIPSE {
                        point: p(ex, ey),
                        radiusX: rx * 0.78,
                        radiusY: ry * 0.78,
                    },
                    &red_core,
                    0.8,
                    None,
                );

                // PUPIL '+' DEVIL UNION (Persis Anime):
                // Simbol plus/cross anti-magic Liebe bercahaya merah crimson di pusat mata
                let arm_h = 2.4 * s;
                let arm_v = (3.6 * s * serious_squash * open).max(0.8);

                // Pendaran radial glow mini di sekitar pupil '+'
                self.draw_soft_radial_glow(rt, p(ex, ey), 7.0, 7.0, 1.0, 0.14, 0.26, 0.45 * alpha);

                // Lengan Horizontal & Vertikal '+' (Merah Crimson Menyala)
                line(&[(ex - arm_h, ey), (ex + arm_h, ey)], &red, 1.6);
                line(&[(ex, ey - arm_v), (ex, ey + arm_v)], &red, 1.6);

                // Highlight Core Putih-Merah di Inti Persimpangan '+' (Anime Glint)
                line(&[(ex - arm_h * 0.55, ey), (ex + arm_h * 0.55, ey)], &red_core, 0.8);
                line(&[(ex, ey - arm_v * 0.55), (ex, ey + arm_v * 0.55)], &red_core, 0.8);
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: p(ex, ey),
                        radiusX: 0.75 * s,
                        radiusY: 0.75 * s,
                    },
                    &red_core,
                );
            }
        }

        // 6. PEDANG DEMON-SLAYER & TANGAN MENGGENGGAM (DEVIL UNION) / TANGAN MELAMBAI (HAPPY)
        if !self.is_cheer_mode {
            let (sx, sy) = self.sword_anchor(center_x, center_y);
            let sword = |pts: &[(f32, f32)]| {
                pts.iter()
                    .map(|&(x, y)| (sx + x * s, sy + y * s))
                    .collect::<Vec<_>>()
            };

            // Iluminasi Pedang Crimson
            self.draw_soft_radial_glow(
                rt,
                p(sx + 6.0 * s, sy - 8.0 * s),
                16.0,
                16.0,
                1.0,
                0.12,
                0.24,
                0.35 * alpha,
            );

            // Bilah Pedang (Blade)
            poly(
                &sword(&[
                    (-3., -3.),
                    (10., -22.),
                    (19., -25.),
                    (21., -17.),
                    (5., 1.),
                ]),
                &dark,
            );
            line(
                &sword(&[
                    (-3., -3.),
                    (10., -22.),
                    (19., -25.),
                    (21., -17.),
                    (5., 1.),
                    (-3., -3.),
                ]),
                &highlight,
                1.0,
            );

            // Retakan Cahaya Anti-Magic Merah Menyala di Bilah Pedang
            line(
                &sword(&[
                    (2., -3.),
                    (8., -10.),
                    (6., -11.),
                    (13., -15.),
                    (12., -19.),
                    (17., -22.),
                ]),
                &red,
                1.4,
            );
            line(
                &sword(&[
                    (2., -3.),
                    (8., -10.),
                    (6., -11.),
                    (13., -15.),
                    (12., -19.),
                    (17., -22.),
                ]),
                &red_core,
                0.7,
            );
            line(&sword(&[(8., -10.), (13., -10.), (16., -15.)]), &red, 1.0);

            // Crossguard & Gagang (Hilt)
            line(&sword(&[(-6., -4.), (7., 4.)]), &hair, 3.0);
            line(&sword(&[(-1., 1.), (-6., 8.)]), &hair, 3.2);
            for j in 0..3 {
                let t = j as f32;
                line(
                    &sword(&[(-2. - t, 2. + t * 1.5), (-0.5 - t, 3. + t * 1.5)]),
                    &highlight,
                    0.8,
                );
            }
            // Pommel bulat
            rt.DrawEllipse(
                &D2D1_ELLIPSE {
                    point: p(sx - 7.0 * s, sy + 9.0 * s),
                    radiusX: 2.2 * s,
                    radiusY: 2.2 * s,
                },
                &hair,
                1.4,
                None,
            );

            // Tangan yang Meraih & Menggenggam Gagang Pedang (Genggaman Menetap!)
            if self.grab_reach > 0.02 {
                let hx = (cx + 19.0 * s) + (sx - 3.5 * s - (cx + 19.0 * s)) * self.grab_reach;
                let hy = (cy + 10.0 * s) + (sy + 4.5 * s - (cy + 10.0 * s)) * self.grab_reach;
                let hand = [
                    (hx - 3.2 * s, hy + 3.2 * s),
                    (hx - 4.2 * s, hy),
                    (hx - 3.2 * s, hy - 3.2 * s),
                    (hx, hy - 3.8 * s),
                    (hx + 2.2 * s, hy - 1.6 * s),
                    (hx + 4.2 * s, hy - 2.2 * s),
                    (hx + 4.2 * s, hy + 1.2 * s),
                    (hx + 1.6 * s, hy + 3.8 * s),
                    (hx - 3.2 * s, hy + 3.2 * s),
                ];

                self.draw_soft_radial_glow(rt, p(hx, hy), 10.0, 10.0, 1.0, 0.20, 0.32, 0.55 * alpha);
                poly(&hand, &black);
                line(&hand, &red, 1.5);
                line(&[(hx - 2.0 * s, hy - 1.0 * s), (hx + 1.0 * s, hy), (hx + 1.5 * s, hy + 1.5 * s)], &red_core, 1.1);
            }
        } else {
            // Tangan Melambai Gembira (Waving Mitten) di Mode Happy
            let wave = (self.phase_time * 5.5).sin();
            let hx = cx + 28.0 * s + wave * 1.5;
            let hy = cy + 4.0 * s + wave * 1.6;

            self.draw_soft_radial_glow(rt, p(hx, hy), 12.0, 12.0, 0.24, 0.96, 0.55, 0.55 * alpha);

            if let Ok(path) = factory.CreatePathGeometry() {
                if let Ok(sink) = path.Open() {
                    sink.BeginFigure(p(hx - 3.0 * s, hy + 5.0 * s), D2D1_FIGURE_BEGIN_FILLED);
                    sink.AddBezier(&D2D1_BEZIER_SEGMENT {
                        point1: p(hx - 5.0 * s, hy + 2.0 * s),
                        point2: p(hx - 5.0 * s, hy - 6.0 * s),
                        point3: p(hx - 1.0 * s, hy - 6.0 * s),
                    });
                    sink.AddBezier(&D2D1_BEZIER_SEGMENT {
                        point1: p(hx + 3.0 * s, hy - 7.0 * s),
                        point2: p(hx + 3.0 * s, hy - 2.0 * s),
                        point3: p(hx + 3.0 * s, hy - 1.0 * s),
                    });
                    sink.AddBezier(&D2D1_BEZIER_SEGMENT {
                        point1: p(hx + 7.0 * s, hy - 4.0 * s),
                        point2: p(hx + 7.0 * s, hy + 2.0 * s),
                        point3: p(hx + 3.0 * s, hy + 5.0 * s),
                    });
                    sink.EndFigure(D2D1_FIGURE_END_CLOSED);
                    if sink.Close().is_ok() {
                        rt.FillGeometry(&path, &black, None);
                        rt.DrawGeometry(&path, &mint, 1.6, stroke.as_ref());
                        rt.DrawGeometry(&path, &mint_core, 0.8, stroke.as_ref());
                    }
                }
            }
            line(&[(hx + 8.0 * s, hy - 6.0 * s), (hx + 9.5 * s, hy - 8.0 * s)], &mint_core, 1.2);
        }
    }

    pub fn sword_anchor(&self, center_x: f32, center_y: f32) -> (f32, f32) {
        let bob = self.sword_float_phase.sin() * 1.2 * (1.0 - self.grab_reach);
        // sx di sebelah kanan notch, sy nyaman pas di tinggi notch
        (
            center_x + 32.0 - self.sword_pull * 2.0,
            center_y + 4.5 + bob + self.sword_pull * 3.5,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sword_grab_precedes_downward_pull() {
        let mut eyes = DevilUnionEyes::new();
        eyes.update(1.0, None);
        assert_eq!(eyes.grab_reach, 0.0);
        eyes.update(1.5, None);
        assert!(eyes.grab_reach > 0.0);
    }

    #[test]
    fn grip_is_sustained_until_happy() {
        let mut eyes = DevilUnionEyes::new();
        // At t = 3.5s, grab_reach is 1.0 and sword_pull > 0
        eyes.update(3.5, None);
        assert_eq!(eyes.grab_reach, 1.0);
        assert!(eyes.sword_pull > 0.0);

        // At t = 6.0s, grab_reach stays 1.0 and sword_pull stays 1.0
        eyes.update(2.5, None);
        assert_eq!(eyes.grab_reach, 1.0);
        assert_eq!(eyes.sword_pull, 1.0);
        assert!(!eyes.is_cheer_mode);

        // At t = 8.0s, transitions to Happy mode
        eyes.update(2.0, None);
        assert!(eyes.is_cheer_mode);
    }

    #[test]
    fn cycle_returns_to_devil_and_keeps_animation_bounded() {
        let mut eyes = DevilUnionEyes::new();
        for _ in 0..1800 {
            eyes.update(1.0 / 60.0, None);
            assert!((0.0..=1.0).contains(&eyes.grab_reach));
            assert!((0.0..=1.0).contains(&eyes.sword_pull));
        }
        eyes.idle_cycle_timer = 15.0;
        eyes.update(0.0, None);
        assert!(!eyes.is_cheer_mode);
        assert_eq!(eyes.grab_reach, 0.0);
    }
}
