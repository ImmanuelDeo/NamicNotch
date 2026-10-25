// src/notch/renderer.rs
use chrono::Local;
use windows::core::{Interface, PCWSTR};
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Direct2D::Common::{
    D2D1_ALPHA_MODE_PREMULTIPLIED, D2D1_BEZIER_SEGMENT, D2D1_COLOR_F, D2D1_FIGURE_BEGIN_FILLED,
    D2D1_FIGURE_END_CLOSED, D2D1_PIXEL_FORMAT, D2D_POINT_2F, D2D_RECT_F, D2D_SIZE_U,
};
use windows::Win32::Graphics::Direct2D::{
    D2D1CreateFactory, ID2D1Factory1, ID2D1HwndRenderTarget, ID2D1PathGeometry, ID2D1StrokeStyle,
    D2D1_ANTIALIAS_MODE_PER_PRIMITIVE, D2D1_CAP_STYLE_ROUND, D2D1_DASH_STYLE_DASH, D2D1_ELLIPSE,
    D2D1_FACTORY_TYPE_SINGLE_THREADED, D2D1_FEATURE_LEVEL_DEFAULT,
    D2D1_HWND_RENDER_TARGET_PROPERTIES, D2D1_LINE_JOIN_ROUND, D2D1_PRESENT_OPTIONS_IMMEDIATELY,
    D2D1_RENDER_TARGET_PROPERTIES, D2D1_RENDER_TARGET_TYPE_DEFAULT, D2D1_RENDER_TARGET_USAGE_NONE,
    D2D1_ROUNDED_RECT, D2D1_STROKE_STYLE_PROPERTIES1, D2D1_STROKE_TRANSFORM_TYPE_NORMAL,
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

pub struct AntiMagicNeutralPalette;

impl AntiMagicNeutralPalette {
    pub const PILL_SOLID_BLACK: D2D1_COLOR_F = D2D1_COLOR_F { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const CARD_ELEVATED: D2D1_COLOR_F = D2D1_COLOR_F { r: 0.115, g: 0.148, b: 0.212, a: 0.92 };
    pub const ACCENT_BLUE: D2D1_COLOR_F = D2D1_COLOR_F { r: 0.231, g: 0.510, b: 0.965, a: 1.0 }; // #3B82F6
    pub const ACCENT_CYAN: D2D1_COLOR_F = D2D1_COLOR_F { r: 0.220, g: 0.741, b: 0.973, a: 1.0 }; // #38BDF8
    pub const ACCENT_WINE: D2D1_COLOR_F = D2D1_COLOR_F { r: 0.680, g: 0.220, b: 0.310, a: 0.95 };
    pub const TEXT_PRIMARY: D2D1_COLOR_F = D2D1_COLOR_F { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const TEXT_SECONDARY: D2D1_COLOR_F = D2D1_COLOR_F { r: 0.612, g: 0.639, b: 0.686, a: 1.0 }; // #9CA3AF
}

pub struct NotchRenderer {
    pub factory: ID2D1Factory1,
    pub rt: ID2D1HwndRenderTarget,
    pub dwrite: IDWriteFactory,
    pub dashed_stroke: Option<ID2D1StrokeStyle>,
    pub fmt_clock_hero: IDWriteTextFormat,
    pub fmt_title: IDWriteTextFormat,
    pub fmt_body: IDWriteTextFormat,
    pub fmt_quote_italic: IDWriteTextFormat,
    pub fmt_caption: IDWriteTextFormat,
}

impl NotchRenderer {
    pub fn new(
        hwnd: HWND,
        width: u32,
        height: u32,
        dpi_scale: f32,
    ) -> windows::core::Result<Self> {
        unsafe {
            Self::new_internal(hwnd, width, height, dpi_scale)
        }
    }

    unsafe fn new_internal(
        hwnd: HWND,
        width: u32,
        height: u32,
        dpi_scale: f32,
    ) -> windows::core::Result<Self> {
        let factory: ID2D1Factory1 =
            D2D1CreateFactory(D2D1_FACTORY_TYPE_SINGLE_THREADED, None)?;
        let dwrite: IDWriteFactory = DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED)?;

        let dpi = 96.0 * dpi_scale.max(1.0);
        let render_props = D2D1_RENDER_TARGET_PROPERTIES {
            r#type: D2D1_RENDER_TARGET_TYPE_DEFAULT,
            pixelFormat: D2D1_PIXEL_FORMAT {
                format: DXGI_FORMAT_B8G8R8A8_UNORM,
                alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED,
            },
            dpiX: dpi,
            dpiY: dpi,
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
        let font_family: Vec<u16> = "Segoe UI Variable Display\0".encode_utf16().collect();
        let locale: Vec<u16> = "en-us\0".encode_utf16().collect();

        // Stroke garis putus-putus (dashed border) untuk Dropzone Shelf seperti Nusa Original
                let props = D2D1_STROKE_STYLE_PROPERTIES1 {
                            startCap: D2D1_CAP_STYLE_ROUND,
                            endCap: D2D1_CAP_STYLE_ROUND,
                            dashCap: D2D1_CAP_STYLE_ROUND,
                            lineJoin: D2D1_LINE_JOIN_ROUND,
                            miterLimit: 10.0,
                            dashStyle: D2D1_DASH_STYLE_DASH,
                            dashOffset: 0.0,
                            transformType: D2D1_STROKE_TRANSFORM_TYPE_NORMAL,
                            };
                            let dashed_stroke: Option<ID2D1StrokeStyle> = factory
                            .CreateStrokeStyle(
                                &props,
                                None,
                            )
                            .ok()
                            .map(|s| s.into());

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
        })
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
                Ok(path.into())
            }

    /// Indikator Baterai Lingkaran Hijau (`⚡`) ala Nusa Original (`Screenshot 2026-10-25 210657.png`)
    unsafe fn draw_nusa_ring_battery(&self, cx: f32, cy: f32, pct: u8, charging: bool, alpha: f32) {
        let ring_col = if charging || pct > 20 {
            D2D1_COLOR_F {
                r: 0.13,
                g: 0.85,
                b: 0.38,
                a: alpha,
            }
        } else {
            D2D1_COLOR_F {
                r: 0.94,
                g: 0.27,
                b: 0.27,
                a: alpha,
            }
        };

        if let Ok(rb) = self.rt.CreateSolidColorBrush(&ring_col, None) {
            self.rt.DrawEllipse(
                &D2D1_ELLIPSE {
                    point: D2D_POINT_2F { x: cx, y: cy },
                    radiusX: 7.0,
                    radiusY: 7.0,
                },
                &rb,
                2.0,
                None,
            );
        }

        let bolt = if charging { "⚡" } else { "•" };
        self.draw_text_centered(
            bolt,
            &self.fmt_caption,
            D2D_RECT_F {
                left: cx - 8.0,
                top: cy - 8.0,
                right: cx + 8.0,
                bottom: cy + 8.0,
            },
            D2D1_COLOR_F {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: alpha,
            },
        );
    }

    pub unsafe fn render_frame(&self, ctrl: &mut NotchController, pill_rect: D2D_RECT_F) {
            self.draw_frame(ctrl, pill_rect);
        }

        pub unsafe fn draw_frame(&self, ctrl: &mut NotchController, pill_rect: D2D_RECT_F) {
        let radius = ctrl.corner_radius.value;
        let alpha = ctrl.content_alpha.value.clamp(0.0, 1.0);
        let flare = if pill_rect.bottom - pill_rect.top > 60.0 {
            20.0
        } else {
            13.0
        };

        // 1. BASE NOTCH SOLID BLACK (#000000) TANPA BORDER MENGGANGGU
        let pure_black = D2D1_COLOR_F {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        };
        if let Ok(bezel_geom) = self.create_top_bezel_notch_geometry(pill_rect, radius, flare) {
            if let Ok(base_brush) = self.rt.CreateSolidColorBrush(&pure_black, None) {
                self.rt.FillGeometry(&bezel_geom, &base_brush, None);
            }
        } else if let Ok(base_brush) = self.rt.CreateSolidColorBrush(&pure_black, None) {
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: pill_rect,
                    radiusX: radius,
                    radiusY: radius,
                },
                &base_brush,
            );
        }

        // 2. SAAT TERTUTUP + ADA SYSTEM OVERLAY
        if !ctrl.is_hovered {
            if let Some(ref ov) = ctrl.active_overlay {
                self.draw_system_overlay(ov, pill_rect, alpha);
                return;
            }
        }

        // 3. KONTEN SESUAI STATE
        if ctrl.height.value < 55.0 {
            if ctrl.state == NotchState::Idle {
                self.draw_closed_clock_pill(ctrl, pill_rect, alpha);
            } else {
                // ANIMASI AURA BACKGROUND DINAMIS SAAT IDLE (Merah-Hitam Petir / Hijau Healing -> Kembali Hitam)
                ctrl.devil_eyes.draw_idle_aura_background(
                    &self.rt,
                    &self.factory,
                    pill_rect,
                    alpha,
                );
                let cx = (pill_rect.left + pill_rect.right) * 0.5;
                let cy = (pill_rect.top + pill_rect.bottom) * 0.5;
                ctrl.devil_eyes
                    .draw(&self.rt, &self.factory, cx, cy, alpha);
            }
        } else {
            let content_rect = self.draw_clean_nusa_shell(ctrl, pill_rect, alpha);
            match ctrl.state {
                NotchState::Hub => self.draw_hub_panel(ctrl, content_rect, alpha),
                NotchState::Music => self.draw_music_panel(ctrl, content_rect, alpha),
                NotchState::Control => self.draw_control_panel(ctrl, content_rect, alpha),
                NotchState::Pomo => self.draw_pomo_panel(ctrl, content_rect, alpha),
                NotchState::Shelf => self.draw_shelf_panel(ctrl, content_rect, alpha),
                _ => {}
            }
        }

        // 4. MASKOT CHIBI LIEBE
        ctrl.chibi_liebe
            .draw(&self.rt, &self.factory, pill_rect, ctrl.state);
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

    /// Shell Expanded Notch Bersih Seperti Nusa Original (`Screenshot 2026-10-25 210657.png` & `210820.png`):
    /// - TIDAK ADA nested card box pembungkus konten (langsung di atas background hitam `#000000`)
    /// - Tombol navigasi bulat `<` dan `>` di tepi kiri & kanan tengah (`cy = pill_center_y`)
    /// - Bar navigasi atas yang bersih + baterai bundar hijau & jam di kanan
    unsafe fn draw_clean_nusa_shell(
        &self,
        ctrl: &NotchController,
        pill_rect: D2D_RECT_F,
        alpha: f32,
    ) -> D2D_RECT_F {
        let tabs = ["Hub", "Music", "Control", "Focus", "Shelf", "Stages"];
        let active_idx = match ctrl.state {
            NotchState::Hub => 0,
            NotchState::Music => 1,
            NotchState::Control => 2,
            NotchState::Pomo => 3,
            NotchState::Shelf => 4,
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

        // 1. Tombol Bulat `<` dan `>` di Tepi Kiri & Kanan Tengah (Persis Referensi Nusa Original!)
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

        // 2. Bar Navigasi 6-Slot di Atas (Tanpa Kotak Tebal, Bersih & Minimalis)
        let tabs_left = pill_rect.left + 36.0;
        let tabs_right = pill_rect.right - 36.0;
        let slot_w = (tabs_right - tabs_left) / 6.0;

        for (i, label) in tabs.iter().enumerate() {
            let sl = tabs_left + slot_w * i as f32;
            let sr = sl + slot_w;
            let cx = (sl + sr) * 0.5;
            let is_active = i == active_idx || (i == 5 && ctrl.stagedock_flash_timer > 0.0);

            let text_col = if is_active {
                white
            } else if i == 5 {
                AntiMagicNeutralPalette::ACCENT_CYAN
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

            // Garis bawah tipis biru-putih hanya pada tab yang aktif
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

        // 3. Indikator Titik / Pill Tipis di Bagian Paling Bawah Notch (Persis Nusa Original)
        let dot_y = pill_rect.bottom - 4.5;
        let total_dots = 5;
        let dots_w = 44.0;
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

        // Kembalikan area konten langsung di atas background hitam (tanpa menggambar card container!)
        D2D_RECT_F {
            left: pill_rect.left + 32.0,
            top: pill_rect.top + 26.0,
            right: pill_rect.right - 32.0,
            bottom: pill_rect.bottom - 10.0,
        }
    }

    // =========================================================================
    // SECTION 0: HUB (Langsung di Atas Background Hitam, Tanpa Nested Cards!)
    // =========================================================================
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

        // Kolom Kiri (x: area.left .. area.left + 138.0): Jam Besar, Tanggal, & Ring Baterai Hijau
        self.draw_text_left(
            &time_big,
            &self.fmt_clock_hero,
            D2D_RECT_F {
                left: area.left + 4.0,
                top: area.top + 8.0,
                right: area.left + 138.0,
                bottom: area.top + 40.0,
            },
            white,
        );

        self.draw_text_left(
            &date_str,
            &self.fmt_caption,
            D2D_RECT_F {
                left: area.left + 4.0,
                top: area.top + 42.0,
                right: area.left + 138.0,
                bottom: area.top + 60.0,
            },
            muted,
        );

        self.draw_nusa_ring_battery(
            area.left + 12.0,
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
                left: area.left + 24.0,
                top: area.bottom - 27.0,
                right: area.left + 138.0,
                bottom: area.bottom - 9.0,
            },
            white,
        );

        // Garis pemisah vertikal tipis berwarna biru-abu gelap
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
                    x: area.left + 142.0,
                    y: area.top + 10.0,
                },
                D2D_POINT_2F {
                    x: area.left + 142.0,
                    y: area.bottom - 10.0,
                },
                &div_b,
                1.0,
                None,
            );
        }

        // Kolom Kanan: Word / Quote of the Day langsung di atas background hitam
        let q_left = area.left + 154.0;
        let daily = ctrl.current_quote();
        let header = format!("✨ {}", daily.word_of_the_day);
        self.draw_text_left(
            &header,
            &self.fmt_caption,
            D2D_RECT_F {
                left: q_left,
                top: area.top + 8.0,
                right: area.right - 4.0,
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
                right: area.right - 4.0,
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
                right: area.right - 4.0,
                bottom: area.bottom - 6.0,
            },
            muted,
        );
    }

    // =========================================================================
    // SECTION 1: MUSIC (Persis Referensi `Screenshot 2026-10-25 210657.png` + Side Lyrics!)
    // =========================================================================
    unsafe fn draw_music_panel(&self, ctrl: &NotchController, area: D2D_RECT_F, alpha: f32) {
        let white = D2D1_COLOR_F {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: alpha,
        };
        let muted = D2D1_COLOR_F {
            r: 0.60,
            g: 0.64,
            b: 0.70,
            a: alpha,
        };

        let total_w = area.right - area.left;
        let player_right = if ctrl.show_side_lyrics {
            area.left + total_w * 0.57
        } else {
            area.right - 4.0
        };

        // 1. Album Art Rounded Square di Kiri Atas (`48x48`)
        let art_rect = D2D_RECT_F {
            left: area.left + 4.0,
            top: area.top + 4.0,
            right: area.left + 52.0,
            bottom: area.top + 52.0,
        };
        if let Ok(art_bg) = self.rt.CreateSolidColorBrush(
            &D2D1_COLOR_F {
                r: 0.14,
                g: 0.22,
                b: 0.34,
                a: alpha,
            },
            None,
        ) {
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: art_rect,
                    radiusX: 10.0,
                    radiusY: 10.0,
                },
                &art_bg,
            );
        }
        self.draw_text_centered("🎵", &self.fmt_title, art_rect, white);

        // 2. Judul Lagu (Putih Tebal) & Nama Artis (Abu-abu) di Sebelah Kanan Cover
        self.draw_text_left(
            &ctrl.media_title,
            &self.fmt_title,
            D2D_RECT_F {
                left: art_rect.right + 10.0,
                top: area.top + 6.0,
                right: player_right,
                bottom: area.top + 28.0,
            },
            white,
        );
        self.draw_text_left(
            &ctrl.media_artist,
            &self.fmt_caption,
            D2D_RECT_F {
                left: art_rect.right + 10.0,
                top: area.top + 27.0,
                right: player_right,
                bottom: area.top + 46.0,
            },
            muted,
        );

        // 3. Baris Timeline (`00:03  ●──────────  -04:37`) Langsung di Atas Hitam
        let elapsed_s = ctrl.media_elapsed_exact.max(0.0) as u32;
        let total_s = ctrl.media_total_secs.max(1.0) as u32;
        let rem_s = total_s.saturating_sub(elapsed_s);

        let left_time = format!("{:02}:{:02}", elapsed_s / 60, elapsed_s % 60);
        let right_time = format!("-{:02}:{:02}", rem_s / 60, rem_s % 60);

        let bar_y = area.top + 64.0;
        self.draw_text_left(
            &left_time,
            &self.fmt_caption,
            D2D_RECT_F {
                left: area.left + 4.0,
                top: bar_y - 9.0,
                right: area.left + 40.0,
                bottom: bar_y + 9.0,
            },
            white,
        );
        self.draw_text_right(
            &right_time,
            &self.fmt_caption,
            D2D_RECT_F {
                left: player_right - 44.0,
                top: bar_y - 9.0,
                right: player_right,
                bottom: bar_y + 9.0,
            },
            white,
        );

        let track_l = area.left + 42.0;
        let track_r = (player_right - 46.0).max(track_l + 30.0);
        if let Ok(tb) = self.rt.CreateSolidColorBrush(
            &D2D1_COLOR_F {
                r: 0.28,
                g: 0.30,
                b: 0.34,
                a: alpha,
            },
            None,
        ) {
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: D2D_RECT_F {
                        left: track_l,
                        top: bar_y - 2.0,
                        right: track_r,
                        bottom: bar_y + 2.0,
                    },
                    radiusX: 2.0,
                    radiusY: 2.0,
                },
                &tb,
            );
        }

        let dot_x = track_l + (track_r - track_l) * ctrl.media_progress.clamp(0.0, 1.0);
        if let Ok(fb) = self
            .rt
            .CreateSolidColorBrush(&AntiMagicNeutralPalette::ACCENT_BLUE, None)
        {
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: D2D_RECT_F {
                        left: track_l,
                        top: bar_y - 2.0,
                        right: dot_x.max(track_l + 2.0),
                        bottom: bar_y + 2.0,
                    },
                    radiusX: 2.0,
                    radiusY: 2.0,
                },
                &fb,
            );
        }
        if let Ok(wb) = self.rt.CreateSolidColorBrush(&white, None) {
            self.rt.FillEllipse(
                &D2D1_ELLIPSE {
                    point: D2D_POINT_2F { x: dot_x, y: bar_y },
                    radiusX: 3.5,
                    radiusY: 3.5,
                },
                &wb,
            );
        }

        // 4. Baris Tombol Bawah: `[💬]` di kiri, `[⏮]  [▶/⏸]  [⏭]` di tengah
        let ctrl_y = area.top + 88.0;
        let mid_x = (area.left + player_right) * 0.52;

        let lyr_col = if ctrl.show_side_lyrics {
            AntiMagicNeutralPalette::ACCENT_CYAN
        } else {
            muted
        };
        self.draw_text_centered(
            "💬",
            &self.fmt_body,
            D2D_RECT_F {
                left: mid_x - 76.0,
                top: ctrl_y - 10.0,
                right: mid_x - 48.0,
                bottom: ctrl_y + 12.0,
            },
            lyr_col,
        );
        self.draw_text_centered(
            "⏮",
            &self.fmt_title,
            D2D_RECT_F {
                left: mid_x - 44.0,
                top: ctrl_y - 10.0,
                right: mid_x - 16.0,
                bottom: ctrl_y + 12.0,
            },
            white,
        );
        self.draw_text_centered(
            if ctrl.media_playing { "⏸" } else { "▶" },
            &self.fmt_title,
            D2D_RECT_F {
                left: mid_x - 14.0,
                top: ctrl_y - 10.0,
                right: mid_x + 14.0,
                bottom: ctrl_y + 12.0,
            },
            white,
        );
        self.draw_text_centered(
            "⏭",
            &self.fmt_title,
            D2D_RECT_F {
                left: mid_x + 16.0,
                top: ctrl_y - 10.0,
                right: mid_x + 44.0,
                bottom: ctrl_y + 12.0,
            },
            white,
        );

        // 5. Panel Lirik Live di Sebelah Kanan (Langsung di Atas Hitam Tanpa Kotak Penuh)
        if ctrl.show_side_lyrics {
            let lyr_left = player_right + 10.0;
            if let Ok(div_b) = self.rt.CreateSolidColorBrush(
                &D2D1_COLOR_F {
                    r: 0.18,
                    g: 0.24,
                    b: 0.36,
                    a: 0.6 * alpha,
                },
                None,
            ) {
                self.rt.DrawLine(
                    D2D_POINT_2F {
                        x: player_right + 4.0,
                        y: area.top + 6.0,
                    },
                    D2D_POINT_2F {
                        x: player_right + 4.0,
                        y: area.bottom - 8.0,
                    },
                    &div_b,
                    1.0,
                    None,
                );
            }

            let total_lines = ctrl.timed_lyrics.len().max(1);
            let cur_idx = ctrl.active_lyric_idx % total_lines;
            let prev_line = if cur_idx > 0 {
                ctrl.timed_lyrics
                    .get(cur_idx - 1)
                    .map(|(_, s)| s.as_str())
                    .unwrap_or("")
            } else {
                "♪ · · ·"
            };
            let cur_line = ctrl
                .timed_lyrics
                .get(cur_idx)
                .map(|(_, s)| s.as_str())
                .unwrap_or("♪ ...");
            let next_line = ctrl
                .timed_lyrics
                .get((cur_idx + 1) % total_lines)
                .map(|(_, s)| s.as_str())
                .unwrap_or("");

            self.draw_text_left(
                prev_line,
                &self.fmt_caption,
                D2D_RECT_F {
                    left: lyr_left,
                    top: area.top + 6.0,
                    right: area.right,
                    bottom: area.top + 26.0,
                },
                muted,
            );
            self.draw_text_left(
                cur_line,
                &self.fmt_body,
                D2D_RECT_F {
                    left: lyr_left,
                    top: area.top + 28.0,
                    right: area.right,
                    bottom: area.top + 72.0,
                },
                white,
            );
            self.draw_text_left(
                next_line,
                &self.fmt_caption,
                D2D_RECT_F {
                    left: lyr_left,
                    top: area.top + 74.0,
                    right: area.right,
                    bottom: area.top + 96.0,
                },
                muted,
            );
        }
    }

    // =========================================================================
    // SECTION 2: CONTROL (Langsung di Atas Background Hitam Tanpa Kotak Kartu!)
    // =========================================================================
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

            // 1. Kolom Label Kiri
            self.draw_text_left(
                label,
                &self.fmt_body,
                D2D_RECT_F {
                    left: area.left + 4.0,
                    top: ry,
                    right: area.left + 100.0,
                    bottom: ry + row_h,
                },
                white,
            );

            // 2. Track Slider Tipis di Tengah
            let track_l = area.left + 104.0;
            let track_r = (area.right - 52.0).max(track_l + 20.0);

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

            // 3. Angka Persentase di Kanan
            self.draw_text_right(
                &format!("{}%", val),
                &self.fmt_body,
                D2D_RECT_F {
                    left: area.right - 48.0,
                    top: ry,
                    right: area.right - 4.0,
                    bottom: ry + row_h,
                },
                white,
            );
        }
    }

    // =========================================================================
    // SECTION 3: POMODORO TIMER (Dengan Opsi Tombol `Focus 25m` & `Break 5m`!)
    // =========================================================================
    unsafe fn draw_pomo_panel(&self, ctrl: &NotchController, area: D2D_RECT_F, alpha: f32) {
        let white = D2D1_COLOR_F {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: alpha,
        };
        let muted = D2D1_COLOR_F {
            r: 0.60,
            g: 0.64,
            b: 0.72,
            a: alpha,
        };

        let mins = ctrl.pomo_remaining_secs / 60;
        let secs = ctrl.pomo_remaining_secs % 60;
        let time_str = format!("{:02}:{:02}", mins, secs);

        // Kiri: Angka Countdown Besar & Label Status
        self.draw_text_left(
            &time_str,
            &self.fmt_clock_hero,
            D2D_RECT_F {
                left: area.left + 6.0,
                top: area.top + 14.0,
                right: area.left + 112.0,
                bottom: area.top + 54.0,
            },
            white,
        );
        self.draw_text_left(
            if ctrl.pomo_is_break {
                "☕ Break Mode"
            } else {
                "🎯 Focus Mode"
            },
            &self.fmt_caption,
            D2D_RECT_F {
                left: area.left + 6.0,
                top: area.top + 56.0,
                right: area.left + 112.0,
                bottom: area.top + 78.0,
            },
            AntiMagicNeutralPalette::ACCENT_CYAN,
        );

        // Kanan Baris 1 (top + 12..42): Opsi Pemilih Mode [Focus 25m] dan [Break 5m]
        let focus_mode_rect = D2D_RECT_F {
            left: area.left + 118.0,
            top: area.top + 12.0,
            right: area.left + 214.0,
            bottom: area.top + 42.0,
        };
        let break_mode_rect = D2D_RECT_F {
            left: area.left + 222.0,
            top: area.top + 12.0,
            right: area.left + 316.0,
            bottom: area.top + 42.0,
        };

        let active_pill_col = D2D1_COLOR_F {
            r: 0.23,
            g: 0.51,
            b: 0.96,
            a: 0.90 * alpha,
        };
        let inactive_pill_col = D2D1_COLOR_F {
            r: 0.14,
            g: 0.17,
            b: 0.24,
            a: 0.90 * alpha,
        };

        if let Ok(fb) = self.rt.CreateSolidColorBrush(
            if !ctrl.pomo_is_break {
                &active_pill_col
            } else {
                &inactive_pill_col
            },
            None,
        ) {
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: focus_mode_rect,
                    radiusX: 8.0,
                    radiusY: 8.0,
                },
                &fb,
            );
        }
        if let Ok(bb) = self.rt.CreateSolidColorBrush(
            if ctrl.pomo_is_break {
                &active_pill_col
            } else {
                &inactive_pill_col
            },
            None,
        ) {
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: break_mode_rect,
                    radiusX: 8.0,
                    radiusY: 8.0,
                },
                &bb,
            );
        }

        self.draw_text_centered(
            "Focus 25m",
            &self.fmt_caption,
            focus_mode_rect,
            if !ctrl.pomo_is_break { white } else { muted },
        );
        self.draw_text_centered(
            "Break 5m",
            &self.fmt_caption,
            break_mode_rect,
            if ctrl.pomo_is_break { white } else { muted },
        );

        // Kanan Baris 2 (top + 54..86): Tombol [▶ Start / ⏸ Pause] dan [↺ Reset]
        let start_rect = D2D_RECT_F {
            left: area.left + 118.0,
            top: area.top + 54.0,
            right: area.left + 214.0,
            bottom: area.top + 86.0,
        };
        let reset_rect = D2D_RECT_F {
            left: area.left + 222.0,
            top: area.top + 54.0,
            right: area.left + 300.0,
            bottom: area.top + 86.0,
        };

        if let Ok(sb) = self.rt.CreateSolidColorBrush(
            &D2D1_COLOR_F {
                r: 0.18,
                g: 0.24,
                b: 0.36,
                a: alpha,
            },
            None,
        ) {
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: start_rect,
                    radiusX: 8.0,
                    radiusY: 8.0,
                },
                &sb,
            );
        }
        if let Ok(rb) = self.rt.CreateSolidColorBrush(&inactive_pill_col, None) {
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: reset_rect,
                    radiusX: 8.0,
                    radiusY: 8.0,
                },
                &rb,
            );
        }

        self.draw_text_centered(
            if ctrl.pomo_running {
                "⏸ Pause"
            } else {
                "▶ Start"
            },
            &self.fmt_body,
            start_rect,
            white,
        );
        self.draw_text_centered("↺ Reset", &self.fmt_caption, reset_rect, muted);
    }

    // =========================================================================
    // SECTION 4: SHELF (Persis Referensi `Screenshot 2026-10-25 210820.png`: Dashed Border!)
    // =========================================================================
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

        // Garis tepi putus-putus (Dashed Rounded Rect) langsung di atas background hitam!
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
}