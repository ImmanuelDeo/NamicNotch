// src/mascot/eyes.rs
use windows::Win32::Graphics::Direct2D::Common::{
    D2D1_BEZIER_SEGMENT, D2D1_COLOR_F, D2D1_FIGURE_BEGIN_FILLED, D2D1_FIGURE_BEGIN_HOLLOW,
    D2D1_FIGURE_END_CLOSED, D2D1_FIGURE_END_OPEN, D2D1_GRADIENT_STOP, D2D_POINT_2F, D2D_RECT_F,
};
use windows::Win32::Graphics::Direct2D::{
    ID2D1Factory, ID2D1HwndRenderTarget, ID2D1PathGeometry, D2D1_ELLIPSE, D2D1_EXTEND_MODE_CLAMP,
    D2D1_GAMMA_2_2, D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES, D2D1_ROUNDED_RECT,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AstaIdleMode {
    /// Devil Union Mode: Tatapan tajam serius, iris Muted Crimson, pupil '+'
    DevilUnionSerious { timer: f32 },
    /// Normal Asta Mode: Mata asli Hijau Emerald berbinar, ekspresi ramah/antusias
    NormalEmerald { timer: f32 },
    /// Happy Gesture: Mata melengkung senang '^ ^' khas Asta saat menyapa
    HappySmileGesture { timer: f32 },
    /// Sword Summon & Grab: Pedang Demon-Slayer melayang lalu ditangkap oleh tangan Asta!
    SwordSummonGrab { timer: f32 },
    /// Dozing: Istirahat tenang setelah lama tidak ada aktivitas
    Dozing { timer: f32 },
}

#[derive(Debug, Clone, Copy)]
pub struct AntiMagicSpark {
    pub offset_x: f32,
    pub offset_y: f32,
    pub vx: f32,
    pub vy: f32,
    pub size: f32,
    pub life: f32,
    pub max_life: f32,
    pub kind: u8,
}

pub struct DevilUnionEyes {
    pub mode: AstaIdleMode,
    pub time: f32,
    pub look_x: f32,
    pub look_y: f32,
    pub target_look_x: f32,
    pub target_look_y: f32,
    pub openness: f32,
    pub blink_timer: f32,
    pub next_blink_in: f32,
    pub glance_timer: f32,
    pub aura_intensity: f32,
    pub pupil_scale: f32,
    /// 0.0 = Devil Union (Crimson + '+'), 1.0 = Normal Asta (Emerald Green + Star Highlight)
    pub emerald_blend: f32,
    pub sparks: [AntiMagicSpark; 10],
    rng_seed: u32,
}

impl Default for DevilUnionEyes {
    fn default() -> Self {
        Self::new()
    }
}

impl DevilUnionEyes {
    pub fn new() -> Self {
        let mut eyes = Self {
            mode: AstaIdleMode::DevilUnionSerious { timer: 0.0 },
            time: 0.0,
            look_x: 0.0,
            look_y: 0.0,
            target_look_x: 0.0,
            target_look_y: 0.0,
            openness: 0.1,
            blink_timer: 0.0,
            next_blink_in: 3.0,
            glance_timer: 1.8,
            aura_intensity: 1.0,
            pupil_scale: 1.1,
            emerald_blend: 0.0,
            sparks: [AntiMagicSpark {
                offset_x: 0.0,
                offset_y: 0.0,
                vx: 0.0,
                vy: 0.0,
                size: 2.0,
                life: 0.0,
                max_life: 1.0,
                kind: 0,
            }; 10],
            rng_seed: 0xA57A_7777,
        };
        for i in 0..eyes.sparks.len() {
            eyes.respawn_spark(i, true);
        }
        eyes
    }

    fn next_f32(&mut self) -> f32 {
        let mut x = self.rng_seed;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.rng_seed = x;
        (x as f32) / (u32::MAX as f32)
    }

    fn respawn_spark(&mut self, idx: usize, random_life: bool) {
        let side = if idx % 2 == 0 { -1.0 } else { 1.0 };
        let rx = (self.next_f32() - 0.5) * 24.0 + side * 16.0;
        let ry = (self.next_f32() - 0.5) * 8.0;
        let vx = (self.next_f32() - 0.5) * 12.0 + side * 3.0;
        let vy = -8.0 - self.next_f32() * 14.0;
        let max_life = 0.50 + self.next_f32() * 0.60;
        let life = if random_life {
            self.next_f32() * max_life
        } else {
            0.0
        };
        let size = 1.3 + self.next_f32() * 1.9;
        let kind = (idx % 3) as u8;

        self.sparks[idx] = AntiMagicSpark {
            offset_x: rx,
            offset_y: ry,
            vx,
            vy,
            size,
            life,
            max_life,
            kind,
        };
    }

    pub fn trigger_awakening(&mut self) {
        self.mode = AstaIdleMode::DevilUnionSerious { timer: 0.0 };
        self.openness = 0.15;
        self.pupil_scale = 1.25;
        self.aura_intensity = 1.25;
    }

    pub fn trigger_wake_surge(&mut self) {
        // Saat pengguna kembali atau menyentuh notch, Asta menyapa dengan mata Hijau Emerald senang sebentar!
        self.mode = AstaIdleMode::HappySmileGesture { timer: 0.0 };
        self.aura_intensity = 1.2;
    }

    pub fn update(&mut self, dt: f32, cursor_offset: Option<(f32, f32)>) {
        let dt = dt.clamp(0.001, 0.05);
        self.time += dt;

        let mut cursor_near = false;
        if let Some((dx, dy)) = cursor_offset {
            let dist = (dx * dx + dy * dy).sqrt();
            if dist < 360.0 {
                cursor_near = true;
                self.target_look_x = (dx / 170.0).clamp(-1.0, 1.0);
                self.target_look_y = (dy / 105.0).clamp(-0.75, 0.85);
                if matches!(self.mode, AstaIdleMode::Dozing { .. }) {
                    self.mode = AstaIdleMode::HappySmileGesture { timer: 0.0 };
                }
            }
        }

        // Lirikan acak organik jika kursor tidak dekat
        if !cursor_near {
            self.glance_timer -= dt;
            if self.glance_timer <= 0.0 {
                let r = self.next_f32();
                if r < 0.35 {
                    self.target_look_x = 0.0;
                    self.target_look_y = 0.0;
                } else {
                    self.target_look_x = (self.next_f32() - 0.5) * 1.4;
                    self.target_look_y = (self.next_f32() - 0.5) * 0.55;
                }
                self.glance_timer = 1.8 + self.next_f32() * 2.5;
            }
        }

        let mut target_openness = 1.0;
        let mut target_aura = 0.85;
        let mut target_pupil = 1.0;
        let mut target_emerald = 0.0;

        match &mut self.mode {
            AstaIdleMode::DevilUnionSerious { timer } => {
                *timer += dt;
                target_openness = 1.0;
                target_emerald = 0.0;
                target_aura = if cursor_near { 1.15 } else { 0.82 + 0.12 * (self.time * 3.0).sin() };
                target_pupil = if cursor_near { 0.86 } else { 1.0 };

                // Setelah 7 detik di Devil Union, bergantian masuk ke aksi Pedang Melayang atau Mata Asli Emerald!
                if *timer >= 7.5 {
                    if self.next_f32() < 0.55 {
                        self.mode = AstaIdleMode::SwordSummonGrab { timer: 0.0 };
                    } else {
                        self.mode = AstaIdleMode::NormalEmerald { timer: 0.0 };
                    }
                }
            }
            AstaIdleMode::SwordSummonGrab { timer } => {
                *timer += dt;
                target_openness = 1.02;
                target_emerald = 0.0; // Tetap Devil Union serius sambil memanggil & menangkap pedang
                target_aura = 1.25;
                target_pupil = 0.82;

                // Mata Asta otomatis melirik ke arah pedang yang melayang di sisi kanan (x = +0.85)
                let p = (*timer / 3.8).clamp(0.0, 1.0);
                self.target_look_x = 0.85 * (1.0 - (p - 0.5).abs() * 1.2).clamp(0.2, 1.0);
                self.target_look_y = -0.15 + (self.time * 4.0).sin() * 0.20;

                if *timer >= 3.8 {
                    // Setelah berhasil menangkap pedang, Asta berubah ke mata asli Hijau Emerald & senang!
                    self.mode = AstaIdleMode::NormalEmerald { timer: 0.0 };
                }
            }
            AstaIdleMode::NormalEmerald { timer } => {
                *timer += dt;
                target_openness = 1.06; // Mata asli Asta sedikit lebih bulat & antusias
                target_emerald = 1.0;
                target_aura = 0.45;
                target_pupil = 1.08;

                if *timer >= 4.2 {
                    self.mode = AstaIdleMode::HappySmileGesture { timer: 0.0 };
                }
            }
            AstaIdleMode::HappySmileGesture { timer } => {
                *timer += dt;
                target_openness = 1.0;
                target_emerald = 1.0;
                target_aura = 0.60;
                target_pupil = 1.0;

                if *timer >= 2.2 {
                    // Kembali fokus ke Devil Union Mode
                    self.mode = AstaIdleMode::DevilUnionSerious { timer: 0.0 };
                }
            }
            AstaIdleMode::Dozing { timer } => {
                *timer += dt;
                target_openness = 0.30 + 0.04 * (self.time * 1.4).sin();
                target_emerald = 0.0;
                target_aura = 0.35;
                target_pupil = 1.05;
            }
        }

        // Kedipan cepat
        self.next_blink_in -= dt;
        if self.next_blink_in <= 0.0 {
            self.blink_timer = 0.13;
            self.next_blink_in = 2.6 + self.next_f32() * 3.4;
        }

        let blink_factor = if self.blink_timer > 0.0 {
            self.blink_timer -= dt;
            let norm = (self.blink_timer / 0.13).clamp(0.0, 1.0);
            (norm - 0.5).abs() * 2.0
        } else {
            1.0
        };

        let smooth = (14.0 * dt).min(1.0);
        self.look_x += (self.target_look_x - self.look_x) * smooth;
        self.look_y += (self.target_look_y - self.look_y) * smooth;
        self.openness += (target_openness * blink_factor - self.openness) * (22.0 * dt).min(1.0);
        self.aura_intensity += (target_aura - self.aura_intensity) * (10.0 * dt).min(1.0);
        self.pupil_scale += (target_pupil - self.pupil_scale) * (12.0 * dt).min(1.0);
        self.emerald_blend += (target_emerald - self.emerald_blend) * (8.0 * dt).min(1.0);

        for i in 0..self.sparks.len() {
            self.sparks[i].life += dt * self.aura_intensity.max(0.45);
            if self.sparks[i].life >= self.sparks[i].max_life {
                self.respawn_spark(i, false);
            } else {
                self.sparks[i].offset_x += self.sparks[i].vx * dt;
                self.sparks[i].offset_y += self.sparks[i].vy * dt;
            }
        }
    }

    pub fn render(
        &self,
        factory: &ID2D1Factory,
        rt: &ID2D1HwndRenderTarget,
        pill_rect: D2D_RECT_F,
        alpha_mult: f32,
    ) {
        if alpha_mult <= 0.01 {
            return;
        }

        let center_x = (pill_rect.left + pill_rect.right) * 0.5;
        let center_y = (pill_rect.top + pill_rect.bottom) * 0.5 + 1.0;

        // Geser sedikit posisi mata ke kiri saat pedang melayang di kanan agar seimbang di dalam pill
        let sword_shift = if let AstaIdleMode::SwordSummonGrab { timer } = self.mode {
            let s = (timer * 2.5).min(1.0) * ((3.8 - timer) * 2.5).clamp(0.0, 1.0);
            -12.0 * s
        } else {
            0.0
        };

        let eyes_cx = center_x + sword_shift;
        let eye_spacing = 16.5;

        unsafe {
            // 1. Partikel Anti-Magic / Starlight
            self.draw_anti_magic_sparks(rt, eyes_cx, center_y, alpha_mult);

            // 2. Jika sedang dalam gesture HappySmileGesture ('^ ^'), gambar lengkungan mata senang!
            if let AstaIdleMode::HappySmileGesture { timer } = self.mode {
                let bounce = (timer * 10.0).sin().abs() * -1.8;
                for side in [-1.0_f32, 1.0_f32] {
                    self.draw_happy_arc_eye(
                        factory,
                        rt,
                        eyes_cx + side * eye_spacing,
                        center_y + bounce,
                        side,
                        alpha_mult,
                    );
                }
            } else {
                // 3. Gambar Sepasang Mata Asta (Transisi Mulus Devil Union <-> Normal Emerald)
                for side in [-1.0_f32, 1.0_f32] {
                    let base_x = eyes_cx + side * eye_spacing;
                    self.draw_single_asta_eye(factory, rt, base_x, center_y, side, alpha_mult);
                }
            }

            // 4. Jika sedang dalam fase SwordSummonGrab, gambar Pedang Melayang & Tangan Asta Menangkap!
            if let AstaIdleMode::SwordSummonGrab { timer } = self.mode {
                self.draw_floating_sword_and_grabbing_hand(
                    factory,
                    rt,
                    center_x + 36.0,
                    center_y,
                    pill_rect.bottom,
                    timer,
                    alpha_mult,
                );
            }
        }
    }

    /// Gambar mata melengkung senang ('^ ^') berwarna Hijau Emerald / Ice-Blue saat Asta gembira
    unsafe fn draw_happy_arc_eye(
        &self,
        factory: &ID2D1Factory,
        rt: &ID2D1HwndRenderTarget,
        ex: f32,
        ey: f32,
        side: f32,
        alpha: f32,
    ) {
        if let Ok(geom) = factory.CreatePathGeometry() {
            if let Ok(sink) = geom.Open() {
                sink.BeginFigure(
                    D2D_POINT_2F {
                        x: ex - 6.5,
                        y: ey + 2.5,
                    },
                    D2D1_FIGURE_BEGIN_HOLLOW,
                );
                sink.AddBezier(&D2D1_BEZIER_SEGMENT {
                    point1: D2D_POINT_2F {
                        x: ex - 3.0,
                        y: ey - 5.5,
                    },
                    point2: D2D_POINT_2F {
                        x: ex + 3.0,
                        y: ey - 5.5,
                    },
                    point3: D2D_POINT_2F {
                        x: ex + 6.5,
                        y: ey + 2.5,
                    },
                });
                sink.EndFigure(D2D1_FIGURE_END_OPEN);
                let _ = sink.Close();

                let emerald_col = D2D1_COLOR_F {
                    r: 0.22,
                    g: 0.88,
                    b: 0.62,
                    a: alpha,
                };
                if let Ok(b) = rt.CreateSolidColorBrush(&emerald_col, None) {
                    rt.DrawGeometry(&geom, &b, 2.4, None);
                }
            }
        }

        // Bintang kecil berbinar di sudut luar mata saat senang
        let star_x = ex + side * 8.5;
        let star_y = ey - 3.5;
        let star_col = D2D1_COLOR_F {
            r: 0.90,
            g: 0.98,
            b: 0.95,
            a: 0.85 * alpha,
        };
        if let Ok(sb) = rt.CreateSolidColorBrush(&star_col, None) {
            rt.DrawLine(
                D2D_POINT_2F {
                    x: star_x - 2.5,
                    y: star_y,
                },
                D2D_POINT_2F {
                    x: star_x + 2.5,
                    y: star_y,
                },
                &sb,
                1.2,
                None,
            );
            rt.DrawLine(
                D2D_POINT_2F {
                    x: star_x,
                    y: star_y - 2.5,
                },
                D2D_POINT_2F {
                    x: star_x,
                    y: star_y + 2.5,
                },
                &sb,
                1.2,
                None,
            );
        }
    }

    /// Gambar Pedang Demon-Slayer Melayang & Tangan Asta yang Menjulur Menangkap Gagangnya
    unsafe fn draw_floating_sword_and_grabbing_hand(
        &self,
        factory: &ID2D1Factory,
        rt: &ID2D1HwndRenderTarget,
        sx: f32,
        sy: f32,
        pill_bottom: f32,
        timer: f32,
        alpha: f32,
    ) {
        // Fase waktu (total 3.8 detik):
        // 0.0 .. 1.6s: Pedang melayang turun-naik & bergetar dengan aura Anti-Magic
        // 1.6 .. 2.7s: Tangan hitam Asta menjulur dari bawah & menangkap gagang pedang!
        // 2.7 .. 3.8s: Tangan menarik pedang turun ke dalam bayangan disertai kilatan slash
        let hover_bob = (timer * 6.5).sin() * 2.2;
        let retract = if timer > 2.7 {
            ((timer - 2.7) / 1.1).clamp(0.0, 1.0).powi(2) * 18.0
        } else {
            0.0
        };

        let sword_x = sx;
        let sword_y = sy + hover_bob + retract;

        let blade_dark = D2D1_COLOR_F {
            r: 0.14,
            g: 0.17,
            b: 0.24,
            a: alpha,
        };
        let blade_edge = D2D1_COLOR_F {
            r: 0.52,
            g: 0.60,
            b: 0.74,
            a: alpha * 0.9,
        };
        let crimson_core = D2D1_COLOR_F {
            r: 0.72,
            g: 0.22,
            b: 0.30,
            a: alpha * 0.85,
        };
        let hand_obsidian = D2D1_COLOR_F {
            r: 0.08,
            g: 0.10,
            b: 0.15,
            a: alpha,
        };

        let Ok(blade_b) = rt.CreateSolidColorBrush(&blade_dark, None) else { return };
        let Ok(edge_b) = rt.CreateSolidColorBrush(&blade_edge, None) else { return };
        let Ok(core_b) = rt.CreateSolidColorBrush(&crimson_core, None) else { return };
        let Ok(hand_b) = rt.CreateSolidColorBrush(&hand_obsidian, None) else { return };

        // 1. Bilah Besar Demon-Slayer Sword (Danma no Tsurugi)
        if let Ok(sword_geom) = factory.CreatePathGeometry() {
            if let Ok(sink) = sword_geom.Open() {
                // Ujung atas meruncing berat khas greatsword Asta
                sink.BeginFigure(
                    D2D_POINT_2F {
                        x: sword_x,
                        y: sword_y - 12.5,
                    },
                    D2D1_FIGURE_BEGIN_FILLED,
                );
                sink.AddLines(&[
                    D2D_POINT_2F {
                        x: sword_x + 4.2,
                        y: sword_y - 7.5,
                    },
                    D2D_POINT_2F {
                        x: sword_x + 3.4,
                        y: sword_y + 3.5,
                    },
                    D2D_POINT_2F {
                        x: sword_x - 3.4,
                        y: sword_y + 3.5,
                    },
                    D2D_POINT_2F {
                        x: sword_x - 4.2,
                        y: sword_y - 7.5,
                    },
                ]);
                sink.EndFigure(D2D1_FIGURE_END_CLOSED);
                let _ = sink.Close();
                rt.FillGeometry(&sword_geom, &blade_b, None);
                rt.DrawGeometry(&sword_geom, &edge_b, 0.9, None);
            }
        }

        // Garis urat Anti-Magic di tengah bilah pedang
        rt.DrawLine(
            D2D_POINT_2F {
                x: sword_x,
                y: sword_y - 10.0,
            },
            D2D_POINT_2F {
                x: sword_x,
                y: sword_y + 3.0,
            },
            &core_b,
            1.2,
            None,
        );

        // Crossguard & Gagang Pedang (Hilt) di bawah bilah
        rt.DrawLine(
            D2D_POINT_2F {
                x: sword_x - 5.5,
                y: sword_y + 3.8,
            },
            D2D_POINT_2F {
                x: sword_x + 5.5,
                y: sword_y + 3.8,
            },
            &edge_b,
            1.8,
            None,
        );
        rt.DrawLine(
            D2D_POINT_2F {
                x: sword_x,
                y: sword_y + 4.0,
            },
            D2D_POINT_2F {
                x: sword_x,
                y: sword_y + 10.5,
            },
            &edge_b,
            2.0,
            None,
        );

        // 2. Animasi Tangan Asta (Anti-Magic Claw/Hand) Menjulur & Menangkap Gagang Pedang!
        if timer >= 1.2 {
            let reach_progress = if timer < 2.0 {
                ((timer - 1.2) / 0.8).clamp(0.0, 1.0)
            } else {
                1.0
            };

            let hand_top_y =
                pill_bottom - (pill_bottom - (sword_y + 6.5)) * reach_progress;

            // Lengan bawah menjulur dari tepi bawah pill
            let arm_rect = D2D1_ROUNDED_RECT {
                rect: D2D_RECT_F {
                    left: sword_x - 3.2,
                    top: hand_top_y,
                    right: sword_x + 3.2,
                    bottom: pill_bottom,
                },
                radiusX: 2.0,
                radiusY: 2.0,
            };
            rt.FillRoundedRectangle(&arm_rect, &hand_b);
            rt.DrawRoundedRectangle(&arm_rect, &core_b, 0.9, None);

            // Jari-jari menggenggam gagang saat reach_progress hampir penuh
            if reach_progress > 0.75 {
                for f in 0..3 {
                    let fy = hand_top_y + f as f32 * 1.8;
                    rt.DrawLine(
                        D2D_POINT_2F {
                            x: sword_x - 3.8,
                            y: fy,
                        },
                        D2D_POINT_2F {
                            x: sword_x + 3.8,
                            y: fy,
                        },
                        &edge_b,
                        1.2,
                        None,
                    );
                }

                // Efek kilatan tangkapan (Catch Impact Spark!) tepat saat pedang tertangkap
                if (1.85..=2.35).contains(&timer) {
                    rt.DrawLine(
                        D2D_POINT_2F {
                            x: sword_x - 9.0,
                            y: sword_y + 6.0,
                        },
                        D2D_POINT_2F {
                            x: sword_x + 9.0,
                            y: sword_y + 6.0,
                        },
                        &core_b,
                        1.5,
                        None,
                    );
                }
            }
        }
    }

    unsafe fn draw_anti_magic_sparks(
        &self,
        rt: &ID2D1HwndRenderTarget,
        cx: f32,
        cy: f32,
        alpha_mult: f32,
    ) {
        for spark in &self.sparks {
            let progress = (spark.life / spark.max_life).clamp(0.0, 1.0);
            let spark_alpha = (1.0 - (2.0 * progress - 1.0).powi(2))
                * self.aura_intensity.clamp(0.2, 1.1)
                * alpha_mult;

            if spark_alpha <= 0.02 {
                continue;
            }

            let color = if self.emerald_blend > 0.5 {
                // Saat mode Normal Emerald Asta, partikel menjadi kilau Emerald-Mint lembut
                D2D1_COLOR_F {
                    r: 0.25,
                    g: 0.85,
                    b: 0.65,
                    a: spark_alpha * 0.55,
                }
            } else {
                match spark.kind {
                    0 => D2D1_COLOR_F {
                        r: 0.12,
                        g: 0.15,
                        b: 0.22,
                        a: spark_alpha * 0.85,
                    },
                    1 => D2D1_COLOR_F {
                        r: 0.48,
                        g: 0.56,
                        b: 0.72,
                        a: spark_alpha * 0.60,
                    },
                    _ => D2D1_COLOR_F {
                        r: 0.68,
                        g: 0.24,
                        b: 0.32,
                        a: spark_alpha * 0.55,
                    },
                }
            };

            if let Ok(brush) = rt.CreateSolidColorBrush(&color, None) {
                let x = cx + spark.offset_x;
                let y = cy + spark.offset_y;
                let r = spark.size * (1.0 - 0.4 * progress);
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F { x, y },
                        radiusX: r * 0.8,
                        radiusY: r * 1.4,
                    },
                    &brush,
                );
            }
        }
    }

    unsafe fn draw_single_asta_eye(
        &self,
        factory: &ID2D1Factory,
        rt: &ID2D1HwndRenderTarget,
        base_x: f32,
        base_y: f32,
        side: f32,
        alpha_mult: f32,
    ) {
        let shift_x = self.look_x * 4.0;
        let shift_y = self.look_y * 2.0;
        let ex = base_x + shift_x;
        let ey = base_y + shift_y;

        let eye_rx = 8.0;
        let eye_ry = (6.0 * self.openness).max(0.6);

        let eb = self.emerald_blend.clamp(0.0, 1.0);

        // Jika berkedip
        if self.openness < 0.16 {
            let slit_color = D2D1_COLOR_F {
                r: 0.68 * (1.0 - eb) + 0.25 * eb,
                g: 0.32 * (1.0 - eb) + 0.85 * eb,
                b: 0.40 * (1.0 - eb) + 0.60 * eb,
                a: 0.85 * alpha_mult,
            };
            if let Ok(brush) = rt.CreateSolidColorBrush(&slit_color, None) {
                rt.FillRoundedRectangle(
                    &D2D1_ROUNDED_RECT {
                        rect: D2D_RECT_F {
                            left: ex - eye_rx,
                            top: ey - 0.8,
                            right: ex + eye_rx,
                            bottom: ey + 0.8,
                        },
                        radiusX: 0.8,
                        radiusY: 0.8,
                    },
                    &brush,
                );
            }
            return;
        }

        // --- LAYER 1: Iris (Blend Mulus antara Muted Wine-Crimson <-> Bright Emerald Green) ---
        let iris_rx = 6.3;
        let iris_ry = (5.5 * self.openness).max(1.2);

        let c0 = D2D1_COLOR_F {
            r: 0.82 * (1.0 - eb) + 0.28 * eb,
            g: 0.32 * (1.0 - eb) + 0.92 * eb,
            b: 0.38 * (1.0 - eb) + 0.64 * eb,
            a: alpha_mult,
        };
        let c1 = D2D1_COLOR_F {
            r: 0.62 * (1.0 - eb) + 0.12 * eb,
            g: 0.16 * (1.0 - eb) + 0.68 * eb,
            b: 0.24 * (1.0 - eb) + 0.48 * eb,
            a: alpha_mult,
        };
        let c2 = D2D1_COLOR_F {
            r: 0.24 * (1.0 - eb) + 0.05 * eb,
            g: 0.08 * (1.0 - eb) + 0.32 * eb,
            b: 0.14 * (1.0 - eb) + 0.25 * eb,
            a: alpha_mult,
        };

        let iris_stops = [
            D2D1_GRADIENT_STOP {
                position: 0.0,
                color: c0,
            },
            D2D1_GRADIENT_STOP {
                position: 0.68,
                color: c1,
            },
            D2D1_GRADIENT_STOP {
                position: 1.0,
                color: c2,
            },
        ];

        if let Ok(stop_collection) =
            rt.CreateGradientStopCollection(&iris_stops, D2D1_GAMMA_2_2, D2D1_EXTEND_MODE_CLAMP)
        {
            let props = D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES {
                center: D2D_POINT_2F {
                    x: ex,
                    y: ey + iris_ry * 0.15,
                },
                gradientOriginOffset: D2D_POINT_2F { x: 0.0, y: 0.0 },
                radiusX: iris_rx,
                radiusY: iris_ry,
            };
            if let Ok(iris_brush) = rt.CreateRadialGradientBrush(&props, None, &stop_collection) {
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F { x: ex, y: ey },
                        radiusX: iris_rx,
                        radiusY: iris_ry,
                    },
                    &iris_brush,
                );
            }
        }

        // --- LAYER 2: Pupil ('+' saat Devil Union, Bulat saat Normal Emerald) ---
        let px = base_x + shift_x * 1.18;
        let py = base_y + shift_y * 1.18;
        let pupil_color = D2D1_COLOR_F {
            r: 0.04,
            g: 0.05,
            b: 0.08,
            a: alpha_mult,
        };

        if eb < 0.65 {
            // Pupil '+' khas Devil Union
            let arm_h = 4.6 * self.pupil_scale;
            let arm_v = (4.4 * self.pupil_scale * self.openness).max(1.0);
            let half_thick = 1.15 * self.pupil_scale;

            if let Ok(plus_geom) =
                self.create_tapered_plus_geometry(factory, px, py, arm_h, arm_v, half_thick)
            {
                if let Ok(pb) = rt.CreateSolidColorBrush(&pupil_color, None) {
                    rt.FillGeometry(&plus_geom, &pb, None);
                }
            }
        } else {
            // Pupil Bulat khas Mata Asli Asta
            if let Ok(pb) = rt.CreateSolidColorBrush(&pupil_color, None) {
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F { x: px, y: py },
                        radiusX: 2.6 * self.pupil_scale,
                        radiusY: 2.8 * self.pupil_scale * self.openness,
                    },
                    &pb,
                );
            }
        }

        // --- LAYER 3: Upper Eyelid Cut (Serius tajam saat Devil Union, Terbuka ramah saat Normal Emerald) ---
        if eb < 0.8 {
            self.draw_sharp_upper_eyelid_mask(
                factory,
                rt,
                ex,
                ey,
                iris_rx + 1.5,
                iris_ry,
                side,
                alpha_mult * (1.0 - eb),
            );
        }

        // --- LAYER 4: Specular Highlight (Glint) ---
        let glint_color = D2D1_COLOR_F {
            r: 0.92,
            g: 0.96,
            b: 1.0,
            a: 0.80 * alpha_mult,
        };
        if let Ok(glint_brush) = rt.CreateSolidColorBrush(&glint_color, None) {
            rt.FillEllipse(
                &D2D1_ELLIPSE {
                    point: D2D_POINT_2F {
                        x: px - side * 2.1,
                        y: py - iris_ry * 0.35,
                    },
                    radiusX: 1.1 + 0.5 * eb, // Highlight lebih besar berbinar saat mata asli Asta
                    radiusY: (0.8 + 0.4 * eb) * self.openness,
                },
                &glint_brush,
            );
        }
    }

    unsafe fn create_tapered_plus_geometry(
        &self,
        factory: &ID2D1Factory,
        cx: f32,
        cy: f32,
        arm_h: f32,
        arm_v: f32,
        half_t: f32,
    ) -> windows::core::Result<ID2D1PathGeometry> {
        let geom = factory.CreatePathGeometry()?;
        let sink = geom.Open()?;
        sink.BeginFigure(
            D2D_POINT_2F {
                x: cx,
                y: cy - arm_v,
            },
            D2D1_FIGURE_BEGIN_FILLED,
        );

        let tip_t = half_t * 0.32;
        let pts = [
            D2D_POINT_2F { x: cx + tip_t, y: cy - arm_v * 0.65 },
            D2D_POINT_2F { x: cx + half_t, y: cy - half_t },
            D2D_POINT_2F { x: cx + arm_h * 0.65, y: cy - tip_t },
            D2D_POINT_2F { x: cx + arm_h, y: cy },
            D2D_POINT_2F { x: cx + arm_h * 0.65, y: cy + tip_t },
            D2D_POINT_2F { x: cx + half_t, y: cy + half_t },
            D2D_POINT_2F { x: cx + tip_t, y: cy + arm_v * 0.65 },
            D2D_POINT_2F { x: cx, y: cy + arm_v },
            D2D_POINT_2F { x: cx - tip_t, y: cy + arm_v * 0.65 },
            D2D_POINT_2F { x: cx - half_t, y: cy + half_t },
            D2D_POINT_2F { x: cx - arm_h * 0.65, y: cy + tip_t },
            D2D_POINT_2F { x: cx - arm_h, y: cy },
            D2D_POINT_2F { x: cx - arm_h * 0.65, y: cy - tip_t },
            D2D_POINT_2F { x: cx - half_t, y: cy - half_t },
            D2D_POINT_2F { x: cx - tip_t, y: cy - arm_v * 0.65 },
        ];

        sink.AddLines(&pts);
        sink.EndFigure(D2D1_FIGURE_END_CLOSED);
        sink.Close()?;
        Ok(geom)
    }

    unsafe fn draw_sharp_upper_eyelid_mask(
        &self,
        factory: &ID2D1Factory,
        rt: &ID2D1HwndRenderTarget,
        ex: f32,
        ey: f32,
        rx: f32,
        ry: f32,
        side: f32,
        alpha_mult: f32,
    ) {
        let outer_x = ex + side * rx;
        let inner_x = ex - side * rx;
        let outer_y = ey - ry * 0.92;
        let inner_y = ey - ry * 0.42;

        if let Ok(geom) = factory.CreatePathGeometry() {
            if let Ok(sink) = geom.Open() {
                sink.BeginFigure(
                    D2D_POINT_2F { x: outer_x, y: outer_y },
                    D2D1_FIGURE_BEGIN_FILLED,
                );
                sink.AddLines(&[
                    D2D_POINT_2F { x: inner_x, y: inner_y },
                    D2D_POINT_2F { x: inner_x, y: ey - ry - 4.0 },
                    D2D_POINT_2F { x: outer_x, y: ey - ry - 4.0 },
                ]);
                sink.EndFigure(D2D1_FIGURE_END_CLOSED);
                let _ = sink.Close();

                let mask_color = D2D1_COLOR_F {
                    r: 0.043,
                    g: 0.055,
                    b: 0.078,
                    a: alpha_mult,
                };
                if let Ok(brush) = rt.CreateSolidColorBrush(&mask_color, None) {
                    rt.FillGeometry(&geom, &brush, None);
                }
            }
        }
    }
}