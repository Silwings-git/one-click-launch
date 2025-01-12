use anyhow::Result;
use tauri::State;
use tracing::info;

use crate::{
    db::{launcher, launcher_resource},
    error::OneClickLaunchError,
    open_using_default_program, AppState, DatabaseManager,
};

/// 创建新的启动器
#[tauri::command]
pub async fn craete_launcher(
    db: State<'_, DatabaseManager>,
    name: &str,
) -> Result<i64, OneClickLaunchError> {
    let launcher_id = launcher::create(&db.pool, name).await?;
    Ok(launcher_id)
}

/// 复制启动器,包含启动器关联的资源数据
#[tauri::command]
pub async fn copy_launcher(
    db: State<'_, DatabaseManager>,
    launcher_id: i64,
) -> Result<i64, OneClickLaunchError> {
    let mut tx = db.pool.begin().await?;

    // 1. 复制启动器
    let launcher = launcher::find_by_id(&mut tx, launcher_id).await?;

    let launcher_resoures = launcher_resource::query_by_launcher_id(&mut tx, launcher_id).await?;

    // 2. 复制资源
    let new_launcher_id = launcher::create(&mut tx, &launcher.name).await?;

    for res in launcher_resoures.iter() {
        launcher_resource::create(&mut tx, new_launcher_id, &res.name, &res.path).await?;
    }

    tx.commit().await?;

    Ok(new_launcher_id)
}

/// 删除启动器
#[tauri::command]
pub async fn delete(
    db: State<'_, DatabaseManager>,
    launcher_id: i64,
) -> Result<(), OneClickLaunchError> {
    let mut tx = db.pool.begin().await?;

    launcher::delete_by_id(&mut tx, launcher_id).await?;

    let resources = launcher_resource::query_by_launcher_id(&mut tx, launcher_id).await?;

    for res in resources.iter() {
        launcher_resource::delete_by_id(&mut tx, res.id).await?;
    }

    tx.commit().await?;

    Ok(())
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct LauncherSort {
    id: i64,
    sort: i32,
}

/// 调整启动器顺序
#[tauri::command]
pub async fn modify_launcher_sort(
    db: State<'_, DatabaseManager>,
    launchers: Vec<LauncherSort>,
) -> Result<(), OneClickLaunchError> {
    let mut tx = db.pool.begin().await?;

    for ls in launchers.iter() {
        launcher::modify_launcher_sort(&mut tx, ls.id, ls.sort).await?
    }

    tx.commit().await?;

    Ok(())
}

/// 为启动器添加资源
#[tauri::command]
pub async fn add_resource(
    db: State<'_, DatabaseManager>,
    launcher_id: i64,
    path: &str,
) -> Result<i64, OneClickLaunchError> {
    let name = generate_name(path);

    let resource_id = launcher_resource::create(&db.pool, launcher_id, &name, path).await?;

    Ok(resource_id)
}

fn generate_name(path: &str) -> String {
    if path.starts_with("http") {
        return path.to_string();
    }

    match path.rfind('\\') {
        Some(index) => path[index + 1..].to_string(),
        None => path.to_string(),
    }
}

/// 修改资源名称
#[tauri::command]
pub async fn modify_resource_name(
    db: State<'_, DatabaseManager>,
    resource_id: i64,
    name: &str,
) -> Result<(), OneClickLaunchError> {
    launcher_resource::modify_name(&db.pool, resource_id, name).await?;
    Ok(())
}

/// 删除启动器中的资源
#[tauri::command]
pub async fn delete_resource(
    db: State<'_, DatabaseManager>,
    resource_id: i64,
) -> Result<(), OneClickLaunchError> {
    launcher_resource::delete_by_id(&db.pool, resource_id).await?;
    Ok(())
}

/// 启动启动器
#[tauri::command]
pub async fn launch(
    app: State<'_, AppState>,
    db: State<'_, DatabaseManager>,
    launcher_id: i64,
) -> Result<(), OneClickLaunchError> {
    let resources = launcher_resource::query_by_launcher_id(&db.pool, launcher_id).await?;

    for resource in resources.iter() {
        if let Err(e) = open_using_default_program(app.clone(), resource.path.as_str()) {
            info!(
                "启动资源失败,资源名称: {:?},资源路径: {:?},错误信息: {:?}",
                &resource.name, &resource.path, e
            );
        }
    }

    Ok(())
}
