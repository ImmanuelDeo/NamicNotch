// src/mascot/vfx.rs
// VFX v3.0 — semua efek digambar sebagai pita tertutup dengan lebar meruncing
// ke nol, bukan stroke berlebar konstan. Tidak ada DrawGeometry / DrawLine.
use super::core::*;
use windows::Win32::Graphics::Direct2D::Common::{D2D1_COLOR_F, D2D_POINT_2F, D2D_RECT_F};
use windows::Win32::Graphics::Direct2D::{ID2D1Factory, ID2D1RenderTarget};

fn rgb3(c: (f32, f32, f32), a: f32) -> D2D1_COLOR_F {
    D2D1_COLOR_F {
        r: c.0,
        g: c.1,
        b: c.2,
        a,
    }
}

/// Titik pada kurva Bezier kubik dengan 4 kontrol.
fn bezier(p0: D2D_POINT_2F, p1: D2D_POINT_2F, p2: D2D_POINT_2F, p3: D2D_POINT_2F, t: f32) -> D2D_POINT_2F {
    let u = 1.0 - t;
    let (a, b, c, d) = (u * u * u, 3.0 * u * u * t, 3.0 * u * t * t, t * t * t);
    D2D_POINT_2F {
        x: p0.x * a + p1.x * b + p2.x * c + p3.x * d,
        y: p0.y * a + p1.y * b + p2.y * c + p3.y * d,
    }
}

/// Pita tertutup: spine Bezier, lebar maksimum di awal lalu meruncing ke 0.
pub unsafe fn fill_tapered_ribbon(
    rt: &ID2D1RenderTarget,
    factory: &ID2D1Factory,
    p0: D2D_POINT_2F,
    p1: D2D_POINT_2F,
    p2: D2D_POINT_2F,
    p3: D2D_POINT_2F,
    width: f32,
    color: D2D1_COLOR_F,
    samples: usize,
) {
    if color.a <= 0.004 || width <= 0.05 {
        return;
    }
    let n = samples.max(6);
    let mut left = Vec::with_capacity(n);
    let mut right = Vec::with_capacity(n);
    for i in 0..n {
        let f = i as f32 / (n - 1) as f32;
        let pt = bezier(p0, p1, p2, p3, f);
        let nxt = bezier(p0, p1, p2, p3, (f + 0.02).min(1.0));
        let dx = nxt.x - pt.x;
        let dy = nxt.y - pt.y;
        let len = (dx * dx + dy * dy).sqrt().max(0.001);
        let (nx, ny) = (-dy / len, dx / len);
        // Profil: naik cepat di 15% awal, lalu meruncing tajam ke ujung.
        let profile = if f < 0.15 {
            f / 0.15
        } else {
            let k = (f - 0.15) / 0.85;
            (1.0 - k).powf(1.35)
        };
        let hw = width * 0.5 * profile;
        left.push(D2D_POINT_2F {
            x: pt.x + nx * hw,
            y: pt.y + ny * hw,
        });
        right.push(D2D_POINT_2F {
            x: pt.x - nx * hw,
            y: pt.y - ny * hw,
        });
    }
    let mut poly = left;
    poly.extend(right.into_iter().rev());
    fill_smooth_shape(rt, factory, &poly, color);
}

/// Bloom 6 tingkat eksponensial di atas satu pita.
pub unsafe fn fill_ultra_smooth_ribbon(
    rt: &ID2D1RenderTarget,
    factory: &ID2D1Factory,
    p0: D2D_POINT_2F,
    p1: D2D_POINT_2F,
    p2: D2D_POINT_2F,
    p3: D2D_POINT_2F,
    width: f32,
    color: D2D1_COLOR_F,
    core: bool,
) {
    let tiers = [
        (3.6_f32, 0.10_f32),
        (2.6, 0.16),
        (1.8, 0.24),
        (1.25, 0.34),
        (0.85, 0.55),
        (1.0, 1.0),
    ];
    for (k, (w, a)) in tiers.iter().enumerate() {
        let c = if k == tiers.len() - 1 && core {
            D2D1_COLOR_F {
                r: (color.r + 0.55).min(1.0),
                g: (color.g + 0.55).min(1.0),
                b: (color.b + 0.55).min(1.0),
                a: color.a,
            }
        } else {
            color
        };
        fill_tapered_ribbon(rt, factory, p0, p1, p2, p3, width * w, D2D1_COLOR_F { a: color.a * a, ..c }, 22);
    }
}

/// Iluminasi atmosfer notch: gradien dua tone di badan pill + rim glow tipis.
pub unsafe fn draw_ambient_notch_illumination(
    rt: &ID2D1RenderTarget,
    pill: D2D_RECT_F,
    top_rgb: (f32, f32, f32),
    bottom_rgb: (f32, f32, f32),
    accent_rgb: (f32, f32, f32),
    intensity: f32,
    alpha: f32,
) {
    let pw = pill.right - pill.left;
    let ph = pill.bottom - pill.top;
    let cx = (pill.left + pill.right) * 0.5;

    draw_bottom_gradient(
        rt,
        pill,
        rgb3(bottom_rgb, alpha),
        ph * 0.72,
        (0.30 + 0.26 * intensity) * alpha,
    );

    draw_soft_radial_glow(
        rt,
        D2D_POINT_2F {
            x: cx,
            y: pill.top + ph * 0.46,
        },
        pw * 0.34,
        ph * 0.62,
        top_rgb.0,
        top_rgb.1,
        top_rgb.2,
        (0.14 + 0.12 * intensity) * alpha,
    );

    // Rim glow: garis cahaya tipis di tepi dalam pill.
    draw_soft_radial_glow(
        rt,
        D2D_POINT_2F {
            x: cx,
            y: pill.bottom - 1.0,
        },
        pw * 0.46,
        2.6,
        accent_rgb.0,
        accent_rgb.1,
        accent_rgb.2,
        (0.26 + 0.18 * intensity) * alpha,
    );
    draw_soft_radial_glow(
        rt,
        D2D_POINT_2F {
            x: cx,
            y: pill.top + 1.5,
        },
        pw * 0.30,
        1.8,
        top_rgb.0,
        top_rgb.1,
        top_rgb.2,
        0.10 * alpha,
    );
}

/// Bokeh halus: titik cahaya tanpa tepi, ukuran bervariasi.
pub unsafe fn draw_bokeh_motes(
    rt: &ID2D1RenderTarget,
    pill: D2D_RECT_F,
    count: usize,
    t: f32,
    seed: f32,
    color: (f32, f32, f32),
    alpha: f32,
) {
    let pw = pill.right - pill.left;
    let ph = pill.bottom - pill.top;
    for i in 0..count {
        let fi = i as f32;
        let s = (fi * 0.618_034 + seed).fract();
        let life = ((t * (0.07 + 0.05 * s)) + s * 3.7).fract();
        let a = (life * (1.0 - life) * 4.2).min(1.0) * alpha * (0.10 + 0.28 * s);
        let x = pill.left + pw * (0.04 + 0.92 * ((fi * 0.271 + seed).fract()))
            + (t * 0.35 + fi * 1.7).sin() * 2.6;
        let y = pill.bottom - 3.0 - life * ph * 0.95;
        let r = 1.2 + s * 3.4;
        draw_soft_circle(rt, D2D_POINT_2F { x, y }, r, r * 0.92, rgb3(color, a));
    }
}

/// Petir anime: 16 segmen meruncing, 3-4 cabang, jitter pelan per frame.
pub unsafe fn draw_tapered_anime_lightning(
    rt: &ID2D1RenderTarget,
    factory: &ID2D1Factory,
    top: D2D_POINT_2F,
    bottom: D2D_POINT_2F,
    t: f32,
    seed: f32,
    color: (f32, f32, f32),
    alpha: f32,
    scale: f32,
    branches: usize,
) {
    if alpha <= 0.01 {
        return;
    }
    let segments = 16;
    let span = D2D_POINT_2F {
        x: bottom.x - top.x,
        y: bottom.y - top.y,
    };
    let len = (span.x * span.x + span.y * span.y).sqrt().max(1.0);
    let (nx, ny) = (-span.y / len, span.x / len);
    let jitter_seed = ((t * 14.0) as i32) as f32 * 0.37 + seed;

    let mut spine = Vec::with_capacity(segments + 1);
    for i in 0..=segments {
        let f = i as f32 / segments as f32;
        let wob = ((f * 9.0 + jitter_seed).sin() * 0.6 + (f * 21.0 + jitter_seed * 1.7).sin() * 0.4) * scale;
        spine.push(D2D_POINT_2F {
            x: top.x + span.x * f + nx * wob,
            y: top.y + span.y * f + ny * wob,
        });
    }

    // Bloom 4 lapis dari segmen yang sama (bukan satu stroke tebal).
    for (w, a) in [(3.4_f32, 0.10_f32), (2.2, 0.18), (1.2, 0.38), (0.55, 1.0)] {
        for i in 0..segments {
            let p0 = spine[i];
            let p1 = spine[i + 1];
            let mid = D2D_POINT_2F {
                x: (p0.x + p1.x) * 0.5,
                y: (p0.y + p1.y) * 0.5,
            };
            fill_tapered_ribbon(
                rt,
                factory,
                p0,
                D2D_POINT_2F {
                    x: p0.x + (mid.x - p0.x) * 0.4,
                    y: p0.y + (mid.y - p0.y) * 0.4,
                },
                D2D_POINT_2F {
                    x: p1.x - (p1.x - mid.x) * 0.4,
                    y: p1.y - (p1.y - mid.y) * 0.4,
                },
                p1,
                1.1 * w * scale,
                rgb3(color, alpha * a),
                6,
            );
        }
    }

    // Cabang.
    for b in 0..branches {
        let fb = b as f32;
        let at = 0.28 + 0.18 * fb;
        let idx = ((segments as f32 * at) as usize).min(segments - 1);
        let origin = spine[idx];
        let dir = if b % 2 == 0 { 1.0 } else { -1.0 };
        let reach = (10.0 + 7.0 * fb) * scale;
        let tip = D2D_POINT_2F {
            x: origin.x + nx * reach * dir + span.x * 0.06,
            y: origin.y + ny * reach * dir + span.y * 0.10,
        };
        fill_ultra_smooth_ribbon(
            rt,
            factory,
            origin,
            D2D_POINT_2F {
                x: origin.x + (tip.x - origin.x) * 0.4 + nx * 2.0 * dir,
                y: origin.y + (tip.y - origin.y) * 0.4,
            },
            D2D_POINT_2F {
                x: origin.x + (tip.x - origin.x) * 0.75,
                y: origin.y + (tip.y - origin.y) * 0.75,
            },
            tip,
            1.4 * scale,
            rgb3(color, alpha * 0.55),
            false,
        );
    }
}

/// Pita api / kecepatan: 30 sampel, ujung meruncing, opsional inti putih.
pub unsafe fn draw_tapered_flame_ribbon(
    rt: &ID2D1RenderTarget,
    factory: &ID2D1Factory,
    p0: D2D_POINT_2F,
    p1: D2D_POINT_2F,
    p2: D2D_POINT_2F,
    p3: D2D_POINT_2F,
    width: f32,
    color: (f32, f32, f32),
    alpha: f32,
    white_core: bool,
) {
    fill_ultra_smooth_ribbon(rt, factory, p0, p1, p2, p3, width, rgb3(color, alpha), white_core);
}

/// Sabit Dismantle: dua pita meruncing yang membentuk bilah tipis melengkung.
pub unsafe fn draw_crescent_dismantle_slash(
    rt: &ID2D1RenderTarget,
    factory: &ID2D1Factory,
    center: D2D_POINT_2F,
    angle: f32,
    length: f32,
    curve: f32,
    alpha: f32,
) {
    if alpha <= 0.01 {
        return;
    }
    let (ca, sa) = (angle.cos(), angle.sin());
    let rot = |x: f32, y: f32| D2D_POINT_2F {
        x: center.x + x * ca - y * sa,
        y: center.y + x * sa + y * ca,
    };
    let p0 = rot(-length * 0.5, curve);
    let p1 = rot(-length * 0.15, -curve * 0.55);
    let p2 = rot(length * 0.2, -curve * 0.35);
    let p3 = rot(length * 0.55, 0.0);

    fill_ultra_smooth_ribbon(
        rt,
        factory,
        p0,
        p1,
        p2,
        p3,
        3.0,
        D2D1_COLOR_F {
            r: 1.0,
            g: 0.94,
            b: 0.96,
            a: alpha,
        },
        true,
    );
    fill_tapered_ribbon(
        rt,
        factory,
        p0,
        p1,
        p2,
        p3,
        8.0,
        D2D1_COLOR_F {
            r: 0.98,
            g: 0.20,
            b: 0.30,
            a: alpha * 0.35,
        },
        24,
    );
}

/// Ledakan menyatu: starburst 24 titik asimetris, 4 lapis, inti putih radial.
pub unsafe fn draw_connected_anime_explosion(
    rt: &ID2D1RenderTarget,
    factory: &ID2D1Factory,
    center: D2D_POINT_2F,
    radius: f32,
    t: f32,
    alpha: f32,
) {
    if alpha <= 0.01 {
        return;
    }
    let spikes = 24;
    let mut pts = Vec::with_capacity(spikes * 2);
    for i in 0..spikes * 2 {
        let f = i as f32 / (spikes * 2) as f32;
        let ang = f * std::f32::consts::TAU + (t * 0.4).sin() * 0.04;
        let wob = 0.72 + 0.28 * ((i as f32 * 1.7).sin().abs());
        let r = if i % 2 == 0 {
            radius * wob
        } else {
            radius * 0.34 * wob
        };
        pts.push(D2D_POINT_2F {
            x: center.x + ang.cos() * r,
            y: center.y + ang.sin() * r * 0.92,
        });
    }
    for (scale, c, a) in [
        (1.32_f32, (1.0_f32, 0.36_f32, 0.10_f32), 0.16_f32),
        (1.05, (1.0, 0.58, 0.16), 0.26),
        (0.78, (1.0, 0.80, 0.34), 0.42),
        (0.48, (1.0, 0.95, 0.82), 0.72),
    ] {
        let scaled: Vec<D2D_POINT_2F> = pts
            .iter()
            .map(|p| D2D_POINT_2F {
                x: center.x + (p.x - center.x) * scale,
                y: center.y + (p.y - center.y) * scale,
            })
            .collect();
        fill_smooth_shape(
            rt,
            factory,
            &scaled,
            D2D1_COLOR_F {
                r: c.0,
                g: c.1,
                b: c.2,
                a: alpha * a,
            },
        );
    }
    draw_soft_radial_glow(
        rt,
        center,
        radius * 0.62,
        radius * 0.55,
        1.0,
        0.98,
        0.92,
        alpha * 0.85,
    );
    // Jarum bara meruncing keluar.
    for k in 0..10 {
        let fk = k as f32;
        let ang = -2.1 + fk * 0.42 + (t * 0.5).sin() * 0.05;
        let inner = radius * 0.32;
        let outer = radius * (0.95 + 0.25 * (fk * 0.7).sin().abs());
        let s = D2D_POINT_2F {
            x: center.x + ang.cos() * inner,
            y: center.y + ang.sin() * inner,
        };
        let e = D2D_POINT_2F {
            x: center.x + ang.cos() * outer,
            y: center.y + ang.sin() * outer,
        };
        fill_tapered_ribbon(
            rt,
            factory,
            s,
            D2D_POINT_2F {
                x: s.x + (e.x - s.x) * 0.45,
                y: s.y + (e.y - s.y) * 0.45,
            },
            D2D_POINT_2F {
                x: s.x + (e.x - s.x) * 0.8,
                y: s.y + (e.y - s.y) * 0.8,
            },
            e,
            2.2,
            D2D1_COLOR_F {
                r: 1.0,
                g: 0.74,
                b: 0.30,
                a: alpha * 0.55,
            },
            10,
        );
    }
}

/// Uap titan: 18 puff radial bergradien, mengalir naik dengan alfa halus.
pub unsafe fn draw_feathered_titan_steam(
    rt: &ID2D1RenderTarget,
    pill: D2D_RECT_F,
    cx: f32,
    count: usize,
    t: f32,
    seed: f32,
    color: (f32, f32, f32),
    alpha: f32,
) {
    let ph = pill.bottom - pill.top;
    let pw = pill.right - pill.left;
    for i in 0..count {
        let fi = i as f32;
        let s = (fi * 0.618_034 + seed).fract();
        let life = ((t * (0.10 + 0.05 * s)) + s * 2.9).fract();
        let a = (life * (1.0 - life) * 4.0).min(1.0) * alpha * (0.10 + 0.18 * s);
        let side = if i % 2 == 0 { 1.0 } else { -1.0 };
        let x = cx + side * pw * (0.10 + 0.16 * s) + (t * 0.5 + fi).sin() * 3.0;
        let y = pill.bottom - 5.0 - life * ph * 0.85;
        let r = 5.0 + 9.0 * s + life * 7.0;
        draw_soft_circle(rt, D2D_POINT_2F { x, y }, r, r * 0.82, rgb3(color, a));
    }
}
