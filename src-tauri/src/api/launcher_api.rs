use anyhow::Result;
use rand::{Rng, distributions::Alphanumeric};
use serde::Deserialize;
use tauri::{AppHandle, State};
use tracing::info;

use crate::{
    DatabaseManager,
    db::{
        launcher,
        launcher_resource::{self, CreateResourceParam, LauncherResource},
    },
    error::OneClickLaunchError,
    events::{
        EventDispatcher,
        types::{
            LauncherBasicInfoUpdated, LauncherBasicInfoUpdatedPayload, LauncherLaunched,
            LauncherLaunchedPayload,
        },
    },
    open_using_default_program,
};

/// 创建新的启动器
#[tauri::command]
pub async fn craete_launcher(
    app: AppHandle,
    db: State<'_, DatabaseManager>,
    name: Option<String>,
) -> Result<i64, OneClickLaunchError> {
    let name = name
        .filter(|s| !s.is_empty())
        .unwrap_or_else(generate_default_launcher_name);
    let launcher_id = launcher::create(&db.pool, &name, None).await?;

    let _ = EventDispatcher::<LauncherBasicInfoUpdated>::send_event(
        &app,
        LauncherBasicInfoUpdatedPayload {
            launcher_ids: vec![launcher_id],
        },
    );

    Ok(launcher_id)
}

fn generate_default_launcher_name() -> String {
    let mut name = "双击编辑".to_string();
    let rad_str = generate_random_string(4);
    name.push_str(&rad_str);
    name
}

fn generate_random_string(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

/// 修改启动器名称
#[tauri::command]
pub async fn modify_launcher_name(
    app: AppHandle,
    db: State<'_, DatabaseManager>,
    launcher_id: i64,
    name: String,
) -> Result<(), OneClickLaunchError> {
    launcher::modify_launcher_name(&db.pool, launcher_id, &name).await?;

    let _ = EventDispatcher::<LauncherBasicInfoUpdated>::send_event(
        &app,
        LauncherBasicInfoUpdatedPayload {
            launcher_ids: vec![launcher_id],
        },
    );

    Ok(())
}

/// 复制启动器,包含启动器关联的资源数据
#[tauri::command]
pub async fn copy_launcher(
    app: AppHandle,
    db: State<'_, DatabaseManager>,
    launcher_id: i64,
) -> Result<i64, OneClickLaunchError> {
    let mut tx = db.pool.begin().await?;

    // 1. 复制启动器
    let launcher = launcher::find_by_id(&mut tx, launcher_id).await?;

    let launcher_resoures = launcher_resource::query_by_launcher_id(&mut tx, launcher_id).await?;

    let new_name = format!("{}-副本", launcher.name);

    // 2. 复制资源
    let new_launcher_id = launcher::create(&mut tx, &new_name, Some(launcher.sort)).await?;

    for res in launcher_resoures.iter() {
        launcher_resource::create(&mut tx, new_launcher_id, &res.name, &res.path).await?;
    }

    tx.commit().await?;

    let _ = EventDispatcher::<LauncherBasicInfoUpdated>::send_event(
        &app,
        LauncherBasicInfoUpdatedPayload {
            launcher_ids: vec![launcher_id],
        },
    );

    Ok(new_launcher_id)
}

/// 查询启动器列表
#[tauri::command]
pub async fn query_launchers(
    db: State<'_, DatabaseManager>,
) -> Result<Vec<LauncherVo>, OneClickLaunchError> {
    let launchers = launcher::query(&db.pool).await?;

    let resources = launcher_resource::query_all(&db.pool).await?;

    let launcher_vos = launchers
        .into_iter()
        .map(|launcher| {
            let res_vos = resources
                .iter()
                .filter(|resource| resource.launcher_id == launcher.id)
                .map(|resource| LauncherResourceVo {
                    id: resource.id,
                    launcher_id: resource.launcher_id,
                    name: resource.name.clone(),
                    path: resource.path.clone(),
                })
                .collect();
            LauncherVo {
                id: launcher.id,
                name: launcher.name,
                resources: res_vos,
            }
        })
        .collect();

    Ok(launcher_vos)
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct LauncherVo {
    pub id: i64,
    pub name: String,
    pub resources: Vec<LauncherResourceVo>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct LauncherResourceVo {
    pub id: i64,
    pub launcher_id: i64,
    pub name: String,
    pub path: String,
}

/// 删除启动器
#[tauri::command]
pub async fn delete_launcher(
    app: AppHandle,
    db: State<'_, DatabaseManager>,
    launcher_id: i64,
) -> Result<(), OneClickLaunchError> {
    let mut tx = db.pool.begin().await?;

    launcher::delete_by_id(&mut tx, launcher_id).await?;

    launcher_resource::delete_by_launcher(&mut tx, launcher_id).await?;

    tx.commit().await?;

    let _ = EventDispatcher::<LauncherBasicInfoUpdated>::send_event(
        &app,
        LauncherBasicInfoUpdatedPayload {
            launcher_ids: vec![launcher_id],
        },
    );

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
    app: AppHandle,
    db: State<'_, DatabaseManager>,
    launchers: Vec<LauncherSort>,
) -> Result<(), OneClickLaunchError> {
    let mut tx = db.pool.begin().await?;

    for ls in launchers.iter() {
        launcher::modify_launcher_sort(&mut tx, ls.id, ls.sort).await?
    }

    tx.commit().await?;

    let _ = EventDispatcher::<LauncherBasicInfoUpdated>::send_event(
        &app,
        LauncherBasicInfoUpdatedPayload {
            launcher_ids: launchers.iter().map(|e| e.id).collect(),
        },
    );

    Ok(())
}

/// 为启动器添加资源
#[tauri::command]
pub async fn add_resource(
    db: State<'_, DatabaseManager>,
    launcher_id: i64,
    name: Option<String>,
    path: &str,
) -> Result<i64, OneClickLaunchError> {
    let name = name.unwrap_or_else(|| generate_name(path));

    let resource_id = launcher_resource::create(&db.pool, launcher_id, &name, path).await?;

    Ok(resource_id)
}

/// 为启动器添加资源
#[tauri::command]
pub async fn add_resources(
    db: State<'_, DatabaseManager>,
    launcher_id: i64,
    resources: Vec<ResourceParam>,
) -> Result<(), OneClickLaunchError> {
    let crps = resources
        .into_iter()
        .map(|r| CreateResourceParam {
            name: r
                .name
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| generate_name(r.path.as_str())),
            path: r.path,
        })
        .collect::<Vec<CreateResourceParam>>();

    launcher_resource::create_resources(&db.pool, launcher_id, &crps).await?;

    Ok(())
}

#[derive(Deserialize, Debug)]
pub struct ResourceParam {
    pub name: Option<String>,
    pub path: String,
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
    app: AppHandle,
    db: State<'_, DatabaseManager>,
    launcher_id: i64,
) -> Result<(), OneClickLaunchError> {
    let resources = launcher_resource::query_by_launcher_id(&db.pool, launcher_id).await?;

    launch_resources(&app, &resources);

    EventDispatcher::<LauncherLaunched>::send_event(
        &app,
        LauncherLaunchedPayload {
            launcher_ids: vec![launcher_id],
        },
    )?;

    Ok(())
}

pub fn launch_resources(app: &AppHandle, resources: &[LauncherResource]) {
    for resource in resources.iter() {
        if let Err(e) = open_using_default_program(app, resource.path.as_str()) {
            info!(
                "启动资源失败,资源名称: {:?},资源路径: {:?},错误信息: {:?}",
                &resource.name, &resource.path, e
            );
        }
    }
}
