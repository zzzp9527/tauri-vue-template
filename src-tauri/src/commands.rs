// src-tauri/src/commands.rs

// 使用 #[tauri::command] 宏来定义一个命令
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// 如果未来有更多命令，都可以在这里添加
// #[tauri::command]
// pub fn another_command() -> String { ... }