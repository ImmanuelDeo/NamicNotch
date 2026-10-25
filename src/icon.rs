// src/icon.rs
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Gdi::{
    CreateBitmap, CreateDIBSection, DeleteObject, GetDC, ReleaseDC, BITMAPINFO, BITMAPINFOHEADER,
    BI_RGB, DIB_RGB_COLORS,
};
use windows::Win32::UI::WindowsAndMessaging::{CreateIconIndirect, HICON, ICONINFO};

/// Membuat HICON 32x32 bertema Nusa Anti-Magic (Hitam Obsidian + Biru Royal + Kilat Merah/Cyan)
/// untuk digunakan di System Tray dan Window Class.
pub fn create_antimagic_hicon() -> HICON {
    unsafe {
        let size = 32i32;
        let bmi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: size,
                biHeight: -size, // Top-down DIB
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB.0,
                ..Default::default()
            },
            ..Default::default()
        };

        let mut bits: *mut std::ffi::c_void = std::ptr::null_mut();
        let hdc = GetDC(HWND::default());
        let hbm_color = CreateDIBSection(hdc, &bmi, DIB_RGB_COLORS, &mut bits, None, 0)
            .unwrap_or_default();
        ReleaseDC(HWND::default(), hdc);

        if !hbm_color.is_invalid() && !bits.is_null() {
            let pixels = std::slice::from_raw_parts_mut(bits as *mut u32, (size * size) as usize);
            let center = 15.5_f32;

            for y in 0..size {
                for x in 0..size {
                    let fx = x as f32 - center;
                    let fy = y as f32 - center;
                    let dist = (fx * fx + fy * fy).sqrt();

                    // Bentuk dasar: Rounded Shield / Lingkaran Hitam Anti-Magic
                    if dist <= 14.8 {
                        // Base Obsidian Black (#070A12)
                        let mut r = 7u32;
                        let mut g = 10u32;
                        let mut b = 18u32;
                        let a = if dist > 13.8 {
                            ((14.8 - dist).clamp(0.0, 1.0) * 255.0) as u32
                        } else {
                            255u32
                        };

                        // Ring tepi luar Biru Royal (#3B82F6)
                        if (12.2..=14.0).contains(&dist) {
                            r = 59;
                            g = 130;
                            b = 246;
                        }

                        // Bintang / Pedang Anti-Magic 4-Sudut di tengah
                        let ax = fx.abs();
                        let ay = fy.abs();
                        let star = (ax * 0.35 + ay).min(ay * 0.35 + ax);
                        if star < 3.6 && dist < 11.8 {
                            // Gradien Cyan ke Biru Royal
                            r = 56;
                            g = 189;
                            b = 248;
                        } else if star < 5.0 && dist < 10.5 {
                            r = 37;
                            g = 99;
                            b = 235;
                        }

                        // Inti Merah Devil Union di pusat bintang
                        if dist < 2.6 {
                            r = 239;
                            g = 68;
                            b = 68;
                        }

                        // Pre-multiplied BGRA (0xAARRGGBB)
                        pixels[(y * size + x) as usize] = (a << 24) | (r << 16) | (g << 8) | b;
                    } else {
                        pixels[(y * size + x) as usize] = 0x00000000;
                    }
                }
            }
        }

        let hbm_mask = CreateBitmap(size, size, 1, 1, None);
        let icon_info = ICONINFO {
            fIcon: true.into(),
            xHotspot: 0,
            yHotspot: 0,
            hbmMask: hbm_mask,
            hbmColor: hbm_color,
        };

        let hicon = CreateIconIndirect(&icon_info).unwrap_or_default();
        let _ = DeleteObject(hbm_color);
        let _ = DeleteObject(hbm_mask);
        hicon
    }
}