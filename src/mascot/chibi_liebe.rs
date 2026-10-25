// src/mascot/chibi_liebe.rs
use windows::Win32::Graphics::Direct2D::Common::{
    D2D1_COLOR_F, D2D1_FIGURE_BEGIN_FILLED, D2D1_FIGURE_END_CLOSED, D2D_POINT_2F, D2D_RECT_F,
};
use windows::Win32::Graphics::Direct2D::{
    ID2D1Factory, ID2D1HwndRenderTarget, D2D1_ELLIPSE, D2D1_ROUNDED_RECT,
};

use crate::notch::states::NotchState;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LiebeAction {
    /// Tidak tampil (untuk state Idle, Pet, Lyrics, Water, Todo)
    Hidden,
    /// State Music: Duduk santai sambil headbop mengikuti irama musik
    MusicHeadbop,
    /// State Pomodoro: Berdiri melipat tangan mengawasi fokus timer
    PomoFocusGuard,
    /// State Shelf: Mengintip dari bawah drop-zone siap menangkap file
    ShelfCatchingFiles,
}

pub struct ChibiLiebe {
    pub action: LiebeAction,
    pub time: f32,
    /// Timer saat Chibi Liebe diklik/di-poke oleh pengguna (efek loncat & kesal lucu)
    pub poked_timer: f32,
    /// Posisi pusat Chibi Liebe terakhir untuk deteksi klik
    pub last_center: (f32, f32),
}

impl Default for ChibiLiebe {
    fn default() -> Self {
        Self::new()
    }
}

impl ChibiLiebe {
    pub fn draw(
        &mut self,
        rt: &windows::Win32::Graphics::Direct2D::ID2D1HwndRenderTarget,
        factory: &windows::Win32::Graphics::Direct2D::ID2D1Factory,
        pill_rect: D2D_RECT_F,
        _state: NotchState,
    ) {
        self.render(factory, rt, pill_rect, 1.0);
    }
}

impl ChibiLiebe {
    pub fn new() -> Self {
        Self {
            action: LiebeAction::Hidden,
            time: 0.0,
            poked_timer: 0.0,
            last_center: (0.0, 0.0),
        }
    }

    /// Tentukan apakah Chibi Liebe aktif berdasarkan NotchState saat ini (HANYA 3 state)
    pub fn sync_with_state(&mut self, state: NotchState) {
        self.action = match state {
            NotchState::Music => LiebeAction::MusicHeadbop,
            NotchState::Pomo => LiebeAction::PomoFocusGuard,
            NotchState::Shelf => LiebeAction::ShelfCatchingFiles,
            _ => LiebeAction::Hidden,
        };
    }

    pub fn is_visible(&self) -> bool {
        self.action != LiebeAction::Hidden
    }

    pub fn update(&mut self, dt: f32) {
        self.time += dt.clamp(0.001, 0.05);
        if self.poked_timer > 0.0 {
            self.poked_timer = (self.poked_timer - dt).max(0.0);
        }
    }

    /// Cek apakah klik mouse mengenai tubuh Chibi Liebe (radius 22px)
    pub fn handle_click(&mut self, click_x: f32, click_y: f32) -> bool {
        if !self.is_visible() {
            return false;
        }
        let dx = click_x - self.last_center.0;
        let dy = click_y - self.last_center.1;
        if (dx * dx + dy * dy).sqrt() <= 22.0 {
            self.poked_timer = 0.85; // Animasi kesal/kaget selama 0.85 detik
            return true;
        }
        false
    }

    pub fn render(
        &mut self,
        factory: &ID2D1Factory,
        rt: &ID2D1HwndRenderTarget,
        pill_rect: D2D_RECT_F,
        alpha: f32,
    ) {
        if !self.is_visible() || alpha <= 0.05 {
            return;
        }

        // Tentukan koordinat anchor Chibi Liebe berdasarkan fitur aktif
        let (base_x, base_y) = match self.action {
            LiebeAction::MusicHeadbop => (pill_rect.right - 38.0, pill_rect.bottom - 34.0),
            LiebeAction::PomoFocusGuard => (pill_rect.right - 42.0, pill_rect.bottom - 36.0),
            LiebeAction::ShelfCatchingFiles => (
                (pill_rect.left + pill_rect.right) * 0.5 + 125.0,
                pill_rect.bottom - 30.0,
            ),
            LiebeAction::Hidden => return,
        };

        // Offset animasi per pose
        let (bob_x, bob_y, wing_flap) = if self.poked_timer > 0.0 {
            let jump = (self.poked_timer * 18.0).sin().abs() * -6.5;
            (0.0, jump, (self.time * 28.0).sin() * 5.0)
        } else {
            match self.action {
                LiebeAction::MusicHeadbop => {
                    // Mengangguk ritmis (~120 BPM)
                    let beat = (self.time * 6.2).sin();
                    (beat * 1.5, beat.abs() * -3.2, (self.time * 12.4).cos() * 2.5)
                }
                LiebeAction::PomoFocusGuard => {
                    // Bernapas tenang & mengawasi
                    let breath = (self.time * 2.4).sin();
                    (0.0, breath * -1.2, breath * 1.0)
                }
                LiebeAction::ShelfCatchingFiles => {
                    // Bergoyang antusias menunggu drop file
                    let sway = (self.time * 4.5).sin();
                    (sway * 2.2, (self.time * 9.0).cos() * -1.5, sway * 3.0)
                }
                LiebeAction::Hidden => (0.0, 0.0, 0.0),
            }
        };

        let cx = base_x + bob_x;
        let cy = base_y + bob_y;
        self.last_center = (cx, cy);

        unsafe {
            // Palet Chibi Liebe (Neutral Cool Grey, Ash Silver, Obsidian & Muted Wine)
            let obsidian_body = D2D1_COLOR_F { r: 0.09, g: 0.11, b: 0.16, a: alpha };
            let slate_outline = D2D1_COLOR_F { r: 0.32, g: 0.39, b: 0.52, a: alpha * 0.85 };
            let ash_silver_hair = D2D1_COLOR_F { r: 0.62, g: 0.68, b: 0.78, a: alpha };
            let skin_grey = D2D1_COLOR_F { r: 0.78, g: 0.82, b: 0.88, a: alpha };
            let horn_dark = D2D1_COLOR_F { r: 0.06, g: 0.07, b: 0.10, a: alpha };
            let muted_crimson = D2D1_COLOR_F { r: 0.72, g: 0.22, b: 0.30, a: alpha };

            let Ok(body_brush) = rt.CreateSolidColorBrush(&obsidian_body, None) else { return };
            let Ok(outline_brush) = rt.CreateSolidColorBrush(&slate_outline, None) else { return };
            let Ok(hair_brush) = rt.CreateSolidColorBrush(&ash_silver_hair, None) else { return };
            let Ok(skin_brush) = rt.CreateSolidColorBrush(&skin_grey, None) else { return };
            let Ok(horn_brush) = rt.CreateSolidColorBrush(&horn_dark, None) else { return };
            let Ok(eye_brush) = rt.CreateSolidColorBrush(&muted_crimson, None) else { return };

            // 1. Sayap Mungil & Ekor Panah di Belakang Tubuh
            for side in [-1.0_f32, 1.0_f32] {
                if let Ok(wing_geom) = factory.CreatePathGeometry() {
                    if let Ok(sink) = wing_geom.Open() {
                        sink.BeginFigure(
                            D2D_POINT_2F { x: cx + side * 4.0, y: cy + 5.0 },
                            D2D1_FIGURE_BEGIN_FILLED,
                        );
                        sink.AddLines(&[
                            D2D_POINT_2F {
                                x: cx + side * 13.5,
                                y: cy + 1.0 - wing_flap,
                            },
                            D2D_POINT_2F {
                                x: cx + side * 11.0,
                                y: cy + 8.5 - wing_flap * 0.5,
                            },
                            D2D_POINT_2F { x: cx + side * 4.5, y: cy + 9.0 },
                        ]);
                        sink.EndFigure(D2D1_FIGURE_END_CLOSED);
                        let _ = sink.Close();
                        rt.FillGeometry(&wing_geom, &body_brush, None);
                        rt.DrawGeometry(&wing_geom, &outline_brush, 0.9, None);
                    }
                }
            }

            // 2. Torso Mungil Chibi
            let torso_rect = D2D1_ROUNDED_RECT {
                rect: D2D_RECT_F {
                    left: cx - 5.5,
                    top: cy + 3.5,
                    right: cx + 5.5,
                    bottom: cy + 14.5,
                },
                radiusX: 3.5,
                radiusY: 3.5,
            };
            rt.FillRoundedRectangle(&torso_rect, &body_brush);
            rt.DrawRoundedRectangle(&torso_rect, &outline_brush, 0.9, None);

            // Pose tangan sesuai state
            match self.action {
                LiebeAction::ShelfCatchingFiles => {
                    // Kedua tangan diangkat ke atas siap menangkap file
                    for side in [-1.0_f32, 1.0_f32] {
                        rt.DrawLine(
                            D2D_POINT_2F { x: cx + side * 5.0, y: cy + 6.0 },
                            D2D_POINT_2F { x: cx + side * 10.5, y: cy - 1.0 },
                            &outline_brush,
                            2.2,
                            None,
                        );
                    }
                }
                LiebeAction::PomoFocusGuard => {
                    // Tangan dilipat di dada (crossed arms)
                    rt.DrawLine(
                        D2D_POINT_2F { x: cx - 4.5, y: cy + 7.5 },
                        D2D_POINT_2F { x: cx + 4.5, y: cy + 7.5 },
                        &outline_brush,
                        2.4,
                        None,
                    );
                }
                _ => {}
            }

            // 3. Kepala Chibi Bulat
            let head_ellipse = D2D1_ELLIPSE {
                point: D2D_POINT_2F { x: cx, y: cy - 2.5 },
                radiusX: 8.5,
                radiusY: 7.5,
            };
            rt.FillEllipse(&head_ellipse, &skin_brush);

            // 4. Rambut Runcing Ash-Silver Khas Liebe
            if let Ok(hair_geom) = factory.CreatePathGeometry() {
                if let Ok(sink) = hair_geom.Open() {
                    sink.BeginFigure(
                        D2D_POINT_2F { x: cx - 9.5, y: cy - 3.0 },
                        D2D1_FIGURE_BEGIN_FILLED,
                    );
                    sink.AddLines(&[
                        D2D_POINT_2F { x: cx - 11.5, y: cy - 7.5 },
                        D2D_POINT_2F { x: cx - 6.5, y: cy - 8.5 },
                        D2D_POINT_2F { x: cx - 4.0, y: cy - 12.0 },
                        D2D_POINT_2F { x: cx, y: cy - 9.5 },
                        D2D_POINT_2F { x: cx + 4.0, y: cy - 12.0 },
                        D2D_POINT_2F { x: cx + 6.5, y: cy - 8.5 },
                        D2D_POINT_2F { x: cx + 11.5, y: cy - 7.5 },
                        D2D_POINT_2F { x: cx + 9.5, y: cy - 3.0 },
                        D2D_POINT_2F { x: cx + 4.5, y: cy - 5.5 },
                        D2D_POINT_2F { x: cx, y: cy - 3.5 },
                        D2D_POINT_2F { x: cx - 4.5, y: cy - 5.5 },
                    ]);
                    sink.EndFigure(D2D1_FIGURE_END_CLOSED);
                    let _ = sink.Close();
                    rt.FillGeometry(&hair_geom, &hair_brush, None);
                }
            }

            // 5. Sepasang Tanduk Melengkung di Kepala
            for side in [-1.0_f32, 1.0_f32] {
                if let Ok(horn_geom) = factory.CreatePathGeometry() {
                    if let Ok(sink) = horn_geom.Open() {
                        sink.BeginFigure(
                            D2D_POINT_2F { x: cx + side * 4.5, y: cy - 8.5 },
                            D2D1_FIGURE_BEGIN_FILLED,
                        );
                        sink.AddLines(&[
                            D2D_POINT_2F { x: cx + side * 9.5, y: cy - 15.5 },
                            D2D_POINT_2F { x: cx + side * 7.5, y: cy - 8.0 },
                        ]);
                        sink.EndFigure(D2D1_FIGURE_END_CLOSED);
                        let _ = sink.Close();
                        rt.FillGeometry(&horn_geom, &horn_brush, None);
                        rt.DrawGeometry(&horn_geom, &outline_brush, 0.8, None);
                    }
                }
            }

            // 6. Mata Deadpan/Smug & Garis Pipi Khas Liebe
            for side in [-1.0_f32, 1.0_f32] {
                let ex = cx + side * 3.4;
                let ey = cy - 1.5;
                rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F { x: ex, y: ey },
                        radiusX: 1.5,
                        radiusY: if self.poked_timer > 0.0 { 1.8 } else { 1.1 },
                    },
                    &eye_brush,
                );
                // Garis hitam vertikal tipis di bawah mata khas Liebe
                rt.DrawLine(
                    D2D_POINT_2F { x: ex, y: ey + 1.2 },
                    D2D_POINT_2F { x: ex, y: ey + 3.4 },
                    &horn_brush,
                    0.9,
                    None,
                );
            }

            // 7. Jika baru diklik (Poked): gambar simbol kesal lucu di atas kepala
            if self.poked_timer > 0.0 {
                let ax = cx + 9.0;
                let ay = cy - 13.0;
                rt.DrawLine(
                    D2D_POINT_2F { x: ax - 2.5, y: ay },
                    D2D_POINT_2F { x: ax + 2.5, y: ay },
                    &eye_brush,
                    1.3,
                    None,
                );
                rt.DrawLine(
                    D2D_POINT_2F { x: ax, y: ay - 2.5 },
                    D2D_POINT_2F { x: ax, y: ay + 2.5 },
                    &eye_brush,
                    1.3,
                    None,
                );
            }
        }
    }
}
