// src-tauri/src/commands.rs
use crate::store::{get_global_settings, set_global_setting, GlobalSettings};
use tauri::AppHandle;
use tauri_plugin_autostart::ManagerExt;

// 使用 #[tauri::command] 宏来定义一个命令
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// 获取和更新全局设置
#[tauri::command]
pub fn get_global_settings_command(app_handle: AppHandle) -> Result<GlobalSettings, String> {
    // 直接调用我们修改后的 store 函数
    get_global_settings(&app_handle)
}

#[tauri::command]
pub fn set_global_settings_command(
    app_handle: AppHandle,
    key: String,
    value: serde_json::Value,
) -> Result<(), String> {
    set_global_setting(&app_handle, &key, value)
}

// 获取和更新自动启动设置
#[tauri::command]
pub fn get_autostart_status_command(app_handle: AppHandle) -> bool {
    app_handle.autolaunch().is_enabled().unwrap_or(false)
}

#[tauri::command]
pub fn set_autostart_status_command(app_handle: AppHandle, status: bool) -> Result<(), String> {
    if status {
        app_handle
            .autolaunch()
            .enable()
            .map_err(|e| e.to_string())?;
    } else {
        app_handle
            .autolaunch()
            .disable()
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}
