// src/notch/renderer.rs
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Direct2D::Common::{
    D2D1_ALPHA_MODE_PREMULTIPLIED, D2D1_COLOR_F, D2D1_PIXEL_FORMAT, D2D_POINT_2F, D2D_RECT_F,
    D2D_SIZE_U,
};
use windows::Win32::Graphics::Direct2D::*;
use windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_B8G8R8A8_UNORM;
use windows::Win32::Graphics::Gdi::{BeginPaint, EndPaint, PAINTSTRUCT};

use super::states::{NotchController, NotchState};

/// Konstanta Palet Tema Neutral Slate-Blue Anti-Magic
pub struct AntiMagicNeutralPalette;

impl AntiMagicNeutralPalette {
    /// Obsidian Slate Background (#0B0E14)
    pub const PILL_BG: D2D1_COLOR_F = D2D1_COLOR_F {
        r: 0.043,
        g: 0.055,
        b: 0.078,
        a: 0.98,
    };
    /// Dark Slate Card / Inner Surface (#131822)
    pub const CARD_SURFACE: D2D1_COLOR_F = D2D1_COLOR_F {
        r: 0.075,
        g: 0.094,
        b: 0.133,
        a: 0.94,
    };
    /// Cool Steel Border (#263042)
    pub const STEEL_BORDER: D2D1_COLOR_F = D2D1_COLOR_F {
        r: 0.180,
        g: 0.230,
        b: 0.320,
        a: 0.70,
    };
    /// Steel Blue Accent (#6488EA)
    pub const ACCENT_BLUE: D2D1_COLOR_F = D2D1_COLOR_F {
        r: 0.392,
        g: 0.533,
        b: 0.918,
        a: 0.95,
    };
}

pub struct NotchRenderer {
    pub factory: ID2D1Factory,
    pub rt: ID2D1HwndRenderTarget,
}

impl NotchRenderer {
    pub fn new(hwnd: HWND, width: u32, height: u32) -> windows::core::Result<Self> {
        unsafe {
            let factory: ID2D1Factory =
                D2D1CreateFactory(D2D1_FACTORY_TYPE_SINGLE_THREADED, None)?;

            let render_props = D2D1_RENDER_TARGET_PROPERTIES {
                r#type: D2D1_RENDER_TARGET_TYPE_DEFAULT,
                pixelFormat: D2D1_PIXEL_FORMAT {
                    format: DXGI_FORMAT_B8G8R8A8_UNORM,
                    alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED,
                },
                dpiX: 96.0,
                dpiY: 96.0,
                usage: D2D1_RENDER_TARGET_USAGE_NONE,
                minLevel: D2D1_FEATURE_LEVEL_DEFAULT,
            };

            let hwnd_props = D2D1_HWND_RENDER_TARGET_PROPERTIES {
                hwnd,
                pixelSize: D2D_SIZE_U {
                    width: width.max(100),
                    height: height.max(50),
                },
                presentOptions: D2D1_PRESENT_OPTIONS_IMMEDIATELY,
            };

            let rt = factory.CreateHwndRenderTarget(&render_props, &hwnd_props)?;
            rt.SetAntialiasMode(D2D1_ANTIALIAS_MODE_PER_PRIMITIVE);

            Ok(Self { factory, rt })
        }
    }

    pub fn render_frame(&self, controller: &mut NotchController, pill_rect: D2D_RECT_F) {
        unsafe {
            let mut ps = PAINTSTRUCT::default();
            let hwnd = self.rt.GetHwnd();
            let _ = BeginPaint(hwnd, &mut ps);

            self.rt.BeginDraw();

            // 1. Bersihkan seluruh canvas dengan alpha 0.0 (DWM glass composition)
            self.rt.Clear(Some(&D2D1_COLOR_F {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            }));

            let rounded_pill = D2D1_ROUNDED_RECT {
                rect: pill_rect,
                radiusX: controller.corner_radius.value,
                radiusY: controller.corner_radius.value,
            };

            // 2. Gambar latar belakang pill Obsidian Slate (#0B0E14)
            if let Ok(bg_brush) = self
                .rt
                .CreateSolidColorBrush(&AntiMagicNeutralPalette::PILL_BG, None)
            {
                self.rt.FillRoundedRectangle(&rounded_pill, &bg_brush);
            }

            // 3. Gambar tepi luar (Cool Steel Grey dengan denyut Slate-Wine halus saat mode Pet)
            let border_color = if controller.state == NotchState::Pet {
                let pulse = controller.devil_eyes.aura_intensity.clamp(0.4, 1.2);
                D2D1_COLOR_F {
                    r: 0.28 * pulse,
                    g: 0.25,
                    b: 0.38,
                    a: 0.65,
                }
            } else {
                AntiMagicNeutralPalette::STEEL_BORDER
            };

            if let Ok(border_brush) = self.rt.CreateSolidColorBrush(&border_color, None) {
                self.rt
                    .DrawRoundedRectangle(&rounded_pill, &border_brush, 1.25, None);
            }

            // 4. Render konten berdasarkan state aktif
            if controller.state == NotchState::Pet && !controller.minimal_mode {
                let collapse_factor = ((60.0 - controller.height.value) / 26.0).clamp(0.0, 1.0);
                controller.devil_eyes.render(
                    &self.factory,
                    &self.rt,
                    pill_rect,
                    collapse_factor,
                );
            } else if controller.height.value > 65.0 {
                let content_alpha = controller.content_alpha.value.clamp(0.0, 1.0);

                // Gambar bar navigasi tab atas + kartu dalam
                self.draw_expanded_panel_shell(controller.state, pill_rect, content_alpha);

                // Render Chibi Liebe secara selektif (HANYA tampil pada Music, Pomo, atau Shelf)
                if controller.mascot_enabled {
                    controller.chibi_liebe.render(
                        &self.factory,
                        &self.rt,
                        pill_rect,
                        content_alpha,
                    );
                }
            }

            let _ = self.rt.EndDraw(None, None);
            let _ = EndPaint(hwnd, &ps);
        }
    }

    unsafe fn draw_expanded_panel_shell(
        &self,
        state: NotchState,
        pill_rect: D2D_RECT_F,
        alpha: f32,
    ) {
        let total_w = pill_rect.right - pill_rect.left;
        let slot_w = total_w / 5.0;

        // Gambar titik/indikator kelima tab di bagian atas panel agar pengguna tahu posisi klik tab
        let active_idx = match state {
            NotchState::Music => 0,
            NotchState::Lyrics => 1,
            NotchState::Pomo => 2,
            NotchState::Water => 3,
            NotchState::Shelf | NotchState::Todo => 4,
            _ => return,
        };

        for i in 0..5 {
            let cx = pill_rect.left + slot_w * (i as f32 + 0.5);
            let cy = pill_rect.top + 12.0;

            let color = if i == active_idx {
                let mut c = AntiMagicNeutralPalette::ACCENT_BLUE;
                c.a *= alpha;
                c
            } else {
                D2D1_COLOR_F {
                    r: 0.35,
                    g: 0.42,
                    b: 0.55,
                    a: 0.45 * alpha,
                }
            };

            if let Ok(brush) = self.rt.CreateSolidColorBrush(&color, None) {
                let half_len = if i == active_idx { 14.0 } else { 6.0 };
                self.rt.DrawLine(
                    D2D_POINT_2F {
                        x: cx - half_len,
                        y: cy,
                    },
                    D2D_POINT_2F {
                        x: cx + half_len,
                        y: cy,
                    },
                    &brush,
                    2.4,
                    None,
                );
            }
        }

        // Gambar kartu permukaan dalam (Inner Slate Card #131822) untuk konten fitur
        let mut card_color = AntiMagicNeutralPalette::CARD_SURFACE;
        card_color.a *= alpha;
        if let Ok(card_brush) = self.rt.CreateSolidColorBrush(&card_color, None) {
            let inner_card = D2D1_ROUNDED_RECT {
                rect: D2D_RECT_F {
                    left: pill_rect.left + 12.0,
                    top: pill_rect.top + 26.0,
                    right: pill_rect.right - 12.0,
                    bottom: pill_rect.bottom - 12.0,
                },
                radiusX: 16.0,
                radiusY: 16.0,
            };
            self.rt.FillRoundedRectangle(&inner_card, &card_brush);
        }
    }
}