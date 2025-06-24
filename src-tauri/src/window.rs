use tauri::{PhysicalPosition, PhysicalSize, Window, Wry};

pub fn show_window_impl(window: &Window<Wry>) -> Result<(), String> {
    // ウィンドウを画面中央に配置
    let monitor = window.current_monitor().map_err(|e| e.to_string())?;
    if let Some(monitor) = monitor {
        let screen_size = monitor.size();
        let window_size = PhysicalSize::new(600, 500);
        let x = (screen_size.width - window_size.width) / 2;
        let y = (screen_size.height - window_size.height) / 2;

        let _ = window.set_position(PhysicalPosition::new(x as i32, y as i32));
    }

    window.show().map_err(|e| e.to_string())?;
    window.set_focus().map_err(|e| e.to_string())?;
    Ok(())
}

pub fn hide_window_impl(window: &Window<Wry>) -> Result<(), String> {
    window.hide().map_err(|e| e.to_string())?;
    Ok(())
}
