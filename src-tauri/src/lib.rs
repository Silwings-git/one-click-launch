use crate::error::OneClickLaunchError;
use anyhow::Result;
use api::{launcher_api, setting_api, window_api};
use db::{launcher, launcher_resource, settings};
use lazy_static::lazy_static;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use std::path::PathBuf;
use std::sync::Mutex;
use std::{env, fs};
use tauri::Emitter;
use tauri::tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconEvent};
use tauri::{AppHandle, Manager, tray::TrayIconBuilder};
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_opener::OpenerExt;
use tracing::{debug, info};
mod api;
mod db;
pub mod error;

lazy_static! {
    static ref AUTO_START_FLAG: String = "--auto".to_string();
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
    pub tray_icon: TrayIcon,
}

fn get_db_path() -> Result<PathBuf> {
    // 获取用户的 AppData 目录路径
    let app_data = env::var("APPDATA")?;

    // 拼接到特定的文件夹路径
    let db_path = PathBuf::from(app_data)
        .join("one_click_launch")
        .join("data")
        .join("one_click_launch.db");

    // 确保目录存在
    if let Some(parent) = db_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }

    // 检查数据库文件是否存在，如果不存在则创建空文件
    if !db_path.exists() {
        // 创建空的数据库文件
        fs::File::create(&db_path)?;
        println!("Database file created at {:?}", db_path);
    }

    Ok(db_path)
}

async fn init_db() -> Result<DatabaseManager> {
    let db_path = get_db_path()?;

    // 打印数据库路径用于调试
    debug!("db_path:{:?}", db_path);

    // 创建连接池
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(db_path.to_string_lossy().as_ref())
        .await?;

    launcher::initialize(&pool).await?;

    launcher_resource::initialize(&pool).await?;

    settings::initialize(&pool).await?;

    Ok(DatabaseManager { pool })
}

/// 获取自启编组id集
async fn query_auto_start_launcher_ids(
    db_manager: &DatabaseManager,
) -> Mutex<Option<Vec<launcher_resource::LauncherResource>>> {
    let auto_start_resources = if let Ok(Some(settings)) =
        settings::read(&db_manager.pool, "auto_start_launcher_ids").await
    {
        let auto_start_launcher_ids: Vec<i64> = serde_json::from_str(&settings.value).unwrap();
        launcher_resource::query_by_launcher_ids(&db_manager.pool, auto_start_launcher_ids)
            .await
            .ok()
    } else {
        None
    };
    Mutex::new(auto_start_resources)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() -> Result<()> {
    let db_manager = init_db().await?;

    // 获取命令行参数
    let args: Vec<String> = env::args().collect();
    debug!("命令行参数: {:?}", args);
    // 检查是否包含 `--auto` 参数, 当系统自动启动应用程序时需要加载自启动编组进行启动
    let auto_start_resources = if args.contains(&AUTO_START_FLAG) {
        query_auto_start_launcher_ids(&db_manager).await
    } else {
        Mutex::new(None)
    };

    tauri::Builder::default()
        .setup(move |app| {
            let tray_icon = TrayIconBuilder::new()
                .tooltip("一键启动")
                .show_menu_on_left_click(false)
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
                        // println!("unhandled event {event:?}");
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

            let window_context = WindowContext { tray_icon };

            app.manage(window_context);

            // 检查是否包含 `--auto` 参数
            if args.contains(&AUTO_START_FLAG) {
                if let Ok(mut data) = auto_start_resources.lock() {
                    if let Some(lrs) = data.take() {
                        if !lrs.is_empty() {
                            launcher_api::launch_launcher_resources(app.app_handle(), &lrs);
                            let res = window_api::hide_window_sync(app.app_handle().clone());
                            debug!("启动自启编组结果: {:?}. 编组信息: {:?}", res, lrs);
                        } else {
                            debug!("开启了自启编组,但编组为空");
                        }
                    }
                }
            }

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.hide();
            }
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
            // 操作系统以开机自启启动应用程序时携带的参数
            Some(vec![AUTO_START_FLAG.as_str()]),
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
            window_api::refresh_tray,
            window_api::change_windows_theme,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
