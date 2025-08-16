use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::json;
use std::sync::Arc;
use tauri::{App, AppHandle, Wry};
use tauri_plugin_store::{Store, StoreExt};

/**
 * 如果在命令行中注意 Application 后面会跟一个 \
 * mac: ~/Library/Application Support/{Bundle Identifier}
 * win: C:\Users\{Username}\AppData\Roaming\{Bundle Identifier}
 */
const SETTINGS_FILE: &str = "settings.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GlobalSettings {
    pub theme: String,
    pub auto_start: bool,
}

// 为 GlobalSettings 提供一个默认实现，这在初始化时非常有用
impl Default for GlobalSettings {
    fn default() -> Self {
        GlobalSettings {
            theme: "light".to_string(),
            auto_start: false,
        }
    }
}

/**
 * 确保 store 中 key 对应的字段类型为 T
 * 如果不存在或类型错误，将使用默认值
 */
fn ensure_field<T: Serialize + for<'de> Deserialize<'de>>(
    store: &Arc<Store<Wry>>,
    key: &str,
    default: T,
) -> Result<(), Box<dyn std::error::Error>> {
    let value_ok = store
        .get(key)
        .and_then(|v| serde_json::from_value::<T>(v.clone()).ok())
        .is_some();

    if !value_ok {
        store.set(key, json!(default));
    }

    Ok(())
}

pub fn get_with_default<T: DeserializeOwned + Clone>(
    store: &Store<Wry>,
    key: &str,
    default: T,
) -> T {
    store
        .get(key)
        .and_then(|v| serde_json::from_value::<T>(v.clone()).ok())
        .unwrap_or(default)
}

/**
 * 初始化 store 并读取配置
 * 如果在命令行中注意 Application 后面会跟一个 \
 * mac: ~/Library/Application Support/{Bundle Identifier}
 * win: C:\Users\{Username}\AppData\Roaming\{Bundle Identifier}
 */
pub fn init_store(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let store = app.store(SETTINGS_FILE)?;

    // 检查并修正每个字段
    let defaults = GlobalSettings::default();
    ensure_field(&store, "theme", defaults.theme.clone())?;
    ensure_field(&store, "auto_start", defaults.auto_start)?;

    // 保存修正后的配置
    store.save()?;

    Ok(())
}

/**
 * 获取全局配置
 */
pub fn get_global_settings(app_handle: &AppHandle) -> Result<GlobalSettings, String> {
    let store = app_handle.store(SETTINGS_FILE).map_err(|e| e.to_string())?;
    let defaults = GlobalSettings::default();

    let theme = get_with_default(&store, "theme", defaults.theme.clone());
    let auto_start = get_with_default(&store, "auto_start", defaults.auto_start);

    Ok(GlobalSettings { theme, auto_start })
}

pub fn set_global_setting<T: Serialize + for<'de> Deserialize<'de>>(
    app_handle: &AppHandle,
    key: &str,
    value: T,
) -> Result<(), String> {
    let store = app_handle.store(SETTINGS_FILE).map_err(|e| e.to_string())?;
    store.set(key, json!(value));
    store.save().map_err(|e| e.to_string())
}
