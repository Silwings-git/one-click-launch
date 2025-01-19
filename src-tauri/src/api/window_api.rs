use tauri::{
    menu::{MenuBuilder, MenuItem},
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
pub async fn reflush_tray(app: AppHandle) -> Result<(), OneClickLaunchError> {
    // 获取全局状态
    let window_context: State<'_, WindowContext> = app.state();
    let database_manager: State<'_, DatabaseManager> = app.state();
    let launchers = launcher::query(&database_manager.pool).await?;

    let mut menu_builder = MenuBuilder::new(&app);
    // 创建动态菜单项
    for launcher in &launchers {
        let id = format!("launch_{}", launcher.id);
        let title = format!("启动: {}", launcher.name);
        menu_builder =
            menu_builder.item(&MenuItem::with_id(&app, &id, &title, true, None::<&str>)?);
    }
    // 添加退出按钮
    let quit_item = MenuItem::with_id(&app, "quit", "退出", true, None::<&str>)?;
    let menu = menu_builder.separator().item(&quit_item).build()?;

    // 设置菜单到托盘图标
    window_context.tray_icon.set_menu(Some(menu))?;
    Ok(())
}
