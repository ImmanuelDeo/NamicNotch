//! Win32 utility functions and helpers

use windows::Win32::Foundation::{HWND, RECT, POINT};
use windows::Win32::Graphics::Gdi::{
    GetMonitorInfoW, MONITORINFO, MonitorFromWindow, MonitorFromPoint,
    MONITOR_DEFAULTTONEAREST, MONITOR_DEFAULTTOPRIMARY,
};
use windows::Win32::UI::WindowsAndMessaging::{
    SetWindowPos, SWP_NOACTIVATE, SWP_NOOWNERZORDER, SWP_NOMOVE, SWP_NOSIZE,
    HWND_TOPMOST, HWND_NOTOPMOST,
};
use windows::Win32::UI::HiDpi::GetDpiForWindow;
use anyhow::Result;

/// Get the monitor work area for a window
pub fn get_monitor_work_area(hwnd: HWND) -> Result<RECT> {
    unsafe {
        let monitor = MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST);
        let mut info = MONITORINFO {
            cbSize: std::mem::size_of::<MONITORINFO>() as u32,
            ..Default::default()
        };
        GetMonitorInfoW(monitor, &mut info).ok()?;
        Ok(info.rcWork)
    }
}

/// Get window DPI scale factor
pub fn get_dpi_scale(hwnd: HWND) -> f32 {
    unsafe {
        let dpi = GetDpiForWindow(hwnd);
        dpi as f32 / 96.0
    }
}

/// Set window position without activating
pub fn set_window_pos_no_activate(hwnd: HWND, x: i32, y: i32, width: i32, height: i32) -> Result<()> {
    unsafe {
        SetWindowPos(
            hwnd,
            HWND_TOPMOST,
            x,
            y,
            width,
            height,
            SWP_NOACTIVATE | SWP_NOOWNERZORDER,
        )?;
        Ok(())
    }
}

/// Set window to topmost or not
pub fn set_window_topmost(hwnd: HWND, topmost: bool) -> Result<()> {
    unsafe {
        let insert_after = if topmost { HWND_TOPMOST } else { HWND_NOTOPMOST };
        SetWindowPos(
            hwnd,
            insert_after,
            0, 0, 0, 0,
            SWP_NOACTIVATE | SWP_NOMOVE | SWP_NOSIZE | SWP_NOOWNERZORDER,
        )?;
        Ok(())
    }
}

/// Convert logical pixels to physical pixels
pub fn logical_to_physical(logical: i32, scale: f32) -> i32 {
    (logical as f32 * scale).round() as i32
}

/// Convert physical pixels to logical pixels
pub fn physical_to_logical(physical: i32, scale: f32) -> i32 {
    (physical as f32 / scale).round() as i32
}

/// Center a rect within another rect
pub fn center_rect(outer: RECT, inner_width: i32, inner_height: i32) -> RECT {
    let outer_width = outer.right - outer.left;
    let outer_height = outer.bottom - outer.top;
    let left = outer.left + (outer_width - inner_width) / 2;
    let top = outer.top + (outer_height - inner_height) / 2;
    RECT {
        left,
        top,
        right: left + inner_width,
        bottom: top + inner_height,
    }
}

/// Get primary monitor work area
pub fn get_primary_work_area() -> Result<RECT> {
    unsafe {
        let monitor = MonitorFromPoint(POINT { x: 0, y: 0 }, MONITOR_DEFAULTTOPRIMARY);
        let mut info = MONITORINFO {
            cbSize: std::mem::size_of::<MONITORINFO>() as u32,
            ..Default::default()
        };
        GetMonitorInfoW(monitor, &mut info).ok()?;
        Ok(info.rcWork)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_center_rect() {
        let outer = RECT { left: 0, top: 0, right: 1920, bottom: 1080 };
        let centered = center_rect(outer, 300, 100);
        assert_eq!(centered.left, 810);
        assert_eq!(centered.top, 490);
        assert_eq!(centered.right, 1110);
        assert_eq!(centered.bottom, 590);
    }
}
