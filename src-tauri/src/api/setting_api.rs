use tauri::State;

use crate::{
    DatabaseManager,
    db::settings::{self, Settings},
    error::OneClickLaunchError,
};

/// 保存设置
#[tauri::command]
pub async fn save_setting(
    db: State<'_, DatabaseManager>,
    key: String,
    value: String,
) -> Result<(), OneClickLaunchError> {
    let setting = Settings { key, value };
    settings::save(&db.pool, &setting).await?;
    Ok(())
}

/// 读取设置
#[tauri::command]
pub async fn read_setting(
    db: State<'_, DatabaseManager>,
    key: String,
) -> Result<Option<Settings>, OneClickLaunchError> {
    let setting = settings::read(&db.pool, &key).await?;
    Ok(setting)
}

/// 读取全部设置
#[tauri::command]
pub async fn read_all_setting(
    db: State<'_, DatabaseManager>,
) -> Result<Vec<Settings>, OneClickLaunchError> {
    let setting = settings::read_all(&db.pool).await?;
    Ok(setting)
}
