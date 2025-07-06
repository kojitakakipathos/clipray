use tauri::{AppHandle, Emitter, Manager, Wry};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};

pub fn register_hotkey(app_handle: &AppHandle<Wry>, hotkey: &str) -> Result<(), String> {
    let shortcut_manager = app_handle.global_shortcut();
    let window = app_handle.get_webview_window("main").unwrap();
    let shortcut = hotkey
        .parse::<Shortcut>()
        .map_err(|e| format!("Invalid hotkey format: {}", e))?;

    let window_clone = window.clone();
    shortcut_manager
        .on_shortcut(shortcut, move |_app, _shortcut, _event| {
            let _ = window_clone.emit("show-clipboard", ());
            let _ = window_clone.show();
            let _ = window_clone.set_focus();
        })
        .map_err(|e| format!("Failed to set hotkey handler: {}", e))?;
    Ok(())
}

pub fn unregister_hotkey(app_handle: &AppHandle<Wry>, hotkey: &str) -> Result<(), String> {
    let shortcut_manager = app_handle.global_shortcut();
    if let Ok(shortcut) = hotkey.parse::<Shortcut>() {
        shortcut_manager
            .unregister(shortcut)
            .map_err(|e| format!("Failed to unregister hotkey: {}", e))?;
    }
    Ok(())
}

pub fn update_hotkey(
    app_handle: &AppHandle<Wry>,
    old_hotkey: &str,
    new_hotkey: &str,
) -> Result<(), String> {
    // Remove old hotkey
    let _ = unregister_hotkey(app_handle, old_hotkey);

    // Register new hotkey
    register_hotkey(app_handle, new_hotkey)?;

    Ok(())
}
