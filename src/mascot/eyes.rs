// src/mascot/eyes.rs
use windows::Win32::Graphics::Direct2D::Common::{
    D2D1_BEZIER_SEGMENT, D2D1_COLOR_F, D2D1_FIGURE_BEGIN_FILLED, D2D1_FIGURE_BEGIN_HOLLOW,
    D2D1_FIGURE_END_CLOSED, D2D1_FIGURE_END_OPEN, D2D_POINT_2F, D2D_RECT_F,
};
use windows::Win32::Graphics::Direct2D::{
    ID2D1Factory, ID2D1HwndRenderTarget, D2D1_ELLIPSE, D2D1_ROUNDED_RECT,
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

    // Siklus Ekspresi Hidup Asta + Aura Background Dinamis
    pub idle_cycle_timer: f32,
    pub is_cheer_mode: bool,    // false = Devil Union (Pedang + Petir Merah), true = Normal Happy (Healing Hijau)
    pub mode_aura_timer: f32,   // Berjalan 0.0..4.2 detik di awal pergantian mode, lalu kembali normal (hitam)
    pub sword_float_phase: f32,
    pub hand_grab_progress: f32,
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

    /// Mengembalikan intensitas efek aura background (0.0 = hitam normal, 0.0..1.0 = aura aktif)
    /// Hanya aktif selama 4.2 detik pertama setiap kali Asta berganti mode, lalu kembali 0.0 (hitam normal)
    pub fn current_aura_envelope(&self) -> f32 {
        let dur = 4.2_f32;
        if self.mode_aura_timer >= dur {
            0.0
        } else if self.mode_aura_timer < 0.6 {
            (self.mode_aura_timer / 0.6).clamp(0.0, 1.0)
        } else if self.mode_aura_timer > dur - 1.1 {
            ((dur - self.mode_aura_timer) / 1.1).clamp(0.0, 1.0)
        } else {
            1.0
        }
    }

    pub fn update(&mut self, dt: f32, cursor_offset: Option<(f32, f32)>) {
        self.phase_time += dt;
        self.idle_cycle_timer += dt;
        self.mode_aura_timer += dt;
        self.sword_float_phase += dt * 3.1;

        // Berganti mode setiap 9.0 detik:
        // 0..9s   = Devil Union Mode (4.2s pertama: Aura Merah-Hitam + Kilatan Petir Merah, 4.8s berikutnya: Hitam Normal)
        // 9..18s  = Normal Happy Mode (4.2s pertama: Aura Hijau Healing + Partikel +, 4.8s berikutnya: Hitam Normal)
        let cycle = self.idle_cycle_timer % 18.0;
        let next_cheer = cycle >= 9.0;
        if next_cheer != self.is_cheer_mode {
            self.is_cheer_mode = next_cheer;
            self.mode_aura_timer = 0.0; // Picu efek background selama 4.2 detik!
            self.blink_progress = 0.85;
        }

        let grab_wave = (self.sword_float_phase * 0.75).sin();
        self.hand_grab_progress = ((grab_wave + 0.3) / 1.3).clamp(0.0, 1.0);

        if let Some((dx, dy)) = cursor_offset {
            self.target_look_x = (dx / 180.0).clamp(-1.0, 1.0) * 3.0;
            self.target_look_y = (dy / 100.0).clamp(-1.0, 1.0) * 1.4;
        } else {
            self.target_look_x = (self.phase_time * 0.9).sin() * 1.6;
            self.target_look_y = (self.phase_time * 0.6).cos() * 0.6;
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
                self.blink_timer = 2.6 + (self.phase_time.sin().abs() * 2.2);
            }
        }
    }

    /// Menggambar efek background dinamis di dalam pill saat Idle (`NotchState::Pet`):
    /// - Mode Devil Union (`!is_cheer_mode`): Gelombang Merah-Hitam + Kilatan Petir Merah Anti-Magic
    /// - Mode Normal Happy (`is_cheer_mode`): Gelombang Hijau Healing + Partikel '+' & Orb Emerald
    /// Setelah 4.2 detik, `env` menjadi 0.0 sehingga background kembali Hitam Pekat (#000000) normal!
    pub unsafe fn draw_idle_aura_background(
        &self,
        rt: &ID2D1HwndRenderTarget,
        factory: &ID2D1Factory,
        pill_rect: D2D_RECT_F,
        alpha: f32,
    ) {
        let env = self.current_aura_envelope() * alpha;
        if env <= 0.01 {
            return;
        }

        let pw = pill_rect.right - pill_rect.left;
        let ph = pill_rect.bottom - pill_rect.top;
        let cx = (pill_rect.left + pill_rect.right) * 0.5;
        let cy = (pill_rect.top + pill_rect.bottom) * 0.5;

        if !self.is_cheer_mode {
            // =================================================================
            // 1. DEVIL UNION SWORD AURA: Merah-Hitam Bergerak + Kilatan Petir Merah
            // =================================================================
            let wave_x = (self.phase_time * 3.8).sin() * (pw * 0.22);
            let pulse = 0.70 + 0.30 * (self.phase_time * 9.5).sin().abs();

            // Lapisan kabut merah-hitam gelap di dalam pill
            if let Ok(bg_crimson) = rt.CreateSolidColorBrush(
                &D2D1_COLOR_F {
                    r: 0.24 * pulse,
                    g: 0.02,
                    b: 0.04,
                    a: 0.72 * env,
                },
                None,
            ) {
                rt.FillRoundedRectangle(
                    &D2D1_ROUNDED_RECT {
                        rect: D2D_RECT_F {
                            left: pill_rect.left + 3.0,
                            top: pill_rect.top + 2.0,
                            right: pill_rect.right - 3.0,
                            bottom: pill_rect.bottom - 2.0,
                        },
                        radiusX: 14.0,
                        radiusY: 14.0,
                    },
                    &bg_crimson,
                );
            }

            // 2 Orb energi merah-hitam yang bergerak menyapu kiri-kanan
            if let Ok(orb_brush) = rt.CreateSolidColorBrush(
                &D2D1_COLOR_F {
                    r: 0.55,
                    g: 0.05,
                    b: 0.09,
                    a: 0.48 * env * pulse,
                },
                None,
            ) {
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F {
                            x: cx + wave_x,
                            y: cy,
                        },
                        radiusX: pw * 0.30,
                        radiusY: ph * 0.46,
                    },
                    &orb_brush,
                );
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F {
                            x: cx - wave_x * 0.7,
                            y: cy,
                        },
                        radiusX: pw * 0.22,
                        radiusY: ph * 0.40,
                    },
                    &orb_brush,
                );
            }

            // Kilatan-kilatan Petir Merah Anti-Magic (Red Lightning Flashes)
            let flash_trigger = ((self.phase_time * 13.0).sin() + (self.phase_time * 23.0).cos()) * 0.5;
            if flash_trigger > -0.15 {
                let bolt_alpha = ((flash_trigger + 0.2) * 1.2).clamp(0.25, 1.0) * env;
                if let Ok(bolt_brush) = rt.CreateSolidColorBrush(
                    &D2D1_COLOR_F {
                        r: 0.96,
                        g: 0.20,
                        b: 0.26,
                        a: bolt_alpha,
                    },
                    None,
                ) {
                    // Gambar 3 sambaran petir zig-zag di kiri, tengah, dan kanan
                    let offsets = [-pw * 0.30, 0.0, pw * 0.26];
                    for (idx, ox) in offsets.iter().enumerate() {
                        let phase_shift = (self.phase_time * 16.0 + idx as f32 * 2.1).sin();
                        let bx = cx + ox + phase_shift * 8.0;
                        let by = pill_rect.top + 5.0;

                        if let Ok(path) = factory.CreatePathGeometry() {
                            if let Ok(sink) = path.Open() {
                                sink.BeginFigure(
                                    D2D_POINT_2F { x: bx - 10.0, y: by + 2.0 },
                                    D2D1_FIGURE_BEGIN_HOLLOW,
                                );
                                sink.AddLines(&[
                                    D2D_POINT_2F { x: bx - 2.0, y: by + 10.0 },
                                    D2D_POINT_2F { x: bx - 6.0, y: by + 13.0 },
                                    D2D_POINT_2F { x: bx + 5.0, y: by + 24.0 },
                                    D2D_POINT_2F { x: bx + 1.0, y: by + 16.0 },
                                    D2D_POINT_2F { x: bx + 11.0, y: by + 26.0 },
                                ]);
                                sink.EndFigure(D2D1_FIGURE_END_OPEN);
                                let _ = sink.Close();
                                rt.DrawGeometry(&path, &bolt_brush, 1.6, None);
                            }
                        }
                    }
                }
            }
        } else {
            // =================================================================
            // 2. NORMAL HAPPY / CHEER MODE AURA: Hijau Bergerak + Partikel Healing (+)
            // =================================================================
            let wave_x = (self.phase_time * 2.4).sin() * (pw * 0.20);
            let gentle_pulse = 0.75 + 0.25 * (self.phase_time * 4.0).sin();

            // Lapisan dasar hijau zamrud gelap yang menenangkan
            if let Ok(bg_heal) = rt.CreateSolidColorBrush(
                &D2D1_COLOR_F {
                    r: 0.02,
                    g: 0.22 * gentle_pulse,
                    b: 0.14 * gentle_pulse,
                    a: 0.68 * env,
                },
                None,
            ) {
                rt.FillRoundedRectangle(
                    &D2D1_ROUNDED_RECT {
                        rect: D2D_RECT_F {
                            left: pill_rect.left + 3.0,
                            top: pill_rect.top + 2.0,
                            right: pill_rect.right - 3.0,
                            bottom: pill_rect.bottom - 2.0,
                        },
                        radiusX: 14.0,
                        radiusY: 14.0,
                    },
                    &bg_heal,
                );
            }

            // Gelombang cahaya hijau zamrud bergerak lembut
            if let Ok(wave_brush) = rt.CreateSolidColorBrush(
                &D2D1_COLOR_F {
                    r: 0.06,
                    g: 0.52,
                    b: 0.34,
                    a: 0.42 * env * gentle_pulse,
                },
                None,
            ) {
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F {
                            x: cx + wave_x,
                            y: cy + 2.0,
                        },
                        radiusX: pw * 0.32,
                        radiusY: ph * 0.44,
                    },
                    &wave_brush,
                );
            }

            // Partikel-partikel Healing (Simbol '+' dan Orb Hijau Mint yang Melayang Naik)
            if let Ok(part_brush) = rt.CreateSolidColorBrush(
                &D2D1_COLOR_F {
                    r: 0.43,
                    g: 0.91,
                    b: 0.72,
                    a: 0.88 * env,
                },
                None,
            ) {
                let slots = [-0.36_f32, -0.22, -0.08, 0.10, 0.24, 0.38];
                for (i, rel_x) in slots.iter().enumerate() {
                    let speed = 14.0 + (i as f32 * 2.5);
                    let raw_y = (self.phase_time * speed + i as f32 * 7.0) % (ph - 8.0);
                    let py = pill_rect.bottom - 5.0 - raw_y;
                    let px = cx + rel_x * pw + (self.phase_time * 3.0 + i as f32).sin() * 3.0;

                    if i % 2 == 0 {
                        // Gambar simbol Healing '+'
                        let arm = 2.8_f32;
                        rt.DrawLine(
                            D2D_POINT_2F { x: px - arm, y: py },
                            D2D_POINT_2F { x: px + arm, y: py },
                            &part_brush,
                            1.5,
                            None,
                        );
                        rt.DrawLine(
                            D2D_POINT_2F { x: px, y: py - arm },
                            D2D_POINT_2F { x: px, y: py + arm },
                            &part_brush,
                            1.5,
                            None,
                        );
                    } else {
                        // Gambar orb cahaya healing kecil
                        rt.FillEllipse(
                            &D2D1_ELLIPSE {
                                point: D2D_POINT_2F { x: px, y: py },
                                radiusX: 2.0,
                                radiusY: 2.0,
                            },
                            &part_brush,
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

        let eye_spacing = 15.0;
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
            r: 0.08,
            g: 0.10,
            b: 0.14,
            a: alpha,
        };
        let crimson_core = D2D1_COLOR_F {
            r: 0.94,
            g: 0.18,
            b: 0.24,
            a: alpha,
        };
        let emerald_green = D2D1_COLOR_F {
            r: 0.16,
            g: 0.82,
            b: 0.48,
            a: alpha,
        };
        let sclera_white = D2D1_COLOR_F {
            r: 0.97,
            g: 0.98,
            b: 1.00,
            a: alpha,
        };

        let Ok(skin_brush) = rt.CreateSolidColorBrush(&skin_peach, None) else { return };
        let Ok(hair_light_brush) = rt.CreateSolidColorBrush(&hair_ash_light, None) else { return };
        let Ok(hair_mid_brush) = rt.CreateSolidColorBrush(&hair_ash_mid, None) else { return };
        let Ok(band_brush) = rt.CreateSolidColorBrush(&headband_obsidian, None) else { return };
        let Ok(crimson_brush) = rt.CreateSolidColorBrush(&crimson_core, None) else { return };
        let Ok(emerald_brush) = rt.CreateSolidColorBrush(&emerald_green, None) else { return };
        let Ok(sclera_brush) = rt.CreateSolidColorBrush(&sclera_white, None) else { return };

        // 1. KEPALA & WAJAH CHIBI ASTA
        rt.FillRoundedRectangle(
            &D2D1_ROUNDED_RECT {
                rect: D2D_RECT_F {
                    left: center_x - 24.0,
                    top: center_y - 10.0,
                    right: center_x + 24.0,
                    bottom: center_y + 13.0,
                },
                radiusX: 10.0,
                radiusY: 10.0,
            },
            &skin_brush,
        );

        // 2. IKAT KEPALA BLACK BULLS
        rt.FillRoundedRectangle(
            &D2D1_ROUNDED_RECT {
                rect: D2D_RECT_F {
                    left: center_x - 25.0,
                    top: center_y - 11.5,
                    right: center_x + 25.0,
                    bottom: center_y - 4.0,
                },
                radiusX: 3.5,
                radiusY: 3.5,
            },
            &band_brush,
        );

        // Insignia Cross Merah di sisi kiri headband
        let bx = center_x - 16.5;
        let by = center_y - 7.8;
        rt.DrawLine(
            D2D_POINT_2F { x: bx - 3.0, y: by },
            D2D_POINT_2F { x: bx + 3.0, y: by },
            &crimson_brush,
            1.6,
            None,
        );
        rt.DrawLine(
            D2D_POINT_2F { x: bx, y: by - 3.0 },
            D2D_POINT_2F { x: bx, y: by + 3.0 },
            &crimson_brush,
            1.6,
            None,
        );

        // 3. RAMBUT SPIKY ABU-ABU PERAK ASTA
        let spikes = [
            (-26.0, -5.0, -32.0, -14.0, -18.0, -12.0),
            (-18.0, -10.0, -20.0, -19.0, -8.0, -12.0),
            (-9.0, -11.0, -6.0, -20.0, 3.0, -11.0),
            (1.0, -11.0, 8.0, -20.0, 14.0, -10.0),
            (11.0, -10.0, 21.0, -18.0, 23.0, -8.0),
            (18.0, -8.0, 31.0, -13.0, 25.0, -3.0),
        ];
        for (i, &(x1, y1, tx, ty, x2, y2)) in spikes.iter().enumerate() {
            if let Ok(path) = factory.CreatePathGeometry() {
                if let Ok(sink) = path.Open() {
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
                    let b = if i % 2 == 0 {
                        &hair_light_brush
                    } else {
                        &hair_mid_brush
                    };
                    rt.FillGeometry(&path, b, None);
                }
            }
        }

        // 4. TANDUK DEVIL UNION (Saat Mode Devil Union) ATAU BINTANG KILAU (Saat Mode Cheer)
        if !self.is_cheer_mode {
            for side in [-1.0_f32, 1.0_f32] {
                if let Ok(h_path) = factory.CreatePathGeometry() {
                    if let Ok(sink) = h_path.Open() {
                        let hx = center_x + side * 14.0;
                        let hy = center_y - 11.0;
                        sink.BeginFigure(D2D_POINT_2F { x: hx, y: hy }, D2D1_FIGURE_BEGIN_FILLED);
                        sink.AddBezier(&D2D1_BEZIER_SEGMENT {
                            point1: D2D_POINT_2F {
                                x: hx + side * 8.0,
                                y: hy - 6.0,
                            },
                            point2: D2D_POINT_2F {
                                x: hx + side * 6.0,
                                y: hy - 14.0,
                            },
                            point3: D2D_POINT_2F {
                                x: hx + side * 1.5,
                                y: hy - 17.5,
                            },
                        });
                        sink.AddBezier(&D2D1_BEZIER_SEGMENT {
                            point1: D2D_POINT_2F {
                                x: hx + side * 2.5,
                                y: hy - 11.0,
                            },
                            point2: D2D_POINT_2F {
                                x: hx - side * 1.5,
                                y: hy - 5.0,
                            },
                            point3: D2D_POINT_2F {
                                x: hx - side * 4.5,
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
        } else {
            let sx = center_x - 32.0;
            let sy = center_y - 8.0 + (self.phase_time * 5.0).sin() * 1.5;
            rt.DrawLine(
                D2D_POINT_2F { x: sx - 3.5, y: sy },
                D2D_POINT_2F { x: sx + 3.5, y: sy },
                &emerald_brush,
                1.4,
                None,
            );
            rt.DrawLine(
                D2D_POINT_2F { x: sx, y: sy - 3.5 },
                D2D_POINT_2F { x: sx, y: sy + 3.5 },
                &emerald_brush,
                1.4,
                None,
            );
        }

        // 5. MATA ASTA
        for side in [-1.0_f32, 1.0_f32] {
            let ex = center_x + side * eye_spacing;
            let ey = center_y + 2.0;

            if self.is_cheer_mode && self.blink_progress < 0.4 {
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F { x: ex, y: ey },
                        radiusX: 5.8,
                        radiusY: 4.5 * open_factor,
                    },
                    &sclera_brush,
                );
                let px = ex + self.look_x * 0.55;
                let py = ey + self.look_y * 0.55;
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F { x: px, y: py },
                        radiusX: 3.4,
                        radiusY: 3.5 * open_factor,
                    },
                    &emerald_brush,
                );
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F {
                            x: px - 1.1,
                            y: py - 1.1,
                        },
                        radiusX: 1.1,
                        radiusY: 1.1,
                    },
                    &sclera_brush,
                );
                rt.DrawLine(
                    D2D_POINT_2F {
                        x: ex - 5.5,
                        y: ey - 5.2,
                    },
                    D2D_POINT_2F {
                        x: ex + 5.0,
                        y: ey - 4.2,
                    },
                    &band_brush,
                    1.5,
                    None,
                );
            } else {
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F { x: ex, y: ey },
                        radiusX: 6.2,
                        radiusY: 3.8 * open_factor,
                    },
                    &sclera_brush,
                );
                let px = ex + self.look_x * 0.65;
                let py = ey + self.look_y * 0.65;
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F { x: px, y: py },
                        radiusX: 3.6,
                        radiusY: 3.3 * open_factor,
                    },
                    &crimson_brush,
                );
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F { x: px, y: py },
                        radiusX: 0.9,
                        radiusY: 2.9 * open_factor,
                    },
                    &band_brush,
                );
                rt.DrawLine(
                    D2D_POINT_2F {
                        x: ex - side * 5.0,
                        y: ey - 2.6,
                    },
                    D2D_POINT_2F {
                        x: ex + side * 6.2,
                        y: ey - 5.2,
                    },
                    &band_brush,
                    1.8,
                    None,
                );
            }
        }

        // 6. PEDANG DEMON-SLAYER & TANGAN MENGGENGGAM
        let sword_bob = self.sword_float_phase.sin() * 2.2;
        let sx = center_x + 42.0;
        let sy = center_y + sword_bob * 0.5;

        if let Ok(sword_path) = factory.CreatePathGeometry() {
            if let Ok(sink) = sword_path.Open() {
                sink.BeginFigure(
                    D2D_POINT_2F {
                        x: sx - 10.0,
                        y: sy - 2.5,
                    },
                    D2D1_FIGURE_BEGIN_FILLED,
                );
                sink.AddLines(&[
                    D2D_POINT_2F {
                        x: sx + 12.0,
                        y: sy - 7.5,
                    },
                    D2D_POINT_2F {
                        x: sx + 18.0,
                        y: sy,
                    },
                    D2D_POINT_2F {
                        x: sx + 12.0,
                        y: sy + 7.5,
                    },
                    D2D_POINT_2F {
                        x: sx - 10.0,
                        y: sy + 2.5,
                    },
                ]);
                sink.EndFigure(D2D1_FIGURE_END_CLOSED);
                let _ = sink.Close();

                rt.FillGeometry(&sword_path, &band_brush, None);
                rt.DrawGeometry(&sword_path, &crimson_brush, 1.1, None);
            }
        }

        rt.DrawLine(
            D2D_POINT_2F {
                x: sx - 16.0,
                y: sy,
            },
            D2D_POINT_2F {
                x: sx - 10.0,
                y: sy,
            },
            &crimson_brush,
            2.4,
            None,
        );

        let reach_x = (center_x + 25.0) + self.hand_grab_progress * 8.0;
        let reach_y = center_y + 3.0 + sword_bob * 0.35;
        rt.FillEllipse(
            &D2D1_ELLIPSE {
                point: D2D_POINT_2F {
                    x: reach_x,
                    y: reach_y,
                },
                radiusX: 3.4,
                radiusY: 3.0,
            },
            &skin_brush,
        );
    }
}