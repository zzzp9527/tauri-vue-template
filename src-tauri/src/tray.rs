use tauri::{
    menu::{Menu, MenuItem},
    tray::{TrayIconBuilder, TrayIconEvent},
    App,     // .setup() 钩子传递的参数类型
    Manager, // 用于获取 app_handle 和 window
    Result,  // 用于错误处理
};

// 创建一个公共函数，它接收 App 句柄并返回一个 Result
// 所有托盘逻辑都在这里
pub fn init_tray(app: &mut App) -> Result<()> {
    // 1. 创建托盘菜单
    let quit_item = MenuItem::with_id(app, "quit", "退出应用", true, None::<&str>)?;
    let hide_item = MenuItem::with_id(app, "hide", "隐藏窗口", true, None::<&str>)?;
    let show_item = MenuItem::with_id(app, "show", "显示窗口", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&show_item, &hide_item, &quit_item])?;

    // 2. 使用 TrayIconBuilder 创建托盘
    // 注意这里的 `_tray` 变量。尽管它看起来没有被使用，
    // 但它的存在是必须的，以确保托盘对象在 setup 作用域内保持活动状态。
    // Tauri 在内部会接管其生命周期，所以我们不需要返回它。
    let _tray = TrayIconBuilder::new()
        .menu(&menu)
        .tooltip("您的应用名称")
        .icon(app.default_window_icon().cloned().unwrap())
        .on_menu_event(|app_handle, event| {
            // 3. 处理菜单项点击事件
            match event.id.as_ref() {
                "quit" => {
                    println!("点击了退出按钮");
                    app_handle.exit(0);
                }
                "hide" => {
                    println!("点击了隐藏按钮");
                    if let Some(window) = app_handle.get_webview_window("main") {
                        let _ = window.hide();
                    }
                }
                "show" => {
                    println!("点击了显示按钮");
                    if let Some(window) = app_handle.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
                _ => {}
            }
        })
        .on_tray_icon_event(|tray, event| {
            // 4. 处理托盘图标本身的事件（例如左键单击）
            if let TrayIconEvent::Click {
                button: tauri::tray::MouseButton::Left,
                ..
            } = event
            {
                println!("托盘图标被左键单击");
                let app_handle = tray.app_handle();
                if let Some(window) = app_handle.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .build(app)?;

    // 如果所有操作都成功，返回 Ok
    Ok(())
}
