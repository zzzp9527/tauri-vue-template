use serde::{Deserialize, Serialize};
use tauri::{App, AppHandle};
use tauri_plugin_store::StoreExt;

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
 * 初始化 store 并读取配置
 * 如果在命令行中注意 Application 后面会跟一个 \
 * mac: ~/Library/Application Support/{Bundle Identifier}
 * win: C:\Users\{Username}\AppData\Roaming\{Bundle Identifier}
 */
pub fn init_store(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let store = app.store(SETTINGS_FILE)?;
    // TODO 检测对应文件的格式是否正常，如非法 则更正
    Ok(())
}

// 注意：读取数据实际上不需要 &mut App，使用 &App 或 AppHandle 更为恰当。
// 这里保持 &mut App 以与您的原始代码一致，但建议在实际应用中考虑使用 &App。
pub fn get_global_settings(app_handle: &AppHandle) -> Result<GlobalSettings, String> {
    let store = app_handle.store(SETTINGS_FILE).map_err(|e| e.to_string())?;
    let defaults = GlobalSettings::default();

    // 1. 尝试获取 "theme" 的值
    // .and_then(|v| serde_json::from_value(v.clone()).ok()) 尝试将 JsonValue 反序列化为 String
    // 如果 store 中没有 "theme" 键，或者值的类型不正确，则返回 None
    // .unwrap_or(defaults.theme) 如果前面返回 None，则使用默认的 theme 值
    let theme = store.get("theme")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or(defaults.theme);

    // 2. 用同样的方式获取 "auto_start" 的值
    let auto_start = store.get("auto_start")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or(defaults.auto_start);

    // 3. 使用获取到的值（或默认值）构建并返回 GlobalSettings
    Ok(GlobalSettings {
        theme,
        auto_start,
    })
}
