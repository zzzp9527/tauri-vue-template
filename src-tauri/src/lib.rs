// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod commands;
mod startup;
mod store;
mod tray;

// release 隐藏控制台窗口
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
    // 注册所需插件
    .plugin(tauri_plugin_store::Builder::new().build())
    .plugin(tauri_plugin_updater::Builder::new().build())
    .plugin(tauri_plugin_log::Builder::new().build())
    .plugin(tauri_plugin_opener::init())
    .plugin(tauri_plugin_autostart::init(tauri_plugin_autostart::MacosLauncher::LaunchAgent, Some(vec!["--autostart"])))
        // 运行应用前的配置
        .setup(|app| {
            startup::handle_startup(app);
            tray::init_tray(app)?;
            store::init_store(app)?;
            Ok(())
        })
        // 注册命令
        .invoke_handler(tauri::generate_handler![
            commands::greet,
            commands::get_global_settings_command,
            commands::set_global_settings_command,
            commands::get_autostart_status_command,
            commands::set_autostart_status_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
