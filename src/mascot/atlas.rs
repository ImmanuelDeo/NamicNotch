// src/mascot/atlas.rs
// Loader kepala HD (PNG transparan) untuk maskot yang di-upgrade:
// Bakugo, Eren Attack Titan, Okarun.
//
// Kepala disimpan sebagai aset yang di-embed, jadi nusa.exe tetap jalan tanpa
// folder assets di sebelah binary. Kalau file aset ada di disk (build dev), file
// itu yang dipakai lebih dulu supaya bisa diubah tanpa rebuild.
use std::cell::RefCell;
use std::path::{Path, PathBuf};

use windows::core::PCWSTR;
use windows::Win32::Graphics::Direct2D::Common::D2D_RECT_F;
use windows::Win32::Graphics::Direct2D::{
    ID2D1Bitmap, ID2D1RenderTarget, D2D1_BITMAP_INTERPOLATION_MODE_LINEAR,
};
use windows::Win32::Graphics::Imaging::{
    CLSID_WICImagingFactory, GUID_WICPixelFormat32bppPBGRA, IWICImagingFactory,
    WICBitmapDitherTypeNone, WICBitmapPaletteTypeCustom, WICDecodeMetadataCacheOnLoad,
};
use windows::Win32::System::Com::{CoCreateInstance, CLSCTX_INPROC_SERVER};
use windows::Win32::Foundation::GENERIC_READ;

pub const HEAD_W: u32 = 240;
pub const HEAD_H: u32 = 160;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Head {
    Bakugo = 0,
    ErenTitan = 1,
    Okarun = 2,
}

impl Head {
    fn rel_path(self) -> &'static str {
        match self {
            Head::Bakugo => "04_bakugo/head_cap.png",
            Head::ErenTitan => "06_eren_titan/head_cap.png",
            Head::Okarun => "07_okarun/head_cap.png",
        }
    }

    fn embedded(self) -> &'static [u8] {
        match self {
            Head::Bakugo => include_bytes!("../../assets/mascots/04_bakugo/head_cap.png"),
            Head::ErenTitan => include_bytes!("../../assets/mascots/06_eren_titan/head_cap.png"),
            Head::Okarun => include_bytes!("../../assets/mascots/07_okarun/head_cap.png"),
        }
    }
}

thread_local! {
    static CACHE: RefCell<[Option<ID2D1Bitmap>; 3]> = const { RefCell::new([None, None, None]) };
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
        .CreateDecoderFromFilename(PCWSTR(wide.as_ptr()), None, GENERIC_READ, WICDecodeMetadataCacheOnLoad)
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

/// Ambil tekstur kepala; decode sekali, sisanya dari cache thread-local.
pub unsafe fn head_bitmap(rt: &ID2D1RenderTarget, head: Head) -> Option<ID2D1Bitmap> {
    if let Some(bmp) = CACHE.with(|c| c.borrow()[head as usize].clone()) {
        return Some(bmp);
    }
    let bmp = resolve_asset_path(head.rel_path())
        .and_then(|p| decode_file(rt, &p))
        .or_else(|| decode_bytes(rt, head.embedded()))?;
    CACHE.with(|c| c.borrow_mut()[head as usize] = Some(bmp.clone()));
    Some(bmp)
}

/// Gambar kepala terpusat di (cx, cy). `w` = lebar kepala dalam unit maskot.
pub unsafe fn draw_head(
    rt: &ID2D1RenderTarget,
    head: Head,
    cx: f32,
    cy: f32,
    w: f32,
    opacity: f32,
) {
    if opacity <= 0.005 {
        return;
    }
    let Some(bmp) = head_bitmap(rt, head) else { return };
    let h = w * HEAD_H as f32 / HEAD_W as f32;
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
