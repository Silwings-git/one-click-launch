use crate::error::OneClickLaunchError;
use anyhow::Result;
use std::env;
use tauri::{AppHandle, Manager, State};
use tauri_plugin_shell::ShellExt;
pub mod error;

mod db;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(app: State<'_, AppState>, path: &str) -> Result<String, OneClickLaunchError> {
    open_using_default_program(app, path)?;

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
fn open_using_default_program(
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
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
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
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_persisted_scope::init())
        .invoke_handler(tauri::generate_handler![greet, store_selected_path])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
