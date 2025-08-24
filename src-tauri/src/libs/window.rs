use tauri::{PhysicalPosition, PhysicalSize, WebviewWindow, Window, Wry};

#[cfg(windows)]
use windows::Win32::{Foundation::POINT, UI::WindowsAndMessaging::GetCursorPos};

#[cfg(windows)]
fn get_cursor_position() -> Result<(i32, i32), String> {
    // Get current cursor position using Win32 API
    // This function is Windows-specific; other platforms will fallback to centering logic
    unsafe {
        let mut point = POINT::default();
        match GetCursorPos(&mut point) {
            Ok(_) => Ok((point.x, point.y)),
            Err(_) => Err("Failed to get cursor position".to_string()),
        }
    }
}

pub fn show_window_impl(window: &Window<Wry>) -> Result<(), String> {
    // Try to position the window at the current cursor location on supported platforms (Windows).
    // Fallback to centering if cursor position is unavailable.

    let mut positioned_at_cursor = false;

    #[cfg(windows)]
    {
        if let Ok((cursor_x, cursor_y)) = get_cursor_position() {
            // Slight offset so the pointer is not directly over the window's top-left corner
            let desired_x = cursor_x + 8;
            let desired_y = cursor_y + 8;

            let _ = window.set_position(PhysicalPosition::new(desired_x, desired_y));
            positioned_at_cursor = true;
        }
    }

    if !positioned_at_cursor {
        // Center the window on the current monitor as a fallback
        let monitor = window.current_monitor().map_err(|e| e.to_string())?;
        if let Some(monitor) = monitor {
            let screen_size = monitor.size();
            // Use configured size as a conservative default if outer_size is not available
            let window_size = window.outer_size().unwrap_or(PhysicalSize::new(400, 500));
            let x = (screen_size.width as i32 - window_size.width as i32) / 2;
            let y = (screen_size.height as i32 - window_size.height as i32) / 2;
            let _ = window.set_position(PhysicalPosition::new(x, y));
        }
    }

    window.show().map_err(|e| e.to_string())?;
    window.set_focus().map_err(|e| e.to_string())?;
    Ok(())
}

pub fn show_webview_window_impl(window: &WebviewWindow) -> Result<(), String> {
    // Try to position the window at the current cursor location on supported platforms (Windows).
    // Fallback to centering if cursor position is unavailable.

    let mut positioned_at_cursor = false;

    #[cfg(windows)]
    {
        if let Ok((cursor_x, cursor_y)) = get_cursor_position() {
            // Slight offset so the pointer is not directly over the window's top-left corner
            let desired_x = cursor_x + 8;
            let desired_y = cursor_y + 8;

            let _ = window.set_position(PhysicalPosition::new(desired_x, desired_y));
            positioned_at_cursor = true;
        }
    }

    if !positioned_at_cursor {
        // Center the window on the current monitor as a fallback
        let monitor = window.current_monitor().map_err(|e| e.to_string())?;
        if let Some(monitor) = monitor {
            let screen_size = monitor.size();
            let window_size = window.outer_size().unwrap_or(PhysicalSize::new(400, 500));
            let x = (screen_size.width as i32 - window_size.width as i32) / 2;
            let y = (screen_size.height as i32 - window_size.height as i32) / 2;
            let _ = window.set_position(PhysicalPosition::new(x, y));
        }
    }

    window.show().map_err(|e| e.to_string())?;
    window.set_focus().map_err(|e| e.to_string())?;
    Ok(())
}

pub fn hide_window_impl(window: &Window<Wry>) -> Result<(), String> {
    window.hide().map_err(|e| e.to_string())?;
    Ok(())
}
