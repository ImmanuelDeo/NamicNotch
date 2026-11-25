// src/notch/renderer.rs
use anyhow::Result;
use chrono::Local;
use windows::core::{Interface, PCWSTR};
use windows::Foundation::Numerics::Matrix3x2;
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Direct2D::Common::{
    D2D_SIZE_F, D2D1_FIGURE_BEGIN_HOLLOW, D2D1_FIGURE_END_OPEN, D2D1_GRADIENT_STOP,
    D2D1_ALPHA_MODE_PREMULTIPLIED, D2D1_BEZIER_SEGMENT, D2D1_COLOR_F, D2D1_FIGURE_BEGIN_FILLED,
    D2D1_FIGURE_END_CLOSED, D2D1_PIXEL_FORMAT, D2D_POINT_2F, D2D_RECT_F, D2D_SIZE_U,
};
use windows::Win32::Graphics::Direct2D::{
    D2D1_ANTIALIAS_MODE_PER_PRIMITIVE,
    D2D1CreateFactory, ID2D1Factory, ID2D1Geometry, ID2D1GeometrySink, ID2D1HwndRenderTarget,
    ID2D1PathGeometry,
    ID2D1StrokeStyle, D2D1_ARC_SEGMENT, D2D1_ARC_SIZE_LARGE, D2D1_ARC_SIZE_SMALL,
    D2D1_LAYER_OPTIONS_NONE,
    D2D1_LAYER_PARAMETERS, D2D1_SWEEP_DIRECTION_CLOCKWISE,
    D2D1_EXTEND_MODE_CLAMP, D2D1_GAMMA_2_2, D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES,
    D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES, ID2D1LinearGradientBrush, ID2D1SolidColorBrush,
    D2D1_CAP_STYLE_ROUND, D2D1_DASH_STYLE_DASH, D2D1_ELLIPSE, D2D1_FACTORY_TYPE_SINGLE_THREADED,
    D2D1_FEATURE_LEVEL_DEFAULT, D2D1_HWND_RENDER_TARGET_PROPERTIES, D2D1_LINE_JOIN_ROUND,
    D2D1_PRESENT_OPTIONS_IMMEDIATELY, D2D1_RENDER_TARGET_PROPERTIES,
    D2D1_RENDER_TARGET_TYPE_DEFAULT, D2D1_RENDER_TARGET_TYPE_SOFTWARE,
    D2D1_RENDER_TARGET_USAGE_NONE, D2D1_ROUNDED_RECT,
    D2D1_STROKE_STYLE_PROPERTIES,
};
use windows::Win32::Graphics::DirectWrite::{
    DWriteCreateFactory, IDWriteFactory, IDWriteTextFormat, DWRITE_FACTORY_TYPE_SHARED,
    DWRITE_FONT_STRETCH_NORMAL, DWRITE_FONT_STYLE_ITALIC, DWRITE_FONT_STYLE_NORMAL,
    DWRITE_FONT_WEIGHT_BOLD, DWRITE_FONT_WEIGHT_MEDIUM, DWRITE_FONT_WEIGHT_SEMI_BOLD,
    DWRITE_PARAGRAPH_ALIGNMENT_CENTER, DWRITE_TEXT_ALIGNMENT_CENTER, DWRITE_TEXT_ALIGNMENT_LEADING,
    DWRITE_TEXT_ALIGNMENT_TRAILING,
};
use windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_B8G8R8A8_UNORM;

use crate::notch::states::{NotchController, NotchState, OverlayKind};

/// Palet Warna Anti-Magic Biru-Hitam Netral (Didefinisikan langsung agar tidak butuh `mod theme`)
pub struct AntiMagicNeutralPalette;

impl AntiMagicNeutralPalette {
    pub const TEXT_PRIMARY: D2D1_COLOR_F = D2D1_COLOR_F {
        r: 0.97,
        g: 0.98,
        b: 1.00,
        a: 1.0,
    };
    pub const TEXT_SECONDARY: D2D1_COLOR_F = D2D1_COLOR_F {
        r: 0.60,
        g: 0.65,
        b: 0.74,
        a: 1.0,
    };
    pub const ACCENT_BLUE: D2D1_COLOR_F = D2D1_COLOR_F {
        r: 0.23,
        g: 0.51,
        b: 0.96,
        a: 1.0,
    };
    pub const ACCENT_CYAN: D2D1_COLOR_F = D2D1_COLOR_F {
        r: 0.22,
        g: 0.74,
        b: 0.97,
        a: 1.0,
    };
}

pub struct NotchRenderer {
    pub factory: ID2D1Factory,
    pub rt: ID2D1HwndRenderTarget,
    pub dwrite: IDWriteFactory,
    pub dashed_stroke: Option<ID2D1StrokeStyle>,
    pub fmt_clock_hero: IDWriteTextFormat,
    pub fmt_title: IDWriteTextFormat,
    pub fmt_body: IDWriteTextFormat,
    pub fmt_quote_italic: IDWriteTextFormat,
    pub fmt_caption: IDWriteTextFormat,
    /// Mask rounded-rect untuk clip isi notch; dibuat ulang hanya saat ukuran berubah.
    pill_mask: Option<ID2D1Geometry>,
    pill_mask_rect: Option<D2D_RECT_F>,
}

impl NotchRenderer {
    pub fn new(hwnd: HWND, width: u32, height: u32, dpi_scale: f32) -> Result<Self> {
        unsafe {
            let factory: ID2D1Factory =
                D2D1CreateFactory(D2D1_FACTORY_TYPE_SINGLE_THREADED, None)?;

            let rt_props = D2D1_RENDER_TARGET_PROPERTIES {
                r#type: D2D1_RENDER_TARGET_TYPE_DEFAULT,
                pixelFormat: D2D1_PIXEL_FORMAT {
                    format: DXGI_FORMAT_B8G8R8A8_UNORM,
                    alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED,
                },
                dpiX: 96.0 * dpi_scale,
                dpiY: 96.0 * dpi_scale,
                usage: D2D1_RENDER_TARGET_USAGE_NONE,
                minLevel: D2D1_FEATURE_LEVEL_DEFAULT,
            };

            let hwnd_props = D2D1_HWND_RENDER_TARGET_PROPERTIES {
                hwnd,
                pixelSize: D2D_SIZE_U {
                    width: width.max(1),
                    height: height.max(1),
                },
                presentOptions: D2D1_PRESENT_OPTIONS_IMMEDIATELY,
            };

            let rt = match factory.CreateHwndRenderTarget(&rt_props, &hwnd_props) {
                Ok(rt) => {
                    crate::utils::boot_log("render target: hardware");
                    rt
                }
                Err(hw_err) => {
                    crate::utils::boot_log(&format!(
                        "render target hardware failed ({hw_err:?}); retrying software"
                    ));
                    // Hardware/DXGI tidak selalu tersedia (RDP, VM, sesi non-interaktif).
                    // Fallback ke software renderer supaya notch tetap tampil.
                    let sw_props = D2D1_RENDER_TARGET_PROPERTIES {
                        r#type: D2D1_RENDER_TARGET_TYPE_SOFTWARE,
                        ..rt_props
                    };
                    match factory.CreateHwndRenderTarget(&sw_props, &hwnd_props) {
                        Ok(rt) => {
                            crate::utils::boot_log("render target: software");
                            rt
                        }
                        Err(sw_err) => {
                            crate::utils::boot_log(&format!(
                                "render target software failed: {sw_err:?}"
                            ));
                            return Err(sw_err.into());
                        }
                    }
                }
            };
            let dwrite: IDWriteFactory = DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED)?;

            let font_family: Vec<u16> = "Segoe UI\0".encode_utf16().collect();
            let locale: Vec<u16> = "en-us\0".encode_utf16().collect();

            let dashed_stroke = factory
                .CreateStrokeStyle(
                    &D2D1_STROKE_STYLE_PROPERTIES {
                        startCap: D2D1_CAP_STYLE_ROUND,
                        endCap: D2D1_CAP_STYLE_ROUND,
                        dashCap: D2D1_CAP_STYLE_ROUND,
                        lineJoin: D2D1_LINE_JOIN_ROUND,
                        miterLimit: 10.0,
                        dashStyle: D2D1_DASH_STYLE_DASH,
                        dashOffset: 0.0,
                    },
                    None,
                )
                .ok();

            let fmt_clock_hero = dwrite.CreateTextFormat(
                PCWSTR(font_family.as_ptr()),
                None,
                DWRITE_FONT_WEIGHT_BOLD,
                DWRITE_FONT_STYLE_NORMAL,
                DWRITE_FONT_STRETCH_NORMAL,
                22.0,
                PCWSTR(locale.as_ptr()),
            )?;
            fmt_clock_hero.SetParagraphAlignment(DWRITE_PARAGRAPH_ALIGNMENT_CENTER)?;

            let fmt_title = dwrite.CreateTextFormat(
                PCWSTR(font_family.as_ptr()),
                None,
                DWRITE_FONT_WEIGHT_BOLD,
                DWRITE_FONT_STYLE_NORMAL,
                DWRITE_FONT_STRETCH_NORMAL,
                13.0,
                PCWSTR(locale.as_ptr()),
            )?;
            fmt_title.SetParagraphAlignment(DWRITE_PARAGRAPH_ALIGNMENT_CENTER)?;

            let fmt_body = dwrite.CreateTextFormat(
                PCWSTR(font_family.as_ptr()),
                None,
                DWRITE_FONT_WEIGHT_SEMI_BOLD,
                DWRITE_FONT_STYLE_NORMAL,
                DWRITE_FONT_STRETCH_NORMAL,
                11.5,
                PCWSTR(locale.as_ptr()),
            )?;
            fmt_body.SetParagraphAlignment(DWRITE_PARAGRAPH_ALIGNMENT_CENTER)?;

            let fmt_quote_italic = dwrite.CreateTextFormat(
                PCWSTR(font_family.as_ptr()),
                None,
                DWRITE_FONT_WEIGHT_MEDIUM,
                DWRITE_FONT_STYLE_ITALIC,
                DWRITE_FONT_STRETCH_NORMAL,
                11.0,
                PCWSTR(locale.as_ptr()),
            )?;
            fmt_quote_italic.SetParagraphAlignment(DWRITE_PARAGRAPH_ALIGNMENT_CENTER)?;

            let fmt_caption = dwrite.CreateTextFormat(
                PCWSTR(font_family.as_ptr()),
                None,
                DWRITE_FONT_WEIGHT_MEDIUM,
                DWRITE_FONT_STYLE_NORMAL,
                DWRITE_FONT_STRETCH_NORMAL,
                10.0,
                PCWSTR(locale.as_ptr()),
            )?;
            fmt_caption.SetParagraphAlignment(DWRITE_PARAGRAPH_ALIGNMENT_CENTER)?;

            Ok(Self {
                factory,
                rt,
                dwrite,
                dashed_stroke,
                fmt_clock_hero,
                fmt_title,
                fmt_body,
                fmt_quote_italic,
                fmt_caption,
                pill_mask: None,
                pill_mask_rect: None,
            })
        }
    }

    /// Pastikan mask rounded-rect tersedia untuk ukuran pill saat ini.
    unsafe fn ensure_pill_mask(&mut self, pill_rect: D2D_RECT_F, radius: f32) {
        let same = self.pill_mask_rect.map_or(false, |r| {
            (r.left - pill_rect.left).abs() < 0.01
                && (r.top - pill_rect.top).abs() < 0.01
                && (r.right - pill_rect.right).abs() < 0.01
                && (r.bottom - pill_rect.bottom).abs() < 0.01
        });
        if same && self.pill_mask.is_some() {
            return;
        }
        if let Ok(geom) = self
            .factory
            .CreateRoundedRectangleGeometry(&D2D1_ROUNDED_RECT {
                rect: pill_rect,
                radiusX: radius,
                radiusY: radius,
            })
        {
            self.pill_mask = geom.cast::<ID2D1Geometry>().ok();
            self.pill_mask_rect = Some(pill_rect);
        }
    }

    /// Dipanggil langsung oleh `WM_PAINT` di `src/notch/mod.rs`
    pub fn render_frame(&mut self, ctrl: &mut NotchController, pill_rect: D2D_RECT_F) {
        unsafe {
            self.rt.BeginDraw();
            self.rt.Clear(Some(&D2D1_COLOR_F {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            }));

            self.draw_frame(ctrl, pill_rect);

            let _ = self.rt.EndDraw(None, None);
        }
    }

    unsafe fn draw_text_left(
        &self,
        text: &str,
        format: &IDWriteTextFormat,
        rect: D2D_RECT_F,
        color: D2D1_COLOR_F,
    ) {
        let _ = format.SetTextAlignment(DWRITE_TEXT_ALIGNMENT_LEADING);
        let wide: Vec<u16> = text.encode_utf16().collect();
        if let Ok(brush) = self.rt.CreateSolidColorBrush(&color, None) {
            self.rt
                .DrawText(&wide, format, &rect, &brush, Default::default(), Default::default());
        }
    }

    unsafe fn draw_text_right(
        &self,
        text: &str,
        format: &IDWriteTextFormat,
        rect: D2D_RECT_F,
        color: D2D1_COLOR_F,
    ) {
        let _ = format.SetTextAlignment(DWRITE_TEXT_ALIGNMENT_TRAILING);
        let wide: Vec<u16> = text.encode_utf16().collect();
        if let Ok(brush) = self.rt.CreateSolidColorBrush(&color, None) {
            self.rt
                .DrawText(&wide, format, &rect, &brush, Default::default(), Default::default());
        }
    }

    unsafe fn draw_text_centered(
        &self,
        text: &str,
        format: &IDWriteTextFormat,
        rect: D2D_RECT_F,
        color: D2D1_COLOR_F,
    ) {
        let _ = format.SetTextAlignment(DWRITE_TEXT_ALIGNMENT_CENTER);
        let wide: Vec<u16> = text.encode_utf16().collect();
        if let Ok(brush) = self.rt.CreateSolidColorBrush(&color, None) {
            self.rt
                .DrawText(&wide, format, &rect, &brush, Default::default(), Default::default());
        }
    }

    unsafe fn create_top_bezel_notch_geometry(
        &self,
        rect: D2D_RECT_F,
        bottom_radius: f32,
        top_flare: f32,
    ) -> windows::core::Result<ID2D1PathGeometry> {
        let path = self.factory.CreatePathGeometry()?;
        let sink = path.Open()?;

        let h = (rect.bottom - rect.top).max(4.0);
        let r = bottom_radius.min(h * 0.48).max(2.0);
        let flare = top_flare.min(r).max(4.0);

        let left = rect.left;
        let right = rect.right;
        let top = rect.top;
        let bottom = rect.bottom;

        sink.BeginFigure(
            D2D_POINT_2F {
                x: left - flare,
                y: top,
            },
            D2D1_FIGURE_BEGIN_FILLED,
        );
        sink.AddLine(D2D_POINT_2F {
            x: right + flare,
            y: top,
        });
        sink.AddBezier(&D2D1_BEZIER_SEGMENT {
            point1: D2D_POINT_2F {
                x: right + flare * 0.25,
                y: top,
            },
            point2: D2D_POINT_2F {
                x: right,
                y: top + flare * 0.35,
            },
            point3: D2D_POINT_2F {
                x: right,
                y: top + flare,
            },
        });
        sink.AddLine(D2D_POINT_2F {
            x: right,
            y: bottom - r,
        });
        sink.AddBezier(&D2D1_BEZIER_SEGMENT {
            point1: D2D_POINT_2F {
                x: right,
                y: bottom - r * 0.45,
            },
            point2: D2D_POINT_2F {
                x: right - r * 0.45,
                y: bottom,
            },
            point3: D2D_POINT_2F {
                x: right - r,
                y: bottom,
            },
        });
        sink.AddLine(D2D_POINT_2F {
            x: left + r,
            y: bottom,
        });
        sink.AddBezier(&D2D1_BEZIER_SEGMENT {
            point1: D2D_POINT_2F {
                x: left + r * 0.45,
                y: bottom,
            },
            point2: D2D_POINT_2F {
                x: left,
                y: bottom - r * 0.45,
            },
            point3: D2D_POINT_2F {
                x: left,
                y: bottom - r,
            },
        });
        sink.AddLine(D2D_POINT_2F {
            x: left,
            y: top + flare,
        });
        sink.AddBezier(&D2D1_BEZIER_SEGMENT {
            point1: D2D_POINT_2F {
                x: left,
                y: top + flare * 0.35,
            },
            point2: D2D_POINT_2F {
                x: left - flare * 0.25,
                y: top,
            },
            point3: D2D_POINT_2F {
                x: left - flare,
                y: top,
            },
        });

        sink.EndFigure(D2D1_FIGURE_END_CLOSED);
        sink.Close()?;
        Ok(path)
    }

    unsafe fn battery_arc(&self, cx: f32, cy: f32, r: f32, sweep_deg: f32) -> Option<ID2D1PathGeometry> {
        if sweep_deg <= 0.5 {
            return None;
        }
        let geo: ID2D1PathGeometry = self.factory.CreatePathGeometry().ok()?;
        let sink: ID2D1GeometrySink = geo.Open().ok()?;
        let start_deg = -90.0_f32;
        let p0 = D2D_POINT_2F {
            x: cx + r * start_deg.to_radians().cos(),
            y: cy + r * start_deg.to_radians().sin(),
        };
        sink.BeginFigure(p0, D2D1_FIGURE_BEGIN_HOLLOW);
        let mut a0 = start_deg;
        let mut left = sweep_deg.min(359.9);
        while left > 0.2 {
            let step = left.min(180.0);
            let a1 = a0 + step;
            sink.AddArc(&D2D1_ARC_SEGMENT {
                point: D2D_POINT_2F {
                    x: cx + r * a1.to_radians().cos(),
                    y: cy + r * a1.to_radians().sin(),
                },
                size: D2D_SIZE_F { width: r, height: r },
                rotationAngle: 0.0,
                sweepDirection: D2D1_SWEEP_DIRECTION_CLOCKWISE,
                arcSize: if step >= 179.5 {
                    D2D1_ARC_SIZE_LARGE
                } else {
                    D2D1_ARC_SIZE_SMALL
                },
            });
            a0 = a1;
            left -= step;
        }
        sink.EndFigure(D2D1_FIGURE_END_OPEN);
        sink.Close().ok()?;
        Some(geo)
    }

    /// Cincin baterai: busur mengikuti persentase asli, warna mengikuti state.
    unsafe fn draw_nusa_ring_battery(&self, cx: f32, cy: f32, pct: u8, charging: bool, alpha: f32) {
        const R: f32 = 7.0;
        let pct_f = pct.min(100) as f32;

        // Track redup supaya sisa kapasitas terbaca sebagai bagian cincin.
        if let Ok(track) = self.rt.CreateSolidColorBrush(
            &D2D1_COLOR_F { r: 0.55, g: 0.62, b: 0.72, a: 0.20 * alpha },
            None,
        ) {
            self.rt.DrawEllipse(
                &D2D1_ELLIPSE {
                    point: D2D_POINT_2F { x: cx, y: cy },
                    radiusX: R,
                    radiusY: R,
                },
                &track,
                2.4,
                None,
            );
        }

        let (r, g, b) = if charging {
            (0.24, 0.86, 0.94)
        } else if pct_f <= 15.0 {
            (0.95, 0.26, 0.28)
        } else if pct_f <= 40.0 {
            (0.98, 0.68, 0.22)
        } else {
            (0.22, 0.86, 0.42)
        };

        if let Some(geo) = self.battery_arc(cx, cy, R, pct_f * 3.6) {
            if let Ok(brush) = self.rt.CreateSolidColorBrush(
                &D2D1_COLOR_F { r, g, b, a: alpha },
                None,
            ) {
                self.rt.DrawGeometry(&geo, &brush, 2.4, None);
            }
        }

        // Titik/nadi di tengah: petir saat charging, angka tidak muat di 14 px.
        let glyph = if charging { "⚡" } else { "" };
        if !glyph.is_empty() {
            self.draw_text_centered(
                glyph,
                &self.fmt_caption,
                D2D_RECT_F {
                    left: cx - 8.0,
                    top: cy - 8.0,
                    right: cx + 8.0,
                    bottom: cy + 8.0,
                },
                D2D1_COLOR_F { r: 0.90, g: 0.98, b: 1.0, a: alpha },
            );
        }
    }

    pub unsafe fn draw_frame(&mut self, ctrl: &mut NotchController, pill_rect: D2D_RECT_F) {
        let radius = ctrl.corner_radius.value;
        let alpha = ctrl.content_alpha.value.clamp(0.0, 1.0);
        let flare = if pill_rect.bottom - pill_rect.top > 60.0 {
            20.0
        } else {
            13.0
        };

        // Borderless pure-black notch.
        let pitch_black = D2D1_COLOR_F {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        };
        if let Ok(bezel_geom) = self.create_top_bezel_notch_geometry(pill_rect, radius, flare) {
            if let Ok(base_brush) = self.rt.CreateSolidColorBrush(&pitch_black, None) {
                self.rt.FillGeometry(&bezel_geom, &base_brush, None);
            }
        } else if let Ok(base_brush) = self.rt.CreateSolidColorBrush(&pitch_black, None) {
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: pill_rect,
                    radiusX: radius,
                    radiusY: radius,
                },
                &base_brush,
            );
        }

        // Semua isi notch digambar di dalam layer ber-mask rounded-rect.
        // Tanpa ini gradient yang sengaja diperlebar menyisakan pendar di
        // keempat sudut lengkung, sehingga tampak bocor ke desktop.
        self.ensure_pill_mask(pill_rect, radius);
        let mask = self.pill_mask.clone();
        let masked = if let Some(ref geom) = mask {
            let params = D2D1_LAYER_PARAMETERS {
                contentBounds: D2D_RECT_F {
                    left: f32::NEG_INFINITY,
                    top: f32::NEG_INFINITY,
                    right: f32::INFINITY,
                    bottom: f32::INFINITY,
                },
                geometricMask: std::mem::ManuallyDrop::new(Some(geom.clone())),
                maskAntialiasMode: D2D1_ANTIALIAS_MODE_PER_PRIMITIVE,
                maskTransform: Matrix3x2::identity(),
                opacity: 1.0,
                opacityBrush: std::mem::ManuallyDrop::new(None),
                layerOptions: D2D1_LAYER_OPTIONS_NONE,
            };
            self.rt.PushLayer(&params, None);
            true
        } else {
            false
        };

        if !ctrl.is_hovered {
            if let Some(ref ov) = ctrl.active_overlay {
                self.draw_system_overlay(ov, pill_rect, alpha);
                if masked {
                    self.rt.PopLayer();
                }
                return;
            }
        }

        if ctrl.height.value < 55.0 {
            if ctrl.state == NotchState::Idle {
                self.draw_closed_clock_pill(ctrl, pill_rect, alpha);
            } else {
                // ID2D1HwndRenderTarget deref ke ID2D1RenderTarget, jadi bisa
                // dipakai langsung oleh modul maskot.
                ctrl.devil_eyes.draw_idle_aura_background(
                    &self.rt,
                    &self.factory,
                    pill_rect,
                    alpha,
                );
                let cx = (pill_rect.left + pill_rect.right) * 0.5;
                let cy = (pill_rect.top + pill_rect.bottom) * 0.5;
                ctrl.devil_eyes.draw(&self.rt, &self.factory, cx, cy, alpha);
            }
        } else {
            // Tema Solid Liquid Glass + layout rapat 6 section.
            self.draw_glass_surface(ctrl, pill_rect, alpha);
            self.draw_glass_nav(ctrl, pill_rect, alpha);
            self.draw_glass_panel(ctrl, pill_rect, alpha);
        }

        if ctrl.clip_flash_timer > 0.0 && ctrl.height.value > 55.0 {
            let fa = (ctrl.clip_flash_timer / 1.6).clamp(0.0, 1.0);
            let flash_rect = D2D_RECT_F {
                left: pill_rect.left + 30.0,
                top: pill_rect.bottom - 26.0,
                right: pill_rect.right - 30.0,
                bottom: pill_rect.bottom - 8.0,
            };
            if let Ok(bb) = self.rt.CreateSolidColorBrush(
                &D2D1_COLOR_F { r: 0.05, g: 0.07, b: 0.11, a: 0.92 * fa },
                None,
            ) {
                self.rt.FillRoundedRectangle(
                    &D2D1_ROUNDED_RECT { rect: flash_rect, radiusX: 8.0, radiusY: 8.0 },
                    &bb,
                );
            }
            let mut tc = AntiMagicNeutralPalette::ACCENT_CYAN;
            tc.a *= fa;
            self.draw_text_centered(&ctrl.clip_flash_text, &self.fmt_caption, flash_rect, tc);
        }

        ctrl.chibi_liebe
            .draw(&self.rt, &self.factory, pill_rect, ctrl.state);

        if masked {
            self.rt.PopLayer();
        }
    }

    unsafe fn draw_system_overlay(
        &self,
        ov: &OverlayKind,
        pill_rect: D2D_RECT_F,
        alpha: f32,
    ) {
        let mut p_col = AntiMagicNeutralPalette::TEXT_PRIMARY;
        p_col.a *= alpha;

        match ov {
            OverlayKind::Volume(val) | OverlayKind::Brightness(val) => {
                let is_vol = matches!(ov, OverlayKind::Volume(_));
                let icon = if is_vol { "🔊" } else { "🔆" };
                let bar_col = if is_vol {
                    AntiMagicNeutralPalette::ACCENT_BLUE
                } else {
                    AntiMagicNeutralPalette::ACCENT_CYAN
                };

                self.draw_text_left(
                    icon,
                    &self.fmt_title,
                    D2D_RECT_F {
                        left: pill_rect.left + 16.0,
                        top: pill_rect.top,
                        right: pill_rect.left + 48.0,
                        bottom: pill_rect.bottom,
                    },
                    p_col,
                );

                let bar_l = pill_rect.left + 52.0;
                let bar_r = pill_rect.right - 52.0;
                let cy = (pill_rect.top + pill_rect.bottom) * 0.5;

                if let Ok(tb) = self.rt.CreateSolidColorBrush(
                    &D2D1_COLOR_F {
                        r: 0.20,
                        g: 0.22,
                        b: 0.28,
                        a: 0.9 * alpha,
                    },
                    None,
                ) {
                    self.rt.FillRoundedRectangle(
                        &D2D1_ROUNDED_RECT {
                            rect: D2D_RECT_F {
                                left: bar_l,
                                top: cy - 3.0,
                                right: bar_r,
                                bottom: cy + 3.0,
                            },
                            radiusX: 3.0,
                            radiusY: 3.0,
                        },
                        &tb,
                    );
                }

                let fill_w = (bar_r - bar_l) * (*val as f32 / 100.0);
                if let Ok(fb) = self.rt.CreateSolidColorBrush(&bar_col, None) {
                    self.rt.FillRoundedRectangle(
                        &D2D1_ROUNDED_RECT {
                            rect: D2D_RECT_F {
                                left: bar_l,
                                top: cy - 3.0,
                                right: (bar_l + fill_w).max(bar_l + 6.0),
                                bottom: cy + 3.0,
                            },
                            radiusX: 3.0,
                            radiusY: 3.0,
                        },
                        &fb,
                    );
                }

                self.draw_text_right(
                    &format!("{}%", val),
                    &self.fmt_caption,
                    D2D_RECT_F {
                        left: bar_r + 4.0,
                        top: pill_rect.top,
                        right: pill_rect.right - 14.0,
                        bottom: pill_rect.bottom,
                    },
                    p_col,
                );
            }
            OverlayKind::CapsLock(on) => {
                let txt = if *on {
                    "🅰  CAPS LOCK : ON"
                } else {
                    "🅰  CAPS LOCK : OFF"
                };
                self.draw_text_centered(txt, &self.fmt_body, pill_rect, p_col);
            }
            OverlayKind::Charging { percent, plugged } => {
                let txt = if *plugged {
                    format!("⚡ Charging · {}%", percent)
                } else {
                    format!("🔋 On Battery · {}%", percent)
                };
                self.draw_text_centered(&txt, &self.fmt_body, pill_rect, p_col);
            }
        }
    }

    unsafe fn draw_closed_clock_pill(
        &self,
        ctrl: &NotchController,
        pill_rect: D2D_RECT_F,
        alpha: f32,
    ) {
        let now = Local::now();
        let time_str = now.format("%H:%M").to_string();

        let mut p_col = AntiMagicNeutralPalette::TEXT_PRIMARY;
        p_col.a *= alpha;

        self.draw_text_left(
            &time_str,
            &self.fmt_title,
            D2D_RECT_F {
                left: pill_rect.left + 16.0,
                top: pill_rect.top,
                right: pill_rect.left + 90.0,
                bottom: pill_rect.bottom,
            },
            p_col,
        );

        let cy = (pill_rect.top + pill_rect.bottom) * 0.5;
        self.draw_nusa_ring_battery(
            pill_rect.right - 44.0,
            cy,
            ctrl.battery_percent,
            ctrl.is_charging,
            alpha,
        );
        self.draw_text_right(
            &format!("{}%", ctrl.battery_percent),
            &self.fmt_caption,
            D2D_RECT_F {
                left: pill_rect.right - 36.0,
                top: pill_rect.top,
                right: pill_rect.right - 12.0,
                bottom: pill_rect.bottom,
            },
            p_col,
        );
    }

    unsafe fn draw_clean_nusa_shell(
        &self,
        ctrl: &NotchController,
        pill_rect: D2D_RECT_F,
        alpha: f32,
    ) -> D2D_RECT_F {
        let tabs = ["Hub", "Music", "Control", "Focus", "Shelf", "Clips"];
        let active_idx = match ctrl.state {
            NotchState::Hub => 0,
            NotchState::Music => 1,
            NotchState::Control => 2,
            NotchState::Pomo => 3,
            NotchState::Shelf => 4,
            NotchState::Clips => 5,
            _ => 0,
        };

        let white = D2D1_COLOR_F {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: alpha,
        };
        let muted = D2D1_COLOR_F {
            r: 0.58,
            g: 0.63,
            b: 0.72,
            a: alpha * 0.85,
        };

        let mid_y = (pill_rect.top + pill_rect.bottom) * 0.5 + 4.0;
        let left_circle = D2D_POINT_2F {
            x: pill_rect.left + 16.0,
            y: mid_y,
        };
        let right_circle = D2D_POINT_2F {
            x: pill_rect.right - 16.0,
            y: mid_y,
        };

        if let Ok(cb) = self.rt.CreateSolidColorBrush(
            &D2D1_COLOR_F {
                r: 0.22,
                g: 0.24,
                b: 0.28,
                a: 0.85 * alpha,
            },
            None,
        ) {
            self.rt.FillEllipse(
                &D2D1_ELLIPSE {
                    point: left_circle,
                    radiusX: 9.5,
                    radiusY: 9.5,
                },
                &cb,
            );
            self.rt.FillEllipse(
                &D2D1_ELLIPSE {
                    point: right_circle,
                    radiusX: 9.5,
                    radiusY: 9.5,
                },
                &cb,
            );
        }
        self.draw_text_centered(
            "‹",
            &self.fmt_title,
            D2D_RECT_F {
                left: left_circle.x - 9.5,
                top: left_circle.y - 10.5,
                right: left_circle.x + 9.5,
                bottom: left_circle.y + 8.5,
            },
            white,
        );
        self.draw_text_centered(
            "›",
            &self.fmt_title,
            D2D_RECT_F {
                left: right_circle.x - 9.5,
                top: right_circle.y - 10.5,
                right: right_circle.x + 9.5,
                bottom: right_circle.y + 8.5,
            },
            white,
        );

        let tabs_left = pill_rect.left + 28.0;
                let tabs_right = pill_rect.right - 28.0;
                let slot_w = (tabs_right - tabs_left) / 6.0;

        for (i, label) in tabs.iter().enumerate() {
            let sl = tabs_left + slot_w * i as f32;
            let sr = sl + slot_w;
            let cx = (sl + sr) * 0.5;
            let is_active = i == active_idx;

            let text_col = if is_active {
                white
            } else {
                muted
            };

            self.draw_text_centered(
                label,
                &self.fmt_caption,
                D2D_RECT_F {
                    left: sl,
                    top: pill_rect.top + 4.0,
                    right: sr,
                    bottom: pill_rect.top + 22.0,
                },
                text_col,
            );

            if is_active {
                if let Ok(lb) = self
                    .rt
                    .CreateSolidColorBrush(&AntiMagicNeutralPalette::ACCENT_BLUE, None)
                {
                    self.rt.DrawLine(
                        D2D_POINT_2F {
                            x: cx - 11.0,
                            y: pill_rect.top + 22.5,
                        },
                        D2D_POINT_2F {
                            x: cx + 11.0,
                            y: pill_rect.top + 22.5,
                        },
                        &lb,
                        2.0,
                        None,
                    );
                }
            }
        }

        let dot_y = pill_rect.bottom - 4.5;
        let total_dots = 6;
        let dots_w = 54.0;
        let dot_start_x = (pill_rect.left + pill_rect.right - dots_w) * 0.5;
        for d in 0..total_dots {
            let dx = dot_start_x + d as f32 * 9.5;
            let is_cur = d == active_idx;
            let w = if is_cur { 10.0 } else { 4.0 };
            let col = if is_cur {
                white
            } else {
                D2D1_COLOR_F {
                    r: 0.35,
                    g: 0.38,
                    b: 0.45,
                    a: 0.7 * alpha,
                }
            };
            if let Ok(db) = self.rt.CreateSolidColorBrush(&col, None) {
                self.rt.FillRoundedRectangle(
                    &D2D1_ROUNDED_RECT {
                        rect: D2D_RECT_F {
                            left: dx,
                            top: dot_y - 1.5,
                            right: dx + w,
                            bottom: dot_y + 1.5,
                        },
                        radiusX: 1.5,
                        radiusY: 1.5,
                    },
                    &db,
                );
            }
        }

        D2D_RECT_F {
            left: pill_rect.left + 26.0,
            top: pill_rect.top + 26.0,
            right: pill_rect.right - 26.0,
            bottom: pill_rect.bottom - 8.0,
        }
    }

    unsafe fn draw_hub_panel(&self, ctrl: &NotchController, area: D2D_RECT_F, alpha: f32) {
        let white = D2D1_COLOR_F {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: alpha,
        };
        let muted = D2D1_COLOR_F {
            r: 0.62,
            g: 0.66,
            b: 0.74,
            a: alpha,
        };

        let now = Local::now();
        let time_big = now.format("%H:%M").to_string();
        let date_str = now.format("%a %d %b %Y").to_string();

        self.draw_text_left(
            &time_big,
            &self.fmt_clock_hero,
            D2D_RECT_F {
                left: area.left + 2.0,
                top: area.top + 8.0,
                right: area.left + 118.0,
                bottom: area.top + 40.0,
            },
            white,
        );

        self.draw_text_left(
            &date_str,
            &self.fmt_caption,
            D2D_RECT_F {
                left: area.left + 2.0,
                top: area.top + 42.0,
                right: area.left + 118.0,
                bottom: area.top + 60.0,
            },
            muted,
        );

        let sys_line = format!("RAM {}% · C: {}GB free", ctrl.ram_used_pct, ctrl.disk_free_gb);
        let sys_col = if ctrl.ram_used_pct >= 90 || ctrl.disk_free_gb < 10 {
            D2D1_COLOR_F { r: 1.0, g: 0.45, b: 0.35, a: alpha }
        } else {
            muted
        };
        self.draw_text_left(
            &sys_line,
            &self.fmt_caption,
            D2D_RECT_F {
                left: area.left + 2.0,
                top: area.top + 60.0,
                right: area.left + 120.0,
                bottom: area.top + 78.0,
            },
            sys_col,
        );

        self.draw_nusa_ring_battery(
            area.left + 8.0,
            area.bottom - 18.0,
            ctrl.battery_percent,
            ctrl.is_charging,
            alpha,
        );
        let bat_label = if ctrl.is_charging {
            format!("{}% · Charging", ctrl.battery_percent)
        } else {
            format!("{}% · Battery", ctrl.battery_percent)
        };
        self.draw_text_left(
            &bat_label,
            &self.fmt_caption,
            D2D_RECT_F {
                left: area.left + 20.0,
                top: area.bottom - 27.0,
                right: area.left + 118.0,
                bottom: area.bottom - 9.0,
            },
            white,
        );

        if let Ok(div_b) = self.rt.CreateSolidColorBrush(
            &D2D1_COLOR_F {
                r: 0.18,
                g: 0.24,
                b: 0.36,
                a: 0.65 * alpha,
            },
            None,
        ) {
            self.rt.DrawLine(
                D2D_POINT_2F {
                    x: area.left + 122.0,
                    y: area.top + 10.0,
                },
                D2D_POINT_2F {
                    x: area.left + 122.0,
                    y: area.bottom - 10.0,
                },
                &div_b,
                1.0,
                None,
            );
        }

        let q_left = area.left + 130.0;
        let daily = ctrl.current_quote();
        let header = format!("✨ {}", daily.word_of_the_day);
        self.draw_text_left(
            &header,
            &self.fmt_caption,
            D2D_RECT_F {
                left: q_left,
                top: area.top + 8.0,
                right: area.right - 2.0,
                bottom: area.top + 24.0,
            },
            AntiMagicNeutralPalette::ACCENT_CYAN,
        );

        self.draw_text_left(
            daily.quote,
            &self.fmt_quote_italic,
            D2D_RECT_F {
                left: q_left,
                top: area.top + 25.0,
                right: area.right - 2.0,
                bottom: area.bottom - 24.0,
            },
            white,
        );

        let footer = format!("— {}  (Click / Scroll)", daily.author);
        self.draw_text_left(
            &footer,
            &self.fmt_caption,
            D2D_RECT_F {
                left: q_left,
                top: area.bottom - 23.0,
                right: area.right - 2.0,
                bottom: area.bottom - 6.0,
            },
            muted,
        );
    }

    unsafe fn draw_music_panel(&self, ctrl: &NotchController, area: D2D_RECT_F, alpha: f32) {
        let white = D2D1_COLOR_F { r: 1.0, g: 1.0, b: 1.0, a: alpha };
        let muted = D2D1_COLOR_F { r: 0.60, g: 0.64, b: 0.70, a: alpha };
        let cyan = AntiMagicNeutralPalette::ACCENT_CYAN;
        let blue = AntiMagicNeutralPalette::ACCENT_BLUE;

        let total_w = area.right - area.left;
        let player_right = if ctrl.show_side_lyrics {
            area.left + total_w * 0.57
        } else {
            area.right - 4.0
        };
        let content_h = area.bottom - area.top;

        // --- Album art besar + equalizer animasi (ala Spotify now-playing) ---
        let art_rect = D2D_RECT_F {
            left: area.left + 8.0,
            top: area.top + 12.0,
            right: area.left + 86.0,
            bottom: area.top + 90.0,
        };
        if let Ok(art_bg) = self.rt.CreateSolidColorBrush(
            &D2D1_COLOR_F { r: 0.13, g: 0.20, b: 0.32, a: alpha }, None,
        ) {
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT { rect: art_rect, radiusX: 12.0, radiusY: 12.0 },
                &art_bg,
            );
        }
        // Highlight lembut di pojok atas art (kesan depth, tetap flat-theme).
        if let Ok(hl) = self.rt.CreateSolidColorBrush(
            &D2D1_COLOR_F { r: 0.22, g: 0.32, b: 0.48, a: 0.35 * alpha }, None,
        ) {
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: D2D_RECT_F {
                        left: art_rect.left + 4.0, top: art_rect.top + 4.0,
                        right: art_rect.right - 4.0, bottom: art_rect.top + 26.0,
                    },
                    radiusX: 9.0, radiusY: 9.0,
                },
                &hl,
            );
        }
        self.draw_text_centered(
            "🎵", &self.fmt_title,
            D2D_RECT_F {
                left: art_rect.left, top: art_rect.top + 8.0,
                right: art_rect.right, bottom: art_rect.top + 46.0,
            },
            white,
        );
        // Equalizer: 4 bar bergoyang saat playing, diam saat pause.
        let eq_base = art_rect.bottom - 12.0;
        let eq_left = art_rect.left + 16.0;
        for b in 0..4 {
            let phase = ctrl.anim_time * 5.2 + b as f32 * 1.35;
            let amp = if ctrl.media_playing { (phase.sin().abs() * 0.75 + 0.25) } else { 0.18 };
            let bh = 6.0 + amp * 16.0;
            let bx = eq_left + b as f32 * 12.0;
            if let Ok(eb) = self.rt.CreateSolidColorBrush(
                &D2D1_COLOR_F { r: cyan.r, g: cyan.g, b: cyan.b, a: (0.55 + 0.4 * amp) * alpha },
                None,
            ) {
                self.rt.FillRoundedRectangle(
                    &D2D1_ROUNDED_RECT {
                        rect: D2D_RECT_F { left: bx, top: eq_base - bh, right: bx + 7.0, bottom: eq_base },
                        radiusX: 2.0, radiusY: 2.0,
                    },
                    &eb,
                );
            }
        }

        // --- Judul + artist/channel, rata kiri setelah art. Tanpa label status;
        // artist sengaja lebih ke bawah supaya tidak menabrak judul. ---
        let text_l = art_rect.right + 14.0;
        self.draw_text_left(
            &ctrl.media_title, &self.fmt_body,
            D2D_RECT_F { left: text_l, top: area.top + 16.0, right: player_right - 6.0, bottom: area.top + 44.0 },
            white,
        );
        self.draw_text_left(
            &ctrl.media_artist, &self.fmt_caption,
            D2D_RECT_F { left: text_l, top: area.top + 62.0, right: player_right - 6.0, bottom: area.top + 80.0 },
            muted,
        );

        // --- Progress bar full-width + waktu di bawahnya (Spotify layout) ---
        let track_l = area.left + 10.0;
        let track_r = player_right - 10.0;
        let bar_y = area.bottom - 64.0;
        if let Ok(tb) = self.rt.CreateSolidColorBrush(
            &D2D1_COLOR_F { r: 0.24, g: 0.27, b: 0.33, a: alpha }, None,
        ) {
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: D2D_RECT_F { left: track_l, top: bar_y - 2.0, right: track_r, bottom: bar_y + 2.0 },
                    radiusX: 2.0, radiusY: 2.0,
                },
                &tb,
            );
        }
        let dot_x = track_l + (track_r - track_l) * ctrl.media_progress.clamp(0.0, 1.0);
        if let Ok(fb) = self.rt.CreateSolidColorBrush(&cyan, None) {
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: D2D_RECT_F { left: track_l, top: bar_y - 2.0, right: dot_x.max(track_l + 2.0), bottom: bar_y + 2.0 },
                    radiusX: 2.0, radiusY: 2.0,
                },
                &fb,
            );
        }
        // Thumb: tumbuh saat hover/drag.
        let thumb = if ctrl.is_dragging_track_bar { 6.5 } else if ctrl.is_hovered { 4.8 } else { 3.4 };
        let glow = thumb + 3.5;
        if let Ok(gb) = self.rt.CreateSolidColorBrush(
            &D2D1_COLOR_F { r: 0.0, g: 0.96, b: 1.0, a: 0.30 * alpha }, None,
        ) {
            self.rt.FillEllipse(
                &D2D1_ELLIPSE { point: D2D_POINT_2F { x: dot_x, y: bar_y }, radiusX: glow, radiusY: glow },
                &gb,
            );
        }
        if let Ok(wb) = self.rt.CreateSolidColorBrush(&white, None) {
            self.rt.FillEllipse(
                &D2D1_ELLIPSE { point: D2D_POINT_2F { x: dot_x, y: bar_y }, radiusX: thumb, radiusY: thumb },
                &wb,
            );
        }

        let elapsed_s = ctrl.media_elapsed_exact.max(0.0) as u32;
        let total_s = ctrl.media_total_secs.max(1.0) as u32;
        let rem_s = total_s.saturating_sub(elapsed_s);
        self.draw_text_left(
            &format!("{:02}:{:02}", elapsed_s / 60, elapsed_s % 60), &self.fmt_caption,
            D2D_RECT_F { left: track_l, top: bar_y + 6.0, right: track_l + 46.0, bottom: bar_y + 22.0 },
            muted,
        );
        self.draw_text_right(
            &format!("-{:02}:{:02}", rem_s / 60, rem_s % 60), &self.fmt_caption,
            D2D_RECT_F { left: track_r - 46.0, top: bar_y + 6.0, right: track_r, bottom: bar_y + 22.0 },
            muted,
        );

        // --- Baris kontrol: 💬 | ⏮ | (▶ bulat besar) | ⏭ | speed pill ---
        let ctrl_cy = area.bottom - 24.0;
        let mid_x = (area.left + player_right) * 0.5;

        let lyr_col = if ctrl.show_side_lyrics { cyan } else { muted };
        self.draw_text_centered(
            "💬", &self.fmt_body,
            D2D_RECT_F { left: mid_x - 92.0, top: ctrl_cy - 11.0, right: mid_x - 62.0, bottom: ctrl_cy + 11.0 },
            lyr_col,
        );
        self.draw_text_centered(
            "⏮", &self.fmt_title,
            D2D_RECT_F { left: mid_x - 52.0, top: ctrl_cy - 11.0, right: mid_x - 24.0, bottom: ctrl_cy + 11.0 },
            white,
        );
        // Tombol play bulat besar ala Spotify.
        if let Ok(pb) = self.rt.CreateSolidColorBrush(
            &D2D1_COLOR_F { r: blue.r, g: blue.g, b: blue.b, a: 0.95 * alpha }, None,
        ) {
            self.rt.FillEllipse(
                &D2D1_ELLIPSE { point: D2D_POINT_2F { x: mid_x, y: ctrl_cy }, radiusX: 16.0, radiusY: 16.0 },
                &pb,
            );
        }
        self.draw_text_centered(
            if ctrl.media_playing { "⏸" } else { "▶" }, &self.fmt_body,
            D2D_RECT_F { left: mid_x - 14.0, top: ctrl_cy - 12.0, right: mid_x + 14.0, bottom: ctrl_cy + 12.0 },
            white,
        );
        self.draw_text_centered(
            "⏭", &self.fmt_title,
            D2D_RECT_F { left: mid_x + 24.0, top: ctrl_cy - 11.0, right: mid_x + 52.0, bottom: ctrl_cy + 11.0 },
            white,
        );

        let speed_str = format!("{:.2}x", ctrl.playback_rate)
            .replace(".00", "").replace("0x", "x");
        let speed_label = if speed_str == "1x" { "1.0x".to_string() } else { speed_str };
        let speed_rect = D2D_RECT_F {
            left: mid_x + 60.0, top: ctrl_cy - 10.0, right: mid_x + 96.0, bottom: ctrl_cy + 10.0,
        };
        if let Ok(sb) = self.rt.CreateSolidColorBrush(
            &D2D1_COLOR_F { r: 0.14, g: 0.20, b: 0.30, a: alpha }, None,
        ) {
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT { rect: speed_rect, radiusX: 9.0, radiusY: 9.0 }, &sb,
            );
        }
        self.draw_text_centered(&speed_label, &self.fmt_caption, speed_rect, cyan);

        // --- Side lyrics: 3 bait di tengah vertikal + karaoke bar ---
        if ctrl.show_side_lyrics {
            let lyr_left = player_right + 12.0;
            if let Ok(div_b) = self.rt.CreateSolidColorBrush(
                &D2D1_COLOR_F { r: 0.18, g: 0.24, b: 0.36, a: 0.6 * alpha }, None,
            ) {
                self.rt.DrawLine(
                    D2D_POINT_2F { x: player_right + 5.0, y: area.top + 10.0 },
                    D2D_POINT_2F { x: player_right + 5.0, y: area.bottom - 10.0 },
                    &div_b, 1.0, None,
                );
            }

            let total_lines = ctrl.timed_lyrics.len().max(1);
            let cur_idx = ctrl.active_lyric_idx % total_lines;
            let prev_line = if cur_idx > 0 {
                ctrl.timed_lyrics.get(cur_idx - 1).map(|(_, s)| s.as_str()).unwrap_or("")
            } else { "♪ · · ·" };
            let cur_line = ctrl.timed_lyrics.get(cur_idx).map(|(_, s)| s.as_str()).unwrap_or("♪ ...");
            let next_line = ctrl.timed_lyrics.get((cur_idx + 1) % total_lines).map(|(_, s)| s.as_str()).unwrap_or("");

            let ease = 1.0 - (1.0 - ctrl.lyric_anim_progress).powi(3);
            let glide_y = (1.0 - ease) * 8.0;
            let mid_y = (area.top + area.bottom) * 0.5;

            // Bait sebelumnya (redup, di atas)
            self.draw_text_left(
                prev_line, &self.fmt_caption,
                D2D_RECT_F { left: lyr_left + 6.0, top: mid_y - 44.0 - glide_y, right: area.right - 4.0, bottom: mid_y - 24.0 - glide_y },
                D2D1_COLOR_F { r: 0.50, g: 0.55, b: 0.62, a: alpha * 0.55 },
            );

            // Indikator aktif + bait saat ini
            let cur_top = mid_y - 18.0 - glide_y;
            let cur_bottom = mid_y + 16.0 - glide_y;
            if let Ok(ind_b) = self.rt.CreateSolidColorBrush(
                &D2D1_COLOR_F { r: 0.0, g: 0.96, b: 1.0, a: alpha * (0.6 + 0.4 * ease) }, None,
            ) {
                self.rt.FillRoundedRectangle(
                    &D2D1_ROUNDED_RECT {
                        rect: D2D_RECT_F { left: lyr_left, top: cur_top + 4.0, right: lyr_left + 3.0, bottom: cur_top + 24.0 },
                        radiusX: 1.5, radiusY: 1.5,
                    },
                    &ind_b,
                );
            }
            self.draw_text_left(
                cur_line, &self.fmt_body,
                D2D_RECT_F { left: lyr_left + 8.0, top: cur_top, right: area.right - 4.0, bottom: cur_bottom },
                D2D1_COLOR_F { r: 0.95, g: 1.0, b: 1.0, a: alpha * (0.85 + 0.15 * ease) },
            );

            // Karaoke bar hanya untuk lirik asli.
            if ctrl.has_real_lyrics {
                let kb_left = lyr_left + 8.0;
                let kb_right = area.right - 6.0;
                let kb_y = cur_bottom + 3.0;
                if let Ok(bb) = self.rt.CreateSolidColorBrush(
                    &D2D1_COLOR_F { r: 0.16, g: 0.20, b: 0.28, a: 0.8 * alpha }, None,
                ) {
                    self.rt.FillRoundedRectangle(
                        &D2D1_ROUNDED_RECT {
                            rect: D2D_RECT_F { left: kb_left, top: kb_y, right: kb_right, bottom: kb_y + 2.0 },
                            radiusX: 1.0, radiusY: 1.0,
                        }, &bb,
                    );
                }
                let fill_x = kb_left + (kb_right - kb_left) * ctrl.lyric_line_progress;
                if let Ok(fb) = self.rt.CreateSolidColorBrush(&cyan, None) {
                    self.rt.FillRoundedRectangle(
                        &D2D1_ROUNDED_RECT {
                            rect: D2D_RECT_F { left: kb_left, top: kb_y, right: fill_x.max(kb_left + 1.0), bottom: kb_y + 2.0 },
                            radiusX: 1.0, radiusY: 1.0,
                        }, &fb,
                    );
                }
            }

            // Bait berikutnya (menunggu di bawah)
            self.draw_text_left(
                next_line, &self.fmt_caption,
                D2D_RECT_F { left: lyr_left + 6.0, top: mid_y + 30.0 - glide_y, right: area.right - 4.0, bottom: mid_y + 50.0 - glide_y },
                D2D1_COLOR_F { r: 0.58, g: 0.63, b: 0.72, a: alpha * 0.65 },
            );
        }
        let _ = content_h;
    }

    unsafe fn draw_control_panel(&self, ctrl: &NotchController, area: D2D_RECT_F, alpha: f32) {
        let white = D2D1_COLOR_F {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: alpha,
        };
        let area_h = area.bottom - area.top;
        let row_h = area_h * 0.46;

        let rows = [
            (
                area.top + 4.0,
                "🔊  Volume",
                ctrl.volume_level,
                AntiMagicNeutralPalette::ACCENT_BLUE,
            ),
            (
                area.top + 8.0 + row_h,
                "🔆  Brightness",
                ctrl.brightness_level,
                AntiMagicNeutralPalette::ACCENT_CYAN,
            ),
        ];

        for (ry, label, val, accent) in rows {
            let cy = ry + row_h * 0.5;

            self.draw_text_left(
                label,
                &self.fmt_body,
                D2D_RECT_F {
                    left: area.left + 2.0,
                    top: ry,
                    right: area.left + 92.0,
                    bottom: ry + row_h,
                },
                white,
            );

            let track_l = area.left + 96.0;
            let track_r = (area.right - 44.0).max(track_l + 20.0);

            if let Ok(tb) = self.rt.CreateSolidColorBrush(
                &D2D1_COLOR_F {
                    r: 0.22,
                    g: 0.25,
                    b: 0.32,
                    a: alpha,
                },
                None,
            ) {
                self.rt.FillRoundedRectangle(
                    &D2D1_ROUNDED_RECT {
                        rect: D2D_RECT_F {
                            left: track_l,
                            top: cy - 3.0,
                            right: track_r,
                            bottom: cy + 3.0,
                        },
                        radiusX: 3.0,
                        radiusY: 3.0,
                    },
                    &tb,
                );
            }

            let fill_x = track_l + (track_r - track_l) * (val.min(100) as f32 / 100.0);
            if let Ok(fb) = self.rt.CreateSolidColorBrush(&accent, None) {
                self.rt.FillRoundedRectangle(
                    &D2D1_ROUNDED_RECT {
                        rect: D2D_RECT_F {
                            left: track_l,
                            top: cy - 3.0,
                            right: fill_x.max(track_l + 4.0),
                            bottom: cy + 3.0,
                        },
                        radiusX: 3.0,
                        radiusY: 3.0,
                    },
                    &fb,
                );
            }

            if let Ok(kb) = self.rt.CreateSolidColorBrush(&white, None) {
                self.rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F { x: fill_x, y: cy },
                        radiusX: 5.0,
                        radiusY: 5.0,
                    },
                    &kb,
                );
            }

            self.draw_text_right(
                &format!("{}%", val),
                &self.fmt_body,
                D2D_RECT_F {
                    left: area.right - 40.0,
                    top: ry,
                    right: area.right - 2.0,
                    bottom: ry + row_h,
                },
                white,
            );
        }
    }

    unsafe fn draw_pomo_panel(&self, ctrl: &NotchController, area: D2D_RECT_F, alpha: f32) {
        let white = D2D1_COLOR_F { r: 1.0, g: 1.0, b: 1.0, a: alpha };
        let muted = D2D1_COLOR_F { r: 0.60, g: 0.64, b: 0.72, a: alpha };
        let cyan = AntiMagicNeutralPalette::ACCENT_CYAN;
        let blue = AntiMagicNeutralPalette::ACCENT_BLUE;

        let content_w = area.right - area.left;
        let content_h = area.bottom - area.top;

        // --- Kolom kiri: jam besar + mode + progress sesi, center vertikal ---
        let left_w = content_w * 0.42;
        let left_cx = area.left + left_w * 0.5;
        let mid_y = (area.top + area.bottom) * 0.5;

        let mins = ctrl.pomo_remaining_secs / 60;
        let secs = ctrl.pomo_remaining_secs % 60;
        let time_str = format!("{:02}:{:02}", mins, secs);
        self.draw_text_centered(
            &time_str, &self.fmt_clock_hero,
            D2D_RECT_F {
                left: area.left + 4.0, top: mid_y - 34.0,
                right: area.left + left_w - 4.0, bottom: mid_y + 6.0,
            },
            white,
        );
        self.draw_text_centered(
            if ctrl.pomo_is_break { "☕ Break Mode" } else { "🎯 Focus Mode" },
            &self.fmt_caption,
            D2D_RECT_F {
                left: area.left + 4.0, top: mid_y + 10.0,
                right: area.left + left_w - 4.0, bottom: mid_y + 28.0,
            },
            cyan,
        );
        // Progress sesi tipis di bawah mode.
        let prog = if ctrl.pomo_total_secs > 0 {
            1.0 - (ctrl.pomo_remaining_secs as f32 / ctrl.pomo_total_secs as f32)
        } else { 0.0 };
        let pb_l = area.left + 18.0;
        let pb_r = area.left + left_w - 18.0;
        let pb_y = mid_y + 36.0;
        if let Ok(tb) = self.rt.CreateSolidColorBrush(
            &D2D1_COLOR_F { r: 0.22, g: 0.26, b: 0.33, a: alpha }, None,
        ) {
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: D2D_RECT_F { left: pb_l, top: pb_y, right: pb_r, bottom: pb_y + 3.0 },
                    radiusX: 1.5, radiusY: 1.5,
                }, &tb,
            );
        }
        if let Ok(fb) = self.rt.CreateSolidColorBrush(&cyan, None) {
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: D2D_RECT_F { left: pb_l, top: pb_y, right: (pb_l + (pb_r - pb_l) * prog.clamp(0.0, 1.0)).max(pb_l + 1.0), bottom: pb_y + 3.0 },
                    radiusX: 1.5, radiusY: 1.5,
                }, &fb,
            );
        }
        let _ = left_cx;

        // Divider vertikal tipis antara kolom.
        let div_x = area.left + left_w + 6.0;
        if let Ok(db) = self.rt.CreateSolidColorBrush(
            &D2D1_COLOR_F { r: 0.18, g: 0.24, b: 0.36, a: 0.6 * alpha }, None,
        ) {
            self.rt.DrawLine(
                D2D_POINT_2F { x: div_x, y: area.top + 12.0 },
                D2D_POINT_2F { x: div_x, y: area.bottom - 12.0 },
                &db, 1.0, None,
            );
        }

        // --- Kolom kanan: pill mode di atas, Start/Reset besar di bawah ---
        let right_l = div_x + 14.0;
        let right_r = area.right - 8.0;
        let right_w = right_r - right_l;
        let gap = 10.0;
        let pill_w = (right_w - gap) * 0.5;
        let pill_h = 30.0;
        let btn_h = 40.0;
        let block_h = pill_h + 12.0 + btn_h;
        let block_top = mid_y - block_h * 0.5;

        let focus_rect = D2D_RECT_F {
            left: right_l, top: block_top,
            right: right_l + pill_w, bottom: block_top + pill_h,
        };
        let break_rect = D2D_RECT_F {
            left: right_l + pill_w + gap, top: block_top,
            right: right_r, bottom: block_top + pill_h,
        };
        let active_col = D2D1_COLOR_F { r: blue.r, g: blue.g, b: blue.b, a: 0.90 * alpha };
        let inactive_col = D2D1_COLOR_F { r: 0.14, g: 0.17, b: 0.24, a: 0.90 * alpha };

        if let Ok(fb) = self.rt.CreateSolidColorBrush(
            if !ctrl.pomo_is_break { &active_col } else { &inactive_col }, None,
        ) {
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT { rect: focus_rect, radiusX: 9.0, radiusY: 9.0 }, &fb,
            );
        }
        if let Ok(bb) = self.rt.CreateSolidColorBrush(
            if ctrl.pomo_is_break { &active_col } else { &inactive_col }, None,
        ) {
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT { rect: break_rect, radiusX: 9.0, radiusY: 9.0 }, &bb,
            );
        }
        self.draw_text_centered(
            "Focus 25m", &self.fmt_caption, focus_rect,
            if !ctrl.pomo_is_break { white } else { muted },
        );
        self.draw_text_centered(
            "Break 5m", &self.fmt_caption, break_rect,
            if ctrl.pomo_is_break { white } else { muted },
        );

        let btn_top = block_top + pill_h + 12.0;
        let start_w = (right_w - gap) * 0.58;
        let start_rect = D2D_RECT_F {
            left: right_l, top: btn_top,
            right: right_l + start_w, bottom: btn_top + btn_h,
        };
        let reset_rect = D2D_RECT_F {
            left: right_l + start_w + gap, top: btn_top,
            right: right_r, bottom: btn_top + btn_h,
        };
        let start_col = if ctrl.pomo_running {
            D2D1_COLOR_F { r: 0.16, g: 0.22, b: 0.34, a: alpha }
        } else {
            D2D1_COLOR_F { r: blue.r, g: blue.g, b: blue.b, a: 0.95 * alpha }
        };
        if let Ok(sb) = self.rt.CreateSolidColorBrush(&start_col, None) {
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT { rect: start_rect, radiusX: 10.0, radiusY: 10.0 }, &sb,
            );
        }
        if let Ok(rb) = self.rt.CreateSolidColorBrush(&inactive_col, None) {
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT { rect: reset_rect, radiusX: 10.0, radiusY: 10.0 }, &rb,
            );
        }
        self.draw_text_centered(
            if ctrl.pomo_running { "⏸ Pause" } else { "▶ Start" },
            &self.fmt_body, start_rect, white,
        );
        self.draw_text_centered("↺ Reset", &self.fmt_body, reset_rect, muted);
    }

    unsafe fn draw_shelf_panel(&self, ctrl: &NotchController, area: D2D_RECT_F, alpha: f32) {
        let white = D2D1_COLOR_F {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: alpha,
        };
        let muted = D2D1_COLOR_F {
            r: 0.56,
            g: 0.60,
            b: 0.68,
            a: alpha,
        };

        let drop_rect = D2D_RECT_F {
            left: area.left + 4.0,
            top: area.top + 4.0,
            right: area.right - 4.0,
            bottom: area.bottom - 4.0,
        };

        if let Ok(border_b) = self.rt.CreateSolidColorBrush(
            &D2D1_COLOR_F {
                r: 0.32,
                g: 0.36,
                b: 0.44,
                a: 0.85 * alpha,
            },
            None,
        ) {
            self.rt.DrawRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: drop_rect,
                    radiusX: 14.0,
                    radiusY: 14.0,
                },
                &border_b,
                1.4,
                self.dashed_stroke.as_ref(),
            );
        }

        if ctrl.shelf_files.is_empty() {
            let cx = (drop_rect.left + drop_rect.right) * 0.5;
            self.draw_text_centered(
                "📥",
                &self.fmt_clock_hero,
                D2D_RECT_F {
                    left: cx - 30.0,
                    top: drop_rect.top + 8.0,
                    right: cx + 30.0,
                    bottom: drop_rect.top + 44.0,
                },
                white,
            );
            self.draw_text_centered(
                "Drop files here",
                &self.fmt_title,
                D2D_RECT_F {
                    left: drop_rect.left,
                    top: drop_rect.top + 44.0,
                    right: drop_rect.right,
                    bottom: drop_rect.top + 68.0,
                },
                white,
            );
            self.draw_text_centered(
                "Drag out anytime · click to open",
                &self.fmt_caption,
                D2D_RECT_F {
                    left: drop_rect.left,
                    top: drop_rect.top + 66.0,
                    right: drop_rect.right,
                    bottom: drop_rect.bottom - 6.0,
                },
                muted,
            );
        } else {
            let slot_w = (drop_rect.right - drop_rect.left - 24.0) * 0.5;
            for (i, (name, size, _)) in ctrl.shelf_files.iter().take(2).enumerate() {
                let fl = drop_rect.left + 8.0 + i as f32 * (slot_w + 8.0);
                let f_rect = D2D_RECT_F {
                    left: fl,
                    top: drop_rect.top + 10.0,
                    right: fl + slot_w,
                    bottom: drop_rect.bottom - 10.0,
                };
                if let Ok(ib) = self.rt.CreateSolidColorBrush(
                    &D2D1_COLOR_F {
                        r: 0.12,
                        g: 0.16,
                        b: 0.24,
                        a: 0.90 * alpha,
                    },
                    None,
                ) {
                    self.rt.FillRoundedRectangle(
                        &D2D1_ROUNDED_RECT {
                            rect: f_rect,
                            radiusX: 10.0,
                            radiusY: 10.0,
                        },
                        &ib,
                    );
                }
                self.draw_text_left(
                    &format!("📄 {}", name),
                    &self.fmt_body,
                    D2D_RECT_F {
                        left: f_rect.left + 10.0,
                        top: f_rect.top + 8.0,
                        right: f_rect.right - 24.0,
                        bottom: f_rect.top + 34.0,
                    },
                    white,
                );
                self.draw_text_left(
                    &format!("{} · Click to open", size),
                    &self.fmt_caption,
                    D2D_RECT_F {
                        left: f_rect.left + 10.0,
                        top: f_rect.top + 36.0,
                        right: f_rect.right - 10.0,
                        bottom: f_rect.bottom - 6.0,
                    },
                    AntiMagicNeutralPalette::ACCENT_CYAN,
                );
                self.draw_text_centered(
                    "✕",
                    &self.fmt_caption,
                    D2D_RECT_F {
                        left: f_rect.right - 22.0,
                        top: f_rect.top + 6.0,
                        right: f_rect.right - 6.0,
                        bottom: f_rect.top + 22.0,
                    },
                    muted,
                );
            }
        }
    }

    unsafe fn draw_clips_panel(&self, ctrl: &NotchController, area: D2D_RECT_F, alpha: f32) {
        let white = D2D1_COLOR_F { r: 1.0, g: 1.0, b: 1.0, a: alpha };
        let muted = D2D1_COLOR_F { r: 0.56, g: 0.60, b: 0.68, a: alpha };
        let accent = AntiMagicNeutralPalette::ACCENT_BLUE;
        let cyan = AntiMagicNeutralPalette::ACCENT_CYAN;

        // Header: History | Notes | Clear
        let btn_y = area.top;
        let btn_h = 22.0;
        let buttons: [(&str, f32, f32, bool); 3] = [
            ("History", area.left, area.left + 76.0, !ctrl.clips_view_notes),
            ("Notes", area.left + 84.0, area.left + 160.0, ctrl.clips_view_notes),
            ("Clear", area.right - 66.0, area.right, false),
        ];
        for (label, l, r, active) in buttons {
            let rect = D2D_RECT_F { left: l, top: btn_y, right: r, bottom: btn_y + btn_h };
            let bg = if active {
                D2D1_COLOR_F { r: accent.r, g: accent.g, b: accent.b, a: 0.32 * alpha }
            } else {
                D2D1_COLOR_F { r: 0.12, g: 0.15, b: 0.21, a: 0.85 * alpha }
            };
            if let Ok(bb) = self.rt.CreateSolidColorBrush(&bg, None) {
                self.rt.FillRoundedRectangle(
                    &D2D1_ROUNDED_RECT { rect, radiusX: 6.0, radiusY: 6.0 }, &bb,
                );
            }
            self.draw_text_centered(label, &self.fmt_caption, rect, if active { white } else { muted });
        }

        if let Ok(lb) = self.rt.CreateSolidColorBrush(
            &D2D1_COLOR_F { r: 0.18, g: 0.22, b: 0.30, a: 0.7 * alpha }, None,
        ) {
            self.rt.DrawLine(
                D2D_POINT_2F { x: area.left, y: area.top + btn_h + 3.0 },
                D2D_POINT_2F { x: area.right, y: area.top + btn_h + 3.0 },
                &lb, 1.0, None,
            );
        }

        let mut row_top = area.top + 30.0;
        let footer_top = area.bottom - 18.0;

        if ctrl.clips_view_notes {
            let input_rect = D2D_RECT_F {
                left: area.left, top: area.top + 27.0, right: area.right, bottom: area.top + 49.0,
            };
            if let Ok(bb) = self.rt.CreateSolidColorBrush(
                &D2D1_COLOR_F { r: 0.08, g: 0.10, b: 0.15, a: 0.9 * alpha }, None,
            ) {
                self.rt.FillRoundedRectangle(
                    &D2D1_ROUNDED_RECT { rect: input_rect, radiusX: 6.0, radiusY: 6.0 }, &bb,
                );
            }
            let hint = if ctrl.note_input.is_empty() && !ctrl.note_input_active {
                "+ Type a note (click here, Enter saves)"
            } else {
                &ctrl.note_input
            };
            let mut shown = hint.to_string();
            if ctrl.note_input_active { shown.push('|'); }
            self.draw_text_left(
                &shown, &self.fmt_caption,
                D2D_RECT_F { left: input_rect.left + 10.0, ..input_rect },
                if ctrl.note_input.is_empty() && !ctrl.note_input_active { muted } else { white },
            );
            row_top = area.top + 53.0;
        }

        if ctrl.ocr_busy {
            self.draw_text_centered(
                "Reading screen text…", &self.fmt_body,
                D2D_RECT_F { left: area.left, top: row_top + 10.0, right: area.right, bottom: footer_top },
                cyan,
            );
            return;
        }

        let rows = ctrl.clips_rows();
        if rows.is_empty() {
            let msg = if ctrl.clips_view_notes {
                "No notes yet — type above"
            } else {
                "Copy anything — it lands here"
            };
            self.draw_text_centered(
                msg, &self.fmt_body,
                D2D_RECT_F { left: area.left, top: row_top + 12.0, right: area.right, bottom: footer_top },
                muted,
            );
            return;
        }

        let pitch = 30.0_f32;
        let max_rows = (((footer_top - row_top - 4.0) / pitch).floor() as usize).max(1);
        let first = ctrl.clips_scroll.min(rows.len().saturating_sub(1));
        for (i, text) in rows.iter().skip(first).take(max_rows).enumerate() {
            let abs_idx = first + i;
            let ry = row_top + i as f32 * pitch;
            let row_rect = D2D_RECT_F {
                left: area.left, top: ry, right: area.right, bottom: ry + pitch - 4.0,
            };
            if let Ok(bb) = self.rt.CreateSolidColorBrush(
                &D2D1_COLOR_F { r: 0.10, g: 0.13, b: 0.19, a: 0.85 * alpha }, None,
            ) {
                self.rt.FillRoundedRectangle(
                    &D2D1_ROUNDED_RECT { rect: row_rect, radiusX: 7.0, radiusY: 7.0 }, &bb,
                );
            }

            let badge = D2D_RECT_F {
                left: row_rect.left + 7.0, top: ry + 4.0,
                right: row_rect.left + 33.0, bottom: ry + 22.0,
            };
            if let Ok(bb) = self.rt.CreateSolidColorBrush(
                &D2D1_COLOR_F { r: accent.r, g: accent.g, b: accent.b, a: 0.30 * alpha }, None,
            ) {
                self.rt.FillRoundedRectangle(
                    &D2D1_ROUNDED_RECT { rect: badge, radiusX: 5.0, radiusY: 5.0 }, &bb,
                );
            }
            self.draw_text_centered(
                &format!("{:02}", abs_idx + 1), &self.fmt_caption, badge, cyan,
            );

            let delete_zone = if ctrl.clips_view_notes { 30.0 } else { 8.0 };
            self.draw_text_left(
                text, &self.fmt_caption,
                D2D_RECT_F {
                    left: badge.right + 9.0, top: ry + 2.0,
                    right: row_rect.right - delete_zone, bottom: row_rect.bottom,
                },
                white,
            );

            if ctrl.clips_view_notes {
                self.draw_text_centered(
                    "✕", &self.fmt_caption,
                    D2D_RECT_F {
                        left: row_rect.right - 26.0, top: ry + 2.0,
                        right: row_rect.right - 4.0, bottom: row_rect.bottom,
                    },
                    muted,
                );
            }
        }

        if rows.len() > max_rows {
            self.draw_text_left(
                &format!("{}-{} / {}", first + 1, (first + max_rows).min(rows.len()), rows.len()),
                &self.fmt_caption,
                D2D_RECT_F { left: area.left + 4.0, top: footer_top, right: area.right - 130.0, bottom: area.bottom },
                muted,
            );
            self.draw_text_right(
                "scroll ↕  click = copy", &self.fmt_caption,
                D2D_RECT_F { left: area.right - 132.0, top: footer_top, right: area.right, bottom: area.bottom },
                muted,
            );
        } else {
            self.draw_text_right(
                "click = copy", &self.fmt_caption,
                D2D_RECT_F { left: area.right - 90.0, top: footer_top, right: area.right, bottom: area.bottom },
                muted,
            );
        }
    }

}

// ─────────────────────────────────────────────────────────────────────────────
// Solid Liquid Glass theme + compact 6-section layout
// ─────────────────────────────────────────────────────────────────────────────

pub mod glass {
    use windows::Win32::Graphics::Direct2D::Common::{D2D1_COLOR_F, D2D_POINT_2F};

    pub const PAD: f32 = 14.0;
    pub const HDR_TOP: f32 = 11.0;
    pub const HDR_BOT: f32 = 29.0;
    pub const BODY_TOP: f32 = 36.0;
    pub const BODY_BOT: f32 = 142.0;
    pub const DOTS_TOP: f32 = 150.0;

    pub fn col(r: f32, g: f32, b: f32, a: f32) -> D2D1_COLOR_F {
        D2D1_COLOR_F { r, g, b, a }
    }

    pub fn pt(x: f32, y: f32) -> D2D_POINT_2F {
        D2D_POINT_2F { x, y }
    }

    /// Aksen ambient per section: cyan (Hub/Control), indigo (Music),
    /// amber (Focus), emerald saat charging.
    pub fn accent_for(state: crate::notch::states::NotchState, charging: bool) -> (f32, f32, f32) {
        use crate::notch::states::NotchState;
        if charging && state == NotchState::Control {
            return (0.18, 0.86, 0.52);
        }
        match state {
            NotchState::Music => (0.42, 0.42, 0.98),
            NotchState::Pomo => (0.98, 0.68, 0.26),
            NotchState::Clips => (0.36, 0.62, 0.96),
            NotchState::Shelf => (0.32, 0.80, 0.86),
            _ => (0.24, 0.72, 0.98),
        }
    }
}

impl NotchRenderer {
    unsafe fn grad(
        &self,
        p0: D2D_POINT_2F,
        p1: D2D_POINT_2F,
        stops: &[(f32, D2D1_COLOR_F)],
    ) -> Option<ID2D1LinearGradientBrush> {
        let gstops: Vec<_> = stops
            .iter()
            .map(|(p, c)| D2D1_GRADIENT_STOP {
                position: *p,
                color: *c,
            })
            .collect();
        let sc = self
            .rt
            .CreateGradientStopCollection(&gstops, D2D1_GAMMA_2_2, D2D1_EXTEND_MODE_CLAMP)
            .ok()?;
        let props = D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES {
            startPoint: p0,
            endPoint: p1,
        };
        self.rt.CreateLinearGradientBrush(&props, None, &sc).ok()
    }

    unsafe fn radial(
        &self,
        center: D2D_POINT_2F,
        rx: f32,
        ry: f32,
        stops: &[(f32, D2D1_COLOR_F)],
    ) -> Option<windows::Win32::Graphics::Direct2D::ID2D1RadialGradientBrush> {
        let gstops: Vec<_> = stops
            .iter()
            .map(|(p, c)| D2D1_GRADIENT_STOP {
                position: *p,
                color: *c,
            })
            .collect();
        let sc = self
            .rt
            .CreateGradientStopCollection(&gstops, D2D1_GAMMA_2_2, D2D1_EXTEND_MODE_CLAMP)
            .ok()?;
        let props = D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES {
            center,
            gradientOriginOffset: glass::pt(0.0, 0.0),
            radiusX: rx,
            radiusY: ry,
        };
        self.rt.CreateRadialGradientBrush(&props, None, &sc).ok()
    }

    /// Permukaan Solid Liquid Glass: base gradient, kilau atas, ambient tint, rim kaca.
    unsafe fn draw_glass_surface(
        &self,
        ctrl: &NotchController,
        pill: D2D_RECT_F,
        alpha: f32,
    ) {
        let radius = ctrl.corner_radius.value.clamp(7.0, 34.0);
        let rr = D2D1_ROUNDED_RECT {
            rect: pill,
            radiusX: radius,
            radiusY: radius,
        };

        let base = [
            (0.0, glass::col(0.020, 0.024, 0.035, 0.97 * alpha)),
            (0.55, glass::col(0.039, 0.051, 0.078, 0.95 * alpha)),
            (1.0, glass::col(0.051, 0.071, 0.114, 0.96 * alpha)),
        ];
        if let Some(b) = self.grad(
            glass::pt(pill.left, pill.top),
            glass::pt(pill.left, pill.bottom),
            &base,
        ) {
            self.rt.FillRoundedRectangle(&rr, &b);
        }

        // Top specular sheen 26 dip.
        let sheen_bottom = (pill.top + 26.0).min(pill.bottom);
        let sheen = D2D_RECT_F {
            left: pill.left + 1.0,
            top: pill.top,
            right: pill.right - 1.0,
            bottom: sheen_bottom,
        };
        if let Some(b) = self.grad(
            glass::pt(0.0, sheen.top),
            glass::pt(0.0, sheen.bottom),
            &[
                (0.0, glass::col(1.0, 1.0, 1.0, 0.08 * alpha)),
                (1.0, glass::col(1.0, 1.0, 1.0, 0.0)),
            ],
        ) {
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: sheen,
                    radiusX: radius * 0.8,
                    radiusY: radius * 0.8,
                },
                &b,
            );
        }

        // Dynamic ambient tint mengikuti section aktif.
        let (ar, ag, ab) = glass::accent_for(ctrl.state, ctrl.is_charging);
        let cx = (pill.left + pill.right) * 0.5;
        let pw = pill.right - pill.left;
        let ph = pill.bottom - pill.top;
        if let Some(b) = self.radial(
            glass::pt(cx, pill.bottom - 4.0),
            pw * 0.58,
            ph * 0.80,
            &[
                (0.0, glass::col(ar, ag, ab, 0.13 * alpha)),
                (0.55, glass::col(ar, ag, ab, 0.045 * alpha)),
                (1.0, glass::col(0.0, 0.0, 0.0, 0.0)),
            ],
        ) {
            self.rt.FillRoundedRectangle(&rr, &b);
        }

        // Liquid glass rim: diagonal gradient stroke.
        if let Some(b) = self.grad(
            glass::pt(pill.left, pill.top),
            glass::pt(pill.right, pill.bottom),
            &[
                (0.0, glass::col(1.0, 1.0, 1.0, 0.24 * alpha)),
                (0.52, glass::col(0.47, 0.82, 1.0, 0.14 * alpha)),
                (1.0, glass::col(1.0, 1.0, 1.0, 0.07 * alpha)),
            ],
        ) {
            self.rt.DrawRoundedRectangle(&rr, &b, 1.15, None);
        }
    }

    /// Header tiap section: judul kiri, baterai + jam/tanggal kanan.
    unsafe fn draw_glass_header(
        &self,
        ctrl: &NotchController,
        pill: D2D_RECT_F,
        title: &str,
        alpha: f32,
    ) {
        let x = pill.left + glass::PAD;
        let y = pill.top + glass::HDR_TOP;
        self.draw_text_left(
            title,
            &self.fmt_title,
            D2D_RECT_F {
                left: x,
                top: y - 2.0,
                right: x + 130.0,
                bottom: y + 18.0,
            },
            glass::col(0.62, 0.66, 0.72, alpha),
        );

        let now = Local::now();
        let stamp = now.format("%H:%M · %a %d %b").to_string();
        self.draw_text_right(
            &stamp,
            &self.fmt_caption,
            D2D_RECT_F {
                left: pill.right - glass::PAD - 150.0,
                top: y,
                right: pill.right - glass::PAD,
                bottom: y + 16.0,
            },
            glass::col(0.62, 0.68, 0.78, alpha * 0.9),
        );
        self.draw_nusa_ring_battery(
            pill.right - glass::PAD - 160.0,
            y + 7.0,
            ctrl.battery_percent,
            ctrl.is_charging,
            alpha,
        );
    }

    /// Navigasi: chevron kiri/kanan + 6 pager dots di bawah tengah.
    unsafe fn draw_glass_nav(&self, ctrl: &NotchController, pill: D2D_RECT_F, alpha: f32) {
        let cy = (pill.top + pill.bottom) * 0.5;
        for (side, label) in [(-1.0_f32, "<"), (1.0_f32, ">")] {
            let cx = if side < 0.0 {
                pill.left + 12.0
            } else {
                pill.right - 12.0
            };
            if let Ok(b) = self.rt.CreateSolidColorBrush(
                &glass::col(1.0, 1.0, 1.0, if ctrl.is_hovered { 0.10 } else { 0.06 } * alpha),
                None,
            ) {
                self.rt.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: glass::pt(cx, cy),
                        radiusX: 11.0,
                        radiusY: 11.0,
                    },
                    &b,
                );
            }
            self.draw_text_centered(
                label,
                &self.fmt_caption,
                D2D_RECT_F {
                    left: cx - 11.0,
                    top: cy - 8.0,
                    right: cx + 11.0,
                    bottom: cy + 8.0,
                },
                glass::col(0.80, 0.84, 0.90, alpha * 0.85),
            );
        }

        let tabs = NotchController::ordered_tabs();
        let active = tabs.iter().position(|&s| s == ctrl.state).unwrap_or(0);
        let dot_y = pill.bottom - 9.0;
        let spacing = 11.0;
        let start = (pill.left + pill.right) * 0.5 - spacing * 2.5;
        for i in 0..6 {
            let on = i == active;
            let cx = start + spacing * i as f32;
            if let Ok(b) = self.rt.CreateSolidColorBrush(
                &glass::col(
                    1.0,
                    1.0,
                    1.0,
                    if on { 0.92 } else { 0.22 } * alpha,
                ),
                None,
            ) {
                if on {
                    self.rt.FillRoundedRectangle(
                        &D2D1_ROUNDED_RECT {
                            rect: D2D_RECT_F {
                                left: cx - 7.0,
                                top: dot_y - 2.0,
                                right: cx + 7.0,
                                bottom: dot_y + 2.0,
                            },
                            radiusX: 2.0,
                            radiusY: 2.0,
                        },
                        &b,
                    );
                } else {
                    self.rt.FillEllipse(
                        &D2D1_ELLIPSE {
                            point: glass::pt(cx, dot_y),
                            radiusX: 2.0,
                            radiusY: 2.0,
                        },
                        &b,
                    );
                }
            }
        }
    }

    unsafe fn hairline(&self, x0: f32, x1: f32, y: f32, alpha: f32) {
        if let Ok(b) = self.rt.CreateSolidColorBrush(&glass::col(1.0, 1.0, 1.0, 0.08 * alpha), None) {
            self.rt.FillRectangle(
                &D2D_RECT_F {
                    left: x0,
                    top: y,
                    right: x1,
                    bottom: y + 1.0,
                },
                &b,
            );
        }
    }

    unsafe fn glass_pill(
        &self,
        rect: D2D_RECT_F,
        radius: f32,
        fill: D2D1_COLOR_F,
        stroke: Option<D2D1_COLOR_F>,
        stroke_w: f32,
    ) {
        if let Ok(b) = self.rt.CreateSolidColorBrush(&fill, None) {
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect,
                    radiusX: radius,
                    radiusY: radius,
                },
                &b,
            );
        }
        if let Some(sc) = stroke {
            if let Ok(b) = self.rt.CreateSolidColorBrush(&sc, None) {
                self.rt.DrawRoundedRectangle(
                    &D2D1_ROUNDED_RECT {
                        rect,
                        radiusX: radius,
                        radiusY: radius,
                    },
                    &b,
                    stroke_w,
                    None,
                );
            }
        }
    }

    // ── Section 1: Hub ──────────────────────────────────────────────────────
    unsafe fn draw_hub_glass(&self, ctrl: &NotchController, pill: D2D_RECT_F, alpha: f32) {
        self.draw_glass_header(ctrl, pill, "Hub", alpha);

        let left = pill.left + glass::PAD;
        let top = pill.top + glass::BODY_TOP;
        let mid_x = left + (pill.right - pill.left - glass::PAD * 2.0) * 0.40;
        let bottom = pill.top + glass::BODY_BOT;

        let now = Local::now();
        self.draw_text_left(
            &now.format("%H:%M").to_string(),
            &self.fmt_clock_hero,
            D2D_RECT_F {
                left,
                top: top + 4.0,
                right: mid_x - 12.0,
                bottom: top + 62.0,
            },
            glass::col(1.0, 1.0, 1.0, alpha),
        );
        self.draw_text_left(
            &now.format("%A · %d %B").to_string(),
            &self.fmt_caption,
            D2D_RECT_F {
                left: left + 2.0,
                top: top + 62.0,
                right: mid_x - 8.0,
                bottom: top + 80.0,
            },
            glass::col(0.56, 0.62, 0.72, alpha),
        );

        // Pemisah vertikal 1px.
        if let Ok(b) = self.rt.CreateSolidColorBrush(&glass::col(1.0, 1.0, 1.0, 0.08 * alpha), None)
        {
            self.rt.FillRectangle(
                &D2D_RECT_F {
                    left: mid_x,
                    top: top + 2.0,
                    right: mid_x + 1.0,
                    bottom: bottom - 10.0,
                },
                &b,
            );
        }

        let quote = ctrl.current_quote();
        self.draw_text_left(
            quote.quote,
            &self.fmt_body,
            D2D_RECT_F {
                left: mid_x + glass::PAD,
                top: top + 2.0,
                right: pill.right - glass::PAD,
                bottom: top + 62.0,
            },
            glass::col(0.85, 0.87, 0.91, alpha),
        );
        self.draw_text_left(
            quote.author,
            &self.fmt_caption,
            D2D_RECT_F {
                left: mid_x + glass::PAD,
                top: top + 64.0,
                right: pill.right - glass::PAD,
                bottom: top + 80.0,
            },
            glass::col(0.48, 0.51, 0.57, alpha),
        );
    }

    // ── Section 2: Music (2A info, 2B lyrics) ───────────────────────────────
    unsafe fn draw_music_glass(&self, ctrl: &NotchController, pill: D2D_RECT_F, alpha: f32) {
        self.draw_glass_header(ctrl, pill, "Music", alpha);
        let left = pill.left + glass::PAD;
        let right = pill.right - glass::PAD;

        if ctrl.show_side_lyrics {
            // 2B. Mini header + 3 baris lirik + progress tipis.
            let mini_y = pill.top + 34.0;
            self.glass_pill(
                D2D_RECT_F {
                    left: left,
                    top: mini_y,
                    right: left + 22.0,
                    bottom: mini_y + 22.0,
                },
                6.0,
                glass::col(0.10, 0.12, 0.18, 0.9 * alpha),
                Some(glass::col(1.0, 1.0, 1.0, 0.10 * alpha)),
                0.8,
            );
            self.draw_text_left(
                &format!("{} · {}", ctrl.media_title, ctrl.media_artist),
                &self.fmt_caption,
                D2D_RECT_F {
                    left: left + 30.0,
                    top: mini_y + 2.0,
                    right: right - 62.0,
                    bottom: mini_y + 20.0,
                },
                glass::col(0.82, 0.84, 0.88, alpha),
            );
            // Prev / Play / Next kompak + tombol Lyrics aktif.
            let by = mini_y + 11.0;
            for (dx, label) in [(-46.0_f32, "⏮"), (-24.0, if ctrl.media_playing { "⏸" } else { "⏵" }), (0.0, "⏭")] {
                self.draw_text_centered(
                    label,
                    &self.fmt_caption,
                    D2D_RECT_F {
                        left: right + dx - 10.0,
                        top: by - 10.0,
                        right: right + dx + 10.0,
                        bottom: by + 10.0,
                    },
                    glass::col(0.92, 0.94, 0.98, alpha),
                );
            }
            self.glass_pill(
                D2D_RECT_F {
                    left: right - 30.0,
                    top: mini_y,
                    right: right,
                    bottom: mini_y + 22.0,
                },
                11.0,
                glass::col(0.55, 0.85, 1.0, 0.22 * alpha),
                Some(glass::col(0.60, 0.88, 1.0, 0.45 * alpha)),
                0.9,
            );

            let stage_top = pill.top + 62.0;
            let prev_line = if ctrl.active_lyric_idx > 0 {
                ctrl.timed_lyrics
                    .get(ctrl.active_lyric_idx - 1)
                    .map(|(_, t)| t.clone())
                    .unwrap_or_default()
            } else {
                String::new()
            };
            let cur_line = ctrl
                .timed_lyrics
                .get(ctrl.active_lyric_idx)
                .map(|(_, t)| t.clone())
                .unwrap_or_else(|| "No synced lyrics available".to_string());
            let next_line = ctrl
                .timed_lyrics
                .get(ctrl.active_lyric_idx + 1)
                .map(|(_, t)| t.clone())
                .unwrap_or_default();

            self.draw_text_centered(
                &prev_line,
                &self.fmt_caption,
                D2D_RECT_F {
                    left: left,
                    top: stage_top,
                    right: right,
                    bottom: stage_top + 18.0,
                },
                glass::col(0.42, 0.45, 0.50, alpha * 0.75),
            );
            self.draw_text_centered(
                &cur_line,
                &self.fmt_title,
                D2D_RECT_F {
                    left: left,
                    top: stage_top + 24.0,
                    right: right,
                    bottom: stage_top + 62.0,
                },
                glass::col(1.0, 1.0, 1.0, alpha),
            );
            self.draw_text_centered(
                &next_line,
                &self.fmt_caption,
                D2D_RECT_F {
                    left: left,
                    top: stage_top + 66.0,
                    right: right,
                    bottom: stage_top + 84.0,
                },
                glass::col(0.61, 0.64, 0.70, alpha * 0.8),
            );

            // Slim progress 2.5 dip (interaktif).
            let py = pill.top + 146.0;
            self.glass_pill(
                D2D_RECT_F {
                    left: left,
                    top: py,
                    right: right,
                    bottom: py + 2.5,
                },
                1.25,
                glass::col(1.0, 1.0, 1.0, 0.12 * alpha),
                None,
                0.0,
            );
            let fill_r = left + (right - left) * ctrl.media_progress.clamp(0.0, 1.0);
            self.glass_pill(
                D2D_RECT_F {
                    left: left,
                    top: py,
                    right: fill_r.max(left + 2.0),
                    bottom: py + 2.5,
                },
                1.25,
                glass::col(0.62, 0.86, 1.0, 0.95 * alpha),
                None,
                0.0,
            );
            return;
        }

        // 2A. Track row.
        let thumb_y = pill.top + 36.0;
        self.glass_pill(
            D2D_RECT_F {
                left: left,
                top: thumb_y,
                right: left + 48.0,
                bottom: thumb_y + 48.0,
            },
            10.0,
            glass::col(0.09, 0.11, 0.16, 0.95 * alpha),
            Some(glass::col(1.0, 1.0, 1.0, 0.12 * alpha)),
            0.8,
        );
        self.draw_text_left(
            &ctrl.media_title,
            &self.fmt_title,
            D2D_RECT_F {
                left: left + 58.0,
                top: thumb_y + 2.0,
                right: right - 56.0,
                bottom: thumb_y + 24.0,
            },
            glass::col(1.0, 1.0, 1.0, alpha),
        );
        self.draw_text_left(
            &ctrl.media_artist,
            &self.fmt_caption,
            D2D_RECT_F {
                left: left + 58.0,
                top: thumb_y + 24.0,
                right: right - 56.0,
                bottom: thumb_y + 42.0,
            },
            glass::col(0.60, 0.64, 0.70, alpha),
        );
        // Toggle Lyrics.
        self.glass_pill(
            D2D_RECT_F {
                left: right - 44.0,
                top: thumb_y + 2.0,
                right: right,
                bottom: thumb_y + 22.0,
            },
            10.0,
            glass::col(1.0, 1.0, 1.0, 0.10 * alpha),
            Some(glass::col(1.0, 1.0, 1.0, 0.16 * alpha)),
            0.8,
        );
        self.draw_text_centered(
            "Lyrics",
            &self.fmt_caption,
            D2D_RECT_F {
                left: right - 44.0,
                top: thumb_y + 3.0,
                right: right,
                bottom: thumb_y + 21.0,
            },
            glass::col(0.86, 0.92, 1.0, alpha),
        );

        // Seek bar.
        let bar_y = pill.top + 104.0;
        let bar_l = left + 42.0;
        let bar_r = right - 42.0;
        self.draw_text_left(
            &format!("{:02}:{:02}", (ctrl.media_elapsed_exact as u32) / 60, (ctrl.media_elapsed_exact as u32) % 60),
            &self.fmt_caption,
            D2D_RECT_F { left, top: bar_y - 8.0, right: left + 40.0, bottom: bar_y + 8.0 },
            glass::col(0.55, 0.60, 0.68, alpha),
        );
        let remain = (ctrl.media_total_secs - ctrl.media_elapsed_exact).max(0.0) as u32;
        self.draw_text_right(
            &format!("-{:02}:{:02}", remain / 60, remain % 60),
            &self.fmt_caption,
            D2D_RECT_F { left: right - 42.0, top: bar_y - 8.0, right, bottom: bar_y + 8.0 },
            glass::col(0.55, 0.60, 0.68, alpha),
        );
        self.glass_pill(
            D2D_RECT_F { left: bar_l, top: bar_y - 2.0, right: bar_r, bottom: bar_y + 2.0 },
            2.0,
            glass::col(1.0, 1.0, 1.0, 0.12 * alpha),
            None,
            0.0,
        );
        let fx = bar_l + (bar_r - bar_l) * ctrl.media_progress.clamp(0.0, 1.0);
        self.glass_pill(
            D2D_RECT_F { left: bar_l, top: bar_y - 2.0, right: fx.max(bar_l + 2.0), bottom: bar_y + 2.0 },
            2.0,
            glass::col(0.92, 0.96, 1.0, 0.95 * alpha),
            None,
            0.0,
        );
        if let Ok(b) = self.rt.CreateSolidColorBrush(&glass::col(1.0, 1.0, 1.0, alpha), None) {
            self.rt.FillEllipse(
                &D2D1_ELLIPSE { point: glass::pt(fx, bar_y), radiusX: 4.0, radiusY: 4.0 },
                &b,
            );
        }

        // Controls: prev / play / next, rapat 24 dip.
        let ctrl_y = pill.top + 130.0;
        let cx = (pill.left + pill.right) * 0.5;
        self.glass_pill(
            D2D_RECT_F { left: cx - 16.0, top: ctrl_y - 16.0, right: cx + 16.0, bottom: ctrl_y + 16.0 },
            16.0,
            glass::col(1.0, 1.0, 1.0, 0.12 * alpha),
            Some(glass::col(0.70, 0.90, 1.0, 0.30 * alpha)),
            1.0,
        );
        self.draw_text_centered(
            if ctrl.media_playing { "⏸" } else { "⏵" },
            &self.fmt_title,
            D2D_RECT_F { left: cx - 14.0, top: ctrl_y - 11.0, right: cx + 14.0, bottom: ctrl_y + 11.0 },
            glass::col(1.0, 1.0, 1.0, alpha),
        );
        for (dx, label) in [(-40.0_f32, "⏮"), (40.0, "⏭")] {
            self.draw_text_centered(
                label,
                &self.fmt_title,
                D2D_RECT_F {
                    left: cx + dx - 14.0,
                    top: ctrl_y - 11.0,
                    right: cx + dx + 14.0,
                    bottom: ctrl_y + 11.0,
                },
                glass::col(0.86, 0.90, 0.96, alpha * 0.9),
            );
        }
    }


    // ── Section 3: Control ─────────────────────────────────────────────────
    unsafe fn draw_control_glass(&self, ctrl: &NotchController, pill: D2D_RECT_F, alpha: f32) {
        self.draw_glass_header(ctrl, pill, "Control", alpha);
        let left = pill.left + glass::PAD;
        let right = pill.right - glass::PAD;
        let w = right - left;

        for (i, (icon, pct, warm)) in [
            ("🔊", ctrl.volume_level, false),
            ("🔆", ctrl.brightness_level, true),
        ]
        .iter()
        .enumerate()
        {
            let y = pill.top + 46.0 + i as f32 * 46.0;
            let row = D2D_RECT_F {
                left,
                top: y,
                right,
                bottom: y + 32.0,
            };
            self.glass_pill(
                row,
                16.0,
                glass::col(1.0, 1.0, 1.0, 0.08 * alpha),
                Some(glass::col(1.0, 1.0, 1.0, 0.12 * alpha)),
                0.8,
            );
            let fill_w = w * (*pct as f32 / 100.0);
            let (fr, fg, fb) = if *warm {
                (1.0, 0.86, 0.62)
            } else {
                (0.72, 0.92, 1.0)
            };
            if fill_w > 8.0 {
                self.glass_pill(
                    D2D_RECT_F {
                        left: left + 1.0,
                        top: y + 1.0,
                        right: (left + fill_w).min(right - 1.0),
                        bottom: y + 31.0,
                    },
                    15.0,
                    glass::col(fr, fg, fb, 0.16 * alpha),
                    None,
                    0.0,
                );
            }
            self.draw_text_left(
                icon,
                &self.fmt_title,
                D2D_RECT_F {
                    left: left + 10.0,
                    top: y + 5.0,
                    right: left + 40.0,
                    bottom: y + 28.0,
                },
                glass::col(0.92, 0.95, 1.0, alpha),
            );
            self.draw_text_right(
                &format!("{}%", pct),
                &self.fmt_title,
                D2D_RECT_F {
                    left: right - 62.0,
                    top: y + 5.0,
                    right: right - 10.0,
                    bottom: y + 28.0,
                },
                glass::col(0.95, 0.97, 1.0, alpha),
            );
        }

        self.draw_text_left(
            &format!("Vol {}%  ·  Bright {}%", ctrl.volume_level, ctrl.brightness_level),
            &self.fmt_caption,
            D2D_RECT_F {
                left,
                top: pill.top + 136.0,
                right,
                bottom: pill.top + 150.0,
            },
            glass::col(0.48, 0.52, 0.60, alpha),
        );
    }

    // ── Section 4: Focus (Pomodoro) ────────────────────────────────────────
    unsafe fn draw_focus_glass(&self, ctrl: &NotchController, pill: D2D_RECT_F, alpha: f32) {
        self.draw_glass_header(ctrl, pill, "Focus", alpha);
        let left = pill.left + glass::PAD;
        let right = pill.right - glass::PAD;

        // Segmented toggle 25m / 5m.
        let seg = D2D_RECT_F {
            left: right - 132.0,
            top: pill.top + 8.0,
            right,
            bottom: pill.top + 32.0,
        };
        self.glass_pill(
            seg,
            12.0,
            glass::col(1.0, 1.0, 1.0, 0.08 * alpha),
            Some(glass::col(1.0, 1.0, 1.0, 0.10 * alpha)),
            0.8,
        );
        let half = (seg.right - seg.left) * 0.5;
        let active_x = if ctrl.pomo_is_break { seg.left + half } else { seg.left };
        self.glass_pill(
            D2D_RECT_F {
                left: active_x + 1.0,
                top: seg.top + 1.0,
                right: active_x + half - 1.0,
                bottom: seg.bottom - 1.0,
            },
            11.0,
            glass::col(0.62, 0.86, 1.0, 0.28 * alpha),
            None,
            0.0,
        );
        self.draw_text_centered(
            "25m Focus",
            &self.fmt_caption,
            D2D_RECT_F { left: seg.left, top: seg.top + 2.0, right: seg.left + half, bottom: seg.bottom - 1.0 },
            glass::col(1.0, 1.0, 1.0, if ctrl.pomo_is_break { alpha * 0.55 } else { alpha }),
        );
        self.draw_text_centered(
            "5m Break",
            &self.fmt_caption,
            D2D_RECT_F { left: seg.left + half, top: seg.top + 2.0, right: seg.right, bottom: seg.bottom - 1.0 },
            glass::col(1.0, 1.0, 1.0, if ctrl.pomo_is_break { alpha } else { alpha * 0.55 }),
        );

        let mins = ctrl.pomo_remaining_secs / 60;
        let secs = ctrl.pomo_remaining_secs % 60;
        self.draw_text_left(
            &format!("{:02}:{:02}", mins, secs),
            &self.fmt_clock_hero,
            D2D_RECT_F {
                left,
                top: pill.top + 44.0,
                right: right - 96.0,
                bottom: pill.top + 106.0,
            },
            glass::col(1.0, 1.0, 1.0, alpha),
        );

        let btn_y = pill.top + 74.0;
        self.glass_pill(
            D2D_RECT_F { left: right - 76.0, top: btn_y - 18.0, right: right - 40.0, bottom: btn_y + 18.0 },
            18.0,
            glass::col(0.62, 0.86, 1.0, 0.20 * alpha),
            Some(glass::col(0.70, 0.90, 1.0, 0.35 * alpha)),
            1.0,
        );
        self.draw_text_centered(
            if ctrl.pomo_running { "⏸" } else { "⏵" },
            &self.fmt_title,
            D2D_RECT_F { left: right - 76.0, top: btn_y - 12.0, right: right - 40.0, bottom: btn_y + 12.0 },
            glass::col(1.0, 1.0, 1.0, alpha),
        );
        self.glass_pill(
            D2D_RECT_F { left: right - 34.0, top: btn_y - 16.0, right: right - 2.0, bottom: btn_y + 16.0 },
            16.0,
            glass::col(1.0, 1.0, 1.0, 0.07 * alpha),
            Some(glass::col(1.0, 1.0, 1.0, 0.12 * alpha)),
            0.8,
        );
        self.draw_text_centered(
            "↺",
            &self.fmt_title,
            D2D_RECT_F { left: right - 34.0, top: btn_y - 12.0, right: right - 2.0, bottom: btn_y + 12.0 },
            glass::col(0.90, 0.93, 0.98, alpha),
        );

        // Progress line bawah.
        let py = pill.top + 128.0;
        let ratio = if ctrl.pomo_total_secs > 0 {
            1.0 - (ctrl.pomo_remaining_secs as f32 / ctrl.pomo_total_secs as f32)
        } else {
            0.0
        }
        .clamp(0.0, 1.0);
        self.glass_pill(
            D2D_RECT_F { left, top: py, right, bottom: py + 3.5 },
            1.75,
            glass::col(1.0, 1.0, 1.0, 0.12 * alpha),
            None,
            0.0,
        );
        let fx = left + (right - left) * ratio;
        self.glass_pill(
            D2D_RECT_F { left, top: py, right: fx.max(left + 2.0), bottom: py + 3.5 },
            1.75,
            glass::col(1.0, 0.80, 0.45, 0.95 * alpha),
            None,
            0.0,
        );
    }

    // ── Section 5: Shelf ───────────────────────────────────────────────────
    unsafe fn draw_shelf_glass(&self, ctrl: &NotchController, pill: D2D_RECT_F, alpha: f32) {
        self.draw_glass_header(ctrl, pill, "Shelf", alpha);
        let left = pill.left + glass::PAD;
        let right = pill.right - glass::PAD;
        let top = pill.top + 38.0;
        let bottom = pill.top + glass::BODY_BOT;

        if ctrl.shelf_files.is_empty() {
            // Dashed dropzone bersih.
            if let Ok(style) = self.rt.CreateSolidColorBrush(&glass::col(1.0, 1.0, 1.0, 0.18 * alpha), None)
            {
                let _ = style;
            }
            self.glass_pill(
                D2D_RECT_F { left, top, right, bottom },
                14.0,
                glass::col(1.0, 1.0, 1.0, 0.03 * alpha),
                Some(glass::col(1.0, 1.0, 1.0, 0.16 * alpha)),
                1.0,
            );
            self.draw_text_centered(
                "Drop files here",
                &self.fmt_title,
                D2D_RECT_F { left, top: top + 32.0, right, bottom: top + 56.0 },
                glass::col(0.92, 0.95, 1.0, alpha),
            );
            self.draw_text_centered(
                "Drag out anytime · Click to open",
                &self.fmt_caption,
                D2D_RECT_F { left, top: top + 58.0, right, bottom: top + 74.0 },
                glass::col(0.52, 0.56, 0.63, alpha),
            );
            return;
        }

        // Chip baris, maks 4 per baris.
        let mut x = left;
        let mut y = top;
        let chip_w = ((right - left) * 0.5) - 6.0;
        for (name, size, _path) in ctrl.shelf_files.iter().take(6) {
            if x + chip_w > right {
                x = left;
                y += 36.0;
            }
            self.glass_pill(
                D2D_RECT_F { left: x, top: y, right: x + chip_w, bottom: y + 30.0 },
                10.0,
                glass::col(1.0, 1.0, 1.0, 0.07 * alpha),
                Some(glass::col(1.0, 1.0, 1.0, 0.10 * alpha)),
                0.8,
            );
            self.draw_text_left(
                name,
                &self.fmt_caption,
                D2D_RECT_F { left: x + 10.0, top: y + 3.0, right: x + chip_w - 8.0, bottom: y + 18.0 },
                glass::col(0.92, 0.94, 0.98, alpha),
            );
            self.draw_text_left(
                size,
                &self.fmt_caption,
                D2D_RECT_F { left: x + 10.0, top: y + 16.0, right: x + chip_w - 8.0, bottom: y + 29.0 },
                glass::col(0.52, 0.56, 0.63, alpha),
            );
            x += chip_w + 12.0;
        }
    }

    // ── Section 6: Clips (clipboard + quick notes) ─────────────────────────
    unsafe fn draw_clips_glass(&self, ctrl: &NotchController, pill: D2D_RECT_F, alpha: f32) {
        self.draw_glass_header(ctrl, pill, "Clips", alpha);
        let left = pill.left + glass::PAD;
        let right = pill.right - glass::PAD;
        let mut y = pill.top + 38.0;

        // Tombol + Note / riwayat.
        self.glass_pill(
            D2D_RECT_F { left: right - 58.0, top: pill.top + 8.0, right, bottom: pill.top + 30.0 },
            11.0,
            glass::col(1.0, 1.0, 1.0, 0.09 * alpha),
            Some(glass::col(1.0, 1.0, 1.0, 0.14 * alpha)),
            0.8,
        );
        self.draw_text_centered(
            if ctrl.clips_view_notes { "History" } else { "+ Note" },
            &self.fmt_caption,
            D2D_RECT_F { left: right - 58.0, top: pill.top + 10.0, right, bottom: pill.top + 28.0 },
            glass::col(0.88, 0.94, 1.0, alpha),
        );

        if ctrl.note_input_active {
            self.glass_pill(
                D2D_RECT_F { left, top: y, right, bottom: y + 26.0 },
                9.0,
                glass::col(1.0, 1.0, 1.0, 0.06 * alpha),
                Some(glass::col(0.62, 0.86, 1.0, 0.35 * alpha)),
                1.0,
            );
            self.draw_text_left(
                if ctrl.note_input.is_empty() { "Type a note, Enter to save" } else { &ctrl.note_input },
                &self.fmt_caption,
                D2D_RECT_F { left: left + 10.0, top: y + 4.0, right: right - 10.0, bottom: y + 23.0 },
                glass::col(0.88, 0.92, 1.0, alpha),
            );
            y += 34.0;
        }

        let rows = ctrl.clips_rows();
        if rows.is_empty() {
            self.draw_text_centered(
                if ctrl.clips_view_notes { "No notes yet" } else { "Clipboard is empty" },
                &self.fmt_caption,
                D2D_RECT_F { left, top: y + 24.0, right, bottom: y + 48.0 },
                glass::col(0.50, 0.54, 0.61, alpha),
            );
        }

        let max_rows = 3;
        for (i, row) in rows.iter().enumerate().skip(ctrl.clips_scroll).take(max_rows) {
            let ry = y + (i - ctrl.clips_scroll) as f32 * 32.0;
            self.draw_text_left(
                row,
                &self.fmt_caption,
                D2D_RECT_F { left: left + 4.0, top: ry + 6.0, right: right - 22.0, bottom: ry + 24.0 },
                glass::col(0.86, 0.88, 0.92, alpha),
            );
            self.hairline(left, right, ry + 30.0, alpha * 0.9);
        }
    }

    unsafe fn draw_glass_panel(&self, ctrl: &NotchController, pill: D2D_RECT_F, alpha: f32) {
        match ctrl.state {
            NotchState::Hub => self.draw_hub_glass(ctrl, pill, alpha),
            NotchState::Music => self.draw_music_glass(ctrl, pill, alpha),
            NotchState::Control => self.draw_control_glass(ctrl, pill, alpha),
            NotchState::Pomo => self.draw_focus_glass(ctrl, pill, alpha),
            NotchState::Shelf => self.draw_shelf_glass(ctrl, pill, alpha),
            NotchState::Clips => self.draw_clips_glass(ctrl, pill, alpha),
            _ => {}
        }
    }
}
