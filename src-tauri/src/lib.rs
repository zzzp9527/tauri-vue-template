// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod tray;
mod commands;

// release 隐藏控制台窗口
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // 注册所需插件
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_opener::init())

        // 运行应用前的配置
        .setup(|app| {
            tray::create_tray(app)?;
            Ok(())
        })

        // 注册命令
        .invoke_handler(tauri::generate_handler![
            commands::greet
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
