// src/mascot/eyes.rs
use windows::Win32::Graphics::Direct2D::Common::{
    D2D1_BEZIER_SEGMENT, D2D1_COLOR_F, D2D1_FIGURE_BEGIN_FILLED, D2D1_FIGURE_BEGIN_HOLLOW,
    D2D1_FIGURE_END_CLOSED, D2D1_FIGURE_END_OPEN, D2D1_GRADIENT_STOP, D2D_POINT_2F, D2D_RECT_F,
};
use windows::Win32::Graphics::Direct2D::{
    ID2D1Factory, ID2D1HwndRenderTarget, D2D1_ANTIALIAS_MODE_PER_PRIMITIVE, D2D1_ELLIPSE,
    D2D1_EXTEND_MODE_CLAMP, D2D1_GAMMA_2_2, D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES,
    D2D1_ROUNDED_RECT,
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

    // Motion & Gesture Versi 6.0 + Iluminasi Background
    pub idle_cycle_timer: f32,
    pub is_cheer_mode: bool,      // false = Devil Union + Pedang & Tangan, true = Normal Happy + Mulut Senyum
    pub mode_aura_timer: f32,     // Durasi iluminasi background saat berganti mode
    pub sword_float_phase: f32,   // Fase melayang pedang Demon-Slayer
    pub hand_grab_progress: f32,  // 0.0 = tangan di bawah, 1.0 = tangan muncul & menggenggam gagang pedang
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
            hand_grab_progress: 0.0,
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

    /// Amplop intensitas iluminasi background (aktif 4.0 detik di awal pergantian mode, lalu meredup ke 0.0)
    pub fn current_aura_envelope(&self) -> f32 {
        let dur = 4.0_f32;
        if self.mode_aura_timer >= dur {
            0.0
        } else if self.mode_aura_timer < 0.65 {
            (self.mode_aura_timer / 0.65).clamp(0.0, 1.0)
        } else if self.mode_aura_timer > dur - 1.15 {
            ((dur - self.mode_aura_timer) / 1.15).clamp(0.0, 1.0)
        } else {
            1.0
        }
    }

    pub fn update(&mut self, dt: f32, cursor_offset: Option<(f32, f32)>) {
        self.phase_time += dt;
        self.idle_cycle_timer += dt;
        self.mode_aura_timer += dt;
        self.sword_float_phase += dt * 2.4;

        // Siklus 16 detik: 0..8s = Devil Union (Pedang + Tangan Meraih), 8..16s = Normal Happy (Mulut Senyum + Healing)
        let cycle = self.idle_cycle_timer % 16.0;
        let next_cheer = cycle >= 8.0;
        if next_cheer != self.is_cheer_mode {
            self.is_cheer_mode = next_cheer;
            self.mode_aura_timer = 0.0;
            self.blink_progress = 0.85;
        }

        // Gerakan Tangan Muncul Perlahan & Mengambil Pedang (0.0 -> 1.0 -> 0.0)
        let grab_wave = (self.sword_float_phase * 0.65).sin();
        self.hand_grab_progress = ((grab_wave + 0.25) / 1.25).clamp(0.0, 1.0);

        // Lirikan Mata Hidup:
        // Saat mode Devil Union dan tangan sedang naik mengambil pedang (`hand_grab_progress > 0.35`),
        // mata Asta otomatis melirik ke kanan ke arah pedang & tangannya!
        if let Some((dx, dy)) = cursor_offset {
            let base_x = (dx / 180.0).clamp(-1.0, 1.0) * 2.5;
            let base_y = (dy / 100.0).clamp(-1.0, 1.0) * 1.2;
            if !self.is_cheer_mode && self.hand_grab_progress > 0.35 {
                let glance_strength = (self.hand_grab_progress - 0.35) / 0.65;
                self.target_look_x = base_x * (1.0 - glance_strength) + 2.4 * glance_strength;
                self.target_look_y = base_y * (1.0 - glance_strength) - 0.4 * glance_strength;
            } else {
                self.target_look_x = base_x;
                self.target_look_y = base_y;
            }
        } else if !self.is_cheer_mode && self.hand_grab_progress > 0.35 {
            let glance_strength = (self.hand_grab_progress - 0.35) / 0.65;
            self.target_look_x = 2.4 * glance_strength;
            self.target_look_y = -0.3 * glance_strength;
        } else {
            self.target_look_x = (self.phase_time * 0.9).sin() * 1.5;
            self.target_look_y = (self.phase_time * 0.6).cos() * 0.5;
        }

        let follow = (dt * 9.0).min(1.0);
        self.look_x += (self.target_look_x - self.look_x) * follow;
        self.look_y += (self.target_look_y - self.look_y) * follow;

        if self.flare_intensity > 1.0 {
            self.flare_intensity = (self.flare_intensity - dt * 2.2).max(1.0);
        }

        self.blink_timer -= dt;
        if self.blink_timer <= 0.0 {
            self.blink_progress += dt * 14.0;
            if self.blink_progress >= 1.0 {
                self.blink_progress = 0.0;
                self.blink_timer = 2.6 + (self.phase_time.sin().abs() * 2.2);
            }
        }
    }

    /// Helper: Iluminasi Radial Blurry (Tanpa Tepi Lingkaran Solid)
    unsafe fn draw_soft_radial_illumination(
        &self,
        rt: &ID2D1HwndRenderTarget,
        center: D2D_POINT_2F,
        radius_x: f32,
        radius_y: f32,
        core_r: f32,
        core_g: f32,
        core_b: f32,
        peak_alpha: f32,
    ) {
        let stops = [
            D2D1_GRADIENT_STOP {
                position: 0.0,
                color: D2D1_COLOR_F {
                    r: core_r,
                    g: core_g,
                    b: core_b,
                    a: peak_alpha,
                },
            },
            D2D1_GRADIENT_STOP {
                position: 0.45,
                color: D2D1_COLOR_F {
                    r: core_r * 0.8,
                    g: core_g * 0.8,
                    b: core_b * 0.8,
                    a: peak_alpha * 0.42,
                },
            },
            D2D1_GRADIENT_STOP {
                position: 0.80,
                color: D2D1_COLOR_F {
                    r: core_r * 0.5,
                    g: core_g * 0.5,
                    b: core_b * 0.5,
                    a: peak_alpha * 0.10,
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

    /// Efek Background Iluminasi Blurry + Partikel HD (Crimson Thunder & Healing)
    pub unsafe fn draw_idle_aura_background(
        &self,
        rt: &ID2D1HwndRenderTarget,
        factory: &ID2D1Factory,
        pill_rect: D2D_RECT_F,
        alpha: f32,
    ) {
        rt.SetAntialiasMode(D2D1_ANTIALIAS_MODE_PER_PRIMITIVE);

        let env = self.current_aura_envelope() * alpha;
        if env <= 0.01 {
            return;
        }

        let pw = pill_rect.right - pill_rect.left;
        let ph = pill_rect.bottom - pill_rect.top;
        let cx = (pill_rect.left + pill_rect.right) * 0.5;
        let cy = (pill_rect.top + pill_rect.bottom) * 0.5;

        if !self.is_cheer_mode {
            // 1. Iluminasi Merah-Hitam Blurry (Devil Union) + Kilatan Petir Merah HD
            let drift_x = (self.phase_time * 2.2).sin() * 10.0;
            let breathe = 0.82 + 0.18 * (self.phase_time * 4.5).sin();
            self.draw_soft_radial_illumination(
                rt,
                D2D_POINT_2F {
                    x: cx + drift_x,
                    y: cy + 1.0,
                },
                pw * 0.48,
                ph * 0.95,
                0.78,
                0.08,
                0.15,
                0.32 * env * breathe,
            );

            let flash_wave =
                ((self.phase_time * 11.0).sin() + (self.phase_time * 19.0).cos()) * 0.5;
            if flash_wave > -0.10 {
                let spark_alpha = ((flash_wave + 0.15) * 1.1).clamp(0.2, 0.92) * env;
                let glow_col = D2D1_COLOR_F {
                    r: 0.90,
                    g: 0.10,
                    b: 0.20,
                    a: spark_alpha * 0.36,
                };
                let core_col = D2D1_COLOR_F {
                    r: 1.0,
                    g: 0.38,
                    b: 0.44,
                    a: spark_alpha,
                };

                if let (Ok(glow_b), Ok(core_b)) = (
                    rt.CreateSolidColorBrush(&glow_col, None),
                    rt.CreateSolidColorBrush(&core_col, None),
                ) {
                    let anchors = [-pw * 0.28, pw * 0.24];
                    for (i, ax) in anchors.iter().enumerate() {
                        let shift = (self.phase_time * 14.0 + i as f32 * 2.5).sin() * 4.0;
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
                                        y: by + 8.0,
                                    },
                                    D2D_POINT_2F {
                                        x: bx - 4.0,
                                        y: by + 11.0,
                                    },
                                    D2D_POINT_2F {
                                        x: bx + 3.0,
                                        y: by + 19.0,
                                    },
                                    D2D_POINT_2F {
                                        x: bx + 0.5,
                                        y: by + 13.5,
                                    },
                                    D2D_POINT_2F {
                                        x: bx + 7.0,
                                        y: by + 21.0,
                                    },
                                ]);
                                sink.EndFigure(D2D1_FIGURE_END_OPEN);
                                let _ = sink.Close();
                                rt.DrawGeometry(&path, &glow_b, 3.0, None);
                                rt.DrawGeometry(&path, &core_b, 1.0, None);
                            }
                        }
                    }
                }
            }
        } else {
            // 2. Iluminasi Hijau Blurry (Healing) + Partikel '+' & Micro-Glow
            let drift_x = (self.phase_time * 1.8).sin() * 10.0;
            let breathe = 0.85 + 0.15 * (self.phase_time * 3.2).sin();
            self.draw_soft_radial_illumination(
                rt,
                D2D_POINT_2F {
                    x: cx + drift_x,
                    y: cy + 2.0,
                },
                pw * 0.46,
                ph * 0.92,
                0.08,
                0.72,
                0.46,
                0.28 * env * breathe,
            );

            let halo_col = D2D1_COLOR_F {
                r: 0.16,
                g: 0.85,
                b: 0.56,
                a: 0.28 * env,
            };
            let core_col = D2D1_COLOR_F {
                r: 0.65,
                g: 0.98,
                b: 0.82,
                a: 0.88 * env,
            };

            if let (Ok(halo_b), Ok(core_b)) = (
                rt.CreateSolidColorBrush(&halo_col, None),
                rt.CreateSolidColorBrush(&core_col, None),
            ) {
                let slots = [-0.34_f32, -0.20, 0.20, 0.34];
                for (i, rel_x) in slots.iter().enumerate() {
                    let speed = 10.5 + (i as f32 * 2.0);
                    let progress = ((self.phase_time * speed + i as f32 * 6.0) % (ph - 8.0))
                        / (ph - 8.0).max(1.0);
                    let py = pill_rect.bottom - 4.0 - progress * (ph - 8.0);
                    let px = cx + rel_x * pw + (self.phase_time * 2.5 + i as f32).sin() * 2.0;

                    let fade = (progress * std::f32::consts::PI).sin().clamp(0.0, 1.0);
                    if fade > 0.1 {
                        if i % 2 == 0 {
                            let arm = 2.2_f32;
                            rt.DrawLine(
                                D2D_POINT_2F { x: px - arm, y: py },
                                D2D_POINT_2F { x: px + arm, y: py },
                                &halo_b,
                                2.6,
                                None,
                            );
                            rt.DrawLine(
                                D2D_POINT_2F { x: px, y: py - arm },
                                D2D_POINT_2F { x: px, y: py + arm },
                                &halo_b,
                                2.6,
                                None,
                            );
                            rt.DrawLine(
                                D2D_POINT_2F { x: px - arm, y: py },
                                D2D_POINT_2F { x: px + arm, y: py },
                                &core_b,
                                1.0,
                                None,
                            );
                            rt.DrawLine(
                                D2D_POINT_2F { x: px, y: py - arm },
                                D2D_POINT_2F { x: px, y: py + arm },
                                &core_b,
                                1.0,
                                None,
                            );
                        } else {
                            self.draw_soft_radial_illumination(
                                rt,
                                D2D_POINT_2F { x: px, y: py },
                                4.0,
                                4.0,
                                0.45,
                                0.96,
                                0.76,
                                0.72 * env * fade,
                            );
                        }
                    }
                }
            }
        }
    }

    /// Menggambar Maskot Asta Versi 6.0:
    /// - Mode Devil Union: Mata Merah + Mahkota Tanduk + Pedang Demon-Slayer Akurat Mengambang + Tangan Muncul Meraih Pedang + Mata Melirik
    /// - Mode Normal Happy: Mata Hijau Khas Asta + Mulut Tersenyum Bahagia (`◡`) + Ikat Kepala Black Bulls
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

        // Geser sedikit pusat mata ke kiri (-6px) saat Devil Union agar ruang kanan pas untuk pedang & tangan
        let face_cx = if !self.is_cheer_mode {
            center_x - 6.0
        } else {
            center_x
        };
        let eye_spacing = 12.5_f32;
        let open_factor = (1.0 - self.blink_progress).clamp(0.08, 1.0);

        let skin_peach = D2D1_COLOR_F {
            r: 0.96,
            g: 0.82,
            b: 0.72,
            a: alpha,
        };
        let hair_ash_light = D2D1_COLOR_F {
            r: 0.85,
            g: 0.87,
            b: 0.91,
            a: alpha,
        };
        let hair_ash_mid = D2D1_COLOR_F {
            r: 0.64,
            g: 0.68,
            b: 0.75,
            a: alpha,
        };
        let headband_obsidian = D2D1_COLOR_F {
            r: 0.09,
            g: 0.11,
            b: 0.15,
            a: alpha,
        };
        let blade_iron = D2D1_COLOR_F {
            r: 0.14,
            g: 0.16,
            b: 0.21,
            a: alpha,
        };
        let crimson_core = D2D1_COLOR_F {
            r: 0.94,
            g: 0.16,
            b: 0.24,
            a: alpha,
        };
        let emerald_green = D2D1_COLOR_F {
            r: 0.18,
            g: 0.84,
            b: 0.46,
            a: alpha,
        };
        let sclera_white = D2D1_COLOR_F {
            r: 0.97,
            g: 0.98,
            b: 1.00,
            a: alpha,
        };
        let smile_pink = D2D1_COLOR_F {
            r: 0.94,
            g: 0.45,
            b: 0.52,
            a: alpha,
        };

        let Ok(skin_brush) = rt.CreateSolidColorBrush(&skin_peach, None) else { return };
        let Ok(hair_light_brush) = rt.CreateSolidColorBrush(&hair_ash_light, None) else { return };
        let Ok(hair_mid_brush) = rt.CreateSolidColorBrush(&hair_ash_mid, None) else { return };
        let Ok(band_brush) = rt.CreateSolidColorBrush(&headband_obsidian, None) else { return };
        let Ok(blade_brush) = rt.CreateSolidColorBrush(&blade_iron, None) else { return };
        let Ok(crimson_brush) = rt.CreateSolidColorBrush(&crimson_core, None) else { return };
        let Ok(emerald_brush) = rt.CreateSolidColorBrush(&emerald_green, None) else { return };
        let Ok(sclera_brush) = rt.CreateSolidColorBrush(&sclera_white, None) else { return };
        let Ok(smile_brush) = rt.CreateSolidColorBrush(&smile_pink, None) else { return };

        // 1. IKAT KEPALA BLACK BULLS & RAMBUT SPIKY ATAS (Simple & Gemas)
        rt.FillRoundedRectangle(
            &D2D1_ROUNDED_RECT {
                rect: D2D_RECT_F {
                    left: face_cx - 21.0,
                    top: center_y - 11.0,
                    right: face_cx + 21.0,
                    bottom: center_y - 4.2,
                },
                radiusX: 3.2,
                radiusY: 3.2,
            },
            &band_brush,
        );

        // Insignia Cross Merah di sisi kiri headband
        let bx = face_cx - 14.0;
        let by = center_y - 7.6;
        rt.DrawLine(
            D2D_POINT_2F { x: bx - 2.6, y: by },
            D2D_POINT_2F { x: bx + 2.6, y: by },
            &crimson_brush,
            1.5,
            None,
        );
        rt.DrawLine(
            D2D_POINT_2F { x: bx, y: by - 2.6 },
            D2D_POINT_2F { x: bx, y: by + 2.6 },
            &crimson_brush,
            1.5,
            None,
        );

        // Spiky Hair Ash-Silver di atas headband
        let spikes = [
            (-21.0_f32, -5.0_f32, -25.0_f32, -13.0_f32, -14.0_f32, -10.5_f32),
            (-15.0, -9.5, -16.0, -16.5, -6.0, -10.5),
            (-7.0, -10.0, -4.0, -17.5, 3.0, -10.0),
            (1.0, -10.0, 7.0, -17.5, 12.0, -9.5),
            (9.0, -9.5, 17.0, -16.0, 19.0, -7.5),
            (15.0, -7.5, 25.0, -12.0, 21.0, -3.5),
        ];
        for (i, &(x1, y1, tx, ty, x2, y2)) in spikes.iter().enumerate() {
            if let Ok(path) = factory.CreatePathGeometry() {
                if let Ok(sink) = path.Open() {
                    sink.BeginFigure(
                        D2D_POINT_2F {
                            x: face_cx + x1,
                            y: center_y + y1,
                        },
                        D2D1_FIGURE_BEGIN_FILLED,
                    );
                    sink.AddLines(&[
                        D2D_POINT_2F {
                            x: face_cx + tx,
                            y: center_y + ty,
                        },
                        D2D_POINT_2F {
                            x: face_cx + x2,
                            y: center_y + y2,
                        },
                    ]);
                    sink.EndFigure(D2D1_FIGURE_END_CLOSED);
                    let _ = sink.Close();
                    let b = if i % 2 == 0 {
                        &hair_light_brush
                    } else {
                        &hair_mid_brush
                    };
                    rt.FillGeometry(&path, b, None);
                }
            }
        }

        // 2. TANDUK DEVIL UNION (Saat Mode Devil Union)
        if !self.is_cheer_mode {
            for side in [-1.0_f32, 1.0_f32] {
                if let Ok(h_path) = factory.CreatePathGeometry() {
                    if let Ok(sink) = h_path.Open() {
                        let hx = face_cx + side * 12.0;
                        let hy = center_y - 10.0;
                        sink.BeginFigure(D2D_POINT_2F { x: hx, y: hy }, D2D1_FIGURE_BEGIN_FILLED);
                        sink.AddBezier(&D2D1_BEZIER_SEGMENT {
                            point1: D2D_POINT_2F {
                                x: hx + side * 7.0,
                                y: hy - 5.0,
                            },
                            point2: D2D_POINT_2F {
                                x: hx + side * 5.0,
                                y: hy - 12.0,
                            },
                            point3: D2D_POINT_2F {
                                x: hx + side * 1.2,
                                y: hy - 15.5,
                            },
                        });
                        sink.AddBezier(&D2D1_BEZIER_SEGMENT {
                            point1: D2D_POINT_2F {
                                x: hx + side * 2.0,
                                y: hy - 9.5,
                            },
                            point2: D2D_POINT_2F {
                                x: hx - side * 1.2,
                                y: hy - 4.5,
                            },
                            point3: D2D_POINT_2F {
                                x: hx - side * 4.0,
                                y: hy,
                            },
                        });
                        sink.EndFigure(D2D1_FIGURE_END_CLOSED);
                        let _ = sink.Close();
                        rt.FillGeometry(&h_path, &band_brush, None);
                        rt.DrawGeometry(&h_path, &crimson_brush, 0.9, None);
                    }
                }
            }
        }

        // 3. MATA HIDUP (Bergeser Melirik Pedang saat Devil Union / Mata Hijau Cerah saat Normal Happy)
        for side in [-1.0_f32, 1.0_f32] {
            let ex = face_cx + side * eye_spacing;
            let ey = center_y + 1.5;

            if self.is_cheer_mode {
                // Mata Hijau Khas Asta yang Berbinar Ceria
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F { x: ex, y: ey },
                        radiusX: 5.4,
                        radiusY: 4.3 * open_factor,
                    },
                    &sclera_brush,
                );
                let px = ex + self.look_x * 0.55;
                let py = ey + self.look_y * 0.55;
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F { x: px, y: py },
                        radiusX: 3.3,
                        radiusY: 3.4 * open_factor,
                    },
                    &emerald_brush,
                );
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F { x: px, y: py },
                        radiusX: 1.3,
                        radiusY: 1.4 * open_factor,
                    },
                    &band_brush,
                );
                // Catchlight putih bersih
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F {
                            x: px - 1.1,
                            y: py - 1.1 * open_factor,
                        },
                        radiusX: 1.1,
                        radiusY: 1.1,
                    },
                    &sclera_brush,
                );
                // Alis ramah melengkung
                rt.DrawLine(
                    D2D_POINT_2F {
                        x: ex - 4.8,
                        y: ey - 4.8,
                    },
                    D2D_POINT_2F {
                        x: ex + 4.8,
                        y: ey - 4.0,
                    },
                    &band_brush,
                    1.5,
                    None,
                );
            } else {
                // Mata Devil Union Tegas (Melirik ke arah pedang saat tangan mengambilnya!)
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F { x: ex, y: ey },
                        radiusX: 5.8,
                        radiusY: 3.7 * open_factor,
                    },
                    &sclera_brush,
                );
                let px = ex + self.look_x * 0.75;
                let py = ey + self.look_y * 0.65;
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F { x: px, y: py },
                        radiusX: 3.4,
                        radiusY: 3.2 * open_factor,
                    },
                    &crimson_brush,
                );
                // Pupil vertikal tajam Devil Union
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F { x: px, y: py },
                        radiusX: 0.9,
                        radiusY: 2.7 * open_factor,
                    },
                    &band_brush,
                );
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F {
                            x: px - 1.0,
                            y: py - 1.0 * open_factor,
                        },
                        radiusX: 0.85,
                        radiusY: 0.85,
                    },
                    &sclera_brush,
                );
                // Alis serius menyudut ke tengah
                rt.DrawLine(
                    D2D_POINT_2F {
                        x: ex - side * 4.6,
                        y: ey - 2.4,
                    },
                    D2D_POINT_2F {
                        x: ex + side * 5.6,
                        y: ey - 4.8,
                    },
                    &band_brush,
                    1.8,
                    None,
                );
            }
        }

        // 4. MULUT SENYUM BAHAGIA SAAT MODE NORMAL & HAPPY (`is_cheer_mode == true`)
        if self.is_cheer_mode {
            // Mulut chibi tersenyum lebar bahagia (`◡` / terbuka ceria) di tengah bawah mata
            if let Ok(mouth_path) = factory.CreatePathGeometry() {
                if let Ok(sink) = mouth_path.Open() {
                    let mx = face_cx;
                    let my = center_y + 7.2;
                    sink.BeginFigure(
                        D2D_POINT_2F {
                            x: mx - 4.2,
                            y: my - 0.5,
                        },
                        D2D1_FIGURE_BEGIN_FILLED,
                    );
                    sink.AddBezier(&D2D1_BEZIER_SEGMENT {
                        point1: D2D_POINT_2F {
                            x: mx - 2.2,
                            y: my + 4.2,
                        },
                        point2: D2D_POINT_2F {
                            x: mx + 2.2,
                            y: my + 4.2,
                        },
                        point3: D2D_POINT_2F {
                            x: mx + 4.2,
                            y: my - 0.5,
                        },
                    });
                    sink.EndFigure(D2D1_FIGURE_END_CLOSED);
                    let _ = sink.Close();
                    rt.FillGeometry(&mouth_path, &smile_brush, None);
                    rt.DrawGeometry(&mouth_path, &sclera_brush, 0.9, None);
                }
            }
        }

        // 5. PEDANG DEMON-SLAYER AKURAT MENGAMBANG & TANGAN MUNCUL PERLAHAN MERAIHNYA (Saat Mode Devil Union)
        if !self.is_cheer_mode {
            // Saat tangan semakin dekat (hand_grab_progress -> 1.0), pedang sedikit merendah menyambut tangan
            let sword_bob = self.sword_float_phase.sin() * 2.4 * (1.0 - self.hand_grab_progress * 0.45);
            let sx = face_cx + 33.0;
            let sy = center_y + 1.0 + sword_bob * 0.6;

            // Bilah Besar Demon-Slayer Greatsword (Bentuk Akurat dengan Punggung Tengah & Ujung Runcing)
            if let Ok(sword_path) = factory.CreatePathGeometry() {
                if let Ok(sink) = sword_path.Open() {
                    sink.BeginFigure(
                        D2D_POINT_2F {
                            x: sx - 7.5,
                            y: sy - 2.2,
                        },
                        D2D1_FIGURE_BEGIN_FILLED,
                    );
                    sink.AddLines(&[
                        D2D_POINT_2F {
                            x: sx + 10.5,
                            y: sy - 6.2,
                        },
                        D2D_POINT_2F {
                            x: sx + 16.5,
                            y: sy,
                        },
                        D2D_POINT_2F {
                            x: sx + 10.5,
                            y: sy + 6.2,
                        },
                        D2D_POINT_2F {
                            x: sx - 7.5,
                            y: sy + 2.2,
                        },
                    ]);
                    sink.EndFigure(D2D1_FIGURE_END_CLOSED);
                    let _ = sink.Close();

                    rt.FillGeometry(&sword_path, &blade_brush, None);
                    rt.DrawGeometry(&sword_path, &crimson_brush, 1.1, None);
                }
            }

            // Garis Anti-Magic Merah di Tengah Bilah Pedang (Fuller)
            rt.DrawLine(
                D2D_POINT_2F {
                    x: sx - 5.5,
                    y: sy,
                },
                D2D_POINT_2F {
                    x: sx + 12.0,
                    y: sy,
                },
                &crimson_brush,
                1.2,
                None,
            );

            // Crossguard (Pelindung Gagang Pedang)
            rt.DrawLine(
                D2D_POINT_2F {
                    x: sx - 7.8,
                    y: sy - 4.2,
                },
                D2D_POINT_2F {
                    x: sx - 7.8,
                    y: sy + 4.2,
                },
                &crimson_brush,
                2.0,
                None,
            );

            // Gagang Pedang (Hilt) + Pommel tempat tangan Asta menggenggam
            let hilt_x = sx - 13.5;
            rt.DrawLine(
                D2D_POINT_2F { x: hilt_x, y: sy },
                D2D_POINT_2F {
                    x: sx - 8.0,
                    y: sy,
                },
                &crimson_brush,
                2.4,
                None,
            );

            // Tangan Asta yang Muncul Perlahan dari Bawah/Kiri dan Menggenggam Gagang Pedang!
            let start_hand_x = face_cx + 16.0;
            let start_hand_y = center_y + 9.5;
            let target_hand_x = hilt_x + 2.2;
            let target_hand_y = sy + 0.5;

            let cur_hand_x =
                start_hand_x + (target_hand_x - start_hand_x) * self.hand_grab_progress;
            let cur_hand_y =
                start_hand_y + (target_hand_y - start_hand_y) * self.hand_grab_progress;

            // Telapak tangan chibi
            rt.FillEllipse(
                &D2D1_ELLIPSE {
                    point: D2D_POINT_2F {
                        x: cur_hand_x,
                        y: cur_hand_y,
                    },
                    radiusX: 3.3,
                    radiusY: 2.9,
                },
                &skin_brush,
            );
            rt.DrawEllipse(
                &D2D1_ELLIPSE {
                    point: D2D_POINT_2F {
                        x: cur_hand_x,
                        y: cur_hand_y,
                    },
                    radiusX: 3.3,
                    radiusY: 2.9,
                },
                &band_brush,
                0.8,
                None,
            );
        }
    }
}