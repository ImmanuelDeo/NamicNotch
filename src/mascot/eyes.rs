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
    pub grab_reach: f32, // 0.0..1.0: Tangan muncul meraih gagang pedang & memicu ekspresi serius
    pub sword_pull: f32, // 0.0..1.0: Tangan menarik pedang mendekat ke arah Asta
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

        // Koreografi 3 Tahap Tangan Mengambil & Menarik Pedang + Ekspresi Serius:
        let t = cycle.clamp(0.0, 7.5);
        if t < 1.6 {
            self.grab_reach = 0.0;
            self.sword_pull = 0.0;
        } else if t < 3.2 {
            let p = ((t - 1.6) / 1.6).clamp(0.0, 1.0);
            self.grab_reach = p * p * (3.0 - 2.0 * p);
            self.sword_pull = 0.0;
        } else if t < 5.6 {
            self.grab_reach = 1.0;
            let p = ((t - 3.2) / 1.4).clamp(0.0, 1.0);
            self.sword_pull = p * p * (3.0 - 2.0 * p);
        } else {
            let p = (1.0 - (t - 5.6) / 1.6).clamp(0.0, 1.0);
            self.grab_reach = p;
            self.sword_pull = p;
        }

        if !self.is_cheer_mode && self.grab_reach > 0.1 {
            let look_target = 2.8 * self.grab_reach - 1.2 * self.sword_pull;
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

        if self.flare_intensity > 1.0 {
            self.flare_intensity = (self.flare_intensity - dt * 2.2).max(1.0);
        }

        self.blink_timer -= dt;
        if self.blink_timer <= 0.0 {
            self.blink_progress += dt * 15.0;
            if self.blink_progress >= 1.0 {
                self.blink_progress = 0.0;
                self.blink_timer = 2.6 + (self.phase_time.sin().abs() * 2.2);
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
                0.85,
                0.12,
                0.24,
                (0.32 * env * pull_boost).min(0.50),
            );

            let flash_wave =
                ((self.phase_time * 12.0).sin() + (self.phase_time * 21.0).cos()) * 0.5;
            if flash_wave > -0.15 || self.sword_pull > 0.3 {
                let spark_alpha =
                    (((flash_wave + 0.25) * 0.9).max(self.sword_pull * 0.85)).clamp(0.2, 0.92)
                        * env;
                let glow_col = D2D1_COLOR_F {
                    r: 0.94,
                    g: 0.12,
                    b: 0.22,
                    a: spark_alpha * 0.38,
                };
                let core_col = D2D1_COLOR_F {
                    r: 1.0,
                    g: 0.42,
                    b: 0.48,
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
                                        y: by + 8.0,
                                    },
                                    D2D_POINT_2F {
                                        x: bx - 4.0,
                                        y: by + 11.0,
                                    },
                                    D2D_POINT_2F {
                                        x: bx + 3.5,
                                        y: by + 19.0,
                                    },
                                    D2D_POINT_2F {
                                        x: bx + 0.5,
                                        y: by + 13.5,
                                    },
                                    D2D_POINT_2F {
                                        x: bx + 7.5,
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
            self.draw_soft_radial_glow(
                rt,
                D2D_POINT_2F {
                    x: cx,
                    y: pill_rect.bottom - 2.0,
                },
                pw * 0.46,
                ph * 0.70,
                0.20,
                0.88,
                0.56,
                0.34 * env,
            );

            let halo_col = D2D1_COLOR_F {
                r: 0.22,
                g: 0.92,
                b: 0.58,
                a: 0.30 * env,
            };
            let core_col = D2D1_COLOR_F {
                r: 0.74,
                g: 1.0,
                b: 0.86,
                a: 0.90 * env,
            };

            if let (Ok(halo_b), Ok(core_b)) = (
                rt.CreateSolidColorBrush(&halo_col, None),
                rt.CreateSolidColorBrush(&core_col, None),
            ) {
                let slots = [-0.33_f32, -0.20, 0.23, 0.35];
                for (i, rel_x) in slots.iter().enumerate() {
                    let speed = 10.5 + (i as f32 * 2.0);
                    let progress = ((self.phase_time * speed + i as f32 * 6.0) % (ph - 8.0))
                        / (ph - 8.0).max(1.0);
                    let py = pill_rect.bottom - 4.0 - progress * (ph - 8.0);
                    let px = cx + rel_x * pw + (self.phase_time * 2.5 + i as f32).sin() * 2.0;

                    let fade = (progress * std::f32::consts::PI).sin().clamp(0.0, 1.0);
                    if fade > 0.1 {
                        let arm = 2.1_f32;
                        rt.DrawLine(
                            D2D_POINT_2F { x: px - arm, y: py },
                            D2D_POINT_2F { x: px + arm, y: py },
                            &halo_b,
                            2.5,
                            None,
                        );
                        rt.DrawLine(
                            D2D_POINT_2F { x: px, y: py - arm },
                            D2D_POINT_2F { x: px, y: py + arm },
                            &halo_b,
                            2.5,
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
                    }
                }
            }
        }
    }

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

        let notch_bg_black = D2D1_COLOR_F {
            r: 0.016,
            g: 0.016,
            b: 0.022,
            a: alpha,
        };
        let obsidian_hair = D2D1_COLOR_F {
            r: 0.08,
            g: 0.09,
            b: 0.13,
            a: alpha,
        };
        let obsidian_horn = D2D1_COLOR_F {
            r: 0.12,
            g: 0.14,
            b: 0.19,
            a: alpha,
        };
        let ash_grey_light = D2D1_COLOR_F {
            r: 0.84,
            g: 0.87,
            b: 0.91,
            a: alpha,
        };
        let ash_grey_mid = D2D1_COLOR_F {
            r: 0.64,
            g: 0.68,
            b: 0.74,
            a: alpha,
        };
        let crimson_crown = D2D1_COLOR_F {
            r: 0.94,
            g: 0.14,
            b: 0.26,
            a: alpha,
        };
        // Warna Mata Merah Devil Union (Desain & Lapisan Sama Persis dengan Hijau Normal!)
        let crimson_eye_outer = D2D1_COLOR_F {
            r: 0.98,
            g: 0.18,
            b: 0.30,
            a: alpha,
        };
        let crimson_eye_core = D2D1_COLOR_F {
            r: 1.0,
            g: 0.72,
            b: 0.78,
            a: alpha,
        };
        // Warna Mata Hijau Muda Normal Asta
        let light_green_outer = D2D1_COLOR_F {
            r: 0.24,
            g: 0.92,
            b: 0.52,
            a: alpha,
        };
        let light_green_core = D2D1_COLOR_F {
            r: 0.78,
            g: 1.0,
            b: 0.86,
            a: alpha,
        };
        let light_green_glow_stroke = D2D1_COLOR_F {
            r: 0.24,
            g: 0.94,
            b: 0.54,
            a: alpha * 0.42,
        };
        let gold_badge = D2D1_COLOR_F {
            r: 0.98,
            g: 0.82,
            b: 0.24,
            a: alpha,
        };

        let Ok(bg_mask_b) = rt.CreateSolidColorBrush(&notch_bg_black, None) else { return };
        let Ok(obs_hair_b) = rt.CreateSolidColorBrush(&obsidian_hair, None) else { return };
        let Ok(obs_horn_b) = rt.CreateSolidColorBrush(&obsidian_horn, None) else { return };
        let Ok(ash_light_b) = rt.CreateSolidColorBrush(&ash_grey_light, None) else { return };
        let Ok(ash_mid_b) = rt.CreateSolidColorBrush(&ash_grey_mid, None) else { return };
        let Ok(crimson_crown_b) = rt.CreateSolidColorBrush(&crimson_crown, None) else { return };
        let Ok(crimson_eye_b) = rt.CreateSolidColorBrush(&crimson_eye_outer, None) else { return };
        let Ok(crimson_core_b) = rt.CreateSolidColorBrush(&crimson_eye_core, None) else { return };
        let Ok(green_eye_b) = rt.CreateSolidColorBrush(&light_green_outer, None) else { return };
        let Ok(green_core_b) = rt.CreateSolidColorBrush(&light_green_core, None) else { return };
        let Ok(green_glow_b) = rt.CreateSolidColorBrush(&light_green_glow_stroke, None) else {
            return;
        };
        let Ok(gold_b) = rt.CreateSolidColorBrush(&gold_badge, None) else { return };

        let round_stroke = factory
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

        let face_cx = if !self.is_cheer_mode {
            center_x - 5.5
        } else {
            center_x - 2.0
        };
        let eye_spacing = 10.0_f32;
        let open_factor = (1.0 - self.blink_progress).clamp(0.10, 1.0);
        let ey = center_y + 3.2 + self.look_y;

        if !self.is_cheer_mode {
            // 1. Rambut Spiky Hitam Devil Union
            let black_spikes = [
                (-20.0_f32, -4.5_f32, -24.0_f32, -12.5_f32, -13.0_f32, -9.5_f32),
                (-14.0, -8.5, -15.0, -16.0, -5.0, -9.5),
                (-6.0, -9.5, -2.5, -17.0, 3.0, -9.5),
                (2.0, -9.5, 7.5, -17.0, 12.0, -8.5),
                (9.0, -8.5, 16.5, -15.5, 18.0, -7.0),
                (14.0, -7.0, 24.0, -11.5, 20.0, -3.5),
            ];
            for &(x1, y1, tx, ty, x2, y2) in &black_spikes {
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
                        rt.FillGeometry(&path, &obs_hair_b, None);
                    }
                }
            }

            // 2. Tanduk Hitam Melengkung Devil Union
            for side in [-1.0_f32, 1.0_f32] {
                if let Ok(h_path) = factory.CreatePathGeometry() {
                    if let Ok(sink) = h_path.Open() {
                        let hx = face_cx + side * 14.5;
                        let hy = center_y - 7.5;
                        sink.BeginFigure(D2D_POINT_2F { x: hx, y: hy }, D2D1_FIGURE_BEGIN_FILLED);
                        sink.AddBezier(&D2D1_BEZIER_SEGMENT {
                            point1: D2D_POINT_2F {
                                x: hx + side * 6.5,
                                y: hy - 4.5,
                            },
                            point2: D2D_POINT_2F {
                                x: hx + side * 5.0,
                                y: hy - 11.5,
                            },
                            point3: D2D_POINT_2F {
                                x: hx + side * 1.0,
                                y: hy - 14.5,
                            },
                        });
                        sink.AddBezier(&D2D1_BEZIER_SEGMENT {
                            point1: D2D_POINT_2F {
                                x: hx + side * 1.5,
                                y: hy - 9.0,
                            },
                            point2: D2D_POINT_2F {
                                x: hx - side * 1.5,
                                y: hy - 4.0,
                            },
                            point3: D2D_POINT_2F {
                                x: hx - side * 4.0,
                                y: hy + 0.5,
                            },
                        });
                        sink.EndFigure(D2D1_FIGURE_END_CLOSED);
                        let _ = sink.Close();
                        rt.FillGeometry(&h_path, &obs_horn_b, None);
                        rt.DrawGeometry(&h_path, &crimson_crown_b, 0.9, None);
                    }
                }
            }

            // 3. Aksesori Kepala Salib Merah Devil Union
            self.draw_soft_radial_glow(
                rt,
                D2D_POINT_2F {
                    x: face_cx,
                    y: center_y - 6.5,
                },
                18.0,
                7.0,
                0.95,
                0.15,
                0.28,
                0.36 * alpha,
            );
            if let Ok(crown) = factory.CreatePathGeometry() {
                if let Ok(sink) = crown.Open() {
                    let cy_top = center_y - 7.5;
                    sink.BeginFigure(
                        D2D_POINT_2F {
                            x: face_cx,
                            y: cy_top - 4.5,
                        },
                        D2D1_FIGURE_BEGIN_FILLED,
                    );
                    sink.AddLines(&[
                        D2D_POINT_2F {
                            x: face_cx + 4.5,
                            y: cy_top - 1.2,
                        },
                        D2D_POINT_2F {
                            x: face_cx + 16.5,
                            y: cy_top - 2.0,
                        },
                        D2D_POINT_2F {
                            x: face_cx + 18.5,
                            y: cy_top + 1.2,
                        },
                        D2D_POINT_2F {
                            x: face_cx + 4.0,
                            y: cy_top + 1.2,
                        },
                        D2D_POINT_2F {
                            x: face_cx,
                            y: cy_top + 5.5,
                        },
                        D2D_POINT_2F {
                            x: face_cx - 4.0,
                            y: cy_top + 1.2,
                        },
                        D2D_POINT_2F {
                            x: face_cx - 18.5,
                            y: cy_top + 1.2,
                        },
                        D2D_POINT_2F {
                            x: face_cx - 16.5,
                            y: cy_top - 2.0,
                        },
                        D2D_POINT_2F {
                            x: face_cx - 4.5,
                            y: cy_top - 1.2,
                        },
                    ]);
                    sink.EndFigure(D2D1_FIGURE_END_CLOSED);
                    let _ = sink.Close();
                    rt.FillGeometry(&crown, &crimson_crown_b, None);
                }
            }

            // 4. MATA DEVIL UNION: Desain Persis Sama dengan Mata Normal (Hanya Beda Warna Merah!),
            //    + EKSPRESI SERIUS Saat Mengambil & Menarik Pedang (`grab_reach > 0.05`)!
            let serious_squash = 1.0 - 0.18 * self.grab_reach;
            for side in [-1.0_f32, 1.0_f32] {
                let ex = face_cx + self.look_x * 0.65 + side * eye_spacing;

                self.draw_soft_radial_glow(
                    rt,
                    D2D_POINT_2F { x: ex, y: ey },
                    11.5,
                    12.5,
                    0.98,
                    0.16,
                    0.28,
                    0.60 * alpha,
                );

                // Bentuk Kapsul Persis Mata Normal (`rx = 3.9, ry = 5.2`)
                let rx = 3.9_f32;
                let ry = 5.2_f32 * open_factor * serious_squash;
                rt.FillRoundedRectangle(
                    &D2D1_ROUNDED_RECT {
                        rect: D2D_RECT_F {
                            left: ex - rx,
                            top: ey - ry,
                            right: ex + rx,
                            bottom: ey + ry,
                        },
                        radiusX: rx,
                        radiusY: ry,
                    },
                    &crimson_eye_b,
                );

                let irx = 2.5_f32;
                let iry = 3.6_f32 * open_factor * serious_squash;
                rt.FillRoundedRectangle(
                    &D2D1_ROUNDED_RECT {
                        rect: D2D_RECT_F {
                            left: ex - irx,
                            top: ey - iry,
                            right: ex + irx,
                            bottom: ey + iry,
                        },
                        radiusX: irx,
                        radiusY: iry,
                    },
                    &crimson_core_b,
                );

                // EKSPRESI SERIUS SAAT MENGAMBIL PEDANG (`grab_reach > 0.05`):
                // Potong miring kelopak atas menyudut ke arah tengah (`\` untuk kiri, `/` untuk kanan)
                if self.grab_reach > 0.05 && open_factor > 0.25 {
                    let slant_depth = 3.4 * self.grab_reach * open_factor;
                    let outer_x = ex - side * (rx + 1.2);
                    let inner_x = ex + side * (rx + 1.2);
                    let top_y = ey - ry - 1.5;

                    if let Ok(brow_mask) = factory.CreatePathGeometry() {
                        if let Ok(sink) = brow_mask.Open() {
                            sink.BeginFigure(
                                D2D_POINT_2F {
                                    x: outer_x,
                                    y: top_y,
                                },
                                D2D1_FIGURE_BEGIN_FILLED,
                            );
                            sink.AddLines(&[
                                D2D_POINT_2F {
                                    x: inner_x,
                                    y: top_y,
                                },
                                D2D_POINT_2F {
                                    x: inner_x,
                                    y: top_y + 1.5 + slant_depth,
                                },
                                D2D_POINT_2F {
                                    x: outer_x,
                                    y: top_y + 0.6,
                                },
                            ]);
                            sink.EndFigure(D2D1_FIGURE_END_CLOSED);
                            let _ = sink.Close();
                            rt.FillGeometry(&brow_mask, &bg_mask_b, None);
                        }
                    }
                }
            }

            // 5. PEDANG DEMON-SLAYER MENGAMBANG & DITARIK OLEH TANGAN KECIL BERSINAR
            let float_bob =
                self.sword_float_phase.sin() * 2.2 * (1.0 - self.grab_reach * 0.65);
            let pull_offset_x = -7.5 * self.sword_pull;
            let pull_offset_y = -0.8 * self.sword_pull;

            let sx = center_x + 28.5 + pull_offset_x;
            let sy = center_y + 1.5 + float_bob + pull_offset_y;

            self.draw_soft_radial_glow(
                rt,
                D2D_POINT_2F { x: sx + 2.0, y: sy },
                15.0,
                9.5,
                0.95,
                0.15,
                0.25,
                (0.32 + 0.25 * self.sword_pull) * alpha,
            );

            if let Ok(sword_path) = factory.CreatePathGeometry() {
                if let Ok(sink) = sword_path.Open() {
                    sink.BeginFigure(
                        D2D_POINT_2F {
                            x: sx - 5.5,
                            y: sy - 2.0,
                        },
                        D2D1_FIGURE_BEGIN_FILLED,
                    );
                    sink.AddLines(&[
                        D2D_POINT_2F {
                            x: sx + 9.5,
                            y: sy - 5.4,
                        },
                        D2D_POINT_2F {
                            x: sx + 15.0,
                            y: sy,
                        },
                        D2D_POINT_2F {
                            x: sx + 9.5,
                            y: sy + 5.4,
                        },
                        D2D_POINT_2F {
                            x: sx - 5.5,
                            y: sy + 2.0,
                        },
                    ]);
                    sink.EndFigure(D2D1_FIGURE_END_CLOSED);
                    let _ = sink.Close();
                    rt.FillGeometry(&sword_path, &obs_horn_b, None);
                    rt.DrawGeometry(&sword_path, &crimson_crown_b, 1.2, None);
                }
            }

            let hilt_x = sx - 10.5;
            rt.DrawLine(
                D2D_POINT_2F { x: hilt_x, y: sy },
                D2D_POINT_2F {
                    x: sx + 10.0,
                    y: sy,
                },
                &crimson_crown_b,
                1.5,
                None,
            );
            rt.DrawLine(
                D2D_POINT_2F {
                    x: sx - 5.8,
                    y: sy - 3.6,
                },
                D2D_POINT_2F {
                    x: sx - 5.8,
                    y: sy + 3.6,
                },
                &crimson_crown_b,
                1.8,
                None,
            );

            if self.grab_reach > 0.02 {
                let hand_alpha = (self.grab_reach * 1.4).clamp(0.0, 1.0) * alpha;
                let start_hx = center_x + 11.5;
                let start_hy = center_y + 8.0;
                let grip_hx = hilt_x + 1.5;
                let grip_hy = sy + 0.5;

                let hx = start_hx + (grip_hx - start_hx) * self.grab_reach;
                let hy = start_hy + (grip_hy - start_hy) * self.grab_reach;

                self.draw_soft_radial_glow(
                    rt,
                    D2D_POINT_2F { x: hx, y: hy },
                    8.5,
                    8.5,
                    0.98,
                    0.28,
                    0.38,
                    0.55 * hand_alpha,
                );

                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F { x: hx, y: hy },
                        radiusX: 3.1,
                        radiusY: 2.9,
                    },
                    &crimson_core_b,
                );
            }
        } else {
            // FASE 2: ASTA NORMAL & HAPPY HEALING MODE
            let ash_spikes = [
                (-20.0_f32, -4.5_f32, -24.0_f32, -12.5_f32, -13.0_f32, -9.5_f32),
                (-14.0, -8.5, -15.0, -16.0, -5.0, -9.5),
                (-6.0, -9.5, -2.5, -17.0, 3.0, -9.5),
                (2.0, -9.5, 7.5, -17.0, 12.0, -8.5),
                (9.0, -8.5, 16.5, -15.5, 18.0, -7.0),
                (14.0, -7.0, 24.0, -11.5, 20.0, -3.5),
            ];
            for (i, &(x1, y1, tx, ty, x2, y2)) in ash_spikes.iter().enumerate() {
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
                        let b = if i % 2 == 0 { &ash_light_b } else { &ash_mid_b };
                        rt.FillGeometry(&path, b, None);
                    }
                }
            }

            rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: D2D_RECT_F {
                        left: face_cx - 20.0,
                        top: center_y - 9.8,
                        right: face_cx + 20.0,
                        bottom: center_y - 3.6,
                    },
                    radiusX: 3.0,
                    radiusY: 3.0,
                },
                &obs_hair_b,
            );

            let rx_cross = face_cx - 13.0;
            let ry_cross = center_y - 6.7;
            rt.DrawLine(
                D2D_POINT_2F {
                    x: rx_cross - 2.8,
                    y: ry_cross,
                },
                D2D_POINT_2F {
                    x: rx_cross + 2.8,
                    y: ry_cross,
                },
                &crimson_crown_b,
                1.5,
                None,
            );
            rt.DrawLine(
                D2D_POINT_2F {
                    x: rx_cross,
                    y: ry_cross - 2.5,
                },
                D2D_POINT_2F {
                    x: rx_cross,
                    y: ry_cross + 2.5,
                },
                &crimson_crown_b,
                1.5,
                None,
            );

            let gx = face_cx + 13.5;
            let gy = center_y - 6.7;
            rt.FillEllipse(
                &D2D1_ELLIPSE {
                    point: D2D_POINT_2F { x: gx, y: gy },
                    radiusX: 2.0,
                    radiusY: 2.0,
                },
                &gold_b,
            );

            if let Ok(bang_path) = factory.CreatePathGeometry() {
                if let Ok(sink) = bang_path.Open() {
                    sink.BeginFigure(
                        D2D_POINT_2F {
                            x: face_cx - 2.5,
                            y: center_y - 4.0,
                        },
                        D2D1_FIGURE_BEGIN_FILLED,
                    );
                    sink.AddLines(&[
                        D2D_POINT_2F {
                            x: face_cx,
                            y: center_y + 1.5,
                        },
                        D2D_POINT_2F {
                            x: face_cx + 2.5,
                            y: center_y - 4.0,
                        },
                    ]);
                    sink.EndFigure(D2D1_FIGURE_END_CLOSED);
                    let _ = sink.Close();
                    rt.FillGeometry(&bang_path, &ash_light_b, None);
                }
            }

            let is_happy_arch_subphase = self.mode_aura_timer < 4.5;
            let bounce_y = (self.phase_time * 4.5).sin() * 0.7;
            let happy_cy = ey + bounce_y;

            for side in [-1.0_f32, 1.0_f32] {
                let ex = face_cx + side * eye_spacing;

                self.draw_soft_radial_glow(
                    rt,
                    D2D_POINT_2F { x: ex, y: happy_cy },
                    11.5,
                    12.0,
                    0.24,
                    0.95,
                    0.56,
                    0.58 * alpha,
                );

                if is_happy_arch_subphase {
                    if let Ok(arch_path) = factory.CreatePathGeometry() {
                        if let Ok(sink) = arch_path.Open() {
                            sink.BeginFigure(
                                D2D_POINT_2F {
                                    x: ex - 4.4,
                                    y: happy_cy + 2.2,
                                },
                                D2D1_FIGURE_BEGIN_HOLLOW,
                            );
                            sink.AddBezier(&D2D1_BEZIER_SEGMENT {
                                point1: D2D_POINT_2F {
                                    x: ex - 2.1,
                                    y: happy_cy - 4.0,
                                },
                                point2: D2D_POINT_2F {
                                    x: ex + 2.1,
                                    y: happy_cy - 4.0,
                                },
                                point3: D2D_POINT_2F {
                                    x: ex + 4.4,
                                    y: happy_cy + 2.2,
                                },
                            });
                            sink.EndFigure(D2D1_FIGURE_END_OPEN);
                            let _ = sink.Close();

                            rt.DrawGeometry(
                                &arch_path,
                                &green_glow_b,
                                6.2,
                                round_stroke.as_ref(),
                            );
                            rt.DrawGeometry(
                                &arch_path,
                                &green_eye_b,
                                3.6,
                                round_stroke.as_ref(),
                            );
                            rt.DrawGeometry(
                                &arch_path,
                                &green_core_b,
                                1.8,
                                round_stroke.as_ref(),
                            );
                        }
                    }
                } else {
                    let rx = 3.9_f32;
                    let ry = 5.2_f32 * open_factor;
                    let lx = ex + self.look_x * 0.65;
                    rt.FillRoundedRectangle(
                        &D2D1_ROUNDED_RECT {
                            rect: D2D_RECT_F {
                                left: lx - rx,
                                top: happy_cy - ry,
                                right: lx + rx,
                                bottom: happy_cy + ry,
                            },
                            radiusX: rx,
                            radiusY: ry,
                        },
                        &green_eye_b,
                    );
                    let irx = 2.5_f32;
                    let iry = 3.6_f32 * open_factor;
                    rt.FillRoundedRectangle(
                        &D2D1_ROUNDED_RECT {
                            rect: D2D_RECT_F {
                                left: lx - irx,
                                top: happy_cy - iry,
                                right: lx + irx,
                                bottom: happy_cy + iry,
                            },
                            radiusX: irx,
                            radiusY: iry,
                        },
                        &green_core_b,
                    );
                }
            }

            let hand_wave = (self.phase_time * 6.0).sin() * 1.4;
            let hx = face_cx + 18.5;
            let hy = happy_cy + 2.2 + hand_wave;

            self.draw_soft_radial_glow(
                rt,
                D2D_POINT_2F { x: hx, y: hy },
                8.5,
                8.5,
                0.24,
                0.95,
                0.56,
                0.55 * alpha,
            );
            rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: D2D_RECT_F {
                        left: hx - 3.0,
                        top: hy - 3.2,
                        right: hx + 3.0,
                        bottom: hy + 3.2,
                    },
                    radiusX: 2.6,
                    radiusY: 2.6,
                },
                &green_core_b,
            );
        }
    }
}
