use crate::error::OneClickLaunchError;
use anyhow::Result;
use db::{launcher, launcher_resource};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::env;
use tauri::{AppHandle, Manager, State};
use tauri_plugin_shell::ShellExt;
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
        .shell()
        .open(path, None)
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
    let url =
        env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://../data/one_click_launch.db".into());
    // 创建连接池
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&url)
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
        .manage(db_manager)
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_persisted_scope::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            store_selected_path,
            web::craete_launcher,
            web::copy_launcher,
            web::delete_launcher,
            web::modify_launcher_sort,
            web::add_resource,
            web::modify_resource_name,
            web::delete_resource,
            web::query_launchers,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
