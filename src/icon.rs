// src/icon.rs
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Gdi::{
    CreateBitmap, CreateDIBSection, DeleteObject, GetDC, ReleaseDC, BITMAPINFO, BITMAPINFOHEADER,
    BI_RGB, DIB_RGB_COLORS,
};
use windows::Win32::UI::WindowsAndMessaging::{
    CreateIconIndirect, GetSystemMetrics, LoadImageW, HICON, ICONINFO, IMAGE_ICON,
    LR_LOADFROMFILE, LR_SHARED, SM_CXICON, SM_CXSMICON, SM_CYICON, SM_CYSMICON,
};
use windows::core::PCWSTR;

/// Mengambil HICON resmi Namic Notch (berdasarkan icon squircle visor + 3D glowing N):
/// 1. Prioritas utama: Resource embedded IDI_ICON1 (1) dari resources/icon.rc
/// 2. Prioritas kedua: File icon.ico / resources/icon.ico dari disk
/// 3. Fallback runtime: Render procedural squircle obsidian + cyan glowing visor & 3D N
pub fn get_app_icon(small: bool) -> HICON {
    unsafe {
        let (cx, cy) = if small {
            (
                GetSystemMetrics(SM_CXSMICON),
                GetSystemMetrics(SM_CYSMICON),
            )
        } else {
            (
                GetSystemMetrics(SM_CXICON),
                GetSystemMetrics(SM_CYICON),
            )
        };

        // 1. Coba load dari embedded resource IDI_ICON1 (1)
        if let Ok(hinstance) = windows::Win32::System::LibraryLoader::GetModuleHandleW(None) {
            if let Ok(icon) = LoadImageW(
                hinstance,
                PCWSTR(1 as *const u16),
                IMAGE_ICON,
                cx,
                cy,
                LR_SHARED,
            ) {
                let hicon = HICON(icon.0);
                if !hicon.is_invalid() {
                    return hicon;
                }
            }
        }

        // 2. Coba load dari file icon.ico atau resources/icon.ico
        let icon_paths = [
            "resources\\icon.ico\0",
            "icon.ico\0",
            "resources\\namic.ico\0",
            "nusa.ico\0",
        ];
        for path_str in icon_paths {
            let wide: Vec<u16> = path_str.encode_utf16().collect();
            if let Ok(icon) = LoadImageW(
                None,
                PCWSTR(wide.as_ptr()),
                IMAGE_ICON,
                cx,
                cy,
                LR_LOADFROMFILE | LR_SHARED,
            ) {
                let hicon = HICON(icon.0);
                if !hicon.is_invalid() {
                    return hicon;
                }
            }
        }

        // 3. Fallback procedural: Buat DIB Namic Notch (Squircle Obsidian + Robot Visor + Glowing Cyan N)
        create_namic_procedural_hicon(if cx > 0 { cx } else { 32 })
    }
}

pub fn create_antimagic_hicon() -> HICON {
    get_app_icon(true)
}

/// Fallback prosedural berkualitas tinggi bergaya Namic Notch (Obsidian Squircle + Cyan Glowing Visor & N)
fn create_namic_procedural_hicon(size: i32) -> HICON {
    unsafe {
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
            let s = size as f32;
            let center = (s - 1.0) * 0.5;
            let rad = s * 0.46;
            let corner_r = s * 0.22;

            for y in 0..size {
                for x in 0..size {
                    let fx = x as f32;
                    let fy = y as f32;

                    // Hitung jarak ke rounded squircle box
                    let dx = (fx - center).abs() - (rad - corner_r);
                    let dy = (fy - center).abs() - (rad - corner_r);
                    let dist_corner = if dx > 0.0 && dy > 0.0 {
                        (dx * dx + dy * dy).sqrt() - corner_r
                    } else {
                        dx.max(dy) - corner_r
                    };

                    if dist_corner <= 0.5 {
                        let a = if dist_corner > -0.5 {
                            ((-dist_corner + 0.5) * 255.0).clamp(0.0, 255.0) as u32
                        } else {
                            255u32
                        };

                        // Base dark slate / obsidian (#121418 ke #1E222A)
                        let mut r = 18u32;
                        let mut g = 20u32;
                        let mut b = 24u32;

                        // Bevel rim highlight di bagian atas/tepi
                        if dist_corner > -1.8 {
                            r = 45;
                            g = 60;
                            b = 78;
                        }

                        // Robot Visor Pill di bagian atas (y: 20%..35%)
                        let vy = (fy - (s * 0.28)).abs();
                        let vx = (fx - center).abs();
                        if vy < s * 0.07 && vx < s * 0.25 {
                            // Visor gelap pekat
                            r = 5;
                            g = 8;
                            b = 12;
                            // Eye slits bercahaya cyan (#00F6FF)
                            if vy < s * 0.035 && (vx > s * 0.08 && vx < s * 0.18) {
                                r = 0;
                                g = 246;
                                b = 255;
                            }
                        }

                        // Huruf 3D Ribbon "N" bercahaya di bagian bawah (y: 42%..85%)
                        let ny = (fy - (s * 0.62)) / (s * 0.25);
                        let nx = (fx - center) / (s * 0.25);
                        if ny >= -1.0 && ny <= 1.0 && nx >= -1.0 && nx <= 1.0 {
                            // Batang vertikal kiri
                            let left_bar = (nx + 0.65).abs() < 0.28;
                            // Batang vertikal kanan
                            let right_bar = (nx - 0.65).abs() < 0.28;
                            // Diagonal menghubungkan kiri-atas ke kanan-bawah
                            let diag = (ny - nx).abs() < 0.32;

                            if left_bar || right_bar || diag {
                                // Gradien Electric Cyan (#00E5FF) ke Cobalt Blue (#1D4ED8)
                                r = ((0.3 + 0.7 * (ny + 1.0) * 0.5) * 0.0) as u32;
                                g = ((1.0 - 0.4 * (ny + 1.0) * 0.5) * 230.0) as u32;
                                b = 255;
                            }
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