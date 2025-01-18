use tauri::{AppHandle, Manager};

use crate::error::OneClickLaunchError;

/// 关闭窗口
#[tauri::command]
pub async fn hide_window(app: AppHandle) -> Result<(), OneClickLaunchError> {
    let window = app.get_webview_window("main").unwrap();
    let _ = window.hide();
    Ok(())
}
