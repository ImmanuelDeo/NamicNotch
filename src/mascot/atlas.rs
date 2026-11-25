// src/mascot/atlas.rs
// Loader tekstur HD (PNG transparan) untuk kepala maskot dan gesture khusus.
//
// Aset di-embed lewat include_bytes!, jadi nusa.exe tetap jalan tanpa folder
// assets. Kalau file ada di disk (build dev / folder install), file itu yang
// dipakai lebih dulu supaya bisa diganti tanpa rebuild.
use std::cell::RefCell;
use std::path::{Path, PathBuf};

use windows::core::PCWSTR;
use windows::Win32::Foundation::GENERIC_READ;
use windows::Win32::Graphics::Direct2D::Common::D2D_RECT_F;
use windows::Win32::Graphics::Direct2D::{
    ID2D1Bitmap, ID2D1RenderTarget, D2D1_BITMAP_INTERPOLATION_MODE_LINEAR,
};
use windows::Win32::Graphics::Imaging::{
    CLSID_WICImagingFactory, GUID_WICPixelFormat32bppPBGRA, IWICImagingFactory,
    WICBitmapDitherTypeNone, WICBitmapPaletteTypeCustom, WICDecodeMetadataCacheOnLoad,
};
use windows::Win32::System::Com::{CoCreateInstance, CLSCTX_INPROC_SERVER};

/// Tekstur HD yang dipakai maskot. Urutan = index cache.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Head {
    Asta = 0,
    Yuji = 1,
    Sukuna = 2,
    ErenHuman = 3,
    Bakugo = 4,
    ErenTitan = 5,
    Okarun = 6,
    GestureBite = 7,
}

impl Head {
    fn rel_path(self) -> &'static str {
        match self {
            Head::Asta => "01_asta/head_cap.png",
            Head::Yuji => "02_yuji/head_cap.png",
            Head::Sukuna => "03_sukuna/head_cap.png",
            Head::ErenHuman => "05_eren_human/head_cap.png",
            Head::Bakugo => "04_bakugo/head_cap.png",
            Head::ErenTitan => "06_eren_titan/head_cap.png",
            Head::Okarun => "07_okarun/head_cap.png",
            Head::GestureBite => "05_eren_human/gesture_bite.png",
        }
    }

    fn embedded(self) -> &'static [u8] {
        match self {
            Head::Asta => include_bytes!("../../assets/mascots/01_asta/head_cap.png"),
            Head::Yuji => include_bytes!("../../assets/mascots/02_yuji/head_cap.png"),
            Head::Sukuna => include_bytes!("../../assets/mascots/03_sukuna/head_cap.png"),
            Head::ErenHuman => include_bytes!("../../assets/mascots/05_eren_human/head_cap.png"),
            Head::Bakugo => include_bytes!("../../assets/mascots/04_bakugo/head_cap.png"),
            Head::ErenTitan => include_bytes!("../../assets/mascots/06_eren_titan/head_cap.png"),
            Head::Okarun => include_bytes!("../../assets/mascots/07_okarun/head_cap.png"),
            Head::GestureBite => {
                include_bytes!("../../assets/mascots/05_eren_human/gesture_bite.png")
            }
        }
    }
}

thread_local! {
    static CACHE: RefCell<[Option<ID2D1Bitmap>; 8]> =
        const { RefCell::new([None, None, None, None, None, None, None, None]) };
}

/// Cari file aset di sebelah exe, satu level di atasnya, lalu di working dir.
pub fn resolve_asset_path(rel_path: &str) -> Option<PathBuf> {
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let p = dir.join("assets").join("mascots").join(rel_path);
            if p.exists() {
                return Some(p);
            }
            if let Some(root) = dir.parent().and_then(|p| p.parent()) {
                let p2 = root.join("assets").join("mascots").join(rel_path);
                if p2.exists() {
                    return Some(p2);
                }
            }
        }
    }
    let cwd = PathBuf::from("assets").join("mascots").join(rel_path);
    if cwd.exists() {
        Some(cwd)
    } else {
        None
    }
}

unsafe fn decode_file(rt: &ID2D1RenderTarget, path: &Path) -> Option<ID2D1Bitmap> {
    let factory: IWICImagingFactory =
        CoCreateInstance(&CLSID_WICImagingFactory, None, CLSCTX_INPROC_SERVER).ok()?;
    let mut wide: Vec<u16> = path.to_string_lossy().encode_utf16().collect();
    wide.push(0);
    let decoder = factory
        .CreateDecoderFromFilename(
            PCWSTR(wide.as_ptr()),
            None,
            GENERIC_READ,
            WICDecodeMetadataCacheOnLoad,
        )
        .ok()?;
    let frame = decoder.GetFrame(0).ok()?;
    let converter = factory.CreateFormatConverter().ok()?;
    converter
        .Initialize(
            &frame,
            &GUID_WICPixelFormat32bppPBGRA,
            WICBitmapDitherTypeNone,
            None,
            0.0,
            WICBitmapPaletteTypeCustom,
        )
        .ok()?;
    rt.CreateBitmapFromWicBitmap(&converter, None).ok()
}

unsafe fn decode_bytes(rt: &ID2D1RenderTarget, bytes: &[u8]) -> Option<ID2D1Bitmap> {
    let factory: IWICImagingFactory =
        CoCreateInstance(&CLSID_WICImagingFactory, None, CLSCTX_INPROC_SERVER).ok()?;
    let stream = factory.CreateStream().ok()?;
    stream.InitializeFromMemory(bytes).ok()?;
    let decoder = factory
        .CreateDecoderFromStream(&stream, std::ptr::null(), WICDecodeMetadataCacheOnLoad)
        .ok()?;
    let frame = decoder.GetFrame(0).ok()?;
    let converter = factory.CreateFormatConverter().ok()?;
    converter
        .Initialize(
            &frame,
            &GUID_WICPixelFormat32bppPBGRA,
            WICBitmapDitherTypeNone,
            None,
            0.0,
            WICBitmapPaletteTypeCustom,
        )
        .ok()?;
    rt.CreateBitmapFromWicBitmap(&converter, None).ok()
}

/// Ambil tekstur; decode sekali, sisanya dari cache thread-local.
pub unsafe fn texture(rt: &ID2D1RenderTarget, head: Head) -> Option<ID2D1Bitmap> {
    if let Some(bmp) = CACHE.with(|c| c.borrow()[head as usize].clone()) {
        return Some(bmp);
    }
    let bmp = resolve_asset_path(head.rel_path())
        .and_then(|p| decode_file(rt, &p))
        .or_else(|| decode_bytes(rt, head.embedded()))?;
    CACHE.with(|c| c.borrow_mut()[head as usize] = Some(bmp.clone()));
    Some(bmp)
}

/// Gambar tekstur terpusat di (cx, cy). `w` = lebar dalam unit maskot,
/// tinggi mengikuti rasio asli bitmap.
pub unsafe fn draw_head(
    rt: &ID2D1RenderTarget,
    head: Head,
    cx: f32,
    cy: f32,
    w: f32,
    opacity: f32,
) {
    if opacity <= 0.005 || w <= 0.5 {
        return;
    }
    let Some(bmp) = texture(rt, head) else { return };
    let size = bmp.GetSize();
    let h = if size.width > 0.5 {
        w * size.height / size.width
    } else {
        w
    };
    let dest = D2D_RECT_F {
        left: cx - w * 0.5,
        top: cy - h * 0.5,
        right: cx + w * 0.5,
        bottom: cy + h * 0.5,
    };
    rt.DrawBitmap(
        &bmp,
        Some(&dest),
        opacity.clamp(0.0, 1.0),
        D2D1_BITMAP_INTERPOLATION_MODE_LINEAR,
        None,
    );
}
