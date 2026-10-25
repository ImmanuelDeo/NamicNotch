// src/notch/renderer.rs
use chrono::Local;
use windows::core::w;
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Direct2D::Common::{
    D2D1_ALPHA_MODE_PREMULTIPLIED, D2D1_BEZIER_SEGMENT, D2D1_COLOR_F, D2D1_FIGURE_BEGIN_FILLED,
    D2D1_FIGURE_END_CLOSED, D2D1_GRADIENT_STOP, D2D1_PIXEL_FORMAT, D2D_POINT_2F, D2D_RECT_F,
    D2D_SIZE_U,
};
use windows::Win32::Graphics::Direct2D::*;
use windows::Win32::Graphics::DirectWrite::{
    DWriteCreateFactory, IDWriteFactory, IDWriteTextFormat, DWRITE_FACTORY_TYPE_SHARED,
    DWRITE_FONT_STRETCH_NORMAL, DWRITE_FONT_STYLE_ITALIC, DWRITE_FONT_STYLE_NORMAL,
    DWRITE_FONT_WEIGHT_BOLD, DWRITE_FONT_WEIGHT_MEDIUM, DWRITE_FONT_WEIGHT_SEMI_BOLD,
    DWRITE_PARAGRAPH_ALIGNMENT_CENTER, DWRITE_TEXT_ALIGNMENT_CENTER, DWRITE_TEXT_ALIGNMENT_LEADING,
};
use windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_B8G8R8A8_UNORM;
use windows::Win32::Graphics::Gdi::{BeginPaint, EndPaint, PAINTSTRUCT};

use super::states::{NotchController, NotchState, OverlayKind};

pub struct AntiMagicNeutralPalette;

impl AntiMagicNeutralPalette {
    /// SOLID BLACK (#000000) untuk Base Notch baik saat Mini maupun Full!
    pub const PILL_SOLID_BLACK: D2D1_COLOR_F = D2D1_COLOR_F {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const CARD_ELEVATED: D2D1_COLOR_F = D2D1_COLOR_F {
        r: 0.115,
        g: 0.148,
        b: 0.212,
        a: 0.92,
    };
    pub const ACCENT_BLUE: D2D1_COLOR_F = D2D1_COLOR_F {
        r: 0.392,
        g: 0.545,
        b: 0.945,
        a: 1.0,
    };
    pub const ACCENT_CYAN: D2D1_COLOR_F = D2D1_COLOR_F {
        r: 0.220,
        g: 0.740,
        b: 0.970,
        a: 1.0,
    };
    pub const ACCENT_WINE: D2D1_COLOR_F = D2D1_COLOR_F {
        r: 0.680,
        g: 0.220,
        b: 0.310,
        a: 0.95,
    };
    pub const TEXT_PRIMARY: D2D1_COLOR_F = D2D1_COLOR_F {
        r: 0.945,
        g: 0.965,
        b: 0.992,
        a: 1.0,
    };
    pub const TEXT_SECONDARY: D2D1_COLOR_F = D2D1_COLOR_F {
        r: 0.600,
        g: 0.675,
        b: 0.785,
        a: 1.0,
    };
}

pub struct NotchRenderer {
    pub factory: ID2D1Factory,
    pub dwrite: IDWriteFactory,
    pub rt: ID2D1HwndRenderTarget,
    pub fmt_clock_hero: IDWriteTextFormat,
    pub fmt_title: IDWriteTextFormat,
    pub fmt_body: IDWriteTextFormat,
    pub fmt_quote_italic: IDWriteTextFormat,
    pub fmt_caption: IDWriteTextFormat,
    pub fmt_battery_inside: IDWriteTextFormat,
}

impl NotchRenderer {
    pub fn new(hwnd: HWND, width: u32, height: u32, dpi_scale: f32) -> windows::core::Result<Self> {
        unsafe {
            let factory: ID2D1Factory =
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

            let fmt_clock_hero = dwrite.CreateTextFormat(
                w!("Segoe UI Variable Display"),
                None,
                DWRITE_FONT_WEIGHT_BOLD,
                DWRITE_FONT_STYLE_NORMAL,
                DWRITE_FONT_STRETCH_NORMAL,
                26.0,
                w!("en-us"),
            )?;

            let fmt_title = dwrite.CreateTextFormat(
                w!("Segoe UI Variable Display"),
                None,
                DWRITE_FONT_WEIGHT_BOLD,
                DWRITE_FONT_STYLE_NORMAL,
                DWRITE_FONT_STRETCH_NORMAL,
                12.0,
                w!("en-us"),
            )?;

            let fmt_body = dwrite.CreateTextFormat(
                w!("Segoe UI Variable Display"),
                None,
                DWRITE_FONT_WEIGHT_MEDIUM,
                DWRITE_FONT_STYLE_NORMAL,
                DWRITE_FONT_STRETCH_NORMAL,
                11.0,
                w!("en-us"),
            )?;

            let fmt_quote_italic = dwrite.CreateTextFormat(
                w!("Segoe UI Variable Display"),
                None,
                DWRITE_FONT_WEIGHT_MEDIUM,
                DWRITE_FONT_STYLE_ITALIC,
                DWRITE_FONT_STRETCH_NORMAL,
                11.0,
                w!("en-us"),
            )?;

            let fmt_caption = dwrite.CreateTextFormat(
                w!("Segoe UI Variable Display"),
                None,
                DWRITE_FONT_WEIGHT_SEMI_BOLD,
                DWRITE_FONT_STYLE_NORMAL,
                DWRITE_FONT_STRETCH_NORMAL,
                9.5,
                w!("en-us"),
            )?;

            let fmt_battery_inside = dwrite.CreateTextFormat(
                w!("Segoe UI Variable Display"),
                None,
                DWRITE_FONT_WEIGHT_BOLD,
                DWRITE_FONT_STYLE_NORMAL,
                DWRITE_FONT_STRETCH_NORMAL,
                8.5,
                w!("en-us"),
            )?;

            Ok(Self {
                factory,
                dwrite,
                rt,
                fmt_clock_hero,
                fmt_title,
                fmt_body,
                fmt_quote_italic,
                fmt_caption,
                fmt_battery_inside,
            })
        }
    }

    unsafe fn create_bezel_notch_geometry(
        &self,
        rect: D2D_RECT_F,
        bottom_radius: f32,
    ) -> windows::core::Result<ID2D1PathGeometry> {
        let geom = self.factory.CreatePathGeometry()?;
        let sink = geom.Open()?;

        let h = (rect.bottom - rect.top).max(4.0);
        let r = bottom_radius.min(h * 0.5).max(2.0);
        // Lengkungan atas ke bezel diperlebar & dipertegas (ear_w = 20.0, ear_h = 14.0) agar menyatu sangat halus
        let ear_w = 20.0f32;
        let ear_h = (r * 0.85).min(14.0f32);

        sink.BeginFigure(
            D2D_POINT_2F {
                x: rect.left - ear_w,
                y: rect.top,
            },
            D2D1_FIGURE_BEGIN_FILLED,
        );
        sink.AddBezier(&D2D1_BEZIER_SEGMENT {
            point1: D2D_POINT_2F {
                x: rect.left - ear_w * 0.45,
                y: rect.top,
            },
            point2: D2D_POINT_2F {
                x: rect.left,
                y: rect.top + ear_h * 0.35,
            },
            point3: D2D_POINT_2F {
                x: rect.left,
                y: rect.top + ear_h,
            },
        });
        sink.AddLine(D2D_POINT_2F {
            x: rect.left,
            y: rect.bottom - r,
        });
        sink.AddBezier(&D2D1_BEZIER_SEGMENT {
            point1: D2D_POINT_2F {
                x: rect.left,
                y: rect.bottom - r * 0.45,
            },
            point2: D2D_POINT_2F {
                x: rect.left + r * 0.45,
                y: rect.bottom,
            },
            point3: D2D_POINT_2F {
                x: rect.left + r,
                y: rect.bottom,
            },
        });
        sink.AddLine(D2D_POINT_2F {
            x: rect.right - r,
            y: rect.bottom,
        });
        sink.AddBezier(&D2D1_BEZIER_SEGMENT {
            point1: D2D_POINT_2F {
                x: rect.right - r * 0.45,
                y: rect.bottom,
            },
            point2: D2D_POINT_2F {
                x: rect.right,
                y: rect.bottom - r * 0.45,
            },
            point3: D2D_POINT_2F {
                x: rect.right,
                y: rect.bottom - r,
            },
        });
        sink.AddLine(D2D_POINT_2F {
            x: rect.right,
            y: rect.top + ear_h,
        });
        sink.AddBezier(&D2D1_BEZIER_SEGMENT {
            point1: D2D_POINT_2F {
                x: rect.right,
                y: rect.top + ear_h * 0.35,
            },
            point2: D2D_POINT_2F {
                x: rect.right + ear_w * 0.45,
                y: rect.top,
            },
            point3: D2D_POINT_2F {
                x: rect.right + ear_w,
                y: rect.top,
            },
        });

        sink.EndFigure(D2D1_FIGURE_END_CLOSED);
        sink.Close()?;
        Ok(geom)
    }

    unsafe fn fill_gradient_glass_card(
        &self,
        rect: D2D_RECT_F,
        radius: f32,
        c_start: D2D1_COLOR_F,
        c_mid: D2D1_COLOR_F,
        c_end: D2D1_COLOR_F,
        border_col: D2D1_COLOR_F,
        border_width: f32,
    ) {
        let rrect = D2D1_ROUNDED_RECT {
            rect,
            radiusX: radius,
            radiusY: radius,
        };
        let stops = [
            D2D1_GRADIENT_STOP {
                position: 0.0,
                color: c_start,
            },
            D2D1_GRADIENT_STOP {
                position: 0.52,
                color: c_mid,
            },
            D2D1_GRADIENT_STOP {
                position: 1.0,
                color: c_end,
            },
        ];
        if let Ok(sc) =
            self.rt
                .CreateGradientStopCollection(&stops, D2D1_GAMMA_2_2, D2D1_EXTEND_MODE_CLAMP)
        {
            let props = D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES {
                startPoint: D2D_POINT_2F {
                    x: rect.left,
                    y: rect.top,
                },
                endPoint: D2D_POINT_2F {
                    x: rect.right,
                    y: rect.bottom,
                },
            };
            if let Ok(gb) = self.rt.CreateLinearGradientBrush(&props, None, &sc) {
                self.rt.FillRoundedRectangle(&rrect, &gb);
            }
        }
        if border_width > 0.0 {
            if let Ok(bb) = self.rt.CreateSolidColorBrush(&border_col, None) {
                self.rt.DrawRoundedRectangle(&rrect, &bb, border_width, None);
            }
        }
    }

    unsafe fn draw_ios_battery_icon(
        &self,
        x: f32,
        y: f32,
        percent: u8,
        charging: bool,
        alpha: f32,
    ) {
        let bw = 34.0;
        let bh = 16.0;
        let body_rect = D2D_RECT_F {
            left: x,
            top: y,
            right: x + bw,
            bottom: y + bh,
        };

        let outline_col = D2D1_COLOR_F {
            r: 0.65,
            g: 0.72,
            b: 0.84,
            a: 0.75 * alpha,
        };
        if let Ok(ob) = self.rt.CreateSolidColorBrush(&outline_col, None) {
            self.rt.DrawRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: body_rect,
                    radiusX: 5.5,
                    radiusY: 5.5,
                },
                &ob,
                1.3,
                None,
            );
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: D2D_RECT_F {
                        left: x + bw + 1.5,
                        top: y + 4.5,
                        right: x + bw + 3.5,
                        bottom: y + bh - 4.5,
                    },
                    radiusX: 1.2,
                    radiusY: 1.2,
                },
                &ob,
            );
        }

        let fill_ratio = (percent.min(100) as f32 / 100.0).max(0.12);
        let fill_w = (bw - 3.6) * fill_ratio;
        let fill_col = if charging {
            D2D1_COLOR_F {
                r: 0.18,
                g: 0.82,
                b: 0.48,
                a: 0.92 * alpha,
            }
        } else if percent <= 20 {
            D2D1_COLOR_F {
                r: 0.85,
                g: 0.25,
                b: 0.32,
                a: 0.92 * alpha,
            }
        } else {
            D2D1_COLOR_F {
                r: 0.34,
                g: 0.50,
                b: 0.88,
                a: 0.88 * alpha,
            }
        };

        if let Ok(fb) = self.rt.CreateSolidColorBrush(&fill_col, None) {
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: D2D_RECT_F {
                        left: x + 1.8,
                        top: y + 1.8,
                        right: x + 1.8 + fill_w,
                        bottom: y + bh - 1.8,
                    },
                    radiusX: 4.0,
                    radiusY: 4.0,
                },
                &fb,
            );
        }

        let label = if charging {
            format!("⚡{}", percent)
        } else {
            format!("{}", percent)
        };
        let txt_col = D2D1_COLOR_F {
            r: 0.98,
            g: 0.99,
            b: 1.0,
            a: alpha,
        };
        self.draw_text_centered(&label, &self.fmt_battery_inside, body_rect, txt_col);
    }

    unsafe fn draw_text_left(
        &self,
        text: &str,
        fmt: &IDWriteTextFormat,
        rect: D2D_RECT_F,
        color: D2D1_COLOR_F,
    ) {
        let _ = fmt.SetTextAlignment(DWRITE_TEXT_ALIGNMENT_LEADING);
        let _ = fmt.SetParagraphAlignment(DWRITE_PARAGRAPH_ALIGNMENT_CENTER);
        let wide: Vec<u16> = text.encode_utf16().collect();
        if let Ok(brush) = self.rt.CreateSolidColorBrush(&color, None) {
            self.rt.DrawText(
                &wide,
                fmt,
                &rect,
                &brush,
                D2D1_DRAW_TEXT_OPTIONS_CLIP,
                Default::default(),
            );
        }
    }

    unsafe fn draw_text_centered(
        &self,
        text: &str,
        fmt: &IDWriteTextFormat,
        rect: D2D_RECT_F,
        color: D2D1_COLOR_F,
    ) {
        let _ = fmt.SetTextAlignment(DWRITE_TEXT_ALIGNMENT_CENTER);
        let _ = fmt.SetParagraphAlignment(DWRITE_PARAGRAPH_ALIGNMENT_CENTER);
        let wide: Vec<u16> = text.encode_utf16().collect();
        if let Ok(brush) = self.rt.CreateSolidColorBrush(&color, None) {
            self.rt.DrawText(
                &wide,
                fmt,
                &rect,
                &brush,
                D2D1_DRAW_TEXT_OPTIONS_CLIP,
                Default::default(),
            );
        }
    }

    pub fn render_frame(&self, controller: &mut NotchController, pill_rect: D2D_RECT_F) {
        unsafe {
            let mut ps = PAINTSTRUCT::default();
            let hwnd = self.rt.GetHwnd();
            let _ = BeginPaint(hwnd, &mut ps);

            self.rt.BeginDraw();
            self.rt.Clear(Some(&D2D1_COLOR_F {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            }));

            // 1. Gambar Base Notch dengan Warna PURE SOLID BLACK (#000000) Baik Saat Mini Maupun Full!
            if let Ok(notch_geom) =
                self.create_bezel_notch_geometry(pill_rect, controller.corner_radius.value)
            {
                            if let Ok(black_brush) = self
                                .rt
                                .CreateSolidColorBrush(&AntiMagicNeutralPalette::PILL_SOLID_BLACK, None)
                            {
                                self.rt.FillGeometry(&notch_geom, &black_brush, None);
                            }

                            // PENTING: HAPUS BORDER LINE agar notch 100% borderless, clean look,
                            // menyatu sempurna dengan bezel hitam pekat laptop!
                            // (Kode DrawGeometry untuk border dihapus total)
                        }

            // 2. Pop-Up Interactive Overlay (Volume, Brightness, CapsLock, Charging)
            if !controller.is_hovered {
                if let Some(ref overlay) = controller.active_overlay.clone() {
                    self.draw_system_overlay(overlay, pill_rect);
                    let _ = self.rt.EndDraw(None, None);
                    let _ = EndPaint(hwnd, &ps);
                    return;
                }
            }

            // 3. Render State Aktif
            if controller.state == NotchState::Pet && !controller.minimal_mode {
                let collapse_factor = ((62.0 - controller.height.value) / 24.0).clamp(0.0, 1.0);
                controller
                    .devil_eyes
                    .render(&self.factory, &self.rt, pill_rect, collapse_factor);
            } else if controller.state == NotchState::Idle && !controller.minimal_mode {
                let alpha = ((62.0 - controller.height.value) / 24.0).clamp(0.0, 1.0);
                self.draw_closed_idle_clock_and_battery(controller, pill_rect, alpha);
            } else if controller.height.value > 65.0 {
                let alpha = controller.content_alpha.value.clamp(0.0, 1.0);
                let card_rect = self.draw_expanded_panel_shell(controller.state, pill_rect, alpha);

                match controller.state {
                                    NotchState::Hub => self.draw_hub_panel(controller, card_rect, alpha),
                                    NotchState::Music => self.draw_music_panel(controller, card_rect, alpha),
                                    NotchState::Control => self.draw_control_panel(controller, card_rect, alpha),
                                    NotchState::Pomo => self.draw_pomo_panel(controller, card_rect, alpha),
                                    NotchState::Shelf => self.draw_shelf_panel(controller, card_rect, alpha),
                                    NotchState::StageDock => self.draw_stagedock_panel(controller, card_rect, alpha),
                                    _ => {}
                                }

                if controller.mascot_enabled {
                    controller
                        .chibi_liebe
                        .render(&self.factory, &self.rt, pill_rect, alpha);
                }
            }

            let _ = self.rt.EndDraw(None, None);
            let _ = EndPaint(hwnd, &ps);
        }
    }

    unsafe fn draw_closed_idle_clock_and_battery(
        &self,
        ctrl: &NotchController,
        pill: D2D_RECT_F,
        alpha: f32,
    ) {
        let now_str = Local::now().format("%H:%M").to_string();
        let mut p_col = AntiMagicNeutralPalette::TEXT_PRIMARY;
        p_col.a *= alpha;

        self.draw_text_left(
            &now_str,
            &self.fmt_title,
            D2D_RECT_F {
                left: pill.left + 16.0,
                top: pill.top,
                right: pill.left + 85.0,
                bottom: pill.bottom,
            },
            p_col,
        );

        let bat_y = (pill.top + pill.bottom - 16.0) * 0.5;
        self.draw_ios_battery_icon(
            pill.right - 52.0,
            bat_y,
            ctrl.battery_percent,
            ctrl.is_charging,
            alpha,
        );
    }

    unsafe fn draw_system_overlay(&self, overlay: &OverlayKind, pill: D2D_RECT_F) {
        let p_col = AntiMagicNeutralPalette::TEXT_PRIMARY;
        let accent = AntiMagicNeutralPalette::ACCENT_BLUE;
        let cyan = AntiMagicNeutralPalette::ACCENT_CYAN;

        match overlay {
            OverlayKind::Volume(val) | OverlayKind::Brightness(val) => {
                let is_vol = matches!(overlay, OverlayKind::Volume(_));
                let label = if is_vol { "🔊 VOL" } else { "🔆 BRI" };
                self.draw_text_left(
                    label,
                    &self.fmt_caption,
                    D2D_RECT_F {
                        left: pill.left + 14.0,
                        top: pill.top,
                        right: pill.left + 58.0,
                        bottom: pill.bottom,
                    },
                    p_col,
                );

                let bar_l = pill.left + 58.0;
                let bar_r = pill.right - 48.0;
                let bar_cy = (pill.top + pill.bottom) * 0.5;

                if let Ok(tb) = self.rt.CreateSolidColorBrush(
                    &D2D1_COLOR_F {
                        r: 0.16,
                        g: 0.20,
                        b: 0.28,
                        a: 0.85,
                    },
                    None,
                ) {
                    self.rt.FillRoundedRectangle(
                        &D2D1_ROUNDED_RECT {
                            rect: D2D_RECT_F {
                                left: bar_l,
                                top: bar_cy - 5.0,
                                right: bar_r,
                                bottom: bar_cy + 5.0,
                            },
                            radiusX: 5.0,
                            radiusY: 5.0,
                        },
                        &tb,
                    );
                }

                let fill_w = (bar_r - bar_l) * ((*val as f32 / 100.0).clamp(0.0, 1.0));
                self.fill_gradient_glass_card(
                    D2D_RECT_F {
                        left: bar_l,
                        top: bar_cy - 5.0,
                        right: bar_l + fill_w.max(8.0),
                        bottom: bar_cy + 5.0,
                    },
                    5.0,
                    accent,
                    cyan,
                    cyan,
                    cyan,
                    0.0,
                );

                self.draw_text_centered(
                    &format!("{}%", val),
                    &self.fmt_caption,
                    D2D_RECT_F {
                        left: pill.right - 46.0,
                        top: pill.top,
                        right: pill.right - 10.0,
                        bottom: pill.bottom,
                    },
                    p_col,
                );
            }
            OverlayKind::CapsLock(on) => {
                let txt = if *on {
                    "CAPS LOCK · ON"
                } else {
                    "CAPS LOCK · OFF"
                };
                self.draw_text_centered(
                    txt,
                    &self.fmt_title,
                    pill,
                    if *on { cyan } else { p_col },
                );
            }
            OverlayKind::Charging { percent, plugged } => {
                self.draw_text_left(
                    if *plugged {
                        "⚡ Power Connected"
                    } else {
                        "On Battery Power"
                    },
                    &self.fmt_title,
                    D2D_RECT_F {
                        left: pill.left + 18.0,
                        top: pill.top,
                        right: pill.right - 68.0,
                        bottom: pill.bottom,
                    },
                    p_col,
                );
                let bat_y = (pill.top + pill.bottom - 16.0) * 0.5;
                self.draw_ios_battery_icon(pill.right - 56.0, bat_y, *percent, *plugged, 1.0);
            }
        }
    }

    /// Bar Navigasi Atas dengan 6 Section (`Hub`, `Music`, `Control`, `Focus`, `Shelf`, `StageDock`) + Panah `<` dan `>`
    unsafe fn draw_expanded_panel_shell(
        &self,
        state: NotchState,
        pill_rect: D2D_RECT_F,
        alpha: f32,
    ) -> D2D_RECT_F {
        let tabs = ["Hub", "Music", "Control", "Focus", "Shelf", "Stages"];
        let active_idx = match state {
            NotchState::Hub => 0,
            NotchState::Music => 1,
            NotchState::Control => 2,
            NotchState::Pomo => 3,
            NotchState::Shelf => 4,
            NotchState::StageDock => 5,
            _ => 1,
        };

        let mut p_col = AntiMagicNeutralPalette::TEXT_PRIMARY;
        p_col.a *= alpha;
        let mut s_col = AntiMagicNeutralPalette::TEXT_SECONDARY;
        s_col.a *= alpha * 0.78;

        let left_arrow_rect = D2D_RECT_F {
            left: pill_rect.left + 12.0,
            top: pill_rect.top + 4.0,
            right: pill_rect.left + 34.0,
            bottom: pill_rect.top + 24.0,
        };
        let right_arrow_rect = D2D_RECT_F {
            left: pill_rect.right - 34.0,
            top: pill_rect.top + 4.0,
            right: pill_rect.right - 12.0,
            bottom: pill_rect.top + 24.0,
        };

        if let Ok(ab) = self.rt.CreateSolidColorBrush(
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
                    rect: left_arrow_rect,
                    radiusX: 6.0,
                    radiusY: 6.0,
                },
                &ab,
            );
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: right_arrow_rect,
                    radiusX: 6.0,
                    radiusY: 6.0,
                },
                &ab,
            );
        }
        self.draw_text_centered("‹", &self.fmt_title, left_arrow_rect, p_col);
        self.draw_text_centered("›", &self.fmt_title, right_arrow_rect, p_col);

        let tabs_left = pill_rect.left + 38.0;
        let tabs_right = pill_rect.right - 38.0;
        let slot_w = (tabs_right - tabs_left) / 6.0;

        for (i, label) in tabs.iter().enumerate() {
            let sl = tabs_left + slot_w * i as f32;
            let sr = sl + slot_w;
            let cx = (sl + sr) * 0.5;
            let is_active = i == active_idx;

            if is_active {
                self.fill_gradient_glass_card(
                    D2D_RECT_F {
                        left: sl + 2.0,
                        top: pill_rect.top + 4.0,
                        right: sr - 2.0,
                        bottom: pill_rect.top + 24.0,
                    },
                    7.0,
                    D2D1_COLOR_F {
                        r: 0.18,
                        g: 0.26,
                        b: 0.42,
                        a: 0.90 * alpha,
                    },
                    D2D1_COLOR_F {
                        r: 0.12,
                        g: 0.18,
                        b: 0.32,
                        a: 0.90 * alpha,
                    },
                    D2D1_COLOR_F {
                        r: 0.16,
                        g: 0.22,
                        b: 0.38,
                        a: 0.90 * alpha,
                    },
                    D2D1_COLOR_F {
                        r: 0.39,
                        g: 0.55,
                        b: 0.95,
                        a: 0.70 * alpha,
                    },
                    1.0,
                );
            }

            self.draw_text_centered(
                label,
                &self.fmt_caption,
                D2D_RECT_F {
                    left: sl,
                    top: pill_rect.top + 4.0,
                    right: sr,
                    bottom: pill_rect.top + 24.0,
                },
                if is_active { p_col } else { s_col },
            );

            if is_active {
                if let Ok(lb) = self
                    .rt
                    .CreateSolidColorBrush(&AntiMagicNeutralPalette::ACCENT_CYAN, None)
                {
                    self.rt.DrawLine(
                        D2D_POINT_2F {
                            x: cx - 12.0,
                            y: pill_rect.top + 25.0,
                        },
                        D2D_POINT_2F {
                            x: cx + 12.0,
                            y: pill_rect.top + 25.0,
                        },
                        &lb,
                        2.0,
                        None,
                    );
                }
            }
        }

        let card_rect = D2D_RECT_F {
            left: pill_rect.left + 14.0,
            top: pill_rect.top + 30.0,
            right: pill_rect.right - 14.0,
            bottom: pill_rect.bottom - 12.0,
        };

        self.fill_gradient_glass_card(
            card_rect,
            16.0,
            D2D1_COLOR_F {
                r: 0.065,
                g: 0.085,
                b: 0.130,
                a: 0.95 * alpha,
            },
            D2D1_COLOR_F {
                r: 0.082,
                g: 0.108,
                b: 0.165,
                a: 0.95 * alpha,
            },
            D2D1_COLOR_F {
                r: 0.058,
                g: 0.076,
                b: 0.120,
                a: 0.95 * alpha,
            },
            D2D1_COLOR_F {
                r: 0.24,
                g: 0.31,
                b: 0.46,
                a: 0.58 * alpha,
            },
            1.3,
        );

        card_rect
    }

    // =========================================================================
    // SECTION 0: HUB (Jam Besar, Baterai iOS, & Daily Motivational Quote Card)
    // =========================================================================
    unsafe fn draw_hub_panel(&self, ctrl: &NotchController, card: D2D_RECT_F, alpha: f32) {
            let mut p_col = AntiMagicNeutralPalette::TEXT_PRIMARY;
            p_col.a *= alpha;
            let mut s_col = AntiMagicNeutralPalette::TEXT_SECONDARY;
            s_col.a *= alpha;

            // Kiri: Kartu Jam Besar, Tanggal, & Ikon Baterai iOS
            let left_card = D2D_RECT_F {
                left: card.left + 10.0,
                top: card.top + 10.0,
                right: card.left + 162.0,
                bottom: card.bottom - 10.0,
            };
            self.fill_gradient_glass_card(
                left_card,
                13.0,
                D2D1_COLOR_F {
                    r: 0.11,
                    g: 0.15,
                    b: 0.24,
                    a: 0.92 * alpha,
                },
                D2D1_COLOR_F {
                    r: 0.09,
                    g: 0.12,
                    b: 0.19,
                    a: 0.92 * alpha,
                },
                D2D1_COLOR_F {
                    r: 0.13,
                    g: 0.17,
                    b: 0.27,
                    a: 0.92 * alpha,
                },
                D2D1_COLOR_F {
                    r: 0.32,
                    g: 0.44,
                    b: 0.68,
                    a: 0.50 * alpha,
                },
                1.1,
            );

            let now = Local::now();
            let time_big = now.format("%H:%M").to_string();
            let date_str = now.format("%a, %d %b %Y").to_string();

            self.draw_text_left(
                &time_big,
                &self.fmt_clock_hero,
                D2D_RECT_F {
                    left: left_card.left + 12.0,
                    top: left_card.top + 6.0,
                    right: left_card.right - 8.0,
                    bottom: left_card.top + 40.0,
                },
                p_col,
            );
            self.draw_text_left(
                &date_str,
                &self.fmt_caption,
                D2D_RECT_F {
                    left: left_card.left + 12.0,
                    top: left_card.top + 40.0,
                    right: left_card.right - 8.0,
                    bottom: left_card.top + 58.0,
                },
                s_col,
            );

            // Baterai iOS lebih ke bawah agar tidak tabrakan dengan tanggal
            self.draw_ios_battery_icon(
                left_card.left + 12.0,
                left_card.bottom - 32.0,
                ctrl.battery_percent,
                ctrl.is_charging,
                alpha,
            );

            self.draw_text_left(
                if ctrl.is_charging {
                    "Charging"
                } else {
                    "Battery OK"
                },
                &self.fmt_caption,
                D2D_RECT_F {
                    left: left_card.left + 56.0,
                    top: left_card.bottom - 34.0,
                    right: left_card.right - 6.0,
                    bottom: left_card.bottom - 10.0,
                },
                AntiMagicNeutralPalette::ACCENT_CYAN,
            );

            // Kanan: Kartu WORD & QUOTE OF THE DAY (Semi-Glass Multi-Gradient)
            let q_card = D2D_RECT_F {
                left: left_card.right + 10.0,
                top: card.top + 10.0,
                right: card.right - 10.0,
                bottom: card.bottom - 10.0,
            };

        self.fill_gradient_glass_card(
            q_card,
            13.0,
            D2D1_COLOR_F {
                r: 0.12,
                g: 0.16,
                b: 0.26,
                a: 0.92 * alpha,
            },
            D2D1_COLOR_F {
                r: 0.15,
                g: 0.19,
                b: 0.31,
                a: 0.92 * alpha,
            },
            D2D1_COLOR_F {
                r: 0.10,
                g: 0.14,
                b: 0.22,
                a: 0.92 * alpha,
            },
            D2D1_COLOR_F {
                r: 0.35,
                g: 0.48,
                b: 0.78,
                a: 0.55 * alpha,
            },
            1.1,
        );

        let daily = ctrl.current_quote();
        let header = format!("✨ WORD OF THE DAY · {}", daily.word_of_the_day);
        self.draw_text_left(
            &header,
            &self.fmt_caption,
            D2D_RECT_F {
                left: q_card.left + 12.0,
                top: q_card.top + 8.0,
                right: q_card.right - 12.0,
                bottom: q_card.top + 26.0,
            },
            AntiMagicNeutralPalette::ACCENT_CYAN,
        );

        self.draw_text_left(
            daily.quote,
            &self.fmt_quote_italic,
            D2D_RECT_F {
                left: q_card.left + 12.0,
                top: q_card.top + 28.0,
                right: q_card.right - 12.0,
                bottom: q_card.bottom - 26.0,
            },
            p_col,
        );

        let footer = format!("— {}  (Click / Scroll for next)", daily.author);
        self.draw_text_left(
            &footer,
            &self.fmt_caption,
            D2D_RECT_F {
                left: q_card.left + 12.0,
                top: q_card.bottom - 24.0,
                right: q_card.right - 12.0,
                bottom: q_card.bottom - 6.0,
            },
            s_col,
        );
    }

    // =========================================================================
    // SECTION 1: MUSIC/VIDEO (Full-Height Cover Art + Controls + Auto-Synced Lyrics)
    // =========================================================================
    unsafe fn draw_music_panel(&self, ctrl: &NotchController, card: D2D_RECT_F, alpha: f32) {
        let mut p_col = AntiMagicNeutralPalette::TEXT_PRIMARY;
        p_col.a *= alpha;
        let mut s_col = AntiMagicNeutralPalette::TEXT_SECONDARY;
        s_col.a *= alpha;

        // 1. FULL-HEIGHT COVER ART CONTAINER (Merentang Penuh dari Atas ke Bawah Kartu!)
        let art_h = (card.bottom - card.top - 16.0).max(80.0);
        let art_rect = D2D_RECT_F {
            left: card.left + 8.0,
            top: card.top + 8.0,
            right: card.left + 8.0 + art_h,
            bottom: card.bottom - 8.0,
        };

        self.fill_gradient_glass_card(
            art_rect,
            13.0,
            D2D1_COLOR_F {
                r: 0.16,
                g: 0.24,
                b: 0.42,
                a: alpha,
            },
            D2D1_COLOR_F {
                r: 0.26,
                g: 0.18,
                b: 0.36,
                a: alpha,
            },
            D2D1_COLOR_F {
                r: 0.10,
                g: 0.16,
                b: 0.30,
                a: alpha,
            },
            D2D1_COLOR_F {
                r: 0.42,
                g: 0.58,
                b: 0.92,
                a: 0.70 * alpha,
            },
            1.2,
        );

        // Visualizer bar dinamis di bagian bawah Full-Height Cover
        if let Ok(eq_b) = self
            .rt
            .CreateSolidColorBrush(&AntiMagicNeutralPalette::ACCENT_CYAN, None)
        {
            let num_bars = 6;
            let total_eq_w = art_h - 24.0;
            let step_x = total_eq_w / num_bars as f32;
            for i in 0..num_bars {
                let phase = ctrl.anim_time * 6.0 + i as f32 * 1.1;
                let h = if ctrl.media_playing {
                    10.0 + phase.sin().abs() * (art_h * 0.42)
                } else {
                    6.0
                };
                let bx = art_rect.left + 14.0 + i as f32 * step_x;
                let by = art_rect.bottom - 12.0;
                self.rt.FillRoundedRectangle(
                    &D2D1_ROUNDED_RECT {
                        rect: D2D_RECT_F {
                            left: bx,
                            top: by - h,
                            right: bx + 6.0,
                            bottom: by,
                        },
                        radiusX: 3.0,
                        radiusY: 3.0,
                    },
                    &eq_b,
                );
            }
        }

        // 2. Kolom Tengah: Judul, Artis, Progress Bar + Timecode, & Tombol Kontrol
        let mid_left = art_rect.right + 10.0;
        let mid_right = if ctrl.show_side_lyrics {
            card.left + 282.0
        } else {
            card.right - 50.0
        };

        self.draw_text_left(
            &ctrl.media_title,
            &self.fmt_title,
            D2D_RECT_F {
                left: mid_left,
                top: card.top + 8.0,
                right: mid_right,
                bottom: card.top + 28.0,
            },
            p_col,
        );
        self.draw_text_left(
            &ctrl.media_artist,
            &self.fmt_caption,
            D2D_RECT_F {
                left: mid_left,
                top: card.top + 28.0,
                right: mid_right,
                bottom: card.top + 46.0,
            },
            s_col,
        );

        let bar_top = card.top + 54.0;
        let bar_w = (mid_right - mid_left).max(16.0);
        if let Ok(tb) = self.rt.CreateSolidColorBrush(
            &D2D1_COLOR_F {
                r: 0.18,
                g: 0.23,
                b: 0.34,
                a: 0.75 * alpha,
            },
            None,
        ) {
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: D2D_RECT_F {
                        left: mid_left,
                        top: bar_top,
                        right: mid_right,
                        bottom: bar_top + 4.5,
                    },
                    radiusX: 2.2,
                    radiusY: 2.2,
                },
                &tb,
            );
        }

        let prog_w = (bar_w * ctrl.media_progress.clamp(0.0, 1.0)).max(5.0);
        self.fill_gradient_glass_card(
            D2D_RECT_F {
                left: mid_left,
                top: bar_top,
                right: mid_left + prog_w,
                bottom: bar_top + 4.5,
            },
            2.2,
            AntiMagicNeutralPalette::ACCENT_BLUE,
            AntiMagicNeutralPalette::ACCENT_CYAN,
            AntiMagicNeutralPalette::ACCENT_CYAN,
            AntiMagicNeutralPalette::ACCENT_CYAN,
            0.0,
        );

        let el_s = ctrl.media_elapsed_exact as u32;
        let tot_s = ctrl.media_total_secs as u32;
        let time_str = format!(
            "{:02}:{:02} / {:02}:{:02}",
            el_s / 60,
            el_s % 60,
            tot_s / 60,
            tot_s % 60
        );
        self.draw_text_left(
            &time_str,
            &self.fmt_caption,
            D2D_RECT_F {
                left: mid_left,
                top: bar_top + 6.0,
                right: mid_right,
                bottom: bar_top + 22.0,
            },
            s_col,
        );

        // Tombol Kontrol: [ |<< ] [ Play/Pause ] [ >>| ] [ Lyrics ]
        let btn_y = card.bottom - 22.0;
        let btns = [
            (mid_left + 15.0, "|<<"),
            (
                mid_left + 51.0,
                if ctrl.media_playing { "||" } else { "|>" },
            ),
            (mid_left + 87.0, ">>|"),
        ];
        for (cx, sym) in btns {
            let r = D2D_RECT_F {
                left: cx - 15.0,
                top: btn_y - 11.0,
                right: cx + 15.0,
                bottom: btn_y + 11.0,
            };
            if let Ok(b) = self
                .rt
                .CreateSolidColorBrush(&AntiMagicNeutralPalette::CARD_ELEVATED, None)
            {
                self.rt.FillRoundedRectangle(
                    &D2D1_ROUNDED_RECT {
                        rect: r,
                        radiusX: 7.0,
                        radiusY: 7.0,
                    },
                    &b,
                );
            }
            self.draw_text_centered(sym, &self.fmt_caption, r, p_col);
        }

        let lyr_btn = D2D_RECT_F {
            left: mid_left + 108.0,
            top: btn_y - 11.0,
            right: mid_left + 160.0,
            bottom: btn_y + 11.0,
        };
        if let Ok(b) = self
            .rt
            .CreateSolidColorBrush(&AntiMagicNeutralPalette::CARD_ELEVATED, None)
        {
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: lyr_btn,
                    radiusX: 7.0,
                    radiusY: 7.0,
                },
                &b,
            );
        }
        self.draw_text_centered(
            "Lyrics",
            &self.fmt_caption,
            lyr_btn,
            AntiMagicNeutralPalette::ACCENT_CYAN,
        );

        // 3. Kolom Kanan: SPOTIFY-STYLE AUTO-SYNCED LYRICS PANE
        if ctrl.show_side_lyrics {
            let lyr_card = D2D_RECT_F {
                left: mid_right + 6.0,
                top: card.top + 8.0,
                right: card.right - 42.0,
                bottom: card.bottom - 8.0,
            };

            self.fill_gradient_glass_card(
                lyr_card,
                13.0,
                D2D1_COLOR_F {
                    r: 0.10,
                    g: 0.14,
                    b: 0.23,
                    a: 0.92 * alpha,
                },
                D2D1_COLOR_F {
                    r: 0.14,
                    g: 0.18,
                    b: 0.29,
                    a: 0.92 * alpha,
                },
                D2D1_COLOR_F {
                    r: 0.09,
                    g: 0.12,
                    b: 0.20,
                    a: 0.92 * alpha,
                },
                D2D1_COLOR_F {
                    r: 0.30,
                    g: 0.42,
                    b: 0.65,
                    a: 0.50 * alpha,
                },
                1.0,
            );

            if !ctrl.timed_lyrics.is_empty() {
                let idx = ctrl.active_lyric_idx.min(ctrl.timed_lyrics.len() - 1);
                let prev_l = if idx > 0 {
                    ctrl.timed_lyrics[idx - 1].1.as_str()
                } else {
                    "♪ · · ·"
                };
                let curr_l = ctrl.timed_lyrics[idx].1.as_str();
                let next_l = if idx + 1 < ctrl.timed_lyrics.len() {
                    ctrl.timed_lyrics[idx + 1].1.as_str()
                } else {
                    "· · ·"
                };

                self.draw_text_left(
                    prev_l,
                    &self.fmt_caption,
                    D2D_RECT_F {
                        left: lyr_card.left + 10.0,
                        top: lyr_card.top + 8.0,
                        right: lyr_card.right - 8.0,
                        bottom: lyr_card.top + 30.0,
                    },
                    s_col,
                );
                self.draw_text_left(
                    curr_l,
                    &self.fmt_title,
                    D2D_RECT_F {
                        left: lyr_card.left + 10.0,
                        top: lyr_card.top + 32.0,
                        right: lyr_card.right - 8.0,
                        bottom: lyr_card.top + 82.0,
                    },
                    p_col,
                );
                self.draw_text_left(
                    next_l,
                    &self.fmt_caption,
                    D2D_RECT_F {
                        left: lyr_card.left + 10.0,
                        top: lyr_card.top + 84.0,
                        right: lyr_card.right - 8.0,
                        bottom: lyr_card.bottom - 6.0,
                    },
                    s_col,
                );
            }
        }
    }

    // =========================================================================
    // SECTION 2: ALL-IN-ONE CONTROL CENTER (Click & Scroll Wheel Supported!)
    // =========================================================================
    unsafe fn draw_control_panel(&self, ctrl: &NotchController, card: D2D_RECT_F, alpha: f32) {
            let mut p_col = AntiMagicNeutralPalette::TEXT_PRIMARY;
            p_col.a *= alpha;

            let card_h = card.bottom - card.top;
        
            // 2 slider cards (Volume & Brightness) dengan tinggi penuh agar fit rapi
            let slider_h = (card_h - 30.0) * 0.5; // 2 slider dengan gap 10px
        
            let sliders = [
                (
                    card.left + 10.0,
                    card.right - 10.0,
                    "🔊 Volume (Click/Scroll)",
                    ctrl.volume_level,
                    card.top + 10.0,
                    card.top + 10.0 + slider_h,
                ),
                (
                    card.left + 10.0,
                    card.right - 10.0,
                    "🔆 Brightness (Click/Scroll)",
                    ctrl.brightness_level,
                    card.top + 20.0 + slider_h,
                    card.bottom - 10.0,
                ),
            ];

            for (l, r, label, val, top, bottom) in sliders {
                let s_rect = D2D_RECT_F {
                    left: l,
                    top,
                    right: r,
                    bottom,
                };
                self.fill_gradient_glass_card(
                    s_rect,
                    12.0,
                    D2D1_COLOR_F {
                        r: 0.11,
                        g: 0.14,
                        b: 0.22,
                        a: 0.92 * alpha,
                    },
                    D2D1_COLOR_F {
                        r: 0.13,
                        g: 0.17,
                        b: 0.26,
                        a: 0.92 * alpha,
                    },
                    D2D1_COLOR_F {
                        r: 0.10,
                        g: 0.13,
                        b: 0.20,
                        a: 0.92 * alpha,
                    },
                    D2D1_COLOR_F {
                        r: 0.28,
                        g: 0.36,
                        b: 0.54,
                        a: 0.50 * alpha,
                    },
                    1.0,
                );

                self.draw_text_left(
                    &format!("{} · {}%", label, val),
                    &self.fmt_caption,
                    D2D_RECT_F {
                        left: l + 10.0,
                        top: s_rect.top + 6.0,
                        right: r - 10.0,
                        bottom: s_rect.top + 26.0,
                    },
                    p_col,
                );

                let tl = l + 10.0;
                let tr = r - 10.0;
                let ty = s_rect.top + (s_rect.bottom - s_rect.top) * 0.5;
                if let Ok(tb) = self.rt.CreateSolidColorBrush(
                    &D2D1_COLOR_F {
                        r: 0.18,
                        g: 0.23,
                        b: 0.34,
                        a: 0.8 * alpha,
                    },
                    None,
                ) {
                    self.rt.FillRoundedRectangle(
                        &D2D1_ROUNDED_RECT {
                            rect: D2D_RECT_F {
                                left: tl,
                                top: ty - 6.0,
                                right: tr,
                                bottom: ty + 6.0,
                            },
                            radiusX: 6.0,
                            radiusY: 6.0,
                        },
                        &tb,
                    );
                }
                let fw = (tr - tl) * (val as f32 / 100.0).clamp(0.0, 1.0);
                self.fill_gradient_glass_card(
                    D2D_RECT_F {
                        left: tl,
                        top: ty - 6.0,
                        right: tl + fw.max(10.0),
                        bottom: ty + 6.0,
                    },
                    6.0,
                    AntiMagicNeutralPalette::ACCENT_BLUE,
                    AntiMagicNeutralPalette::ACCENT_CYAN,
                    AntiMagicNeutralPalette::ACCENT_CYAN,
                    AntiMagicNeutralPalette::ACCENT_CYAN,
                    0.0,
                );
            }
        }

    // =========================================================================
    // SECTION 3: POMODORO FOCUS TIMER
    // =========================================================================
    unsafe fn draw_pomo_panel(&self, ctrl: &NotchController, card: D2D_RECT_F, alpha: f32) {
        let mut p_col = AntiMagicNeutralPalette::TEXT_PRIMARY;
        p_col.a *= alpha;
        let mut s_col = AntiMagicNeutralPalette::TEXT_SECONDARY;
        s_col.a *= alpha;
        let accent = if ctrl.pomo_is_break {
            AntiMagicNeutralPalette::ACCENT_WINE
        } else {
            AntiMagicNeutralPalette::ACCENT_CYAN
        };

        let ring_cx = card.left + 58.0;
        let ring_cy = (card.top + card.bottom) * 0.5;
        let ring_r = 38.0;

        if let Ok(fg_ring) = self.rt.CreateSolidColorBrush(&accent, None) {
            self.rt.DrawEllipse(
                &D2D1_ELLIPSE {
                    point: D2D_POINT_2F {
                        x: ring_cx,
                        y: ring_cy,
                    },
                    radiusX: ring_r,
                    radiusY: ring_r,
                },
                &fg_ring,
                3.6,
                None,
            );
        }

        let timer_text = format!(
            "{:02}:{:02}",
            ctrl.pomo_remaining_secs / 60,
            ctrl.pomo_remaining_secs % 60
        );
        self.draw_text_centered(
            &timer_text,
            &self.fmt_title,
            D2D_RECT_F {
                left: ring_cx - 34.0,
                top: ring_cy - 14.0,
                right: ring_cx + 34.0,
                bottom: ring_cy + 14.0,
            },
            p_col,
        );

        let right_x = card.left + 114.0;
        self.draw_text_left(
            if ctrl.pomo_is_break {
                "SHORT BREAK · REST EYES"
            } else {
                "POMODORO · FOCUS SESSION"
            },
            &self.fmt_title,
            D2D_RECT_F {
                left: right_x,
                top: card.top + 14.0,
                right: card.right - 50.0,
                bottom: card.top + 38.0,
            },
            p_col,
        );

        self.draw_text_left(
            if ctrl.pomo_running {
                "Timer running — Chibi Liebe is guarding your focus"
            } else {
                "Click Start Focus below to begin 25m session"
            },
            &self.fmt_body,
            D2D_RECT_F {
                left: right_x,
                top: card.top + 38.0,
                right: card.right - 50.0,
                bottom: card.top + 60.0,
            },
            s_col,
        );

        let start_rect = D2D_RECT_F {
            left: right_x,
            top: card.top + 66.0,
            right: right_x + 90.0,
            bottom: card.top + 96.0,
        };
        self.fill_gradient_glass_card(
            start_rect,
            9.0,
            AntiMagicNeutralPalette::ACCENT_BLUE,
            accent,
            accent,
            accent,
            1.0,
        );
        self.draw_text_centered(
            if ctrl.pomo_running { "Pause" } else { "Start Focus" },
            &self.fmt_caption,
            start_rect,
            p_col,
        );

        let reset_rect = D2D_RECT_F {
            left: right_x + 98.0,
            top: card.top + 66.0,
            right: right_x + 166.0,
            bottom: card.top + 96.0,
        };
        if let Ok(b) = self
            .rt
            .CreateSolidColorBrush(&AntiMagicNeutralPalette::CARD_ELEVATED, None)
        {
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: reset_rect,
                    radiusX: 9.0,
                    radiusY: 9.0,
                },
                &b,
            );
        }
        self.draw_text_centered("Reset", &self.fmt_caption, reset_rect, s_col);
    }

    // =========================================================================
    // SECTION 4: FILE SHELF
    // =========================================================================
    unsafe fn draw_shelf_panel(&self, ctrl: &NotchController, card: D2D_RECT_F, alpha: f32) {
        let mut p_col = AntiMagicNeutralPalette::TEXT_PRIMARY;
        p_col.a *= alpha;
        let mut s_col = AntiMagicNeutralPalette::TEXT_SECONDARY;
        s_col.a *= alpha;

        self.draw_text_left(
            "FILE SHELF · DRAG FILES INTO NOTCH OR CLICK FILE TO OPEN/USE",
            &self.fmt_caption,
            D2D_RECT_F {
                left: card.left + 14.0,
                top: card.top + 8.0,
                right: card.right - 50.0,
                bottom: card.top + 26.0,
            },
            AntiMagicNeutralPalette::ACCENT_CYAN,
        );

        if ctrl.shelf_files.is_empty() {
            let drop_rect = D2D_RECT_F {
                left: card.left + 14.0,
                top: card.top + 32.0,
                right: card.right - 56.0,
                bottom: card.bottom - 12.0,
            };
            self.fill_gradient_glass_card(
                drop_rect,
                12.0,
                D2D1_COLOR_F {
                    r: 0.10,
                    g: 0.13,
                    b: 0.21,
                    a: 0.85 * alpha,
                },
                D2D1_COLOR_F {
                    r: 0.12,
                    g: 0.16,
                    b: 0.26,
                    a: 0.85 * alpha,
                },
                D2D1_COLOR_F {
                    r: 0.09,
                    g: 0.12,
                    b: 0.19,
                    a: 0.85 * alpha,
                },
                D2D1_COLOR_F {
                    r: 0.35,
                    g: 0.48,
                    b: 0.78,
                    a: 0.55 * alpha,
                },
                1.2,
            );
            self.draw_text_centered(
                "📥 Drag & Drop file dari Explorer/App lain ke area Notch ini",
                &self.fmt_body,
                drop_rect,
                s_col,
            );
            return;
        }

        for (i, (name, meta, _)) in ctrl.shelf_files.iter().take(2).enumerate() {
            let item_w = (card.right - card.left - 80.0) * 0.5;
            let ix = card.left + 14.0 + i as f32 * (item_w + 10.0);
            let item_rect = D2D_RECT_F {
                left: ix,
                top: card.top + 32.0,
                right: ix + item_w,
                bottom: card.bottom - 12.0,
            };

            self.fill_gradient_glass_card(
                item_rect,
                12.0,
                D2D1_COLOR_F {
                    r: 0.12,
                    g: 0.16,
                    b: 0.25,
                    a: 0.92 * alpha,
                },
                D2D1_COLOR_F {
                    r: 0.15,
                    g: 0.20,
                    b: 0.31,
                    a: 0.92 * alpha,
                },
                D2D1_COLOR_F {
                    r: 0.10,
                    g: 0.14,
                    b: 0.22,
                    a: 0.92 * alpha,
                },
                D2D1_COLOR_F {
                    r: 0.35,
                    g: 0.48,
                    b: 0.75,
                    a: 0.55 * alpha,
                },
                1.1,
            );

            self.draw_text_left(
                name,
                &self.fmt_title,
                D2D_RECT_F {
                    left: item_rect.left + 10.0,
                    top: item_rect.top + 10.0,
                    right: item_rect.right - 22.0,
                    bottom: item_rect.top + 36.0,
                },
                p_col,
            );
            self.draw_text_left(
                meta,
                &self.fmt_caption,
                D2D_RECT_F {
                    left: item_rect.left + 10.0,
                    top: item_rect.top + 38.0,
                    right: item_rect.right - 10.0,
                    bottom: item_rect.bottom - 8.0,
                },
                s_col,
            );
            self.draw_text_centered(
                "×",
                &self.fmt_title,
                D2D_RECT_F {
                    left: item_rect.right - 22.0,
                    top: item_rect.top + 6.0,
                    right: item_rect.right - 6.0,
                    bottom: item_rect.top + 24.0,
                },
                s_col,
            );
        }
    }

    // =========================================================================
    // SECTION 5: STAGE DOCK - Active Stages & Windows Switcher
    // =========================================================================
    unsafe fn draw_stagedock_panel(&self, _ctrl: &NotchController, card: D2D_RECT_F, alpha: f32) {
        let mut p_col = AntiMagicNeutralPalette::TEXT_PRIMARY;
        p_col.a *= alpha;

        self.draw_text_left(
            "STAGE DOCK · ACTIVE STAGES & WINDOWS SWITCHER",
            &self.fmt_caption,
            D2D_RECT_F {
                left: card.left + 14.0,
                top: card.top + 8.0,
                right: card.right - 50.0,
                bottom: card.top + 26.0,
            },
            AntiMagicNeutralPalette::ACCENT_CYAN,
        );

        let inner_top = card.top + 32.0;
        let inner_left = card.left + 14.0;
        let inner_right = card.right - 56.0;
        let inner_bottom = card.bottom - 12.0;

        let info_rect = D2D_RECT_F {
            left: inner_left,
            top: inner_top,
            right: inner_right,
            bottom: inner_bottom,
        };
        self.fill_gradient_glass_card(
            info_rect,
            12.0,
            D2D1_COLOR_F {
                r: 0.12,
                g: 0.16,
                b: 0.25,
                a: 0.92 * alpha,
            },
            D2D1_COLOR_F {
                r: 0.15,
                g: 0.20,
                b: 0.31,
                a: 0.92 * alpha,
            },
            D2D1_COLOR_F {
                r: 0.10,
                g: 0.14,
                b: 0.22,
                a: 0.92 * alpha,
            },
            D2D1_COLOR_F {
                r: 0.35,
                g: 0.48,
                b: 0.75,
                a: 0.55 * alpha,
            },
            1.1,
        );

        self.draw_text_centered(
            "⚡ StageDock Integration\n\nMenampilkan Active Stages & Window Groups\nKlik untuk berpindah stage (Win+Alt+1/2/3/4)\natau luncurkan StageDock dari D:\\StageDockV2",
            &self.fmt_body,
            info_rect,
            p_col,
        );
    }
}