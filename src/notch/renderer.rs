// src/notch/renderer.rs
use chrono::Local;
use windows::core::w;
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Direct2D::Common::{
    D2D1_ALPHA_MODE_PREMULTIPLIED, D2D1_BEZIER_SEGMENT, D2D1_COLOR_F, D2D1_FIGURE_BEGIN_FILLED,
    D2D1_FIGURE_END_CLOSED, D2D1_PIXEL_FORMAT, D2D_POINT_2F, D2D_RECT_F, D2D_SIZE_U,
};
use windows::Win32::Graphics::Direct2D::*;
use windows::Win32::Graphics::DirectWrite::{
    DWriteCreateFactory, IDWriteFactory, IDWriteTextFormat, DWRITE_FACTORY_TYPE_SHARED,
    DWRITE_FONT_STRETCH_NORMAL, DWRITE_FONT_STYLE_NORMAL, DWRITE_FONT_WEIGHT_BOLD,
    DWRITE_FONT_WEIGHT_REGULAR, DWRITE_FONT_WEIGHT_SEMI_BOLD, DWRITE_PARAGRAPH_ALIGNMENT_CENTER,
    DWRITE_TEXT_ALIGNMENT_CENTER, DWRITE_TEXT_ALIGNMENT_LEADING,
};
use windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_B8G8R8A8_UNORM;
use windows::Win32::Graphics::Gdi::{BeginPaint, EndPaint, PAINTSTRUCT};

use super::states::{NotchController, NotchState, OverlayKind};

pub struct AntiMagicNeutralPalette;

impl AntiMagicNeutralPalette {
    pub const PILL_BG: D2D1_COLOR_F = D2D1_COLOR_F {
        r: 0.043,
        g: 0.055,
        b: 0.078,
        a: 0.98,
    };
    pub const CARD_SURFACE: D2D1_COLOR_F = D2D1_COLOR_F {
        r: 0.075,
        g: 0.094,
        b: 0.133,
        a: 0.95,
    };
    pub const CARD_ELEVATED: D2D1_COLOR_F = D2D1_COLOR_F {
        r: 0.110,
        g: 0.137,
        b: 0.192,
        a: 0.95,
    };
    pub const STEEL_BORDER: D2D1_COLOR_F = D2D1_COLOR_F {
        r: 0.180,
        g: 0.230,
        b: 0.320,
        a: 0.72,
    };
    pub const ACCENT_BLUE: D2D1_COLOR_F = D2D1_COLOR_F {
        r: 0.392,
        g: 0.533,
        b: 0.918,
        a: 1.0,
    };
    pub const ACCENT_EMERALD: D2D1_COLOR_F = D2D1_COLOR_F {
        r: 0.180,
        g: 0.880,
        b: 0.580,
        a: 1.0,
    };
    pub const ACCENT_WINE: D2D1_COLOR_F = D2D1_COLOR_F {
        r: 0.680,
        g: 0.220,
        b: 0.300,
        a: 0.95,
    };
    pub const TEXT_PRIMARY: D2D1_COLOR_F = D2D1_COLOR_F {
        r: 0.930,
        g: 0.950,
        b: 0.980,
        a: 1.0,
    };
    pub const TEXT_SECONDARY: D2D1_COLOR_F = D2D1_COLOR_F {
        r: 0.580,
        g: 0.650,
        b: 0.760,
        a: 1.0,
    };
}

pub struct NotchRenderer {
    pub factory: ID2D1Factory,
    pub dwrite: IDWriteFactory,
    pub rt: ID2D1HwndRenderTarget,
    pub fmt_title: IDWriteTextFormat,
    pub fmt_body: IDWriteTextFormat,
    pub fmt_caption: IDWriteTextFormat,
}

impl NotchRenderer {
    pub fn new(hwnd: HWND, width: u32, height: u32) -> windows::core::Result<Self> {
        unsafe {
            let factory: ID2D1Factory =
                D2D1CreateFactory(D2D1_FACTORY_TYPE_SINGLE_THREADED, None)?;
            let dwrite: IDWriteFactory = DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED)?;

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

            let fmt_title = dwrite.CreateTextFormat(
                w!("Segoe UI"),
                None,
                DWRITE_FONT_WEIGHT_BOLD,
                DWRITE_FONT_STYLE_NORMAL,
                DWRITE_FONT_STRETCH_NORMAL,
                13.0,
                w!("en-us"),
            )?;

            let fmt_body = dwrite.CreateTextFormat(
                w!("Segoe UI"),
                None,
                DWRITE_FONT_WEIGHT_REGULAR,
                DWRITE_FONT_STYLE_NORMAL,
                DWRITE_FONT_STRETCH_NORMAL,
                11.0,
                w!("en-us"),
            )?;

            let fmt_caption = dwrite.CreateTextFormat(
                w!("Segoe UI"),
                None,
                DWRITE_FONT_WEIGHT_SEMI_BOLD,
                DWRITE_FONT_STYLE_NORMAL,
                DWRITE_FONT_STRETCH_NORMAL,
                9.5,
                w!("en-us"),
            )?;

            Ok(Self {
                factory,
                dwrite,
                rt,
                fmt_title,
                fmt_body,
                fmt_caption,
            })
        }
    }

    /// Geometri Notch yang MENYATU DENGAN BEZEL ATAS LAPTOP (rata di y=0, memiliki bahu halus, dan rounded di bawah)
    unsafe fn create_bezel_notch_geometry(
        &self,
        rect: D2D_RECT_F,
        bottom_radius: f32,
    ) -> windows::core::Result<ID2D1PathGeometry> {
        let geom = self.factory.CreatePathGeometry()?;
        let sink = geom.Open()?;

        let h = (rect.bottom - rect.top).max(4.0);
        let r = bottom_radius.min(h * 0.5).max(2.0);
        let ear = (r * 0.45).min(8.0); // Lekukan bahu atas yang menyatu dengan bezel layar

        // Mulai dari bahu kiri-atas tepat di bezel (top = 0.0)
        sink.BeginFigure(
            D2D_POINT_2F {
                x: rect.left - ear,
                y: rect.top,
            },
            D2D1_FIGURE_BEGIN_FILLED,
        );

        // Bahu kiri-atas melengkung masuk ke dinding kiri
        sink.AddBezier(&D2D1_BEZIER_SEGMENT {
            point1: D2D_POINT_2F {
                x: rect.left - ear * 0.2,
                y: rect.top,
            },
            point2: D2D_POINT_2F {
                x: rect.left,
                y: rect.top + ear * 0.2,
            },
            point3: D2D_POINT_2F {
                x: rect.left,
                y: rect.top + ear,
            },
        });

        // Dinding kiri turun ke sudut kiri-bawah
        sink.AddLine(D2D_POINT_2F {
            x: rect.left,
            y: rect.bottom - r,
        });

        // Sudut kiri-bawah rounded mulus
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

        // Garis bawah horizontal ke sudut kanan-bawah
        sink.AddLine(D2D_POINT_2F {
            x: rect.right - r,
            y: rect.bottom,
        });

        // Sudut kanan-bawah rounded mulus
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

        // Dinding kanan naik ke bahu kanan-atas
        sink.AddLine(D2D_POINT_2F {
            x: rect.right,
            y: rect.top + ear,
        });

        // Bahu kanan-atas melengkung menyatu ke bezel layar (top = 0.0)
        sink.AddBezier(&D2D1_BEZIER_SEGMENT {
            point1: D2D_POINT_2F {
                x: rect.right,
                y: rect.top + ear * 0.2,
            },
            point2: D2D_POINT_2F {
                x: rect.right + ear * 0.2,
                y: rect.top,
            },
            point3: D2D_POINT_2F {
                x: rect.right + ear,
                y: rect.top,
            },
        });

        sink.EndFigure(D2D1_FIGURE_END_CLOSED);
        sink.Close()?;
        Ok(geom)
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

            // 1. Gambar Notch Menyatu dengan Bezel Atas Laptop
            if let Ok(notch_geom) =
                self.create_bezel_notch_geometry(pill_rect, controller.corner_radius.value)
            {
                if let Ok(bg_brush) = self
                    .rt
                    .CreateSolidColorBrush(&AntiMagicNeutralPalette::PILL_BG, None)
                {
                    self.rt.FillGeometry(&notch_geom, &bg_brush, None);
                }

                let border_color = if controller.state == NotchState::Pet {
                    let pulse = controller.devil_eyes.aura_intensity.clamp(0.4, 1.2);
                    D2D1_COLOR_F {
                        r: 0.25 * pulse,
                        g: 0.24,
                        b: 0.36,
                        a: 0.60,
                    }
                } else {
                    AntiMagicNeutralPalette::STEEL_BORDER
                };

                if let Ok(border_brush) = self.rt.CreateSolidColorBrush(&border_color, None) {
                    self.rt.DrawGeometry(&notch_geom, &border_brush, 1.15, None);
                }
            }

            // 2. Jika ada Pop-Up Overlay aktif (Volume, Brightness, CapsLock, Charging, Headphones)
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
                let collapse_factor = ((60.0 - controller.height.value) / 26.0).clamp(0.0, 1.0);
                controller
                    .devil_eyes
                    .render(&self.factory, &self.rt, pill_rect, collapse_factor);
            } else if controller.state == NotchState::Idle && !controller.minimal_mode {
                // Closed Notch seperti Nusa Original: Jam Digital di Kiri, Status Baterai di Kanan
                let alpha = ((60.0 - controller.height.value) / 26.0).clamp(0.0, 1.0);
                self.draw_closed_idle_clock_and_battery(controller, pill_rect, alpha);
            } else if controller.height.value > 65.0 {
                let alpha = controller.content_alpha.value.clamp(0.0, 1.0);
                let card_rect = self.draw_expanded_panel_shell(controller.state, pill_rect, alpha);

                match controller.state {
                    NotchState::Music => self.draw_music_panel(controller, card_rect, alpha),
                    NotchState::Lyrics => self.draw_lyrics_panel(controller, card_rect, alpha),
                    NotchState::Pomo => self.draw_pomo_panel(controller, card_rect, alpha),
                    NotchState::Water => self.draw_water_panel(controller, card_rect, alpha),
                    NotchState::Shelf => self.draw_shelf_panel(controller, card_rect, alpha),
                    NotchState::Todo => self.draw_todo_panel(controller, card_rect, alpha),
                    _ => {}
                }

                // Chibi Liebe selektif pada Music, Pomo, dan Shelf
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

    /// Tampilan Closed Idle seperti Nusa Original: Jam Digital + Indikator Baterai/Charging
    unsafe fn draw_closed_idle_clock_and_battery(
        &self,
        ctrl: &NotchController,
        pill: D2D_RECT_F,
        alpha: f32,
    ) {
        let now_str = Local::now().format("%H:%M").to_string();
        let mut p_col = AntiMagicNeutralPalette::TEXT_PRIMARY;
        p_col.a *= alpha;
        let mut s_col = if ctrl.is_charging {
            AntiMagicNeutralPalette::ACCENT_EMERALD
        } else {
            AntiMagicNeutralPalette::TEXT_SECONDARY
        };
        s_col.a *= alpha;

        self.draw_text_left(
            &now_str,
            &self.fmt_title,
            D2D_RECT_F {
                left: pill.left + 16.0,
                top: pill.top,
                right: pill.left + 90.0,
                bottom: pill.bottom,
            },
            p_col,
        );

        let bat_text = if ctrl.is_charging {
            format!("⚡ {}%", ctrl.battery_percent)
        } else {
            format!("{}%", ctrl.battery_percent)
        };
        self.draw_text_centered(
            &bat_text,
            &self.fmt_caption,
            D2D_RECT_F {
                left: pill.right - 72.0,
                top: pill.top,
                right: pill.right - 12.0,
                bottom: pill.bottom,
            },
            s_col,
        );
    }

    /// Render Pop-up Overlays (Volume, Brightness, CapsLock, Charging, Headphones)
    unsafe fn draw_system_overlay(&self, overlay: &OverlayKind, pill: D2D_RECT_F) {
        let p_col = AntiMagicNeutralPalette::TEXT_PRIMARY;
        let accent = AntiMagicNeutralPalette::ACCENT_BLUE;
        let emerald = AntiMagicNeutralPalette::ACCENT_EMERALD;

        match overlay {
            OverlayKind::Volume(val) | OverlayKind::Brightness(val) => {
                let is_vol = matches!(overlay, OverlayKind::Volume(_));
                let label = if is_vol { "VOL" } else { "BRI" };
                self.draw_text_left(
                    label,
                    &self.fmt_caption,
                    D2D_RECT_F {
                        left: pill.left + 16.0,
                        top: pill.top,
                        right: pill.left + 56.0,
                        bottom: pill.bottom,
                    },
                    p_col,
                );

                // Progress bar di tengah
                let bar_l = pill.left + 56.0;
                let bar_r = pill.right - 48.0;
                let bar_cy = (pill.top + pill.bottom) * 0.5;
                if let Ok(tb) = self.rt.CreateSolidColorBrush(
                    &D2D1_COLOR_F {
                        r: 0.20,
                        g: 0.25,
                        b: 0.35,
                        a: 0.7,
                    },
                    None,
                ) {
                    self.rt.FillRoundedRectangle(
                        &D2D1_ROUNDED_RECT {
                            rect: D2D_RECT_F {
                                left: bar_l,
                                top: bar_cy - 3.0,
                                right: bar_r,
                                bottom: bar_cy + 3.0,
                            },
                            radiusX: 3.0,
                            radiusY: 3.0,
                        },
                        &tb,
                    );
                }
                if let Ok(fb) = self.rt.CreateSolidColorBrush(&accent, None) {
                    let fill_w = (bar_r - bar_l) * ((*val as f32 / 100.0).clamp(0.0, 1.0));
                    self.rt.FillRoundedRectangle(
                        &D2D1_ROUNDED_RECT {
                            rect: D2D_RECT_F {
                                left: bar_l,
                                top: bar_cy - 3.0,
                                right: bar_l + fill_w.max(4.0),
                                bottom: bar_cy + 3.0,
                            },
                            radiusX: 3.0,
                            radiusY: 3.0,
                        },
                        &fb,
                    );
                }

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
                self.draw_text_centered(txt, &self.fmt_title, pill, if *on { accent } else { p_col });
            }
            OverlayKind::Charging { percent, plugged } => {
                let txt = if *plugged {
                    format!("⚡ Charging · {}%", percent)
                } else {
                    format!("Battery · {}%", percent)
                };
                self.draw_text_centered(
                    &txt,
                    &self.fmt_title,
                    pill,
                    if *plugged { emerald } else { p_col },
                );
            }
            OverlayKind::Headphones { name, percent } => {
                let txt = format!("🎧 {} · {}%", name, percent);
                self.draw_text_centered(&txt, &self.fmt_title, pill, accent);
            }
        }
    }

    unsafe fn draw_expanded_panel_shell(
        &self,
        state: NotchState,
        pill_rect: D2D_RECT_F,
        alpha: f32,
    ) -> D2D_RECT_F {
        let tabs = ["Music", "Lyrics", "Focus", "Water", "Shelf", "To-Do"];
        let active_idx = match state {
            NotchState::Music => 0,
            NotchState::Lyrics => 1,
            NotchState::Pomo => 2,
            NotchState::Water => 3,
            NotchState::Shelf => 4,
            NotchState::Todo => 5,
            _ => 0,
        };

        let inner_left = pill_rect.left + 14.0;
        let inner_right = pill_rect.right - 14.0;
        let slot_w = (inner_right - inner_left) / 6.0;

        for (i, label) in tabs.iter().enumerate() {
            let slot_left = inner_left + slot_w * i as f32;
            let slot_right = slot_left + slot_w;
            let cx = (slot_left + slot_right) * 0.5;

            let is_active = i == active_idx;
            let mut text_col = if is_active {
                AntiMagicNeutralPalette::TEXT_PRIMARY
            } else {
                AntiMagicNeutralPalette::TEXT_SECONDARY
            };
            text_col.a *= if is_active { alpha } else { alpha * 0.75 };

            self.draw_text_centered(
                label,
                &self.fmt_caption,
                D2D_RECT_F {
                    left: slot_left,
                    top: pill_rect.top + 3.0,
                    right: slot_right,
                    bottom: pill_rect.top + 21.0,
                },
                text_col,
            );

            let line_col = if is_active {
                let mut c = AntiMagicNeutralPalette::ACCENT_BLUE;
                c.a *= alpha;
                c
            } else {
                D2D1_COLOR_F {
                    r: 0.30,
                    g: 0.36,
                    b: 0.48,
                    a: 0.30 * alpha,
                }
            };

            if let Ok(brush) = self.rt.CreateSolidColorBrush(&line_col, None) {
                let half_len = if is_active { 16.0 } else { 6.0 };
                self.rt.DrawLine(
                    D2D_POINT_2F {
                        x: cx - half_len,
                        y: pill_rect.top + 23.0,
                    },
                    D2D_POINT_2F {
                        x: cx + half_len,
                        y: pill_rect.top + 23.0,
                    },
                    &brush,
                    2.2,
                    None,
                );
            }
        }

        let card_rect = D2D_RECT_F {
            left: inner_left,
            top: pill_rect.top + 28.0,
            right: inner_right,
            bottom: pill_rect.bottom - 10.0,
        };

        let mut card_color = AntiMagicNeutralPalette::CARD_SURFACE;
        card_color.a *= alpha;
        if let Ok(card_brush) = self.rt.CreateSolidColorBrush(&card_color, None) {
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: card_rect,
                    radiusX: 14.0,
                    radiusY: 14.0,
                },
                &card_brush,
            );
        }

        card_rect
    }

    unsafe fn draw_music_panel(
        &self,
        ctrl: &NotchController,
        card: D2D_RECT_F,
        alpha: f32,
    ) {
        let art_rect = D2D_RECT_F {
            left: card.left + 12.0,
            top: card.top + 12.0,
            right: card.left + 92.0,
            bottom: card.top + 92.0,
        };
        let mut art_bg = AntiMagicNeutralPalette::CARD_ELEVATED;
        art_bg.a *= alpha;
        if let Ok(b) = self.rt.CreateSolidColorBrush(&art_bg, None) {
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: art_rect,
                    radiusX: 12.0,
                    radiusY: 12.0,
                },
                &b,
            );
        }

        let mut eq_col = AntiMagicNeutralPalette::ACCENT_BLUE;
        eq_col.a *= alpha;
        if let Ok(eq_brush) = self.rt.CreateSolidColorBrush(&eq_col, None) {
            for i in 0..5 {
                let phase = ctrl.anim_time * 5.5 + i as f32 * 1.1;
                let h = if ctrl.media_playing {
                    10.0 + (phase.sin().abs() * 25.0)
                } else {
                    6.0
                };
                let bx = art_rect.left + 17.0 + i as f32 * 10.0;
                let by = art_rect.bottom - 16.0;
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
                    &eq_brush,
                );
            }
        }

        let text_left = art_rect.right + 14.0;
        let text_right = card.right - 56.0;
        let mut p_col = AntiMagicNeutralPalette::TEXT_PRIMARY;
        p_col.a *= alpha;
        let mut s_col = AntiMagicNeutralPalette::TEXT_SECONDARY;
        s_col.a *= alpha;

        self.draw_text_left(
            &ctrl.media_title,
            &self.fmt_title,
            D2D_RECT_F {
                left: text_left,
                top: card.top + 10.0,
                right: text_right,
                bottom: card.top + 31.0,
            },
            p_col,
        );
        self.draw_text_left(
            &ctrl.media_artist,
            &self.fmt_body,
            D2D_RECT_F {
                left: text_left,
                top: card.top + 30.0,
                right: text_right,
                bottom: card.top + 48.0,
            },
            s_col,
        );

        let bar_top = card.top + 56.0;
        let bar_w = (text_right - text_left).max(10.0);
        if let Ok(track_b) = self.rt.CreateSolidColorBrush(
            &D2D1_COLOR_F {
                r: 0.20,
                g: 0.25,
                b: 0.35,
                a: 0.6 * alpha,
            },
            None,
        ) {
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: D2D_RECT_F {
                        left: text_left,
                        top: bar_top,
                        right: text_right,
                        bottom: bar_top + 5.0,
                    },
                    radiusX: 2.5,
                    radiusY: 2.5,
                },
                &track_b,
            );
        }

        if let Ok(fill_b) = self.rt.CreateSolidColorBrush(&eq_col, None) {
            let prog_w = (bar_w * ctrl.media_progress.clamp(0.0, 1.0)).max(5.0);
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: D2D_RECT_F {
                        left: text_left,
                        top: bar_top,
                        right: text_left + prog_w,
                        bottom: bar_top + 5.0,
                    },
                    radiusX: 2.5,
                    radiusY: 2.5,
                },
                &fill_b,
            );
        }

        let btn_y = card.top + 82.0;
        let controls = [
            (text_left + 16.0, "|<<"),
            (text_left + 58.0, if ctrl.media_playing { "||" } else { "|>" }),
            (text_left + 100.0, ">>|"),
        ];

        for (cx, sym) in controls {
            let r_rect = D2D_RECT_F {
                left: cx - 16.0,
                top: btn_y - 11.0,
                right: cx + 16.0,
                bottom: btn_y + 11.0,
            };
            if let Ok(b) = self.rt.CreateSolidColorBrush(&art_bg, None) {
                self.rt.FillRoundedRectangle(
                    &D2D1_ROUNDED_RECT {
                        rect: r_rect,
                        radiusX: 8.0,
                        radiusY: 8.0,
                    },
                    &b,
                );
            }
            self.draw_text_centered(sym, &self.fmt_caption, r_rect, p_col);
        }

        let lyr_rect = D2D_RECT_F {
            left: text_left + 132.0,
            top: btn_y - 11.0,
            right: text_left + 190.0,
            bottom: btn_y + 11.0,
        };
        if let Ok(b) = self.rt.CreateSolidColorBrush(&art_bg, None) {
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: lyr_rect,
                    radiusX: 8.0,
                    radiusY: 8.0,
                },
                &b,
            );
        }
        self.draw_text_centered("Lyrics", &self.fmt_caption, lyr_rect, eq_col);
    }

    unsafe fn draw_lyrics_panel(
        &self,
        ctrl: &NotchController,
        card: D2D_RECT_F,
        alpha: f32,
    ) {
        let mut p_col = AntiMagicNeutralPalette::TEXT_PRIMARY;
        p_col.a *= alpha;
        let mut s_col = AntiMagicNeutralPalette::TEXT_SECONDARY;
        s_col.a *= alpha * 0.65;
        let mut accent = AntiMagicNeutralPalette::ACCENT_BLUE;
        accent.a *= alpha;

        let header = format!("{} — {}", ctrl.media_title, ctrl.media_artist);
        self.draw_text_centered(
            &header,
            &self.fmt_caption,
            D2D_RECT_F {
                left: card.left + 14.0,
                top: card.top + 6.0,
                right: card.right - 14.0,
                bottom: card.top + 24.0,
            },
            accent,
        );

        if ctrl.lyrics_lines.is_empty() {
            self.draw_text_centered(
                "Searching synced lyrics on LRCLIB...",
                &self.fmt_body,
                card,
                s_col,
            );
            return;
        }

        let idx = ctrl.active_lyric_idx.min(ctrl.lyrics_lines.len() - 1);
        let prev_line = if idx > 0 {
            ctrl.lyrics_lines[idx - 1].as_str()
        } else {
            "· · ·"
        };
        let curr_line = ctrl.lyrics_lines[idx].as_str();
        let next_line = if idx + 1 < ctrl.lyrics_lines.len() {
            ctrl.lyrics_lines[idx + 1].as_str()
        } else {
            "· · ·"
        };

        self.draw_text_centered(
            prev_line,
            &self.fmt_body,
            D2D_RECT_F {
                left: card.left + 16.0,
                top: card.top + 26.0,
                right: card.right - 16.0,
                bottom: card.top + 48.0,
            },
            s_col,
        );
        self.draw_text_centered(
            curr_line,
            &self.fmt_title,
            D2D_RECT_F {
                left: card.left + 12.0,
                top: card.top + 48.0,
                right: card.right - 12.0,
                bottom: card.top + 78.0,
            },
            p_col,
        );
        self.draw_text_centered(
            next_line,
            &self.fmt_body,
            D2D_RECT_F {
                left: card.left + 16.0,
                top: card.top + 78.0,
                right: card.right - 16.0,
                bottom: card.top + 100.0,
            },
            s_col,
        );
    }

    unsafe fn draw_pomo_panel(
        &self,
        ctrl: &NotchController,
        card: D2D_RECT_F,
        alpha: f32,
    ) {
        let mut p_col = AntiMagicNeutralPalette::TEXT_PRIMARY;
        p_col.a *= alpha;
        let mut s_col = AntiMagicNeutralPalette::TEXT_SECONDARY;
        s_col.a *= alpha;
        let mut accent = if ctrl.pomo_is_break {
            AntiMagicNeutralPalette::ACCENT_WINE
        } else {
            AntiMagicNeutralPalette::ACCENT_BLUE
        };
        accent.a *= alpha;

        let ring_cx = card.left + 56.0;
        let ring_cy = (card.top + card.bottom) * 0.5;
        let ring_r = 36.0;

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
                3.5,
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
                left: ring_cx - 32.0,
                top: ring_cy - 14.0,
                right: ring_cx + 32.0,
                bottom: ring_cy + 14.0,
            },
            p_col,
        );

        let right_x = card.left + 112.0;
        self.draw_text_left(
            if ctrl.pomo_is_break {
                "SHORT BREAK · REST EYES"
            } else {
                "POMODORO · FOCUS SESSION"
            },
            &self.fmt_title,
            D2D_RECT_F {
                left: right_x,
                top: card.top + 12.0,
                right: card.right - 50.0,
                bottom: card.top + 36.0,
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
                top: card.top + 36.0,
                right: card.right - 50.0,
                bottom: card.top + 58.0,
            },
            s_col,
        );

        let start_rect = D2D_RECT_F {
            left: right_x,
            top: card.top + 64.0,
            right: right_x + 86.0,
            bottom: card.top + 92.0,
        };
        if let Ok(b) = self.rt.CreateSolidColorBrush(&accent, None) {
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: start_rect,
                    radiusX: 8.0,
                    radiusY: 8.0,
                },
                &b,
            );
        }
        self.draw_text_centered(
            if ctrl.pomo_running { "Pause" } else { "Start Focus" },
            &self.fmt_caption,
            start_rect,
            p_col,
        );

        let reset_rect = D2D_RECT_F {
            left: right_x + 94.0,
            top: card.top + 64.0,
            right: right_x + 158.0,
            bottom: card.top + 92.0,
        };
        let mut elev = AntiMagicNeutralPalette::CARD_ELEVATED;
        elev.a *= alpha;
        if let Ok(b) = self.rt.CreateSolidColorBrush(&elev, None) {
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: reset_rect,
                    radiusX: 8.0,
                    radiusY: 8.0,
                },
                &b,
            );
        }
        self.draw_text_centered("Reset", &self.fmt_caption, reset_rect, s_col);
    }

    unsafe fn draw_water_panel(
        &self,
        ctrl: &NotchController,
        card: D2D_RECT_F,
        alpha: f32,
    ) {
        let mut p_col = AntiMagicNeutralPalette::TEXT_PRIMARY;
        p_col.a *= alpha;
        let mut accent = AntiMagicNeutralPalette::ACCENT_BLUE;
        accent.a *= alpha;

        let pct = ((ctrl.water_current_ml as f32 / ctrl.water_goal_ml.max(1) as f32) * 100.0)
            .round() as u32;
        let header = format!(
            "HYDRATION · {:.2} / {:.2} L ({}%)",
            ctrl.water_current_ml as f32 / 1000.0,
            ctrl.water_goal_ml as f32 / 1000.0,
            pct
        );

        self.draw_text_left(
            &header,
            &self.fmt_title,
            D2D_RECT_F {
                left: card.left + 16.0,
                top: card.top + 10.0,
                right: card.right - 16.0,
                bottom: card.top + 34.0,
            },
            p_col,
        );

        let chart_left = card.left + 18.0;
        let chart_bottom = card.bottom - 14.0;
        if let Ok(bar_b) = self.rt.CreateSolidColorBrush(&accent, None) {
            for (i, &val) in ctrl.water_hourly_ml.iter().enumerate() {
                let h = ((val as f32 / 500.0) * 42.0).clamp(6.0, 44.0);
                let bx = chart_left + i as f32 * 28.0;
                self.rt.FillRoundedRectangle(
                    &D2D1_ROUNDED_RECT {
                        rect: D2D_RECT_F {
                            left: bx,
                            top: chart_bottom - h,
                            right: bx + 16.0,
                            bottom: chart_bottom,
                        },
                        radiusX: 4.0,
                        radiusY: 4.0,
                    },
                    &bar_b,
                );
            }
        }

        let btn_rect = D2D_RECT_F {
            left: card.right - 148.0,
            top: card.top + 44.0,
            right: card.right - 18.0,
            bottom: card.top + 82.0,
        };
        if let Ok(b) = self.rt.CreateSolidColorBrush(&accent, None) {
            self.rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: btn_rect,
                    radiusX: 10.0,
                    radiusY: 10.0,
                },
                &b,
            );
        }
        self.draw_text_centered(
            &format!("+ Drink {} ml", ctrl.water_cup_ml),
            &self.fmt_title,
            btn_rect,
            p_col,
        );
    }

    unsafe fn draw_shelf_panel(
        &self,
        ctrl: &NotchController,
        card: D2D_RECT_F,
        alpha: f32,
    ) {
        let mut p_col = AntiMagicNeutralPalette::TEXT_PRIMARY;
        p_col.a *= alpha;
        let mut s_col = AntiMagicNeutralPalette::TEXT_SECONDARY;
        s_col.a *= alpha;
        let mut elev = AntiMagicNeutralPalette::CARD_ELEVATED;
        elev.a *= alpha;

        self.draw_text_left(
            "FILE SHELF · DRAG FILES HERE OR CLICK TO OPEN",
            &self.fmt_caption,
            D2D_RECT_F {
                left: card.left + 14.0,
                top: card.top + 6.0,
                right: card.right - 50.0,
                bottom: card.top + 24.0,
            },
            s_col,
        );

        for (i, (name, meta)) in ctrl.shelf_files.iter().take(2).enumerate() {
            let item_w = (card.right - card.left - 82.0) * 0.5;
            let ix = card.left + 14.0 + i as f32 * (item_w + 10.0);
            let item_rect = D2D_RECT_F {
                left: ix,
                top: card.top + 28.0,
                right: ix + item_w,
                bottom: card.bottom - 12.0,
            };

            if let Ok(b) = self.rt.CreateSolidColorBrush(&elev, None) {
                self.rt.FillRoundedRectangle(
                    &D2D1_ROUNDED_RECT {
                        rect: item_rect,
                        radiusX: 10.0,
                        radiusY: 10.0,
                    },
                    &b,
                );
            }

            self.draw_text_left(
                name,
                &self.fmt_title,
                D2D_RECT_F {
                    left: item_rect.left + 10.0,
                    top: item_rect.top + 10.0,
                    right: item_rect.right - 8.0,
                    bottom: item_rect.top + 34.0,
                },
                p_col,
            );
            self.draw_text_left(
                meta,
                &self.fmt_caption,
                D2D_RECT_F {
                    left: item_rect.left + 10.0,
                    top: item_rect.top + 36.0,
                    right: item_rect.right - 8.0,
                    bottom: item_rect.top + 56.0,
                },
                s_col,
            );
        }
    }

    unsafe fn draw_todo_panel(
        &self,
        ctrl: &NotchController,
        card: D2D_RECT_F,
        alpha: f32,
    ) {
        let mut p_col = AntiMagicNeutralPalette::TEXT_PRIMARY;
        p_col.a *= alpha;
        let mut s_col = AntiMagicNeutralPalette::TEXT_SECONDARY;
        s_col.a *= alpha;
        let mut accent = AntiMagicNeutralPalette::ACCENT_BLUE;
        accent.a *= alpha;

        for (i, (task, done)) in ctrl.todos.iter().take(3).enumerate() {
            let row_top = card.top + 10.0 + i as f32 * 32.0;
            let box_rect = D2D_RECT_F {
                left: card.left + 16.0,
                top: row_top + 4.0,
                right: card.left + 32.0,
                bottom: row_top + 20.0,
            };

            if let Ok(b) = self.rt.CreateSolidColorBrush(&accent, None) {
                let rbox = D2D1_ROUNDED_RECT {
                    rect: box_rect,
                    radiusX: 4.0,
                    radiusY: 4.0,
                };
                if *done {
                    self.rt.FillRoundedRectangle(&rbox, &b);
                } else {
                    self.rt.DrawRoundedRectangle(&rbox, &b, 1.5, None);
                }
            }

            self.draw_text_left(
                task,
                &self.fmt_body,
                D2D_RECT_F {
                    left: card.left + 42.0,
                    top: row_top,
                    right: card.right - 16.0,
                    bottom: row_top + 25.0,
                },
                if *done { s_col } else { p_col },
            );
        }
    }
}