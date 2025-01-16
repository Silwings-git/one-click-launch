use crate::error::OneClickLaunchError;
use anyhow::Result;
use db::{launcher, launcher_resource};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::{env, fs};
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, State,
};
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_opener::OpenerExt;
use tracing::info;
mod db;
pub mod error;
mod web;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(app: State<'_, AppState>, path: &str) -> Result<String, OneClickLaunchError> {
    match open_using_default_program(app, path) {
        Ok(_) => {}
        Err(e) => info!("打开启动器资源失败: {:?}", e),
    }

    Ok(format!("Hello, {}! You've been greeted from Rust!", path))
}

/// 使用系统默认的程序打开指定的文件或 URL。
///
/// # 参数
/// - `app`: 应用程序状态的引用，用于访问 Tauri 的应用句柄。
/// - `path`: 表示文件路径或 URL 的字符串切片。
///
/// # 返回值
/// - `Ok(())` 表示操作成功。
/// - `Err(OneClickLaunchError)` 表示操作失败。
///
/// # 错误
/// - 如果调用 Tauri 的 `shell().open` 方法失败，将返回 `OneClickLaunchError::ExecutionError`。
pub fn open_using_default_program(
    app: State<'_, AppState>,
    path: &str,
) -> Result<(), OneClickLaunchError> {
    app.app_handle
        .opener()
        .open_path(path, None::<&str>)
        .map_err(|e| OneClickLaunchError::ExecutionError(e.to_string()))?;
    Ok(())
}

#[tauri::command]
fn store_selected_path(path: String) -> Result<(), OneClickLaunchError> {
    println!("存储的路径: {}", path);
    Ok(())
}

pub struct AppState {
    app_handle: AppHandle,
}

pub struct DatabaseManager {
    pub pool: SqlitePool,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() -> Result<()> {
    let db_path = env::current_exe()?
        .parent()
        .map(|dir| dir.join("data").join("one_click_launch.db"))
        .unwrap();

    // 打印数据库路径用于调试
    println!("db_path:{:?}", db_path);

    // 确保数据库所在的目录存在
    if let Some(parent) = db_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // 检查数据库文件是否存在，如果不存在则创建空文件
    if !db_path.exists() {
        // 创建空的数据库文件
        fs::File::create(&db_path)?;
        println!("Database file created at {:?}", db_path);
    }

    // 创建连接池
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(db_path.to_str().unwrap())
        .await?;

    launcher::initialize(&pool).await?;

    launcher_resource::initialize(&pool).await?;

    let db_manager: DatabaseManager = DatabaseManager { pool };

    tauri::Builder::default()
        .setup(|app| {
            // 将 app 存储到 State 中
            let app_state = AppState {
                app_handle: app.handle().clone(),
            };
            // 传递给 State
            app.manage(app_state);
            Ok(())
        })
        .setup(|app| {
            let quit_i = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit_i])?;

            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .menu_on_left_click(false)
                .icon(app.default_window_icon().unwrap().clone())
                .on_tray_icon_event(|tray, event| match event {
                    TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } => {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.unminimize();
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    _ => {
                        println!("unhandled event {event:?}");
                    }
                })
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        info!("quit menu item was clicked");
                        app.exit(0);
                    }
                    _ => {
                        tracing::error!("menu item {:?} not handled", event.id);
                    }
                })
                .build(app)?;
            Ok(())
        })
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                api.prevent_close();
                let _ = window.hide();
            }
            _ => {}
        })
        .manage(db_manager)
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_persisted_scope::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--flag1", "--flag2"]),
        ))
        .invoke_handler(tauri::generate_handler![
            greet,
            store_selected_path,
            web::craete_launcher,
            web::modify_launcher_name,
            web::copy_launcher,
            web::delete_launcher,
            web::modify_launcher_sort,
            web::add_resource,
            web::modify_resource_name,
            web::delete_resource,
            web::query_launchers,
            web::launch,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
