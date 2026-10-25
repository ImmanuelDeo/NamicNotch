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
        a: 0.97,
    };
    /// Dark Slate Card / Inner Surface (#131822)
    pub const CARD_SURFACE: D2D1_COLOR_F = D2D1_COLOR_F {
        r: 0.075,
        g: 0.094,
        b: 0.133,
        a: 0.92,
    };
    /// Cool Steel Border (#263042)
    pub const STEEL_BORDER: D2D1_COLOR_F = D2D1_COLOR_F {
        r: 0.149,
        g: 0.188,
        b: 0.259,
        a: 0.55,
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
                pixelSize: D2D_SIZE_U { width, height },
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

            // 1. Clear with fully transparent black (DWM glass composition)
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

            // 3. Gambar tepi luar (Cool Steel Grey dengan sedikit denyut Slate-Wine saat mode Pet)
            let border_color = if controller.state == NotchState::Pet {
                let pulse = controller.devil_eyes.aura_intensity.clamp(0.3, 1.2);
                D2D1_COLOR_F {
                    r: 0.24 * pulse,
                    g: 0.22,
                    b: 0.34,
                    a: 0.45,
                }
            } else {
                AntiMagicNeutralPalette::STEEL_BORDER
            };

            if let Ok(border_brush) = self.rt.CreateSolidColorBrush(&border_color, None) {
                self.rt
                    .DrawRoundedRectangle(&rounded_pill, &border_brush, 1.15, None);
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

                // Gambar indikator bar navigasi atas bertema Slate-Blue
                self.draw_top_tab_indicator(controller.state, pill_rect, content_alpha);

                // Panggil fungsi render konten panel (Music, Lyrics, Pomo, Water, Shelf, Todo) milikmu di sini

                // Render Chibi Liebe secara selektif (HANYA tampil jika state adalah Music, Pomo, atau Shelf)
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

    unsafe fn draw_top_tab_indicator(
        &self,
        state: NotchState,
        pill_rect: D2D_RECT_F,
        alpha: f32,
    ) {
        let idx = match state {
            NotchState::Music => 0,
            NotchState::Lyrics => 1,
            NotchState::Pomo => 2,
            NotchState::Water => 3,
            NotchState::Shelf | NotchState::Todo => 4,
            _ => return,
        };

        let total_w = pill_rect.right - pill_rect.left;
        let slot_w = total_w / 5.0;
        let indicator_x = pill_rect.left + slot_w * (idx as f32 + 0.5);
        let indicator_y = pill_rect.top + 10.0;

        let mut accent = AntiMagicNeutralPalette::ACCENT_BLUE;
        accent.a *= alpha;

        if let Ok(brush) = self.rt.CreateSolidColorBrush(&accent, None) {
            self.rt.DrawLine(
                D2D_POINT_2F {
                    x: indicator_x - 14.0,
                    y: indicator_y,
                },
                D2D_POINT_2F {
                    x: indicator_x + 14.0,
                    y: indicator_y,
                },
                &brush,
                2.2,
                None,
            );
        }
    }
}
