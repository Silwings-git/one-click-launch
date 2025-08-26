use anyhow::Result;
use api::window_api::{ScaleFactorChangedState, setup_tray};
use api::{launcher_api, setting_api, window_api};
use constants::AUTO_START_FLAG;
use db::{launcher, launcher_resource, settings};
use events::EventDispatcher;
use events::system_listeners::register_system_listeners;
use events::types::{ApplicationStartupComplete, ApplicationStartupCompletePayload};
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use std::path::PathBuf;
use std::sync::Mutex;
use std::{env, fs};
use tauri::Emitter;
use tauri::tray::TrayIcon;
use tauri::{AppHandle, Manager};
use tauri_plugin_autostart::MacosLauncher;
use tracing::{debug, info};
mod api;
mod constants;
mod db;
pub mod error;
mod events;

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

#[cfg(not(feature = "portable"))]
fn get_db_path() -> Result<PathBuf> {
    // 获取用户的 AppData 目录路径
    let app_data = env::var("APPDATA")?;

    // 拼接到特定的文件夹路径
    let db_path = PathBuf::from(app_data)
        .join("one_click_launch")
        .join("data")
        .join("one_click_launch.db");

    Ok(db_path)
}

#[cfg(feature = "portable")]
fn get_db_path() -> Result<PathBuf> {
    // 便携版：使用当前可执行文件的目录
    let exe_path = env::current_exe()?;
    let db_path = exe_path
        .parent()
        .ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::Other, "Cannot get executable directory")
        })?
        .join("data")
        .join("one_click_launch.db");

    Ok(db_path)
}

async fn init_db() -> Result<DatabaseManager> {
    let db_path = get_db_path()?;

    debug!("db_path:{:?}", db_path);

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
        info!("Database file created at {:?}", db_path);
    }

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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() -> Result<()> {
    let db_manager = init_db().await?;

    tauri::Builder::default()
        .setup(move |app| {
            // 注册监听器,之后添加新的监听器时在这个方法内部添加
            register_listeners(app.handle());

            // 初始化窗口
            setup_tray(app.handle())?;

            // 当所有初始化都完成后发送应用程序启动完成事件
            EventDispatcher::<ApplicationStartupComplete>::send_event(
                app.handle(),
                ApplicationStartupCompletePayload {
                    args: env::args().collect(),
                },
            )?;

            Ok(())
        })
        .on_window_event(window_api::handle_window_event)
        .manage(db_manager)
        .manage(ScaleFactorChangedState {
            last_reset: Mutex::new(None),
        })
        // 优先注册单例插件
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            info!("run app: {}, {argv:?}, {cwd}", app.package_info().name);

            if let Some(Ok(launcher_id)) =
                extract_arg_value(&argv, "launch").map(|value| value.parse::<i64>())
            {
                let app_cloned = app.clone();
                tokio::spawn(async move {
                    if let Err(e) = launcher_api::launch(app_cloned, launcher_id).await {
                        tracing::error!("launcher launch fail: {}", e);
                    }
                });
            } else {
                let windows = app
                    .get_webview_window(constants::MAIN_WINDOW_LABEL)
                    .unwrap();
                if windows.is_visible().unwrap() {
                    let _ = windows.unmaximize();
                }
                let _ = windows.show();
                let _ = windows.set_focus();
            }
            app.emit("single-instance", Payload { args: argv, cwd })
                .unwrap();
        }))
        .plugin(tauri_plugin_window_state::Builder::default().build())
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
            launcher_api::add_resources,
            launcher_api::modify_resource_name,
            launcher_api::delete_resource,
            launcher_api::query_launchers,
            launcher_api::launch,
            launcher_api::open_path,
            launcher_api::create_handler_shortcut,
            setting_api::save_setting,
            setting_api::read_setting,
            setting_api::read_all_setting,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}

fn register_listeners(app: &AppHandle) {
    // 注册系统级别的监听器
    register_system_listeners(app);
}

fn extract_arg_value(argv: &[String], key: &str) -> Option<String> {
    let mut iter = argv.iter();
    while let Some(arg) = iter.next() {
        if arg == key {
            if let Some(val) = iter.next() {
                return Some(val.clone());
            }
        }
    }
    None
}
