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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MascotPhase {
    /// 0.0s .. 7.0s: Mata Devil Union (Mahkota Salib Merah + Garis Vertikal + Iris Merah Bintang + Pedang Melayang)
    DevilUnion,
    /// 7.0s .. 11.5s: Mata Happy Berbinar Bintang Kuning (Saat sedang di-heal dengan iluminasi hijau lembut)
    HappySparkle,
    /// 11.5s .. 18.0s: Mata Normal Hijau Khas Asta (Setelah selesai di-heal, background kembali hitam normal)
    NormalGreen,
}

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
    pub current_phase: MascotPhase,
    pub phase_local_timer: f32,
    pub sword_float_phase: f32,
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
            current_phase: MascotPhase::DevilUnion,
            phase_local_timer: 0.0,
            sword_float_phase: 0.0,
        }
    }

    pub fn trigger_awakening(&mut self) {
        self.flare_intensity = 1.8;
        self.blink_progress = 1.0;
        self.phase_local_timer = 0.0;
    }

    pub fn trigger_wake_surge(&mut self) {
        self.flare_intensity = 2.2;
    }

    /// Menghitung intensitas iluminasi background yang halus & tidak lebay (0.0 .. 1.0):
    /// - DevilUnion: menyala lembut selama 3.8 detik pertama lalu meredup kembali ke hitam pekat (0.0)
    /// - HappySparkle: menyala lembut selama proses healing (4.5 detik) lalu meredup saat masuk ke NormalGreen
    /// - NormalGreen: 0.0 (background hitam normal sepenuhnya)
    pub fn current_illumination_envelope(&self) -> f32 {
        match self.current_phase {
            MascotPhase::DevilUnion => {
                let dur = 3.8_f32;
                if self.phase_local_timer >= dur {
                    0.0
                } else if self.phase_local_timer < 0.7 {
                    (self.phase_local_timer / 0.7).clamp(0.0, 1.0)
                } else if self.phase_local_timer > dur - 1.2 {
                    ((dur - self.phase_local_timer) / 1.2).clamp(0.0, 1.0)
                } else {
                    1.0
                }
            }
            MascotPhase::HappySparkle => {
                let dur = 4.5_f32;
                if self.phase_local_timer < 0.6 {
                    (self.phase_local_timer / 0.6).clamp(0.0, 1.0)
                } else if self.phase_local_timer > dur - 1.0 {
                    ((dur - self.phase_local_timer) / 1.0).clamp(0.0, 1.0)
                } else {
                    1.0
                }
            }
            MascotPhase::NormalGreen => 0.0,
        }
    }

    pub fn update(&mut self, dt: f32, cursor_offset: Option<(f32, f32)>) {
        self.phase_time += dt;
        self.idle_cycle_timer += dt;
        self.phase_local_timer += dt;
        self.sword_float_phase += dt * 3.0;

        // Siklus 18.0 detik yang terpisah menjadi 3 fase:
        // 0.0 .. 7.0s   -> Fase 1: Devil Union Mode + Pedang Melayang (Gambar 2)
        // 7.0 .. 11.5s  -> Fase 2: Happy Starry-Eyed Sparkle + Healing (Gambar 1)
        // 11.5 .. 18.0s -> Fase 3: Normal Emerald-Green Eyes (Gambar 3, selesai di-heal)
        let cycle = self.idle_cycle_timer % 18.0;
        let next_phase = if cycle < 7.0 {
            MascotPhase::DevilUnion
        } else if cycle < 11.5 {
            MascotPhase::HappySparkle
        } else {
            MascotPhase::NormalGreen
        };

        if next_phase != self.current_phase {
            self.current_phase = next_phase;
            self.phase_local_timer = 0.0;
            self.blink_progress = 0.90; // Kedip halus saat berganti ekspresi
        }

        if let Some((dx, dy)) = cursor_offset {
            self.target_look_x = (dx / 180.0).clamp(-1.0, 1.0) * 2.6;
            self.target_look_y = (dy / 100.0).clamp(-1.0, 1.0) * 1.2;
        } else {
            self.target_look_x = (self.phase_time * 0.85).sin() * 1.4;
            self.target_look_y = (self.phase_time * 0.55).cos() * 0.5;
        }

        let follow = (dt * 10.0).min(1.0);
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
                self.blink_timer = 2.8 + (self.phase_time.sin().abs() * 2.2);
            }
        }
    }

    /// Helper: Menggambar Soft Blurry Ambient Illumination menggunakan RadialGradientBrush
    /// (Tanpa tepi lingkaran solid sama sekali — menyebar halus seperti backlit glow!)
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
                position: 0.42,
                color: D2D1_COLOR_F {
                    r: core_r * 0.8,
                    g: core_g * 0.8,
                    b: core_b * 0.8,
                    a: peak_alpha * 0.45,
                },
            },
            D2D1_GRADIENT_STOP {
                position: 0.78,
                color: D2D1_COLOR_F {
                    r: core_r * 0.5,
                    g: core_g * 0.5,
                    b: core_b * 0.5,
                    a: peak_alpha * 0.12,
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

    /// Menggambar Iluminasi Background Blurry & Partikel HD (Crimson Thunder / Healing Glints)
    pub unsafe fn draw_idle_aura_background(
        &self,
        rt: &ID2D1HwndRenderTarget,
        factory: &ID2D1Factory,
        pill_rect: D2D_RECT_F,
        alpha: f32,
    ) {
        rt.SetAntialiasMode(D2D1_ANTIALIAS_MODE_PER_PRIMITIVE);

        let env = self.current_illumination_envelope() * alpha;
        if env <= 0.01 {
            return;
        }

        let pw = pill_rect.right - pill_rect.left;
        let ph = pill_rect.bottom - pill_rect.top;
        let cx = (pill_rect.left + pill_rect.right) * 0.5;
        let cy = (pill_rect.top + pill_rect.bottom) * 0.5;

        match self.current_phase {
            MascotPhase::DevilUnion => {
                // 1. SOFT BLURRY CRIMSON BACKLIGHT ILLUMINATION (Bukan lingkaran solid!)
                let drift_x = (self.phase_time * 2.2).sin() * 14.0;
                let breathe = 0.82 + 0.18 * (self.phase_time * 4.5).sin();
                self.draw_soft_radial_illumination(
                    rt,
                    D2D_POINT_2F {
                        x: cx + drift_x,
                        y: cy + 2.0,
                    },
                    pw * 0.48,
                    ph * 0.95,
                    0.78,
                    0.08,
                    0.16,
                    0.34 * env * breathe,
                );

                // 2. HD 2-LAYER CRIMSON THUNDER SPARKS (Outer Soft Glow + Inner Crisp Core)
                let flash_wave =
                    ((self.phase_time * 11.0).sin() + (self.phase_time * 19.0).cos()) * 0.5;
                if flash_wave > -0.10 {
                    let spark_alpha = ((flash_wave + 0.15) * 1.1).clamp(0.2, 0.95) * env;
                    let glow_col = D2D1_COLOR_F {
                        r: 0.90,
                        g: 0.10,
                        b: 0.20,
                        a: spark_alpha * 0.38,
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
                            let shift = (self.phase_time * 14.0 + i as f32 * 2.5).sin() * 5.0;
                            let bx = cx + ax + shift;
                            let by = pill_rect.top + 6.0;

                            if let Ok(path) = factory.CreatePathGeometry() {
                                if let Ok(sink) = path.Open() {
                                    sink.BeginFigure(
                                        D2D_POINT_2F {
                                            x: bx - 7.0,
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
                                            x: bx - 4.5,
                                            y: by + 12.0,
                                        },
                                        D2D_POINT_2F {
                                            x: bx + 3.5,
                                            y: by + 21.0,
                                        },
                                        D2D_POINT_2F {
                                            x: bx + 0.5,
                                            y: by + 14.5,
                                        },
                                        D2D_POINT_2F {
                                            x: bx + 8.0,
                                            y: by + 23.0,
                                        },
                                    ]);
                                    sink.EndFigure(D2D1_FIGURE_END_OPEN);
                                    let _ = sink.Close();
                                    // Lapisan 1: Blurry Outer Glow
                                    rt.DrawGeometry(&path, &glow_b, 3.4, None);
                                    // Lapisan 2: Crisp HD Core
                                    rt.DrawGeometry(&path, &core_b, 1.1, None);
                                }
                            }
                        }
                    }
                }
            }
            MascotPhase::HappySparkle => {
                // 1. SOFT BLURRY EMERALD HEALING ILLUMINATION (Lembut & Natural)
                let drift_x = (self.phase_time * 1.8).sin() * 12.0;
                let breathe = 0.85 + 0.15 * (self.phase_time * 3.2).sin();
                self.draw_soft_radial_illumination(
                    rt,
                    D2D_POINT_2F {
                        x: cx + drift_x,
                        y: cy + 3.0,
                    },
                    pw * 0.46,
                    ph * 0.92,
                    0.08,
                    0.72,
                    0.46,
                    0.30 * env * breathe,
                );

                // 2. HD HEALING PARTICLES (Feathered Micro-Motes & Crisp '+' Glints)
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
                        let speed = 11.0 + (i as f32 * 2.0);
                        let progress = ((self.phase_time * speed + i as f32 * 6.0) % (ph - 10.0))
                            / (ph - 10.0).max(1.0);
                        let py = pill_rect.bottom - 5.0 - progress * (ph - 10.0);
                        let px =
                            cx + rel_x * pw + (self.phase_time * 2.5 + i as f32).sin() * 2.5;

                        // Fade-out lembut di ujung atas & bawah
                        let fade = (progress * std::f32::consts::PI).sin().clamp(0.0, 1.0);
                        if fade > 0.1 {
                            if i % 2 == 0 {
                                let arm = 2.4_f32;
                                rt.DrawLine(
                                    D2D_POINT_2F { x: px - arm, y: py },
                                    D2D_POINT_2F { x: px + arm, y: py },
                                    &halo_b,
                                    2.8,
                                    None,
                                );
                                rt.DrawLine(
                                    D2D_POINT_2F { x: px, y: py - arm },
                                    D2D_POINT_2F { x: px, y: py + arm },
                                    &halo_b,
                                    2.8,
                                    None,
                                );
                                rt.DrawLine(
                                    D2D_POINT_2F { x: px - arm, y: py },
                                    D2D_POINT_2F { x: px + arm, y: py },
                                    &core_b,
                                    1.1,
                                    None,
                                );
                                rt.DrawLine(
                                    D2D_POINT_2F { x: px, y: py - arm },
                                    D2D_POINT_2F { x: px, y: py + arm },
                                    &core_b,
                                    1.1,
                                    None,
                                );
                            } else {
                                self.draw_soft_radial_illumination(
                                    rt,
                                    D2D_POINT_2F { x: px, y: py },
                                    4.5,
                                    4.5,
                                    0.45,
                                    0.96,
                                    0.76,
                                    0.75 * env * fade,
                                );
                            }
                        }
                    }
                }
            }
            MascotPhase::NormalGreen => {}
        }
    }

    /// Helper: Menggambar Bintang 4-Sudut Tajam HD (Untuk Mata Sparkle Kuning & Pupil Devil Union)
    unsafe fn draw_four_point_star(
        &self,
        rt: &ID2D1HwndRenderTarget,
        factory: &ID2D1Factory,
        cx: f32,
        cy: f32,
        rx: f32,
        ry: f32,
        brush: &windows::Win32::Graphics::Direct2D::ID2D1SolidColorBrush,
    ) {
        if let Ok(path) = factory.CreatePathGeometry() {
            if let Ok(sink) = path.Open() {
                let irx = rx * 0.28;
                let iry = ry * 0.28;
                sink.BeginFigure(
                    D2D_POINT_2F { x: cx, y: cy - ry },
                    D2D1_FIGURE_BEGIN_FILLED,
                );
                sink.AddLines(&[
                    D2D_POINT_2F {
                        x: cx + irx,
                        y: cy - iry,
                    },
                    D2D_POINT_2F { x: cx + rx, y: cy },
                    D2D_POINT_2F {
                        x: cx + irx,
                        y: cy + iry,
                    },
                    D2D_POINT_2F { x: cx, y: cy + ry },
                    D2D_POINT_2F {
                        x: cx - irx,
                        y: cy + iry,
                    },
                    D2D_POINT_2F { x: cx - rx, y: cy },
                    D2D_POINT_2F {
                        x: cx - irx,
                        y: cy - iry,
                    },
                ]);
                sink.EndFigure(D2D1_FIGURE_END_CLOSED);
                let _ = sink.Close();
                rt.FillGeometry(&path, brush, None);
            }
        }
    }

    /// Menggambar Maskot Asta Chibi Eyes-Only HD (Tanpa Blok Wajah Penuh!)
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

        let eye_spacing = 15.5_f32;
        let open_factor = (1.0 - self.blink_progress).clamp(0.08, 1.0);

        let obsidian_dark = D2D1_COLOR_F {
            r: 0.07,
            g: 0.08,
            b: 0.11,
            a: alpha,
        };
        let headband_slate = D2D1_COLOR_F {
            r: 0.12,
            g: 0.14,
            b: 0.18,
            a: alpha,
        };
        let hair_ash_light = D2D1_COLOR_F {
            r: 0.82,
            g: 0.85,
            b: 0.88,
            a: alpha,
        };
        let hair_ash_shade = D2D1_COLOR_F {
            r: 0.60,
            g: 0.64,
            b: 0.69,
            a: alpha,
        };
        let crimson_vivid = D2D1_COLOR_F {
            r: 0.92,
            g: 0.11,
            b: 0.27,
            a: alpha,
        };
        let crimson_deep = D2D1_COLOR_F {
            r: 0.58,
            g: 0.05,
            b: 0.15,
            a: alpha,
        };
        let gold_star = D2D1_COLOR_F {
            r: 0.99,
            g: 0.90,
            b: 0.22,
            a: alpha,
        };
        let emerald_outer = D2D1_COLOR_F {
            r: 0.13,
            g: 0.62,
            b: 0.28,
            a: alpha,
        };
        let emerald_mid = D2D1_COLOR_F {
            r: 0.32,
            g: 0.85,
            b: 0.42,
            a: alpha,
        };
        let emerald_glow = D2D1_COLOR_F {
            r: 0.58,
            g: 0.96,
            b: 0.64,
            a: alpha,
        };
        let sclera_white = D2D1_COLOR_F {
            r: 0.97,
            g: 0.98,
            b: 1.00,
            a: alpha,
        };
        let shade_grey = D2D1_COLOR_F {
            r: 0.72,
            g: 0.76,
            b: 0.82,
            a: alpha * 0.75,
        };

        let Ok(obsidian_b) = rt.CreateSolidColorBrush(&obsidian_dark, None) else { return };
        let Ok(band_b) = rt.CreateSolidColorBrush(&headband_slate, None) else { return };
        let Ok(ash_light_b) = rt.CreateSolidColorBrush(&hair_ash_light, None) else { return };
        let Ok(ash_shade_b) = rt.CreateSolidColorBrush(&hair_ash_shade, None) else { return };
        let Ok(crimson_b) = rt.CreateSolidColorBrush(&crimson_vivid, None) else { return };
        let Ok(crimson_deep_b) = rt.CreateSolidColorBrush(&crimson_deep, None) else { return };
        let Ok(gold_b) = rt.CreateSolidColorBrush(&gold_star, None) else { return };
        let Ok(em_outer_b) = rt.CreateSolidColorBrush(&emerald_outer, None) else { return };
        let Ok(em_mid_b) = rt.CreateSolidColorBrush(&emerald_mid, None) else { return };
        let Ok(em_glow_b) = rt.CreateSolidColorBrush(&emerald_glow, None) else { return };
        let Ok(sclera_b) = rt.CreateSolidColorBrush(&sclera_white, None) else { return };
        let Ok(shade_b) = rt.CreateSolidColorBrush(&shade_grey, None) else { return };

        let ey = center_y + 3.2;

        // =====================================================================
        // BAGIAN 1: MAHKOTA DEVIL UNION ATAU IKAT KEPALA BLACK BULLS + PONI CHIBI
        // =====================================================================
        if self.current_phase == MascotPhase::DevilUnion {
            // 1A. Tanduk Melengkung Devil Union di Kiri & Kanan Atas (Gambar 2)
            for side in [-1.0_f32, 1.0_f32] {
                if let Ok(horn) = factory.CreatePathGeometry() {
                    if let Ok(sink) = horn.Open() {
                        let hx = center_x + side * 24.0;
                        let hy = center_y - 4.0;
                        sink.BeginFigure(D2D_POINT_2F { x: hx, y: hy }, D2D1_FIGURE_BEGIN_FILLED);
                        sink.AddBezier(&D2D1_BEZIER_SEGMENT {
                            point1: D2D_POINT_2F {
                                x: hx + side * 7.0,
                                y: hy - 4.0,
                            },
                            point2: D2D_POINT_2F {
                                x: hx + side * 6.0,
                                y: hy - 11.0,
                            },
                            point3: D2D_POINT_2F {
                                x: hx + side * 1.5,
                                y: hy - 14.5,
                            },
                        });
                        sink.AddBezier(&D2D1_BEZIER_SEGMENT {
                            point1: D2D_POINT_2F {
                                x: hx + side * 1.0,
                                y: hy - 9.0,
                            },
                            point2: D2D_POINT_2F {
                                x: hx - side * 2.0,
                                y: hy - 4.0,
                            },
                            point3: D2D_POINT_2F {
                                x: hx - side * 4.5,
                                y: hy + 1.0,
                            },
                        });
                        sink.EndFigure(D2D1_FIGURE_END_CLOSED);
                        let _ = sink.Close();
                        rt.FillGeometry(&horn, &band_b, None);
                        rt.DrawGeometry(&horn, &crimson_b, 0.9, None);
                    }
                }
            }

            // 1B. Mahkota Salib Merah Devil Union (Sesuai Gambar 2: `image_3f6c9a.png`)
            if let Ok(crown) = factory.CreatePathGeometry() {
                if let Ok(sink) = crown.Open() {
                    let cy_top = center_y - 8.5;
                    sink.BeginFigure(
                        D2D_POINT_2F {
                            x: center_x,
                            y: cy_top - 6.0,
                        },
                        D2D1_FIGURE_BEGIN_FILLED,
                    );
                    sink.AddLines(&[
                        D2D_POINT_2F {
                            x: center_x + 7.0,
                            y: cy_top - 1.5,
                        },
                        D2D_POINT_2F {
                            x: center_x + 21.0,
                            y: cy_top - 3.0,
                        },
                        D2D_POINT_2F {
                            x: center_x + 24.0,
                            y: cy_top + 1.5,
                        },
                        D2D_POINT_2F {
                            x: center_x + 6.0,
                            y: cy_top + 1.5,
                        },
                        // Ujung salib merah meruncing ke tengah dahi di antara kedua mata!
                        D2D_POINT_2F {
                            x: center_x,
                            y: cy_top + 8.5,
                        },
                        D2D_POINT_2F {
                            x: center_x - 6.0,
                            y: cy_top + 1.5,
                        },
                        D2D_POINT_2F {
                            x: center_x - 24.0,
                            y: cy_top + 1.5,
                        },
                        D2D_POINT_2F {
                            x: center_x - 21.0,
                            y: cy_top - 3.0,
                        },
                        D2D_POINT_2F {
                            x: center_x - 7.0,
                            y: cy_top - 1.5,
                        },
                    ]);
                    sink.EndFigure(D2D1_FIGURE_END_CLOSED);
                    let _ = sink.Close();
                    rt.FillGeometry(&crown, &crimson_b, None);
                    rt.DrawGeometry(&crown, &obsidian_b, 1.2, None);
                }
            }
        } else {
            // 1C. Ikat Kepala Hitam Khas Asta (Gambar 1 & Gambar 3)
            let band_rect = D2D_RECT_F {
                left: center_x - 26.0,
                top: center_y - 12.5,
                right: center_x + 26.0,
                bottom: center_y - 4.5,
            };
            rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: band_rect,
                    radiusX: 4.0,
                    radiusY: 4.0,
                },
                &band_b,
            );

            // Tanda Silang Merah Bergerigi 3 di Kiri Ikat Kepala (Persis Gambar 3 `image_3f73a0.png`)
            let rx = center_x - 17.0;
            let ry = center_y - 8.5;
            rt.DrawLine(
                D2D_POINT_2F {
                    x: rx - 5.5,
                    y: ry + 1.5,
                },
                D2D_POINT_2F {
                    x: rx + 6.5,
                    y: ry - 1.5,
                },
                &crimson_b,
                2.0,
                None,
            );
            for offset in [-2.5_f32, 0.5, 3.2] {
                rt.DrawLine(
                    D2D_POINT_2F {
                        x: rx + offset,
                        y: ry - 3.2,
                    },
                    D2D_POINT_2F {
                        x: rx + offset - 0.8,
                        y: ry + 2.8,
                    },
                    &crimson_b,
                    1.6,
                    None,
                );
            }

            // Emblem Emas Black Bulls di Kanan Ikat Kepala (Gambar 1 & 3)
            let gx = center_x + 18.5;
            let gy = center_y - 8.5;
            rt.FillEllipse(
                &D2D1_ELLIPSE {
                    point: D2D_POINT_2F { x: gx, y: gy },
                    radiusX: 2.6,
                    radiusY: 2.6,
                },
                &gold_b,
            );
            rt.DrawLine(
                D2D_POINT_2F {
                    x: gx - 3.2,
                    y: gy - 2.4,
                },
                D2D_POINT_2F {
                    x: gx + 3.2,
                    y: gy - 2.4,
                },
                &gold_b,
                1.2,
                None,
            );

            // Helaian Poni Rambut Perak-Abu (Ash-Silver Bangs) di Antara & Tepi Mata
            let bangs = [
                (-3.5_f32, -5.0_f32, 0.0_f32, 4.5_f32, 3.5_f32, -5.0_f32), // Poni tengah turun di antara mata
                (-26.0, -5.0, -29.0, 2.5, -22.0, -3.5),                    // Poni kiri
                (22.0, -3.5, 29.0, 2.5, 26.0, -5.0),                       // Poni kanan
            ];
            for (i, &(x1, y1, tx, ty, x2, y2)) in bangs.iter().enumerate() {
                if let Ok(bpath) = factory.CreatePathGeometry() {
                    if let Ok(sink) = bpath.Open() {
                        sink.BeginFigure(
                            D2D_POINT_2F {
                                x: center_x + x1,
                                y: center_y + y1,
                            },
                            D2D1_FIGURE_BEGIN_FILLED,
                        );
                        sink.AddLines(&[
                            D2D_POINT_2F {
                                x: center_x + tx,
                                y: center_y + ty,
                            },
                            D2D_POINT_2F {
                                x: center_x + x2,
                                y: center_y + y2,
                            },
                        ]);
                        sink.EndFigure(D2D1_FIGURE_END_CLOSED);
                        let _ = sink.Close();
                        rt.FillGeometry(
                            &bpath,
                            if i == 0 { &ash_light_b } else { &ash_shade_b },
                            None,
                        );
                    }
                }
            }
        }

        // =====================================================================
        // BAGIAN 2: MATA CHIBI HD SESUAI 3 REFERENSI GAMBAR
        // =====================================================================
        for side in [-1.0_f32, 1.0_f32] {
            let ex = center_x + side * eye_spacing;

            match self.current_phase {
                MascotPhase::DevilUnion => {
                    // ---------------------------------------------------------
                    // MATA FASE 1: DEVIL UNION (Gambar 2 `image_3f6c9a.png`)
                    // Garis Anti-Magic vertikal + Sklera tajam + Iris Merah + Pupil Bintang 4-Sudut
                    // ---------------------------------------------------------
                    // Garis Tanda Hitam Anti-Magic Vertikal di Atas & Bawah Mata
                    rt.DrawLine(
                        D2D_POINT_2F {
                            x: ex - side * 1.5,
                            y: ey - 8.5,
                        },
                        D2D_POINT_2F {
                            x: ex + side * 0.8,
                            y: ey + 8.5,
                        },
                        &band_b,
                        3.0,
                        None,
                    );

                    // Sklera Putih Tajam
                    let eye_rx = 6.8_f32;
                    let eye_ry = 4.3_f32 * open_factor;
                    rt.FillEllipse(
                        &D2D1_ELLIPSE {
                            point: D2D_POINT_2F { x: ex, y: ey },
                            radiusX: eye_rx,
                            radiusY: eye_ry,
                        },
                        &sclera_b,
                    );

                    if open_factor > 0.25 {
                        let px = ex + self.look_x * 0.55;
                        let py = ey + self.look_y * 0.55;

                        // Ring luar iris merah tua + inti iris merah menyala
                        rt.FillEllipse(
                            &D2D1_ELLIPSE {
                                point: D2D_POINT_2F { x: px, y: py },
                                radiusX: 4.0,
                                radiusY: 3.8 * open_factor,
                            },
                            &crimson_deep_b,
                        );
                        rt.FillEllipse(
                            &D2D1_ELLIPSE {
                                point: D2D_POINT_2F { x: px, y: py },
                                radiusX: 3.1,
                                radiusY: 3.0 * open_factor,
                            },
                            &crimson_b,
                        );

                        // Pupil Bintang 4-Sudut Hitam di dalam Iris Merah (Persis Gambar 2)
                        self.draw_four_point_star(
                            rt,
                            factory,
                            px,
                            py,
                            2.4,
                            2.4 * open_factor,
                            &obsidian_b,
                        );

                        // Titik kilau putih kecil
                        rt.FillEllipse(
                            &D2D1_ELLIPSE {
                                point: D2D_POINT_2F {
                                    x: px - 1.3,
                                    y: py - 1.2 * open_factor,
                                },
                                radiusX: 0.9,
                                radiusY: 0.9,
                            },
                            &sclera_b,
                        );
                    }

                    // Garis Alis & Kelopak Atas Tajam Menyudut ke Tengah
                    rt.DrawLine(
                        D2D_POINT_2F {
                            x: ex - side * 6.5,
                            y: ey - 2.0,
                        },
                        D2D_POINT_2F {
                            x: ex + side * 6.8,
                            y: ey - 5.2,
                        },
                        &obsidian_b,
                        2.2,
                        None,
                    );
                }

                MascotPhase::HappySparkle => {
                    // ---------------------------------------------------------
                    // MATA FASE 2: HAPPY STARRY-EYED SPARKLE (Gambar 1 `image_3f02e7.png`)
                    // Sklera dengan arsir miring + Pupil Hitam Besar + 2 Garis Kilau Putih + Bintang Emas Besar
                    // ---------------------------------------------------------
                    let eye_rx = 7.2_f32;
                    let eye_ry = 5.4_f32 * open_factor;

                    // Sklera Putih Chibi Bulat
                    rt.FillEllipse(
                        &D2D1_ELLIPSE {
                            point: D2D_POINT_2F { x: ex, y: ey },
                            radiusX: eye_rx,
                            radiusY: eye_ry,
                        },
                        &sclera_b,
                    );

                    if open_factor > 0.25 {
                        // Garis arsir anime tipis di tepi luar sklera (ciri khas Gambar 1)
                        for s_idx in -1..=1 {
                            let sy = ey + s_idx as f32 * 2.0;
                            rt.DrawLine(
                                D2D_POINT_2F {
                                    x: ex - eye_rx + 0.8,
                                    y: sy,
                                },
                                D2D_POINT_2F {
                                    x: ex + eye_rx - 0.8,
                                    y: sy,
                                },
                                &shade_b,
                                0.8,
                                None,
                            );
                        }

                        let px = ex + self.look_x * 0.35;
                        let py = ey + self.look_y * 0.35;

                        // Blok Pupil Hitam Besar di Tengah
                        rt.FillEllipse(
                            &D2D1_ELLIPSE {
                                point: D2D_POINT_2F { x: px, y: py },
                                radiusX: 4.9,
                                radiusY: 4.8 * open_factor,
                            },
                            &obsidian_b,
                        );

                        // 2 Batang Kilau Putih Vertikal di Sisi Kiri Pupil (Persis Gambar 1!)
                        rt.FillRoundedRectangle(
                            &D2D1_ROUNDED_RECT {
                                rect: D2D_RECT_F {
                                    left: px - 2.8,
                                    top: py - 3.4 * open_factor,
                                    right: px - 1.3,
                                    bottom: py - 0.3 * open_factor,
                                },
                                radiusX: 0.7,
                                radiusY: 0.7,
                            },
                            &sclera_b,
                        );
                        rt.FillRoundedRectangle(
                            &D2D1_ROUNDED_RECT {
                                rect: D2D_RECT_F {
                                    left: px - 2.8,
                                    top: py + 0.6 * open_factor,
                                    right: px - 1.3,
                                    bottom: py + 3.2 * open_factor,
                                },
                                radiusX: 0.7,
                                radiusY: 0.7,
                            },
                            &sclera_b,
                        );

                        // Bintang Kuning Emas 4-Sudut Berkilau di Sisi Kanan Pupil (Persis Gambar 1!)
                        let star_pulse = 1.0 + 0.16 * (self.phase_time * 8.0).sin();
                        self.draw_four_point_star(
                            rt,
                            factory,
                            px + 1.4,
                            py + 0.4,
                            3.6 * star_pulse,
                            4.1 * star_pulse * open_factor,
                            &gold_b,
                        );
                    }

                    // Kelopak Atas Lengkung + 2 Bulu Mata Chibi di Atas (Gambar 1)
                    rt.DrawLine(
                        D2D_POINT_2F {
                            x: ex - 6.6,
                            y: ey - 4.4 * open_factor,
                        },
                        D2D_POINT_2F {
                            x: ex + 6.6,
                            y: ey - 3.8 * open_factor,
                        },
                        &obsidian_b,
                        1.8,
                        None,
                    );
                    rt.DrawLine(
                        D2D_POINT_2F {
                            x: ex - 1.5,
                            y: ey - 4.6 * open_factor,
                        },
                        D2D_POINT_2F {
                            x: ex - 2.2,
                            y: ey - 6.8 * open_factor,
                        },
                        &obsidian_b,
                        1.3,
                        None,
                    );
                    rt.DrawLine(
                        D2D_POINT_2F {
                            x: ex + 2.5,
                            y: ey - 4.2 * open_factor,
                        },
                        D2D_POINT_2F {
                            x: ex + 3.6,
                            y: ey - 6.4 * open_factor,
                        },
                        &obsidian_b,
                        1.3,
                        None,
                    );
                }

                MascotPhase::NormalGreen => {
                    // ---------------------------------------------------------
                    // MATA FASE 3: NORMAL EMERALD-GREEN EYES (Gambar 3 `image_3f73a0.png`)
                    // Sklera Putih Jernih + Iris Hijau Zamrud Berlapis + Pupil Hitam + Catchlight Putih
                    // ---------------------------------------------------------
                    let eye_rx = 7.0_f32;
                    let eye_ry = 5.0_f32 * open_factor;

                    // Sklera Putih Bersih
                    rt.FillEllipse(
                        &D2D1_ELLIPSE {
                            point: D2D_POINT_2F { x: ex, y: ey },
                            radiusX: eye_rx,
                            radiusY: eye_ry,
                        },
                        &sclera_b,
                    );

                    if open_factor > 0.25 {
                        let px = ex + self.look_x * 0.55;
                        let py = ey + self.look_y * 0.55;

                        // Lapisan 1: Ring Luar Hijau Zamrud Tua
                        rt.FillEllipse(
                            &D2D1_ELLIPSE {
                                point: D2D_POINT_2F { x: px, y: py },
                                radiusX: 3.8,
                                radiusY: 4.0 * open_factor,
                            },
                            &em_outer_b,
                        );
                        // Lapisan 2: Iris Tengah Hijau Cerah Khas Asta
                        rt.FillEllipse(
                            &D2D1_ELLIPSE {
                                point: D2D_POINT_2F { x: px, y: py },
                                radiusX: 3.1,
                                radiusY: 3.3 * open_factor,
                            },
                            &em_mid_b,
                        );
                        // Lapisan 3: Sabit Bawah Hijau Muda Bercahaya
                        rt.FillEllipse(
                            &D2D1_ELLIPSE {
                                point: D2D_POINT_2F {
                                    x: px,
                                    y: py + 1.0 * open_factor,
                                },
                                radiusX: 2.2,
                                radiusY: 1.8 * open_factor,
                            },
                            &em_glow_b,
                        );
                        // Lapisan 4: Pupil Hitam di Pusat Iris
                        rt.FillEllipse(
                            &D2D1_ELLIPSE {
                                point: D2D_POINT_2F { x: px, y: py },
                                radiusX: 1.5,
                                radiusY: 1.7 * open_factor,
                            },
                            &obsidian_b,
                        );
                        // Lapisan 5: Catchlight Putih Bulat Jernih di Kiri Atas Iris (Gambar 3)
                        rt.FillEllipse(
                            &D2D1_ELLIPSE {
                                point: D2D_POINT_2F {
                                    x: px - 1.3,
                                    y: py - 1.4 * open_factor,
                                },
                                radiusX: 1.25,
                                radiusY: 1.25,
                            },
                            &sclera_b,
                        );
                    }

                    // Garis Kelopak Atas Tegas (Gambar 3)
                    rt.DrawLine(
                        D2D_POINT_2F {
                            x: ex - 6.6,
                            y: ey - 3.8 * open_factor,
                        },
                        D2D_POINT_2F {
                            x: ex + 6.6,
                            y: ey - 4.4 * open_factor,
                        },
                        &obsidian_b,
                        2.0,
                        None,
                    );
                }
            }
        }

        // =====================================================================
        // BAGIAN 3: PEDANG DEMON-SLAYER MELAYANG (Khusus Saat Mode Devil Union)
        // =====================================================================
        if self.current_phase == MascotPhase::DevilUnion {
            let sword_bob = self.sword_float_phase.sin() * 2.2;
            let sx = center_x + 44.0;
            let sy = center_y + 1.0 + sword_bob * 0.5;

            if let Ok(sword_path) = factory.CreatePathGeometry() {
                if let Ok(sink) = sword_path.Open() {
                    sink.BeginFigure(
                        D2D_POINT_2F {
                            x: sx - 9.0,
                            y: sy - 2.2,
                        },
                        D2D1_FIGURE_BEGIN_FILLED,
                    );
                    sink.AddLines(&[
                        D2D_POINT_2F {
                            x: sx + 11.0,
                            y: sy - 6.8,
                        },
                        D2D_POINT_2F {
                            x: sx + 17.0,
                            y: sy,
                        },
                        D2D_POINT_2F {
                            x: sx + 11.0,
                            y: sy + 6.8,
                        },
                        D2D_POINT_2F {
                            x: sx - 9.0,
                            y: sy + 2.2,
                        },
                    ]);
                    sink.EndFigure(D2D1_FIGURE_END_CLOSED);
                    let _ = sink.Close();

                    rt.FillGeometry(&sword_path, &obsidian_b, None);
                    rt.DrawGeometry(&sword_path, &crimson_b, 1.1, None);
                }
            }

            // Gagang Pedang Anti-Magic
            rt.DrawLine(
                D2D_POINT_2F {
                    x: sx - 14.5,
                    y: sy,
                },
                D2D_POINT_2F {
                    x: sx - 9.0,
                    y: sy,
                },
                &crimson_b,
                2.2,
                None,
            );
        }
    }
}
