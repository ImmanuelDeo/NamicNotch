// src/mascot/core.rs
// Trait karakter idle + primitif Direct2D bersama (mata kapsul, glow, petir, uap).
use windows::Win32::Graphics::Direct2D::Common::{
    D2D1_BEZIER_SEGMENT, D2D1_COLOR_F, D2D1_FIGURE_BEGIN_FILLED, D2D1_FIGURE_BEGIN_HOLLOW,
    D2D1_FIGURE_END_CLOSED, D2D1_FIGURE_END_OPEN, D2D1_GRADIENT_STOP, D2D_POINT_2F, D2D_RECT_F,
};
use windows::Win32::Graphics::Direct2D::{
    ID2D1Factory, ID2D1HwndRenderTarget, ID2D1PathGeometry, ID2D1StrokeStyle,
    D2D1_CAP_STYLE_ROUND, D2D1_ELLIPSE, D2D1_EXTEND_MODE_CLAMP,
    D2D1_GAMMA_2_2, D2D1_LINE_JOIN_ROUND,
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

/// Garis polyline dengan ujung membulat (outline rambut, bilah pedang, retakan).
pub unsafe fn draw_polyline(
    rt: &ID2D1HwndRenderTarget,
    factory: &ID2D1Factory,
    pts: &[D2D_POINT_2F],
    color: D2D1_COLOR_F,
    width: f32,
) {
    if pts.len() < 2 || color.a <= 0.004 {
        return;
    }
    let stroke = create_round_stroke(factory);
    if let (Ok(brush), Ok(path)) = (
        rt.CreateSolidColorBrush(&color, None),
        factory.CreatePathGeometry(),
    ) {
        if let Ok(sink) = path.Open() {
            sink.BeginFigure(pts[0], D2D1_FIGURE_BEGIN_HOLLOW);
            let _ = sink.AddLines(&pts[1..]);
            sink.EndFigure(D2D1_FIGURE_END_OPEN);
            let _ = sink.Close();
            rt.DrawGeometry(&path, &brush, width, stroke.as_ref());
        }
    }
}

/// Berkas sinar melingkar dari sumber cahaya di belakang karakter.
/// Dipakai untuk aura iluminasi latar (crimson di Asta, emas di Eren, dst).
pub unsafe fn draw_back_rays(
    rt: &ID2D1HwndRenderTarget,
    factory: &ID2D1Factory,
    cx: f32,
    base_y: f32,
    count: usize,
    spread: f32,
    length: f32,
    phase: f32,
    color: D2D1_COLOR_F,
) {
    if color.a <= 0.006 || count == 0 {
        return;
    }
    for i in 0..count {
        let k = i as f32;
        let norm = if count > 1 {
            k / (count - 1) as f32 * 2.0 - 1.0
        } else {
            0.0
        };
        let jitter = (phase * 1.7 + k * 1.13).sin() * 0.5 + 0.5;
        let angle = norm * spread + (phase * 0.6 + k).sin() * 0.03;
        let len = length * (0.55 + 0.45 * jitter);
        let half_w = (0.55 + 0.55 * jitter) * 1.2;

        let tip_x = cx + angle.sin() * len;
        let tip_y = base_y - angle.cos() * len;
        let bx = cx + norm * half_w * 6.0;

        let wedge = [
            D2D_POINT_2F { x: bx - half_w, y: base_y },
            D2D_POINT_2F { x: tip_x, y: tip_y },
            D2D_POINT_2F { x: bx + half_w, y: base_y },
        ];
        fill_polygon(rt, factory, &wedge, color);
    }
}

/// Partikel naik (bara api, debu anti-magic, uap) tersebar di area aura.
pub unsafe fn draw_rising_particles(
    rt: &ID2D1HwndRenderTarget,
    cx: f32,
    base_y: f32,
    half_width: f32,
    rise: f32,
    t: f32,
    count: usize,
    color: D2D1_COLOR_F,
    size: f32,
) {
    if color.a <= 0.006 {
        return;
    }
    for i in 0..count {
        let k = i as f32;
        let seed = (k * 12.9898 + 4.1414).sin() * 43758.5453;
        let rnd = seed - seed.floor();
        let rnd2 = ((k + 7.0) * 78.233).sin() * 43758.5453;
        let rnd2 = rnd2 - rnd2.floor();

        let speed = rise * (0.55 + rnd * 0.9);
        let travel = (t * speed + rnd2 * rise) % rise;
        let y = base_y - (rise - travel);
        let sway = (t * (1.1 + rnd) + k).sin() * 2.2;
        let x = cx + (rnd * 2.0 - 1.0) * half_width + sway;

        let fade = (1.0 - (travel / rise)).clamp(0.0, 1.0);
        let sz = size * (0.5 + rnd * 0.8) * (0.45 + 0.55 * fade);
        draw_soft_circle(
            rt,
            D2D_POINT_2F { x, y },
            sz,
            sz,
            D2D1_COLOR_F {
                a: color.a * fade,
                ..color
            },
        );
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

// ── Primitif kurva halus (gaya rambut anime, bukan poligon kaku) ────────────

/// Loop tertutup mulus lewat `pts` (Catmull-Rom → cubic bezier).
unsafe fn smooth_loop_geometry(
    factory: &ID2D1Factory,
    pts: &[D2D_POINT_2F],
) -> Option<ID2D1PathGeometry> {
    if pts.len() < 3 {
        return None;
    }
    let path = factory.CreatePathGeometry().ok()?;
    let sink = path.Open().ok()?;
    let n = pts.len();
    sink.BeginFigure(pts[0], D2D1_FIGURE_BEGIN_FILLED);
    for i in 0..n {
        let p0 = pts[(i + n - 1) % n];
        let p1 = pts[i];
        let p2 = pts[(i + 1) % n];
        let p3 = pts[(i + 2) % n];
        let seg = D2D1_BEZIER_SEGMENT {
            point1: D2D_POINT_2F {
                x: p1.x + (p2.x - p0.x) / 6.0,
                y: p1.y + (p2.y - p0.y) / 6.0,
            },
            point2: D2D_POINT_2F {
                x: p2.x - (p3.x - p1.x) / 6.0,
                y: p2.y - (p3.y - p1.y) / 6.0,
            },
            point3: p2,
        };
        let _ = sink.AddBezier(&seg);
    }
    sink.EndFigure(D2D1_FIGURE_END_CLOSED);
    let _ = sink.Close();
    Some(path)
}

/// Isi siluet mulus; mengembalikan geometry agar bisa di-stroke sebagai rim light.
pub unsafe fn fill_smooth_shape(
    rt: &ID2D1HwndRenderTarget,
    factory: &ID2D1Factory,
    pts: &[D2D_POINT_2F],
    color: D2D1_COLOR_F,
) -> Option<ID2D1PathGeometry> {
    let path = smooth_loop_geometry(factory, pts)?;
    if color.a > 0.004 {
        if let Ok(brush) = rt.CreateSolidColorBrush(&color, None) {
            rt.FillGeometry(&path, &brush, None);
        }
    }
    Some(path)
}

/// Garis mulus terbuka lewat `pts` (highlight rambut, retakan, tali).
pub unsafe fn stroke_smooth_curve(
    rt: &ID2D1HwndRenderTarget,
    factory: &ID2D1Factory,
    pts: &[D2D_POINT_2F],
    color: D2D1_COLOR_F,
    width: f32,
) {
    if pts.len() < 2 || color.a <= 0.004 {
        return;
    }
    let Ok(path) = factory.CreatePathGeometry() else {
        return;
    };
    let Ok(sink) = path.Open() else { return };
    let n = pts.len();
    sink.BeginFigure(pts[0], D2D1_FIGURE_BEGIN_HOLLOW);
    for i in 0..n - 1 {
        let p0 = pts[i.saturating_sub(1)];
        let p1 = pts[i];
        let p2 = pts[i + 1];
        let p3 = pts[(i + 2).min(n - 1)];
        let seg = D2D1_BEZIER_SEGMENT {
            point1: D2D_POINT_2F {
                x: p1.x + (p2.x - p0.x) / 6.0,
                y: p1.y + (p2.y - p0.y) / 6.0,
            },
            point2: D2D_POINT_2F {
                x: p2.x - (p3.x - p1.x) / 6.0,
                y: p2.y - (p3.y - p1.y) / 6.0,
            },
            point3: p2,
        };
        let _ = sink.AddBezier(&seg);
    }
    sink.EndFigure(D2D1_FIGURE_END_OPEN);
    let _ = sink.Close();
    if let Ok(brush) = rt.CreateSolidColorBrush(&color, None) {
        let stroke = create_round_stroke(factory);
        rt.DrawGeometry(&path, &brush, width, stroke.as_ref());
    }
}

/// Satu helai rambut meruncing (pangkal lebar, ujung runcing, melengkung `bend`).
/// `rim` tipis ditumpuk di atas fill supaya helai punya kilau tepi seperti Asta.
pub unsafe fn hair_lock(
    rt: &ID2D1HwndRenderTarget,
    factory: &ID2D1Factory,
    base: D2D_POINT_2F,
    tip: D2D_POINT_2F,
    bend: f32,
    width: f32,
    color: D2D1_COLOR_F,
    rim: D2D1_COLOR_F,
) {
    let dx = tip.x - base.x;
    let dy = tip.y - base.y;
    let len = (dx * dx + dy * dy).sqrt();
    if len < 0.4 {
        return;
    }
    let (ux, uy) = (dx / len, dy / len);
    let (nx, ny) = (-uy, ux);
    let half = width * 0.5;

    let l = D2D_POINT_2F {
        x: base.x + nx * half,
        y: base.y + ny * half,
    };
    let r = D2D_POINT_2F {
        x: base.x - nx * half,
        y: base.y - ny * half,
    };
    // Titik kontrol mengecil di 2/3 jalur → helai melengkung, ujung tajam.
    let mid_l = D2D_POINT_2F {
        x: lerp(l.x, tip.x, 0.45) + nx * bend * 0.55,
        y: lerp(l.y, tip.y, 0.45) + ny * bend * 0.55,
    };
    let mid_r = D2D_POINT_2F {
        x: lerp(r.x, tip.x, 0.45) + nx * bend * 0.55,
        y: lerp(r.y, tip.y, 0.45) + ny * bend * 0.55,
    };

    let Ok(path) = factory.CreatePathGeometry() else {
        return;
    };
    let Ok(sink) = path.Open() else { return };
    sink.BeginFigure(l, D2D1_FIGURE_BEGIN_FILLED);
    let _ = sink.AddBezier(&D2D1_BEZIER_SEGMENT {
        point1: mid_l,
        point2: D2D_POINT_2F {
            x: lerp(l.x, tip.x, 0.88),
            y: lerp(l.y, tip.y, 0.88),
        },
        point3: tip,
    });
    let _ = sink.AddBezier(&D2D1_BEZIER_SEGMENT {
        point1: D2D_POINT_2F {
            x: lerp(r.x, tip.x, 0.88),
            y: lerp(r.y, tip.y, 0.88),
        },
        point2: mid_r,
        point3: r,
    });
    sink.EndFigure(D2D1_FIGURE_END_CLOSED);
    let _ = sink.Close();

    if color.a > 0.004 {
        if let Ok(brush) = rt.CreateSolidColorBrush(&color, None) {
            rt.FillGeometry(&path, &brush, None);
        }
    }
    if rim.a > 0.004 {
        if let Ok(brush) = rt.CreateSolidColorBrush(&rim, None) {
            let stroke = create_round_stroke(factory);
            rt.DrawGeometry(&path, &brush, 0.9, stroke.as_ref());
        }
    }
}
