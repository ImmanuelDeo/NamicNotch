// Render snapshot idle mascot Namic Notch ke PNG tanpa jendela.
// Memakai Direct2D software render target di atas WIC bitmap, jadi jalan headless.
#![windows_subsystem = "console"]

mod mascot;

use mascot::asta::AstaDevilUnion;
use mascot::bakugo::KatsukiBakugo;
use mascot::core::{IdleCharacter, MascotContext};
use mascot::eren::ErenAttackTitan;
use mascot::okarun::OkarunTurbo;
use mascot::sukuna::YujiSukuna;

use windows::core::{Interface, Result};
use windows::Win32::Graphics::Direct2D::Common::{
    D2D1_ALPHA_MODE_PREMULTIPLIED, D2D1_COLOR_F, D2D1_PIXEL_FORMAT, D2D_RECT_F,
};
use windows::Win32::Graphics::Direct2D::{
    D2D1CreateFactory, ID2D1Factory, ID2D1RenderTarget, D2D1_FACTORY_TYPE_SINGLE_THREADED,
    D2D1_RENDER_TARGET_PROPERTIES, D2D1_RENDER_TARGET_TYPE_SOFTWARE,
};
use windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_B8G8R8A8_UNORM;
use windows::Win32::Graphics::Imaging::{
    CLSID_WICImagingFactory, GUID_WICPixelFormat32bppPBGRA, IWICBitmap, IWICImagingFactory,
    WICBitmapCacheOnLoad, WICBitmapLockRead,
};
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CLSCTX_INPROC_SERVER, COINIT_APARTMENTTHREADED,
};

const W: u32 = 620;
const H: u32 = 240;

struct Shot {
    file: String,
    time: f32,
    draw: fn(&ID2D1RenderTarget, &ID2D1Factory, D2D_RECT_F, &MascotContext),
    duration: f32,
}

fn context(time: f32, duration: f32) -> MascotContext {
    MascotContext {
        phase_time: time,
        local_time: time,
        duration,
        blink_progress: 0.0,
        look_x: 0.0,
        look_y: 0.0,
        flare_intensity: 1.0,
    }
}

fn render(
    factory: &ID2D1Factory,
    wic: &IWICImagingFactory,
    out_dir: &str,
    shot: &Shot,
) -> Result<()> {
    unsafe {
        let bitmap = wic.CreateBitmap(
            W,
            H,
            &GUID_WICPixelFormat32bppPBGRA,
            WICBitmapCacheOnLoad,
        )?;
        let props = D2D1_RENDER_TARGET_PROPERTIES {
            r#type: D2D1_RENDER_TARGET_TYPE_SOFTWARE,
            pixelFormat: D2D1_PIXEL_FORMAT {
                format: DXGI_FORMAT_B8G8R8A8_UNORM,
                alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED,
            },
            dpiX: 96.0,
            dpiY: 96.0,
            usage: Default::default(),
            minLevel: Default::default(),
        };
        let rt: ID2D1RenderTarget = factory.CreateWicBitmapRenderTarget(&bitmap, &props)?;
        // Komposisi notch asli ~200x70 unit; skala 3x supaya dokumen enak dibaca.
        rt.SetTransform(&windows::Foundation::Numerics::Matrix3x2 {
            M11: 3.0,
            M22: 3.0,
            ..Default::default()
        });
        let ctx = context(shot.time, shot.duration);

        rt.BeginDraw();
        rt.Clear(Some(&D2D1_COLOR_F {
            r: 0.016,
            g: 0.016,
            b: 0.024,
            a: 1.0,
        }));
        let pill = D2D_RECT_F {
            left: 6.0,
            top: 6.5,
            right: 200.0,
            bottom: 73.5,
        };
        (shot.draw)(&rt, factory, pill, &ctx);
        rt.EndDraw(None, None)?;

        let path = format!("{out_dir}\\{}", shot.file);
        let bmp = save_bmp(&bitmap, &path)?;
        let _ = bmp;
    }
    Ok(())
}

fn draw_asta(rt: &ID2D1RenderTarget, f: &ID2D1Factory, pill: D2D_RECT_F, ctx: &MascotContext) {
    let c = AstaDevilUnion::new();
    unsafe {
        c.draw_aura(rt, f, pill, 1.0, ctx);
        c.draw_character(rt, f, 103.0, 38.0, 1.0, ctx);
    }
}

fn draw_sukuna(rt: &ID2D1RenderTarget, f: &ID2D1Factory, pill: D2D_RECT_F, ctx: &MascotContext) {
    let c = YujiSukuna::new();
    unsafe {
        c.draw_aura(rt, f, pill, 1.0, ctx);
        c.draw_character(rt, f, 103.0, 38.0, 1.0, ctx);
    }
}

fn draw_bakugo(rt: &ID2D1RenderTarget, f: &ID2D1Factory, pill: D2D_RECT_F, ctx: &MascotContext) {
    let c = KatsukiBakugo::new();
    unsafe {
        c.draw_aura(rt, f, pill, 1.0, ctx);
        c.draw_character(rt, f, 103.0, 38.0, 1.0, ctx);
    }
}

fn draw_eren(rt: &ID2D1RenderTarget, f: &ID2D1Factory, pill: D2D_RECT_F, ctx: &MascotContext) {
    let c = ErenAttackTitan::new();
    unsafe {
        c.draw_aura(rt, f, pill, 1.0, ctx);
        c.draw_character(rt, f, 103.0, 38.0, 1.0, ctx);
    }
}

fn draw_okarun(rt: &ID2D1RenderTarget, f: &ID2D1Factory, pill: D2D_RECT_F, ctx: &MascotContext) {
    let c = OkarunTurbo::new();
    unsafe {
        c.draw_aura(rt, f, pill, 1.0, ctx);
        c.draw_character(rt, f, 103.0, 38.0, 1.0, ctx);
    }
}


/// Simpan WIC bitmap 32bpp PBGRA sebagai BMP (BGRA, baris dibalik). BMP dibaca
/// tanpa codec tambahan; konversi ke PNG dilakukan di sisi dokumen.
fn save_bmp(bitmap: &IWICBitmap, path: &str) -> Result<()> {
    unsafe {
        let rect = windows::Win32::Graphics::Imaging::WICRect {
            X: 0,
            Y: 0,
            Width: W as i32,
            Height: H as i32,
        };
        let lock = bitmap.Lock(&rect, WICBitmapLockRead.0 as u32)?;
        let mut buf_size: u32 = 0;
        let mut ptr: *mut u8 = std::ptr::null_mut();
        lock.GetDataPointer(&mut buf_size, &mut ptr)?;
        let stride: u32 = lock.GetStride()?;
        let mut out = Vec::with_capacity(54 + (W * H * 4) as usize);
        let file_size = 54 + W * H * 4;
        out.extend_from_slice(b"BM");
        out.extend_from_slice(&file_size.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&54u32.to_le_bytes());
        out.extend_from_slice(&40u32.to_le_bytes());
        out.extend_from_slice(&(W as i32).to_le_bytes());
        out.extend_from_slice(&(H as i32).to_le_bytes());
        out.extend_from_slice(&1u16.to_le_bytes());
        out.extend_from_slice(&32u16.to_le_bytes());
        out.extend_from_slice(&0u32.to_le_bytes());
        out.extend_from_slice(&(W * H * 4).to_le_bytes());
        out.extend_from_slice(&2835i32.to_le_bytes());
        out.extend_from_slice(&2835i32.to_le_bytes());
        out.extend_from_slice(&0u32.to_le_bytes());
        out.extend_from_slice(&0u32.to_le_bytes());
        for y in (0..H).rev() {
            let row = ptr.add((y * stride) as usize);
            out.extend_from_slice(std::slice::from_raw_parts(row, (W * 4) as usize));
        }
        std::fs::write(path, &out).map_err(|e| windows::core::Error::new(
            windows::Win32::Foundation::E_FAIL,
            e.to_string(),
        ))?;
    }
    Ok(())
}

fn main() -> Result<()> {
    let out_dir = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "D:\\NusaNotch-RE\\snapshots".to_string());
    let _ = std::fs::create_dir_all(&out_dir);

    unsafe {
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
    }
    let factory: ID2D1Factory =
        unsafe { D2D1CreateFactory(D2D1_FACTORY_TYPE_SINGLE_THREADED, None)? };
    let wic: IWICImagingFactory = unsafe {
        CoCreateInstance(&CLSID_WICImagingFactory, None, CLSCTX_INPROC_SERVER)?
    };

    let shots = vec![
        Shot {
            file: "01-asta-devil-union.png".into(),
            time: 5.2,
            draw: draw_asta,
            duration: AstaDevilUnion::new().duration_secs(),
        },
        Shot {
            file: "02-sukuna-yuji.png".into(),
            time: 1.4,
            draw: draw_sukuna,
            duration: 11.0,
        },
        Shot {
            file: "03-sukuna-awakened.png".into(),
            time: 7.4,
            draw: draw_sukuna,
            duration: 11.0,
        },
        Shot {
            file: "04-bakugo.png".into(),
            time: 0.9,
            draw: draw_bakugo,
            duration: 10.0,
        },
        Shot {
            file: "05-eren-human.png".into(),
            time: 1.6,
            draw: draw_eren,
            duration: 11.5,
        },
        Shot {
            file: "06-eren-titan.png".into(),
            time: 6.2,
            draw: draw_eren,
            duration: 11.5,
        },
        Shot {
            file: "07-okarun.png".into(),
            time: 0.5,
            draw: draw_okarun,
            duration: 10.0,
        },
        Shot {
            file: "08-okarun-dash.png".into(),
            time: 2.2,
            draw: draw_okarun,
            duration: 10.0,
        },
    ];

    for shot in &shots {
        render(&factory, &wic, &out_dir, shot)?;
        println!("ok {}", shot.file);
    }
    println!("DONE {out_dir}");
    Ok(())
}
