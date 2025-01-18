use tauri::{
    menu::{IsMenuItem, Menu, MenuItem},
    AppHandle, Manager, State,
};

use crate::{db::launcher, error::OneClickLaunchError, DatabaseManager, WindowContext};

/// 关闭窗口
#[tauri::command]
pub async fn hide_window(app: AppHandle) -> Result<(), OneClickLaunchError> {
    let window = app.get_webview_window("main").unwrap();
    let _ = window.hide();
    Ok(())
}

/// 刷新系统图标菜单
#[tauri::command]
pub async fn reflush_tray(
    app: AppHandle, // 确保运行时类型为 Wry
) -> Result<(), OneClickLaunchError> {
    // 获取全局状态
    let window_context: State<'_, WindowContext> = app.state();
    let database_manager: State<'_, DatabaseManager> = app.state();
    let launchers = launcher::query(&database_manager.pool).await?;

    // 创建动态菜单项
    let mut menu_items: Vec<Box<dyn IsMenuItem<_>>> = Vec::new();
    for launcher in &launchers {
        let id = format!("launch_{}", launcher.id); // 动态生成 ID
        let title = format!("启动: {}", launcher.name); // 动态生成标题
        let item = MenuItem::with_id(&app, &id, &title, true, None::<&str>)?;
        menu_items.push(Box::new(item)); // 将 MenuItem 转为 Box<dyn IsMenuItem>
    }

    // 添加退出按钮
    let quit_item = MenuItem::with_id(&app, "quit", "退出", true, None::<&str>)?;
    menu_items.push(Box::new(quit_item)); // 转为 Box<dyn IsMenuItem>

    // 将 Vec<Box<dyn IsMenuItem>> 转换为 &[&dyn IsMenuItem]
    let menu_items: Vec<&dyn IsMenuItem<_>> = menu_items.iter().map(|item| item.as_ref()).collect();
    let menu = Menu::with_items(&app, &menu_items)?;

    // 设置菜单到托盘图标
    window_context.tray_icon.set_menu(Some(menu))?;
    Ok(())
}
