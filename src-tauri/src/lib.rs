use crate::error::OneClickLaunchError;
use anyhow::Result;
use api::{launcher_api, setting_api};
use constants::{AUTO_START_FLAG, WINDOW_MIN_HEIGHT, WINDOW_MIN_WIDTH};
use db::{launcher, launcher_resource, settings};
use events::EventDispatcher;
use events::system_listeners::register_system_listeners;
use events::types::{
    ApplicationStartupComplete, ApplicationStartupCompletePayload, DragDropResource,
    DragDropResourcePaylod,
};
use sqlx::{Executor, Sqlite};
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::Instant;
use std::{env, fs};
use tauri::tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconEvent};
use tauri::{AppHandle, Manager, tray::TrayIconBuilder};
use tauri::{DragDropEvent, Emitter};
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_opener::OpenerExt;
use tracing::{debug, info};
mod api;
mod constants;
mod db;
pub mod error;
mod events;

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
        info!("Database file created at {:?}", db_path);
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

async fn check_launch_then_exit<'a, E>(executor: E) -> Result<bool, OneClickLaunchError>
where
    E: Executor<'a, Database = Sqlite>,
{
    match settings::read(executor, "launch_then_exit").await {
        Ok(Some(setting)) => Ok(string_to_bool(&setting.value)),
        _ => Ok(false),
    }
}

fn string_to_bool(s: &str) -> bool {
    matches!(s.trim().to_lowercase().as_str(), "true")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() -> Result<()> {
    let db_manager = init_db().await?;

    tauri::Builder::default()
        .setup(move |app| {
            // 初始化窗口
            setup_tray(app.handle())?;

            // 注册监听器,之后添加新的监听器时在这个方法内部添加
            register_listeners(app.handle());

            // 当所有初始化都完成后发送应用程序启动完成事件
            EventDispatcher::<ApplicationStartupComplete>::send_event(
                app.handle(),
                ApplicationStartupCompletePayload {
                    args: env::args().collect(),
                },
            )?;

            Ok(())
        })
        .on_window_event(handle_window_event)
        .manage(db_manager)
        .manage(ScaleFactorChangedState {
            last_reset: Mutex::new(None),
        })
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
            launcher_api::add_resources,
            launcher_api::modify_resource_name,
            launcher_api::delete_resource,
            launcher_api::query_launchers,
            launcher_api::launch,
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

/// 初始化窗口
fn setup_tray(app: &AppHandle) -> Result<()> {
    let tray_icon = TrayIconBuilder::new()
        // 设置系统托盘的提示,鼠标悬浮时会显示
        .tooltip(constants::APPLICATION_NAME)
        // 左键系统托盘时不显示菜单
        .show_menu_on_left_click(false)
        // 使用主窗口的icon作为系统托盘的图标
        .icon(app.default_window_icon().unwrap().clone())
        // 系统托盘的点击事件
        .on_tray_icon_event(|tray, event| match event {
            TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } => {
                // 当用户点击系统托盘时使应用程序取消最小化,显示并聚焦
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.unminimize();
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            _ => {
                debug!("unhandled event {event:?}");
            }
        })
        .on_menu_event(|app, event| match event.id.as_ref() {
            "quit" => {
                // 退出应用程序
                info!("quit menu item was clicked");
                app.exit(0);
            }
            id if id.starts_with("launch_") => {
                // 系统托盘的菜单id由"launch_"与启动器id拼接而成, 点击菜单后通过解析id,找到要启动的启动器触发启动
                let id = &id["launch_".len()..];
                if let Ok(launcher_id) = id.parse::<i64>() {
                    let app_cloned = app.clone();
                    tauri::async_runtime::spawn(async move {
                        let inner_app = app_cloned.clone();
                        let db = inner_app.state();
                        launcher_api::launch(app_cloned, db, launcher_id).await
                    });
                }
            }
            _ => {
                tracing::error!("menu item {:?} not handled", event.id);
            }
        })
        .build(app)?;

    let window_context = WindowContext { tray_icon };

    app.manage(window_context);

    Ok(())
}

/// 用于保存上次处理分辨率变更事件的时间
struct ScaleFactorChangedState {
    last_reset: Mutex<Option<Instant>>,
}

fn handle_window_event(window: &tauri::Window, event: &tauri::WindowEvent) {
    match event {
        tauri::WindowEvent::CloseRequested { api, .. } => {
            api.prevent_close();
            let _ = window.hide();

            let app_handle = window.app_handle().clone();
            tauri::async_runtime::spawn(async move {
                let db = app_handle.state::<DatabaseManager>();
                let setting = setting_api::read_setting(db, constants::CLOSE_MAIN_PANEL_KEY).await;

                match setting {
                    Ok(Some(setting)) if constants::CLOSE_MAIN_PANEL_EXIT == setting.value => {
                        app_handle.exit(0);
                    }
                    _ => {}
                }
            });
        }
        tauri::WindowEvent::ScaleFactorChanged {
            scale_factor,
            new_inner_size,
            ..
        } => {
            print!(
                "ScaleFactorChanged scale_factor: {}, new_inner_size: {:?}",
                scale_factor, new_inner_size
            );

            let now = Instant::now();
            let state = window.state::<ScaleFactorChangedState>();
            let mut lock = state.last_reset.try_lock();

            if let Ok(ref mut last_reset) = lock {
                // 500ms防抖间隔
                if last_reset.map_or(true, |t| now.duration_since(t).as_millis() > 500) {
                    if let Ok(physical_size) = window.inner_size() {
                        // 如果窗口大小异常，强制调整到正常大小
                        if physical_size.width != WINDOW_MIN_WIDTH
                            || physical_size.height != WINDOW_MIN_HEIGHT
                        {
                            let _ = window.set_size(tauri::Size::Logical(tauri::LogicalSize {
                                width: WINDOW_MIN_WIDTH as f64,
                                height: WINDOW_MIN_HEIGHT as f64,
                            }));
                            **last_reset = Some(now);
                            debug!("DPI发生变化,触发窗口大小重置");
                        }
                    }
                } else {
                    debug!("DPI重置窗口防抖生效");
                }
            }
        }
        tauri::WindowEvent::DragDrop(drag_drop_event) => match drag_drop_event {
            DragDropEvent::Drop { paths, .. } if !paths.is_empty() => {
                let _ = EventDispatcher::<DragDropResource>::send_event(
                    window.app_handle(),
                    DragDropResourcePaylod {
                        paths: paths.clone(),
                    },
                );
            }
            _ => {}
        },
        tauri::WindowEvent::Resized(physical_size) => {
            if physical_size.width == 0 && physical_size.height == 0 {
                // 页面最小化时忽略
                return;
            }
            // 如果窗口大小过小，强制调整到正常大小
            if physical_size.width < WINDOW_MIN_WIDTH || physical_size.height < WINDOW_MIN_HEIGHT {
                let _ = window.set_size(tauri::Size::Logical(tauri::LogicalSize {
                    width: WINDOW_MIN_WIDTH as f64,
                    height: WINDOW_MIN_HEIGHT as f64,
                }));
                debug!("窗口大小过小，触发窗口大小重置: ps: {:?}", physical_size);
            }
        }
        _ => {}
    }
}
