// src/mascot/core.rs
// Trait karakter idle + primitif Direct2D bersama (mata kapsul, glow, petir, uap).
use windows::Win32::Graphics::Direct2D::Common::{
    D2D1_COLOR_F, D2D1_FIGURE_BEGIN_FILLED, D2D1_FIGURE_BEGIN_HOLLOW, D2D1_FIGURE_END_CLOSED,
    D2D1_FIGURE_END_OPEN, D2D1_GRADIENT_STOP, D2D_POINT_2F, D2D_RECT_F,
};
use windows::Win32::Graphics::Direct2D::{
    ID2D1Factory, ID2D1HwndRenderTarget, ID2D1StrokeStyle, D2D1_CAP_STYLE_ROUND, D2D1_ELLIPSE,
    D2D1_EXTEND_MODE_CLAMP, D2D1_GAMMA_2_2, D2D1_LINE_JOIN_ROUND,
    D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES, D2D1_ROUNDED_RECT, D2D1_STROKE_STYLE_PROPERTIES,
};

/// Konteks animasi global yang dibagikan ke karakter aktif setiap frame.
#[derive(Clone, Copy, Debug)]
pub struct MascotContext {
    pub phase_time: f32,
    pub local_time: f32,
    pub duration: f32,
    pub blink_progress: f32,
    pub look_x: f32,
    pub look_y: f32,
    pub flare_intensity: f32,
}

impl MascotContext {
    pub fn open_factor(&self) -> f32 {
        (1.0 - self.blink_progress).clamp(0.08, 1.0)
    }
}

/// Trait modular: satu karakter idle = satu file.
pub trait IdleCharacter: Send + Sync {
    #[allow(dead_code)]
    fn id(&self) -> &'static str;
    fn duration_secs(&self) -> f32 {
        10.5
    }

    /// Override target lirikan mata bila karakter sedang fokus ke gestur kanan.
    fn override_look_target(&self, _ctx: &MascotContext) -> Option<(f32, f32)> {
        None
    }

    unsafe fn draw_aura(
        &self,
        rt: &ID2D1HwndRenderTarget,
        factory: &ID2D1Factory,
        pill_rect: D2D_RECT_F,
        alpha: f32,
        ctx: &MascotContext,
    );

    unsafe fn draw_character(
        &self,
        rt: &ID2D1HwndRenderTarget,
        factory: &ID2D1Factory,
        cx: f32,
        cy: f32,
        alpha: f32,
        ctx: &MascotContext,
    );
}

pub fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t.clamp(0.0, 1.0)
}

pub fn lerp_color(a: D2D1_COLOR_F, b: D2D1_COLOR_F, t: f32) -> D2D1_COLOR_F {
    let k = t.clamp(0.0, 1.0);
    D2D1_COLOR_F {
        r: lerp(a.r, b.r, k),
        g: lerp(a.g, b.g, k),
        b: lerp(a.b, b.b, k),
        a: lerp(a.a, b.a, k),
    }
}

pub unsafe fn create_round_stroke(factory: &ID2D1Factory) -> Option<ID2D1StrokeStyle> {
    let props = D2D1_STROKE_STYLE_PROPERTIES {
        startCap: D2D1_CAP_STYLE_ROUND,
        endCap: D2D1_CAP_STYLE_ROUND,
        dashCap: D2D1_CAP_STYLE_ROUND,
        lineJoin: D2D1_LINE_JOIN_ROUND,
        miterLimit: 10.0,
        ..Default::default()
    };
    factory.CreateStrokeStyle(&props, None).ok()
}

/// Iluminasi radial lembut tanpa ring kasar.
pub unsafe fn draw_soft_radial_glow(
    rt: &ID2D1HwndRenderTarget,
    center: D2D_POINT_2F,
    radius_x: f32,
    radius_y: f32,
    r: f32,
    g: f32,
    b: f32,
    peak_alpha: f32,
) {
    if peak_alpha <= 0.005 || radius_x <= 0.5 || radius_y <= 0.5 {
        return;
    }
    let stops = [
        D2D1_GRADIENT_STOP {
            position: 0.0,
            color: D2D1_COLOR_F { r, g, b, a: peak_alpha },
        },
        D2D1_GRADIENT_STOP {
            position: 0.42,
            color: D2D1_COLOR_F {
                r,
                g,
                b,
                a: peak_alpha * 0.46,
            },
        },
        D2D1_GRADIENT_STOP {
            position: 0.76,
            color: D2D1_COLOR_F {
                r: r * 0.7,
                g: g * 0.7,
                b: b * 0.7,
                a: peak_alpha * 0.14,
            },
        },
        D2D1_GRADIENT_STOP {
            position: 1.0,
            color: D2D1_COLOR_F {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            },
        },
    ];

    if let Ok(stop_collection) =
        rt.CreateGradientStopCollection(&stops, D2D1_GAMMA_2_2, D2D1_EXTEND_MODE_CLAMP)
    {
        let props = D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES {
            center,
            gradientOriginOffset: D2D_POINT_2F { x: 0.0, y: 0.0 },
            radiusX: radius_x,
            radiusY: radius_y,
        };
        if let Ok(rad_brush) = rt.CreateRadialGradientBrush(&props, None, &stop_collection) {
            rt.FillRectangle(
                &D2D_RECT_F {
                    left: center.x - radius_x,
                    top: center.y - radius_y,
                    right: center.x + radius_x,
                    bottom: center.y + radius_y,
                },
                &rad_brush,
            );
        }
    }
}

/// Sepasang mata kapsul bercahaya + masker kelopak miring saat ekspresi serius.
pub unsafe fn draw_capsule_eyes(
    rt: &ID2D1HwndRenderTarget,
    factory: &ID2D1Factory,
    face_cx: f32,
    eye_y: f32,
    eye_spacing: f32,
    outer_col: D2D1_COLOR_F,
    core_col: D2D1_COLOR_F,
    glow_rgb: (f32, f32, f32),
    serious: f32,
    alpha: f32,
    ctx: &MascotContext,
) {
    let open = ctx.open_factor();
    let serious = serious.clamp(0.0, 1.0);
    let squash = 1.0 - 0.22 * serious;
    let rx = 3.9_f32;
    let ry = 5.2_f32 * open * squash;
    let irx = 2.5_f32;
    let iry = 3.6_f32 * open * squash;

    let Ok(outer_b) = rt.CreateSolidColorBrush(
        &D2D1_COLOR_F {
            a: outer_col.a * alpha,
            ..outer_col
        },
        None,
    ) else {
        return;
    };
    let Ok(core_b) = rt.CreateSolidColorBrush(
        &D2D1_COLOR_F {
            a: core_col.a * alpha,
            ..core_col
        },
        None,
    ) else {
        return;
    };
    let mask_b = rt
        .CreateSolidColorBrush(
            &D2D1_COLOR_F {
                r: 0.016,
                g: 0.016,
                b: 0.022,
                a: alpha,
            },
            None,
        )
        .ok();

    for side in [-1.0_f32, 1.0_f32] {
        let ex = face_cx + side * eye_spacing + ctx.look_x * 0.65;
        let ey = eye_y + ctx.look_y * 0.45;

        draw_soft_radial_glow(
            rt,
            D2D_POINT_2F { x: ex, y: ey },
            11.5 * ctx.flare_intensity.min(1.5),
            12.0 * ctx.flare_intensity.min(1.5),
            glow_rgb.0,
            glow_rgb.1,
            glow_rgb.2,
            (0.58 * alpha * ctx.flare_intensity).min(0.85),
        );

        rt.FillRoundedRectangle(
            &D2D1_ROUNDED_RECT {
                rect: D2D_RECT_F {
                    left: ex - rx,
                    top: ey - ry,
                    right: ex + rx,
                    bottom: ey + ry,
                },
                radiusX: rx,
                radiusY: ry,
            },
            &outer_b,
        );

        rt.FillRoundedRectangle(
            &D2D1_ROUNDED_RECT {
                rect: D2D_RECT_F {
                    left: ex - irx,
                    top: ey - iry,
                    right: ex + irx,
                    bottom: ey + iry,
                },
                radiusX: irx,
                radiusY: iry,
            },
            &core_b,
        );

        // Kelopak atas miring: `\` untuk mata kiri, `/` untuk mata kanan.
        if serious > 0.05 {
            if let (Some(mb), Ok(path)) = (&mask_b, factory.CreatePathGeometry()) {
                if let Ok(sink) = path.Open() {
                    let inner_drop = 3.5 * serious * open;
                    let outer_drop = 0.6 * serious * open;
                    let top_y = ey - ry - 1.2;

                    sink.BeginFigure(
                        D2D_POINT_2F {
                            x: ex - rx - 1.0,
                            y: top_y,
                        },
                        D2D1_FIGURE_BEGIN_FILLED,
                    );
                    if side < 0.0 {
                        let _ = sink.AddLines(&[
                            D2D_POINT_2F {
                                x: ex + rx + 1.0,
                                y: top_y,
                            },
                            D2D_POINT_2F {
                                x: ex + rx + 1.0,
                                y: ey - ry + inner_drop,
                            },
                            D2D_POINT_2F {
                                x: ex - rx - 1.0,
                                y: ey - ry + outer_drop,
                            },
                        ]);
                    } else {
                        let _ = sink.AddLines(&[
                            D2D_POINT_2F {
                                x: ex + rx + 1.0,
                                y: top_y,
                            },
                            D2D_POINT_2F {
                                x: ex + rx + 1.0,
                                y: ey - ry + outer_drop,
                            },
                            D2D_POINT_2F {
                                x: ex - rx - 1.0,
                                y: ey - ry + inner_drop,
                            },
                        ]);
                    }
                    sink.EndFigure(D2D1_FIGURE_END_CLOSED);
                    let _ = sink.Close();
                    rt.FillGeometry(&path, mb, None);
                }
            }
        }
    }
}

/// Poligon tertutup sederhana dari irisan titik.
pub unsafe fn fill_polygon(
    rt: &ID2D1HwndRenderTarget,
    factory: &ID2D1Factory,
    pts: &[D2D_POINT_2F],
    color: D2D1_COLOR_F,
) {
    if pts.len() < 3 || color.a <= 0.004 {
        return;
    }
    if let (Ok(brush), Ok(path)) = (
        rt.CreateSolidColorBrush(&color, None),
        factory.CreatePathGeometry(),
    ) {
        if let Ok(sink) = path.Open() {
            sink.BeginFigure(pts[0], D2D1_FIGURE_BEGIN_FILLED);
            let _ = sink.AddLines(&pts[1..]);
            sink.EndFigure(D2D1_FIGURE_END_CLOSED);
            let _ = sink.Close();
            rt.FillGeometry(&path, &brush, None);
        }
    }
}

/// Kilatan petir dua lapis (outer glow + core terang).
pub unsafe fn draw_lightning_bolt(
    rt: &ID2D1HwndRenderTarget,
    factory: &ID2D1Factory,
    pts: &[D2D_POINT_2F],
    glow_col: D2D1_COLOR_F,
    core_col: D2D1_COLOR_F,
    width: f32,
) {
    if pts.len() < 2 || core_col.a <= 0.005 {
        return;
    }
    let stroke = create_round_stroke(factory);
    if let (Ok(glow_b), Ok(core_b), Ok(path)) = (
        rt.CreateSolidColorBrush(&glow_col, None),
        rt.CreateSolidColorBrush(&core_col, None),
        factory.CreatePathGeometry(),
    ) {
        if let Ok(sink) = path.Open() {
            sink.BeginFigure(pts[0], D2D1_FIGURE_BEGIN_HOLLOW);
            let _ = sink.AddLines(&pts[1..]);
            sink.EndFigure(D2D1_FIGURE_END_OPEN);
            let _ = sink.Close();
            rt.DrawGeometry(&path, &glow_b, width * 2.8, stroke.as_ref());
            rt.DrawGeometry(&path, &core_b, width, stroke.as_ref());
        }
    }
}

/// Lingkaran lembut (partikel api, uap, telapak tangan).
pub unsafe fn draw_soft_circle(
    rt: &ID2D1HwndRenderTarget,
    center: D2D_POINT_2F,
    rx: f32,
    ry: f32,
    color: D2D1_COLOR_F,
) {
    if color.a <= 0.004 {
        return;
    }
    if let Ok(b) = rt.CreateSolidColorBrush(&color, None) {
        rt.FillEllipse(
            &D2D1_ELLIPSE {
                point: center,
                radiusX: rx,
                radiusY: ry,
            },
            &b,
        );
    }
}
