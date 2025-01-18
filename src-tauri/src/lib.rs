use crate::error::OneClickLaunchError;
use anyhow::Result;
use api::{launcher_api, setting_api, window_api};
use db::{launcher, launcher_resource, settings};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::{env, fs};
use tauri::tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconEvent};
use tauri::Emitter;
use tauri::{tray::TrayIconBuilder, AppHandle, Manager};
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_opener::OpenerExt;
use tracing::info;
mod api;
mod db;
pub mod error;

/// 使用系统默认的程序打开指定的文件或 URL。
///
/// # 参数
/// - `app`: 应用程序状态的引用，用于访问 Tauri 的应用句柄。
/// - `path`: 表示文件路径或 URL 的字符串切片。
///
/// # 返回值
/// - `Ok(())` 表示操作成功。
/// - `Err(OneClickLaunchError)` 表示操作失败。
pub fn open_using_default_program(app: &AppHandle, path: &str) -> Result<(), OneClickLaunchError> {
    app.opener()
        .open_path(path, None::<&str>)
        .map_err(|e| OneClickLaunchError::ExecutionError(e.to_string()))?;
    Ok(())
}

pub struct DatabaseManager {
    pub pool: SqlitePool,
}

#[derive(Clone, serde::Serialize)]
struct Payload {
    args: Vec<String>,
    cwd: String,
}

pub struct WindowContext {
    pub tary_icon: TrayIcon,
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
        .connect(&db_path.to_string_lossy().to_string())
        .await?;

    launcher::initialize(&pool).await?;

    launcher_resource::initialize(&pool).await?;

    settings::initialize(&pool).await?;

    let db_manager: DatabaseManager = DatabaseManager { pool };

    tauri::Builder::default()
        .setup(|app| {
            let tray_icon = TrayIconBuilder::new()
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
                    id if id.starts_with("launch_") => {
                        if let Ok(launcher_id) = &id["launch_".len()..].parse::<i64>() {
                            let _ = app.emit("launch", *launcher_id);
                        }
                    }
                    _ => {
                        tracing::error!("menu item {:?} not handled", event.id);
                    }
                })
                .build(app)?;

            let window_context = WindowContext {
                tary_icon: tray_icon,
            };

            app.manage(window_context);

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
        // 优先注册单例插件
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            info!("{}, {argv:?}, {cwd}", app.package_info().name);
            let windows = app.get_webview_window("main").unwrap();
            if windows.is_visible().unwrap() {
                let _ = windows.unmaximize();
            }
            let _ = windows.show();
            let _ = windows.set_focus();
            app.emit("single-instance", Payload { args: argv, cwd })
                .unwrap();
        }))
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--flag1", "--flag2"]),
        ))
        .invoke_handler(tauri::generate_handler![
            launcher_api::craete_launcher,
            launcher_api::modify_launcher_name,
            launcher_api::copy_launcher,
            launcher_api::delete_launcher,
            launcher_api::modify_launcher_sort,
            launcher_api::add_resource,
            launcher_api::modify_resource_name,
            launcher_api::delete_resource,
            launcher_api::query_launchers,
            launcher_api::launch,
            window_api::hide_window,
            setting_api::save_setting,
            setting_api::read_setting,
            setting_api::read_all_setting,
            window_api::reflush_tray,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
