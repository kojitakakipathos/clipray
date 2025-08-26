use tauri::{PhysicalPosition, PhysicalSize, WebviewWindow, Window, Wry};

#[cfg(windows)]
use windows::Win32::{
    Foundation::{POINT, RECT},
    UI::WindowsAndMessaging::{GetCursorPos, GetWindowRect, WindowFromPoint},
};

#[cfg(windows)]
fn get_window_at_cursor() -> Result<Option<(i32, i32, i32, i32)>, String> {
    // Get the window handle at the current cursor position
    unsafe {
        let mut point = POINT::default();
        match GetCursorPos(&mut point) {
            Ok(_) => {
                let window_handle = WindowFromPoint(point);
                if !window_handle.is_invalid() {
                    let mut rect = RECT::default();
                    if GetWindowRect(window_handle, &mut rect).is_ok() {
                        let x = rect.left;
                        let y = rect.top;
                        let width = rect.right - rect.left;
                        let height = rect.bottom - rect.top;
                        Ok(Some((x, y, width, height)))
                    } else {
                        Ok(None)
                    }
                } else {
                    Ok(None)
                }
            }
            Err(_) => Err("Failed to get cursor position".to_string()),
        }
    }
}

pub fn show_window_impl(window: &Window<Wry>) -> Result<(), String> {
    // Try to position the window at the center of the window where cursor is located on supported platforms (Windows).
    // Fallback to centering if cursor position is unavailable.

    let mut positioned_at_cursor = false;

    #[cfg(windows)]
    {
        if let Ok(Some((window_x, window_y, window_width, window_height))) = get_window_at_cursor()
        {
            // Calculate the center of the window where cursor is located
            let window_center_x = window_x + (window_width / 2);
            let window_center_y = window_y + (window_height / 2);

            // Get the size of our application window
            let app_window_size = window.outer_size().unwrap_or(PhysicalSize::new(400, 500));

            // Position our window at the center of the target window, offset by half our window size
            let desired_x = window_center_x - (app_window_size.width as i32 / 2);
            let desired_y = window_center_y - (app_window_size.height as i32 / 2);

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
    // Try to position the window at the center of the window where cursor is located on supported platforms (Windows).
    // Fallback to centering if cursor position is unavailable.

    let mut positioned_at_cursor = false;

    #[cfg(windows)]
    {
        if let Ok(Some((window_x, window_y, window_width, window_height))) = get_window_at_cursor()
        {
            // Calculate the center of the window where cursor is located
            let window_center_x = window_x + (window_width / 2);
            let window_center_y = window_y + (window_height / 2);

            // Get the size of our application window
            let app_window_size = window.outer_size().unwrap_or(PhysicalSize::new(400, 500));

            // Position our window at the center of the target window, offset by half our window size
            let desired_x = window_center_x - (app_window_size.width as i32 / 2);
            let desired_y = window_center_y - (app_window_size.height as i32 / 2);

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
