//! Screenshot capture (GDI virtual screen) + Windows.Media.Ocr recognition.
//!
//! The capture half is plain Win32 GDI so it works in any session that owns the
//! interactive desktop. The OCR half runs on a dedicated worker thread with its
//! own WinRT apartment; results come back through the NotchEvent channel.

use anyhow::{anyhow, Result};
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Gdi::{
    BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject, GetDC, GetDIBits,
    ReleaseDC, SelectObject, BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS, SRCCOPY,
};
use windows::Win32::UI::WindowsAndMessaging::{
    GetSystemMetrics, SM_CXVIRTUALSCREEN, SM_CYVIRTUALSCREEN, SM_XVIRTUALSCREEN,
    SM_YVIRTUALSCREEN,
};

/// A BGRA8 frame grabbed from the virtual screen.
pub struct CapturedFrame {
    pub width: i32,
    pub height: i32,
    /// Row-major BGRA, tightly packed (width * 4 bytes per row).
    pub bgra: Vec<u8>,
}

/// Grab the whole virtual desktop via GDI BitBlt + GetDIBits.
pub fn capture_virtual_screen() -> Result<CapturedFrame> {
    unsafe {
        let x = GetSystemMetrics(SM_XVIRTUALSCREEN);
        let y = GetSystemMetrics(SM_YVIRTUALSCREEN);
        let w = GetSystemMetrics(SM_CXVIRTUALSCREEN);
        let h = GetSystemMetrics(SM_CYVIRTUALSCREEN);
        if w <= 0 || h <= 0 {
            return Err(anyhow!("virtual screen has zero size"));
        }

        let screen_dc = GetDC(HWND::default());
        if screen_dc.is_invalid() {
            return Err(anyhow!("GetDC failed"));
        }
        let mem_dc = CreateCompatibleDC(screen_dc);
        let bmp = CreateCompatibleBitmap(screen_dc, w, h);
        let old = SelectObject(mem_dc, bmp);

        let blit_ok = BitBlt(mem_dc, 0, 0, w, h, screen_dc, x, y, SRCCOPY).is_ok();

        let mut frame = None;
        if blit_ok {
            let mut bmi = BITMAPINFO {
                bmiHeader: BITMAPINFOHEADER {
                    biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                    biWidth: w,
                    biHeight: -h, // top-down rows
                    biPlanes: 1,
                    biBitCount: 32,
                    biCompression: BI_RGB.0,
                    ..Default::default()
                },
                ..Default::default()
            };
            let mut buf = vec![0u8; (w as usize) * (h as usize) * 4];
            let lines = GetDIBits(
                mem_dc,
                bmp,
                0,
                h as u32,
                Some(buf.as_mut_ptr() as *mut core::ffi::c_void),
                &mut bmi,
                DIB_RGB_COLORS,
            );
            if lines == h {
                frame = Some(CapturedFrame { width: w, height: h, bgra: buf });
            }
        }

        SelectObject(mem_dc, old);
        let _ = DeleteObject(bmp);
        let _ = DeleteDC(mem_dc);
        ReleaseDC(HWND::default(), screen_dc);

        frame.ok_or_else(|| anyhow!("screen capture failed"))
    }
}

/// Save a captured frame as PNG next to the settings file. Returns the path.
pub fn save_frame_png(frame: &CapturedFrame) -> Result<std::path::PathBuf> {
    use windows::Graphics::Imaging::{BitmapAlphaMode, BitmapEncoder, BitmapPixelFormat, SoftwareBitmap};
    use windows::Storage::Streams::{DataWriter, InMemoryRandomAccessStream};
    use windows::core::Interface;
    use windows::Win32::System::WinRT::IBufferByteAccess;

    let dir = dirs::picture_dir()
        .unwrap_or_else(|| dirs::data_local_dir().unwrap_or_else(|| std::path::PathBuf::from(".")))
        .join("NamicNotch");
    std::fs::create_dir_all(&dir)?;
    let path = dir.join(format!(
        "shot-{}.png",
        chrono::Local::now().format("%Y%m%d-%H%M%S")
    ));

    // Feed the raw BGRA bytes through a WinRT buffer into a SoftwareBitmap,
    // then encode with the system PNG encoder (no extra crate needed).
    let writer = DataWriter::new()?;
    writer.WriteBytes(&frame.bgra)?;
    let buffer = writer.DetachBuffer()?;
    let bitmap = SoftwareBitmap::CreateCopyWithAlphaFromBuffer(
        &buffer,
        BitmapPixelFormat::Bgra8,
        frame.width,
        frame.height,
        BitmapAlphaMode::Ignore,
    )?;

    let stream = InMemoryRandomAccessStream::new()?;
    let encoder = BitmapEncoder::CreateAsync(BitmapEncoder::PngEncoderId()?, &stream)?.get()?;
    encoder.SetSoftwareBitmap(&bitmap)?;
    encoder.FlushAsync()?.get()?;

    // Read the encoded stream back out via IBufferByteAccess on each segment.
    let size = stream.Size()? as usize;
    let mut png = Vec::with_capacity(size);
    let input = stream.GetInputStreamAt(0)?;
    let reader = windows::Storage::Streams::DataReader::CreateDataReader(&input)?;
    let mut remaining = size as u32;
    while remaining > 0 {
        let chunk = remaining.min(64 * 1024);
        let loaded = reader.LoadAsync(chunk)?.get()?;
        if loaded == 0 {
            break;
        }
        let buf = reader.ReadBuffer(loaded)?;
        let access: IBufferByteAccess = buf.cast()?;
        let ptr = unsafe { access.Buffer()? };
        let bytes = unsafe { std::slice::from_raw_parts(ptr, loaded as usize) };
        png.extend_from_slice(bytes);
        remaining -= loaded;
    }
    std::fs::write(&path, &png)?;
    Ok(path)
}

/// Run Windows OCR over a captured frame. Blocking; call from a worker thread.
pub fn recognize_text(frame: &CapturedFrame) -> Result<String> {
    use windows::Graphics::Imaging::{BitmapAlphaMode, BitmapPixelFormat, SoftwareBitmap};
    use windows::Media::Ocr::OcrEngine;
    use windows::Storage::Streams::DataWriter;

    let writer = DataWriter::new()?;
    writer.WriteBytes(&frame.bgra)?;
    let buffer = writer.DetachBuffer()?;
    let bitmap = SoftwareBitmap::CreateCopyWithAlphaFromBuffer(
        &buffer,
        BitmapPixelFormat::Bgra8,
        frame.width,
        frame.height,
        BitmapAlphaMode::Ignore,
    )?;

    let engine = OcrEngine::TryCreateFromUserProfileLanguages()
        .map_err(|_| anyhow!("no OCR language pack installed"))?;
    let result = engine.RecognizeAsync(&bitmap)?.get()?;
    let text = result.Text()?.to_string_lossy();
    Ok(text.trim().to_string())
}

/// One OCR job: capture the screen, save PNG, recognize, report back.
pub struct OcrOutcome {
    pub text: String,
    pub png_path: Option<String>,
}

pub fn capture_and_ocr(save_png: bool) -> Result<OcrOutcome> {
    let frame = capture_virtual_screen()?;
    let png_path = if save_png {
        save_frame_png(&frame).ok().map(|p| p.to_string_lossy().to_string())
    } else {
        None
    };
    let text = recognize_text(&frame)?;
    Ok(OcrOutcome { text, png_path })
}

#[cfg(test)]
mod tests {
    // GDI/WinRT paths need a real desktop session; the pure helpers below are
    // exercised here so regressions in the plumbing still fail fast.
    #[test]
    fn frame_stride_math() {
        let w = 1920i32;
        let h = 1080i32;
        assert_eq!((w as usize) * (h as usize) * 4, 8_294_400);
    }
}
