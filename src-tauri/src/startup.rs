use tauri::{App, Manager};
use log::info;

pub fn handle_startup(app: &mut App) {
    let app_handle = app.handle();
    let args: Vec<String> = std::env::args().collect();

    if args.contains(&"--autostart".to_string()) {
      // TODO 目前开机自启应用会白屏
        info!("开机自启");
        if let Some(window) = app_handle.get_webview_window("main") {
            window.hide().unwrap();
        }
    } else {
        info!("手动启动");
        if let Some(window) = app_handle.get_webview_window("main") {
            window.show().unwrap();
        }
    }
}
