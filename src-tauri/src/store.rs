use serde_json::Value as JsonValue;
use tauri::App;
use tauri_plugin_store::StoreExt;

const SETTINGS_FILE: &str = "settings.json";

/**
 * 初始化 store 并读取配置
 * 如果在命令行中注意 Application 后面会跟一个 \
 * mac: ~/Library/Application Support/{Bundle Identifier}
 * win: TODO
 */
pub fn init_store(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let store = app.store(SETTINGS_FILE)?;

    store.set("a".to_string(), JsonValue::String("b".to_string()));

    Ok(())
}
