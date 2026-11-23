//! Native Win32 helpers for the clipboard (read/write CF_UNICODETEXT).

use anyhow::{anyhow, Result};
use windows::Win32::Foundation::{HANDLE, HWND};
use windows::Win32::System::DataExchange::{
    CloseClipboard, EmptyClipboard, GetClipboardData, GetClipboardSequenceNumber,
    IsClipboardFormatAvailable, OpenClipboard, SetClipboardData,
};
use windows::Win32::System::Memory::{GlobalAlloc, GlobalLock, GlobalUnlock, GMEM_MOVEABLE};
use windows::Win32::System::Ole::CF_UNICODETEXT;

/// Current clipboard sequence number (0 before first use).
pub fn clipboard_seq() -> u32 {
    unsafe { GetClipboardSequenceNumber() }
}

/// Read the clipboard as UTF-8 text. Returns None when no text is available.
pub fn read_clipboard_text() -> Option<String> {
    unsafe {
        if IsClipboardFormatAvailable(CF_UNICODETEXT.0 as u32).is_err() {
            return None;
        }
        if OpenClipboard(HWND::default()).is_err() {
            return None;
        }
        let result = (|| -> Option<String> {
            let handle = GetClipboardData(CF_UNICODETEXT.0 as u32).ok()?;
            if handle.is_invalid() {
                return None;
            }
            let ptr = GlobalLock(windows::Win32::Foundation::HGLOBAL(handle.0)) as *const u16;
            if ptr.is_null() {
                return None;
            }
            let mut len = 0usize;
            while *ptr.add(len) != 0 {
                len += 1;
                if len > 1_000_000 {
                    break; // safety bound: 2 MB of UTF-16
                }
            }
            let slice = std::slice::from_raw_parts(ptr, len);
            let text = String::from_utf16_lossy(slice);
            let _ = GlobalUnlock(windows::Win32::Foundation::HGLOBAL(handle.0));
            Some(text)
        })();
        let _ = CloseClipboard();
        result
    }
}

/// Replace the clipboard with `text` (CF_UNICODETEXT). The sequence number
/// after the write is returned so callers can filter their own echo.
pub fn write_clipboard_text(text: &str) -> Result<u32> {
    unsafe {
        OpenClipboard(HWND::default()).map_err(|_| anyhow!("clipboard busy"))?;
        let result = (|| -> Result<u32> {
            EmptyClipboard()?;
            let wide: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();
            let bytes = wide.len() * 2;
            let hmem = GlobalAlloc(GMEM_MOVEABLE, bytes)?;
            let ptr = GlobalLock(hmem) as *mut u16;
            if ptr.is_null() {
                return Err(anyhow!("GlobalLock failed"));
            }
            std::ptr::copy_nonoverlapping(wide.as_ptr(), ptr, wide.len());
            let _ = GlobalUnlock(hmem);
            SetClipboardData(CF_UNICODETEXT.0 as u32, HANDLE(hmem.0))?;
            Ok(GetClipboardSequenceNumber())
        })();
        let _ = CloseClipboard();
        result
    }
}
