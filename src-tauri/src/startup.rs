use tauri::{App, Manager};
use log::info;
use tauri_plugin_autostart::ManagerExt;

pub fn handle_startup(app: &mut App) {
    let app_handle = app.handle();
    let args: Vec<String> = std::env::args().collect();

    if !app_handle.autolaunch().is_enabled().unwrap_or(false) {
        app_handle.autolaunch().enable().unwrap();
    }

    if args.contains(&"--autostart".to_string()) {
        info!("开机自启 -> 窗口保持隐藏");
    } else {
        info!("手动启动 -> 显示窗口");
        if let Some(window) = app.get_webview_window("main") {
            window.show().unwrap();
            window.set_focus().unwrap();
        }
    }
}
