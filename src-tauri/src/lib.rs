use crate::error::OneClickLaunchError;
use crate::launcher::LaunchAble;
use anyhow::Result;
use std::env;
use tauri::{AppHandle, Manager, State};
use tauri_plugin_shell::ShellExt;
pub mod error;
mod launcher;
pub mod resource;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(app: State<'_, AppState>, name: &str) -> Result<String, OneClickLaunchError> {
    app.app_handle
        .shell()
        .open(name, None)
        .map_err(|e| OneClickLaunchError::ExecutionError(e.to_string()))?;

    Ok(format!("Hello, {}! You've been greeted from Rust!", name))
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
