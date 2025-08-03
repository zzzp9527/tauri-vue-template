// src-tauri/src/commands.rs
use tauri::AppHandle;
use crate::store::{GlobalSettings, get_global_settings};

// 使用 #[tauri::command] 宏来定义一个命令
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub fn get_settings_command(app_handle: AppHandle) -> Result<GlobalSettings, String> {
    // 直接调用我们修改后的 store 函数
    get_global_settings(&app_handle)
}
