use sqlx::{Executor, Sqlite};
use tauri::{AppHandle, State};

use crate::{
    DatabaseManager,
    db::settings::{self, Settings},
    error::OneClickLaunchError,
    events::{
        EventDispatcher,
        types::{SettingUpdated, SettingUpdatedPayload},
    },
};

/// 保存设置
#[tauri::command]
pub async fn save_setting(
    app: AppHandle,
    db: State<'_, DatabaseManager>,
    key: String,
    value: String,
) -> Result<(), OneClickLaunchError> {
    let setting = Settings { key, value };
    settings::save(&db.pool, &setting).await?;

    let _ = EventDispatcher::<SettingUpdated>::send_event(
        &app,
        SettingUpdatedPayload {
            key: setting.key.clone(),
            value: setting.value.clone(),
        },
    );

    Ok(())
}

/// 读取设置
#[tauri::command]
pub async fn read_setting(
    db: State<'_, DatabaseManager>,
    key: &str,
) -> Result<Option<Settings>, OneClickLaunchError> {
    let setting = settings::read(&db.pool, key).await?;
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

pub async fn check_launch_then_exit<'a, E>(executor: E) -> Result<bool, OneClickLaunchError>
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
