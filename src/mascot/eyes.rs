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
    /// Sesuai Referensi 2 (image_fab516): Devil Union dengan Mahkota 5-Salib Merah, Tanduk, Tanda Pipi & Tatapan Serius
    DevilUnionSerious { timer: f32 },
    /// Devil Union + Aksi Pedang Demon-Slayer Melayang & Tangan Asta Menangkap Gagangnya
    SwordSummonGrab { timer: f32 },
    /// Sesuai Referensi 1 (image_fa62bb): Asta Cheer Mode (Rambut Ash-Grey, Mata Olive-Emerald Besar, Mulut Senyum ':D')
    NormalEmeraldCheer { timer: f32 },
    /// Ekspresi Senang Mata Melengkung '^ ^'
    HappySmileGesture { timer: f32 },
    /// Istirahat Tenang
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
    /// 0.0 = Devil Union (Referensi 2), 1.0 = Asta Cheer Olive-Emerald (Referensi 1)
    pub cheer_blend: f32,
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
            openness: 0.2,
            blink_timer: 0.0,
            next_blink_in: 3.0,
            glance_timer: 1.8,
            aura_intensity: 1.0,
            pupil_scale: 1.0,
            cheer_blend: 0.0,
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
            rng_seed: 0xA57A_2026,
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
        let rx = (self.next_f32() - 0.5) * 32.0 + side * 20.0;
        let ry = (self.next_f32() - 0.5) * 8.0;
        let vx = (self.next_f32() - 0.5) * 10.0 + side * 3.0;
        let vy = -7.0 - self.next_f32() * 12.0;
        let max_life = 0.50 + self.next_f32() * 0.55;
        let life = if random_life {
            self.next_f32() * max_life
        } else {
            0.0
        };
        let size = 1.2 + self.next_f32() * 1.8;
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
        self.openness = 0.2;
        self.aura_intensity = 1.25;
    }

    pub fn trigger_wake_surge(&mut self) {
        self.mode = AstaIdleMode::NormalEmeraldCheer { timer: 0.0 };
        self.aura_intensity = 1.15;
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
                    self.mode = AstaIdleMode::NormalEmeraldCheer { timer: 0.0 };
                }
            }
        }

        if !cursor_near {
            self.glance_timer -= dt;
            if self.glance_timer <= 0.0 {
                let r = self.next_f32();
                if r < 0.38 {
                    self.target_look_x = 0.0;
                    self.target_look_y = 0.0;
                } else {
                    self.target_look_x = (self.next_f32() - 0.5) * 1.35;
                    self.target_look_y = (self.next_f32() - 0.5) * 0.50;
                }
                self.glance_timer = 1.8 + self.next_f32() * 2.4;
            }
        }

        let target_openness: f32;
        let target_aura: f32;
        let target_pupil: f32;
        let target_cheer: f32;

        match &mut self.mode {
            AstaIdleMode::DevilUnionSerious { timer } => {
                *timer += dt;
                target_openness = 0.98;
                target_cheer = 0.0;
                target_aura = if cursor_near {
                    1.20
                } else {
                    0.90 + 0.12 * (self.time * 3.2).sin()
                };
                target_pupil = if cursor_near { 0.88 } else { 1.0 };

                if *timer >= 6.8 {
                    if self.next_f32() < 0.58 {
                        self.mode = AstaIdleMode::SwordSummonGrab { timer: 0.0 };
                    } else {
                        self.mode = AstaIdleMode::NormalEmeraldCheer { timer: 0.0 };
                    }
                }
            }
            AstaIdleMode::SwordSummonGrab { timer } => {
                *timer += dt;
                target_openness = 1.02;
                target_cheer = 0.0;
                target_aura = 1.30;
                target_pupil = 0.84;

                let p = (*timer / 3.8).clamp(0.0, 1.0);
                self.target_look_x = 0.85 * (1.0 - (p - 0.5).abs() * 1.2).clamp(0.2, 1.0);
                self.target_look_y = -0.10 + (self.time * 4.0).sin() * 0.16;

                if *timer >= 3.8 {
                    self.mode = AstaIdleMode::NormalEmeraldCheer { timer: 0.0 };
                }
            }
            AstaIdleMode::NormalEmeraldCheer { timer } => {
                *timer += dt;
                target_openness = 1.06;
                target_cheer = 1.0;
                target_aura = 0.50;
                target_pupil = 1.05;

                if *timer >= 4.5 {
                    self.mode = AstaIdleMode::HappySmileGesture { timer: 0.0 };
                }
            }
            AstaIdleMode::HappySmileGesture { timer } => {
                *timer += dt;
                target_openness = 1.0;
                target_cheer = 1.0;
                target_aura = 0.65;
                target_pupil = 1.0;

                if *timer >= 2.2 {
                    self.mode = AstaIdleMode::DevilUnionSerious { timer: 0.0 };
                }
            }
            AstaIdleMode::Dozing { timer } => {
                *timer += dt;
                target_openness = 0.30 + 0.04 * (self.time * 1.4).sin();
                target_cheer = 0.0;
                target_aura = 0.35;
                target_pupil = 1.0;
            }
        }

        self.next_blink_in -= dt;
        if self.next_blink_in <= 0.0 {
            self.blink_timer = 0.13;
            self.next_blink_in = 2.7 + self.next_f32() * 3.3;
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
        self.cheer_blend += (target_cheer - self.cheer_blend) * (9.0 * dt).min(1.0);

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
        let center_y = (pill_rect.top + pill_rect.bottom) * 0.5 + 2.5;

        let sword_shift = if let AstaIdleMode::SwordSummonGrab { timer } = self.mode {
            let s = (timer * 2.5).min(1.0) * ((3.8 - timer) * 2.5).clamp(0.0, 1.0);
            -14.0 * s
        } else {
            0.0
        };

        let face_cx = center_x + sword_shift;
        let eye_spacing = 17.5;
        let cb = self.cheer_blend.clamp(0.0, 1.0);

        unsafe {
            self.draw_anti_magic_sparks(rt, face_cx, center_y, alpha_mult);

            // 1. Gambar Elemen Kepala / Dahi Sesuai Mode (Devil Union Crown & Horns vs Cheer Ash-Grey Hair)
            if cb < 0.55 {
                self.draw_devil_union_crown_and_horns(
                    factory,
                    rt,
                    face_cx,
                    pill_rect.top,
                    center_y,
                    eye_spacing,
                    alpha_mult * (1.0 - cb),
                );
            } else {
                self.draw_cheer_ash_grey_bangs_and_smile(
                    factory,
                    rt,
                    face_cx,
                    pill_rect.top,
                    center_y,
                    pill_rect.bottom,
                    alpha_mult * cb,
                );
            }

            // 2. Gambar Mata (Happy Arc '^ ^' atau Full Anime Reference Eye)
            if let AstaIdleMode::HappySmileGesture { timer } = self.mode {
                let bounce = (timer * 10.0).sin().abs() * -1.5;
                for side in [-1.0_f32, 1.0_f32] {
                    self.draw_happy_arc_eye(
                        factory,
                        rt,
                        face_cx + side * eye_spacing,
                        center_y + bounce,
                        side,
                        alpha_mult,
                    );
                }
            } else {
                for side in [-1.0_f32, 1.0_f32] {
                    self.draw_reference_anime_eye(
                        factory,
                        rt,
                        face_cx + side * eye_spacing,
                        center_y,
                        side,
                        alpha_mult,
                    );
                }
            }

            // 3. Jika Mode SwordSummonGrab: Gambar Pedang Demon-Slayer Melayang & Tangan Asta Menangkap!
            if let AstaIdleMode::SwordSummonGrab { timer } = self.mode {
                self.draw_floating_sword_and_grabbing_hand(
                    factory,
                    rt,
                    center_x + 40.0,
                    center_y - 1.5,
                    pill_rect.bottom,
                    timer,
                    alpha_mult,
                );
            }
        }
    }

    /// Sesuai Referensi 2 (`image_fab516.png`):
    /// Mahkota hitam bergerigi dengan deretan 5 bintang/salib merah menyala di dahi + Tanduk melengkung + Tanda silang hitam di pipi
    unsafe fn draw_devil_union_crown_and_horns(
        &self,
        factory: &ID2D1Factory,
        rt: &ID2D1HwndRenderTarget,
        cx: f32,
        top_y: f32,
        eye_y: f32,
        eye_spacing: f32,
        alpha: f32,
    ) {
        let horn_dark = D2D1_COLOR_F {
            r: 0.14,
            g: 0.15,
            b: 0.20,
            a: alpha,
        };
        let horn_ring = D2D1_COLOR_F {
            r: 0.34,
            g: 0.38,
            b: 0.48,
            a: 0.85 * alpha,
        };
        let crimson_glow = D2D1_COLOR_F {
            r: 1.0,
            g: 0.08,
            b: 0.14,
            a: alpha,
        };
        let scar_black = D2D1_COLOR_F {
            r: 0.12,
            g: 0.14,
            b: 0.18,
            a: 0.95 * alpha,
        };

        let Ok(horn_b) = rt.CreateSolidColorBrush(&horn_dark, None) else { return };
        let Ok(ring_b) = rt.CreateSolidColorBrush(&horn_ring, None) else { return };
        let Ok(red_b) = rt.CreateSolidColorBrush(&crimson_glow, None) else { return };
        let Ok(scar_b) = rt.CreateSolidColorBrush(&scar_black, None) else { return };

        // 1. Sepasang Tanduk Iblis Melengkung di Kiri & Kanan (`image_fab516.png`)
        for side in [-1.0_f32, 1.0_f32] {
            let hx = cx + side * (eye_spacing + 14.0);
            if let Ok(h_geom) = factory.CreatePathGeometry() {
                if let Ok(sink) = h_geom.Open() {
                    sink.BeginFigure(
                        D2D_POINT_2F {
                            x: hx - side * 3.0,
                            y: top_y + 16.0,
                        },
                        D2D1_FIGURE_BEGIN_FILLED,
                    );
                    sink.AddBezier(&D2D1_BEZIER_SEGMENT {
                        point1: D2D_POINT_2F {
                            x: hx + side * 6.0,
                            y: top_y + 11.0,
                        },
                        point2: D2D_POINT_2F {
                            x: hx + side * 8.5,
                            y: top_y + 4.0,
                        },
                        point3: D2D_POINT_2F {
                            x: hx + side * 4.5,
                            y: top_y + 1.0,
                        },
                    });
                    sink.AddLine(D2D_POINT_2F {
                        x: hx - side * 1.5,
                        y: top_y + 8.0,
                    });
                    sink.EndFigure(D2D1_FIGURE_END_CLOSED);
                    let _ = sink.Close();
                    rt.FillGeometry(&h_geom, &horn_b, None);
                    rt.DrawGeometry(&h_geom, &ring_b, 0.9, None);
                }
            }
        }

        // 2. Deretan 5 Bintang/Salib Merah Menyala di Dahi Khas Devil Union (`image_fab516.png`)
        let crown_y = top_y + 7.2;
        // Garis horizontal penghubung cahaya merah
        rt.DrawLine(
            D2D_POINT_2F {
                x: cx - 22.0,
                y: crown_y,
            },
            D2D_POINT_2F {
                x: cx + 22.0,
                y: crown_y,
            },
            &red_b,
            2.2,
            None,
        );

        // 5 Titik Salib/Bintang Vertikal pada Mahkota (Tengah paling tinggi)
        let star_offsets: [(f32, f32); 5] = [
            (-18.0, 2.8),
            (-9.5, 3.6),
            (0.0, 5.2),
            (9.5, 3.6),
            (18.0, 2.8),
        ];
        for (ox, half_h) in star_offsets {
            if let Ok(star_g) = self.create_tapered_plus_geometry(
                factory,
                cx + ox,
                crown_y,
                3.2,
                half_h,
                1.1,
            ) {
                rt.FillGeometry(&star_g, &red_b, None);
            }
        }

        // 3. Poni Hitam Runcing di Antara Kedua Mata (`image_fab516.png`)
        if let Ok(bang_g) = factory.CreatePathGeometry() {
            if let Ok(sink) = bang_g.Open() {
                sink.BeginFigure(
                    D2D_POINT_2F {
                        x: cx - 4.5,
                        y: crown_y + 3.5,
                    },
                    D2D1_FIGURE_BEGIN_FILLED,
                );
                sink.AddLines(&[
                    D2D_POINT_2F {
                        x: cx,
                        y: eye_y + 2.5,
                    },
                    D2D_POINT_2F {
                        x: cx + 4.5,
                        y: crown_y + 3.5,
                    },
                ]);
                sink.EndFigure(D2D1_FIGURE_END_CLOSED);
                let _ = sink.Close();
                rt.FillGeometry(&bang_g, &horn_b, None);
            }
        }

        // 4. Tanda Silang Hitam Tebal (Devil Union Cross-Scars) di Pipi Bawah Mata (`image_fab516.png`)
        for side in [-1.0_f32, 1.0_f32] {
            let ex = cx + side * eye_spacing;
            let sy = eye_y + 5.5;
            // Garis miring utama dari bawah mata ke pipi luar
            rt.DrawLine(
                D2D_POINT_2F {
                    x: ex + side * 1.5,
                    y: sy,
                },
                D2D_POINT_2F {
                    x: ex + side * 5.5,
                    y: sy + 6.5,
                },
                &scar_b,
                2.2,
                None,
            );
            // Garis silang pendek (crossbar)
            rt.DrawLine(
                D2D_POINT_2F {
                    x: ex + side * 0.5,
                    y: sy + 4.2,
                },
                D2D_POINT_2F {
                    x: ex + side * 6.2,
                    y: sy + 2.2,
                },
                &scar_b,
                1.8,
                None,
            );
        }
    }

    /// Sesuai Referensi 1 (`image_fa62bb.png`):
    /// Poni Rambut Ash-Grey Meruncing di Dahi + Mulut Terbuka Tersenyum Lebar (':D')
    unsafe fn draw_cheer_ash_grey_bangs_and_smile(
        &self,
        factory: &ID2D1Factory,
        rt: &ID2D1HwndRenderTarget,
        cx: f32,
        top_y: f32,
        eye_y: f32,
        bottom_y: f32,
        alpha: f32,
    ) {
        let ash_hair = D2D1_COLOR_F {
            r: 0.65,
            g: 0.68,
            b: 0.72,
            a: alpha,
        };
        let hair_line = D2D1_COLOR_F {
            r: 0.35,
            g: 0.38,
            b: 0.42,
            a: alpha,
        };
        let mouth_pink = D2D1_COLOR_F {
            r: 0.78,
            g: 0.45,
            b: 0.46,
            a: alpha,
        };
        let teeth_white = D2D1_COLOR_F {
            r: 0.96,
            g: 0.97,
            b: 0.99,
            a: alpha,
        };

        let Ok(hair_b) = rt.CreateSolidColorBrush(&ash_hair, None) else { return };
        let Ok(line_b) = rt.CreateSolidColorBrush(&hair_line, None) else { return };
        let Ok(mouth_b) = rt.CreateSolidColorBrush(&mouth_pink, None) else { return };
        let Ok(teeth_b) = rt.CreateSolidColorBrush(&teeth_white, None) else { return };

        // 1. Helai Poni Ash-Grey di Dahi Atas (`image_fa62bb.png`)
        if let Ok(b_geom) = factory.CreatePathGeometry() {
            if let Ok(sink) = b_geom.Open() {
                sink.BeginFigure(
                    D2D_POINT_2F {
                        x: cx - 22.0,
                        y: top_y,
                    },
                    D2D1_FIGURE_BEGIN_FILLED,
                );
                sink.AddLines(&[
                    D2D_POINT_2F {
                        x: cx - 14.0,
                        y: top_y + 9.5,
                    },
                    D2D_POINT_2F {
                        x: cx - 7.0,
                        y: top_y + 4.5,
                    },
                    D2D_POINT_2F {
                        x: cx,
                        y: eye_y - 2.0, // Helai V tengah khas Asta
                    },
                    D2D_POINT_2F {
                        x: cx + 7.0,
                        y: top_y + 4.5,
                    },
                    D2D_POINT_2F {
                        x: cx + 14.0,
                        y: top_y + 9.5,
                    },
                    D2D_POINT_2F {
                        x: cx + 22.0,
                        y: top_y,
                    },
                ]);
                sink.EndFigure(D2D1_FIGURE_END_CLOSED);
                let _ = sink.Close();
                rt.FillGeometry(&b_geom, &hair_b, None);
                rt.DrawGeometry(&b_geom, &line_b, 0.9, None);
            }
        }

        // 2. Mulut Terbuka Tersenyum Ceria (':D') di Bagian Tengah Bawah (`image_fa62bb.png`)
        let my = (eye_y + bottom_y) * 0.5 + 3.2;
        if let Ok(m_geom) = factory.CreatePathGeometry() {
            if let Ok(sink) = m_geom.Open() {
                sink.BeginFigure(
                    D2D_POINT_2F {
                        x: cx - 5.2,
                        y: my - 1.5,
                    },
                    D2D1_FIGURE_BEGIN_FILLED,
                );
                sink.AddLine(D2D_POINT_2F {
                    x: cx + 5.2,
                    y: my - 2.0,
                });
                sink.AddBezier(&D2D1_BEZIER_SEGMENT {
                    point1: D2D_POINT_2F {
                        x: cx + 5.0,
                        y: my + 4.2,
                    },
                    point2: D2D_POINT_2F {
                        x: cx - 5.0,
                        y: my + 4.2,
                    },
                    point3: D2D_POINT_2F {
                        x: cx - 5.2,
                        y: my - 1.5,
                    },
                });
                sink.EndFigure(D2D1_FIGURE_END_CLOSED);
                let _ = sink.Close();
                rt.FillGeometry(&m_geom, &mouth_b, None);
            }
        }
        // Garis gigi putih tipis di bagian atas mulut
        rt.DrawLine(
            D2D_POINT_2F {
                x: cx - 4.2,
                y: my - 1.2,
            },
            D2D_POINT_2F {
                x: cx + 4.2,
                y: my - 1.6,
            },
            &teeth_b,
            1.3,
            None,
        );
    }

    /// Gambar Mata Anime Asta Sesuai Kedua Gambar Referensi (`image_fa62bb.png` & `image_fab516.png`)
    unsafe fn draw_reference_anime_eye(
        &self,
        factory: &ID2D1Factory,
        rt: &ID2D1HwndRenderTarget,
        base_x: f32,
        base_y: f32,
        side: f32,
        alpha: f32,
    ) {
        let cb = self.cheer_blend.clamp(0.0, 1.0);
        let open = self.openness.clamp(0.08, 1.12);

        let w = 9.4;
        let h = (5.6 * open).max(0.8);

        // Kemiringan kelopak atas: Tajam mengernyit saat Devil Union (cb=0), Bulat besar saat Cheer (cb=1)
        let inner_drop = (1.0 - cb) * 2.4;
        let outer_lift = (1.0 - cb) * 1.2 + cb * 0.6;

        let inner_pt = D2D_POINT_2F {
            x: base_x - side * w,
            y: base_y + 0.5 + inner_drop * 0.35,
        };
        let outer_pt = D2D_POINT_2F {
            x: base_x + side * w,
            y: base_y - outer_lift * 0.4,
        };

        // 1. Alis Anime (Eyebrow): Tebal pendek ash-grey saat Cheer (`image_fa62bb`), Tajam menyatu saat Devil Union (`image_fab516`)
        let brow_col = D2D1_COLOR_F {
            r: 0.25 * (1.0 - cb) + 0.72 * cb,
            g: 0.27 * (1.0 - cb) + 0.75 * cb,
            b: 0.34 * (1.0 - cb) + 0.78 * cb,
            a: 0.92 * alpha,
        };
        if let Ok(bb) = rt.CreateSolidColorBrush(&brow_col, None) {
            let by_in = base_y - h - 1.5 + (1.0 - cb) * 2.2 - cb * 1.8;
            let by_out = base_y - h - 3.5;
            rt.DrawLine(
                D2D_POINT_2F {
                    x: base_x - side * (w * 0.75),
                    y: by_in,
                },
                D2D_POINT_2F {
                    x: base_x + side * (w * 0.65),
                    y: by_out,
                },
                &bb,
                2.0,
                None,
            );
        }

        if open < 0.16 {
            let lash_col = D2D1_COLOR_F {
                r: 0.90,
                g: 0.92,
                b: 0.96,
                a: 0.90 * alpha,
            };
            if let Ok(lb) = rt.CreateSolidColorBrush(&lash_col, None) {
                rt.DrawLine(inner_pt, outer_pt, &lb, 2.2, None);
            }
            return;
        }

        // 2. Sclera Putih Lebar Kontras (Sesuai Kedua Gambar Referensi)
        if let Ok(sclera_geom) = factory.CreatePathGeometry() {
            if let Ok(sink) = sclera_geom.Open() {
                sink.BeginFigure(inner_pt, D2D1_FIGURE_BEGIN_FILLED);
                sink.AddBezier(&D2D1_BEZIER_SEGMENT {
                    point1: D2D_POINT_2F {
                        x: base_x - side * (w * 0.35),
                        y: base_y - h + inner_drop,
                    },
                    point2: D2D_POINT_2F {
                        x: base_x + side * (w * 0.45),
                        y: base_y - h - outer_lift,
                    },
                    point3: outer_pt,
                });
                sink.AddBezier(&D2D1_BEZIER_SEGMENT {
                    point1: D2D_POINT_2F {
                        x: base_x + side * (w * 0.45),
                        y: base_y + h * 0.95,
                    },
                    point2: D2D_POINT_2F {
                        x: base_x - side * (w * 0.35),
                        y: base_y + h * 0.90,
                    },
                    point3: inner_pt,
                });
                sink.EndFigure(D2D1_FIGURE_END_CLOSED);
                let _ = sink.Close();

                let sclera_white = D2D1_COLOR_F {
                    r: 0.94,
                    g: 0.95,
                    b: 0.97,
                    a: 0.98 * alpha,
                };
                if let Ok(sb) = rt.CreateSolidColorBrush(&sclera_white, None) {
                    rt.FillGeometry(&sclera_geom, &sb, None);
                }
            }
        }

        // 3. Iris: Crimson Red Bulat Tajam (`image_fab516`) vs Olive-Emerald Besar (`image_fa62bb`)
        let shift_x = self.look_x * 2.8;
        let shift_y = self.look_y * 1.4;
        let ex = base_x + shift_x;
        let ey = base_y + shift_y;

        // Pada Devil Union (`image_fab516`), iris merah berukuran medium-kecil di tengah sklera putih sehingga terlihat intens!
        // Pada Cheer (`image_fa62bb`), iris Olive-Green berukuran besar memenuhi tinggi mata!
        let iris_rx = 3.8 * (1.0 - cb) + 5.2 * cb;
        let iris_ry = (3.8 * (1.0 - cb) + 5.2 * cb) * open;

        let c0 = D2D1_COLOR_F {
            r: 0.98 * (1.0 - cb) + 0.62 * cb,
            g: 0.14 * (1.0 - cb) + 0.82 * cb,
            b: 0.18 * (1.0 - cb) + 0.36 * cb,
            a: alpha,
        };
        let c1 = D2D1_COLOR_F {
            r: 0.86 * (1.0 - cb) + 0.46 * cb,
            g: 0.04 * (1.0 - cb) + 0.68 * cb,
            b: 0.08 * (1.0 - cb) + 0.24 * cb,
            a: alpha,
        };
        let c2 = D2D1_COLOR_F {
            r: 0.45 * (1.0 - cb) + 0.24 * cb,
            g: 0.02 * (1.0 - cb) + 0.42 * cb,
            b: 0.04 * (1.0 - cb) + 0.14 * cb,
            a: alpha,
        };

        let iris_stops = [
            D2D1_GRADIENT_STOP {
                position: 0.0,
                color: c0,
            },
            D2D1_GRADIENT_STOP {
                position: 0.70,
                color: c1,
            },
            D2D1_GRADIENT_STOP {
                position: 1.0,
                color: c2,
            },
        ];

        if let Ok(sc) =
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
            if let Ok(ib) = rt.CreateRadialGradientBrush(&props, None, &sc) {
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F { x: ex, y: ey },
                        radiusX: iris_rx,
                        radiusY: iris_ry,
                    },
                    &ib,
                );
            }
        }

        // 4. Pupil & Highlight Sesuai Referensi
        let pupil_dark = D2D1_COLOR_F {
            r: 0.05,
            g: 0.06,
            b: 0.09,
            a: alpha,
        };
        if cb < 0.55 {
            // Pupil '+' Devil Union
            if let Ok(plus_g) =
                self.create_tapered_plus_geometry(factory, ex, ey, 2.8, 2.8 * open, 0.85)
            {
                if let Ok(pb) = rt.CreateSolidColorBrush(&pupil_dark, None) {
                    rt.FillGeometry(&plus_g, &pb, None);
                }
            }
        } else {
            // Pupil vertikal kecil di tengah iris Olive-Green + Highlight putih bulat di kiri-atas (`image_fa62bb.png`)
            if let Ok(pb) = rt.CreateSolidColorBrush(&pupil_dark, None) {
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F { x: ex, y: ey },
                        radiusX: 1.15,
                        radiusY: 1.85 * open,
                    },
                    &pb,
                );
            }
            let glint_white = D2D1_COLOR_F {
                r: 0.99,
                g: 0.99,
                b: 1.0,
                a: 0.96 * alpha,
            };
            if let Ok(gb) = rt.CreateSolidColorBrush(&glint_white, None) {
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F {
                            x: ex - 1.8,
                            y: ey - iris_ry * 0.38,
                        },
                        radiusX: 1.55,
                        radiusY: 1.35 * open,
                    },
                    &gb,
                );
            }
        }

        // 5. Eyeliner / Garis Kelopak Atas & Bawah Hitam Tegas Khas Anime (`image_fa62bb` & `image_fab516`)
        if let Ok(lash_geom) = factory.CreatePathGeometry() {
            if let Ok(sink) = lash_geom.Open() {
                sink.BeginFigure(inner_pt, D2D1_FIGURE_BEGIN_HOLLOW);
                sink.AddBezier(&D2D1_BEZIER_SEGMENT {
                    point1: D2D_POINT_2F {
                        x: base_x - side * (w * 0.35),
                        y: base_y - h + inner_drop,
                    },
                    point2: D2D_POINT_2F {
                        x: base_x + side * (w * 0.45),
                        y: base_y - h - outer_lift,
                    },
                    point3: outer_pt,
                });
                sink.EndFigure(D2D1_FIGURE_END_OPEN);
                let _ = sink.Close();

                let liner_col = D2D1_COLOR_F {
                    r: 0.08,
                    g: 0.09,
                    b: 0.12,
                    a: alpha,
                };
                if let Ok(lb) = rt.CreateSolidColorBrush(&liner_col, None) {
                    rt.DrawGeometry(&lash_geom, &lb, 2.5, None);
                }
            }
        }
    }

    unsafe fn draw_happy_arc_eye(
        &self,
        factory: &ID2D1Factory,
        rt: &ID2D1HwndRenderTarget,
        ex: f32,
        ey: f32,
        _side: f32,
        alpha: f32,
    ) {
        if let Ok(geom) = factory.CreatePathGeometry() {
            if let Ok(sink) = geom.Open() {
                sink.BeginFigure(
                    D2D_POINT_2F {
                        x: ex - 7.0,
                        y: ey + 2.0,
                    },
                    D2D1_FIGURE_BEGIN_HOLLOW,
                );
                sink.AddBezier(&D2D1_BEZIER_SEGMENT {
                    point1: D2D_POINT_2F {
                        x: ex - 3.2,
                        y: ey - 5.5,
                    },
                    point2: D2D_POINT_2F {
                        x: ex + 3.2,
                        y: ey - 5.5,
                    },
                    point3: D2D_POINT_2F {
                        x: ex + 7.0,
                        y: ey + 2.0,
                    },
                });
                sink.EndFigure(D2D1_FIGURE_END_OPEN);
                let _ = sink.Close();

                let olive_col = D2D1_COLOR_F {
                    r: 0.55,
                    g: 0.82,
                    b: 0.32,
                    a: alpha,
                };
                if let Ok(b) = rt.CreateSolidColorBrush(&olive_col, None) {
                    rt.DrawGeometry(&geom, &b, 2.5, None);
                }
            }
        }
    }

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
        let hover_bob = (timer * 6.5).sin() * 2.0;
        let retract = if timer > 2.7 {
            ((timer - 2.7) / 1.1).clamp(0.0, 1.0).powi(2) * 18.0
        } else {
            0.0
        };

        let sword_x = sx;
        let sword_y = sy + hover_bob + retract;

        let blade_dark = D2D1_COLOR_F {
            r: 0.16,
            g: 0.19,
            b: 0.26,
            a: alpha,
        };
        let blade_edge = D2D1_COLOR_F {
            r: 0.56,
            g: 0.64,
            b: 0.78,
            a: alpha * 0.9,
        };
        let crimson_core = D2D1_COLOR_F {
            r: 0.88,
            g: 0.16,
            b: 0.24,
            a: alpha * 0.9,
        };
        let hand_obsidian = D2D1_COLOR_F {
            r: 0.10,
            g: 0.12,
            b: 0.18,
            a: alpha,
        };

        let Ok(blade_b) = rt.CreateSolidColorBrush(&blade_dark, None) else { return };
        let Ok(edge_b) = rt.CreateSolidColorBrush(&blade_edge, None) else { return };
        let Ok(core_b) = rt.CreateSolidColorBrush(&crimson_core, None) else { return };
        let Ok(hand_b) = rt.CreateSolidColorBrush(&hand_obsidian, None) else { return };

        if let Ok(sword_geom) = factory.CreatePathGeometry() {
            if let Ok(sink) = sword_geom.Open() {
                sink.BeginFigure(
                    D2D_POINT_2F {
                        x: sword_x,
                        y: sword_y - 12.0,
                    },
                    D2D1_FIGURE_BEGIN_FILLED,
                );
                sink.AddLines(&[
                    D2D_POINT_2F {
                        x: sword_x + 4.0,
                        y: sword_y - 7.0,
                    },
                    D2D_POINT_2F {
                        x: sword_x + 3.2,
                        y: sword_y + 3.5,
                    },
                    D2D_POINT_2F {
                        x: sword_x - 3.2,
                        y: sword_y + 3.5,
                    },
                    D2D_POINT_2F {
                        x: sword_x - 4.0,
                        y: sword_y - 7.0,
                    },
                ]);
                sink.EndFigure(D2D1_FIGURE_END_CLOSED);
                let _ = sink.Close();
                rt.FillGeometry(&sword_geom, &blade_b, None);
                rt.DrawGeometry(&sword_geom, &edge_b, 0.9, None);
            }
        }

        rt.DrawLine(
            D2D_POINT_2F {
                x: sword_x,
                y: sword_y - 9.5,
            },
            D2D_POINT_2F {
                x: sword_x,
                y: sword_y + 3.0,
            },
            &core_b,
            1.2,
            None,
        );
        rt.DrawLine(
            D2D_POINT_2F {
                x: sword_x - 5.2,
                y: sword_y + 3.8,
            },
            D2D_POINT_2F {
                x: sword_x + 5.2,
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
                y: sword_y + 10.0,
            },
            &edge_b,
            2.0,
            None,
        );

        if timer >= 1.2 {
            let reach_progress = if timer < 2.0 {
                ((timer - 1.2) / 0.8).clamp(0.0, 1.0)
            } else {
                1.0
            };

            let hand_top_y = pill_bottom - (pill_bottom - (sword_y + 6.0)) * reach_progress;
            let arm_rect = D2D1_ROUNDED_RECT {
                rect: D2D_RECT_F {
                    left: sword_x - 3.0,
                    top: hand_top_y,
                    right: sword_x + 3.0,
                    bottom: pill_bottom,
                },
                radiusX: 2.0,
                radiusY: 2.0,
            };
            rt.FillRoundedRectangle(&arm_rect, &hand_b);
            rt.DrawRoundedRectangle(&arm_rect, &core_b, 0.9, None);
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

            let color = if self.cheer_blend > 0.5 {
                D2D1_COLOR_F {
                    r: 0.55,
                    g: 0.85,
                    b: 0.35,
                    a: spark_alpha * 0.50,
                }
            } else {
                match spark.kind {
                    0 => D2D1_COLOR_F {
                        r: 0.85,
                        g: 0.12,
                        b: 0.18,
                        a: spark_alpha * 0.70,
                    },
                    1 => D2D1_COLOR_F {
                        r: 0.48,
                        g: 0.56,
                        b: 0.72,
                        a: spark_alpha * 0.55,
                    },
                    _ => D2D1_COLOR_F {
                        r: 0.20,
                        g: 0.24,
                        b: 0.34,
                        a: spark_alpha * 0.75,
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
}