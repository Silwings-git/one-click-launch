use tauri::{AppHandle, Manager};
use tracing::{debug, error};

use crate::{
    DatabaseManager,
    api::{launcher_api, window_api},
    check_launch_then_exit,
    constants::{AUTO_START_FLAG, THEME},
    db::{launcher_resource, settings},
    events::EventDispatcher,
};

use super::{
    EventSystem,
    types::{
        ApplicationStartupComplete, ApplicationStartupCompletePayload, LauncherBasicInfoUpdated,
        LauncherLaunched, LauncherLaunchedPayload, SettingUpdated, SettingUpdatedPayload,
    },
};

pub fn register_system_listeners(app: &AppHandle) {
    register_launcher_launched_listeners(app);
    register_launcher_basic_info_updated_listeners(app);
    register_application_startup_complete_listeners(app);
    register_setting_updated_listeners(app);
}

/// 注册启动器启动事件监听器
fn register_launcher_launched_listeners(app: &AppHandle) {
    let app_handle = app.clone();
    EventSystem::register_listener(app, LauncherLaunched, move |payload| {
        debug!("launcher_launched_listeners 处理中");
        hide_window(&app_handle, &payload);
        launch_then_exit(&app_handle);
        debug!("launcher_launched_listeners 处理完成");
    });
}

/// 注册启动器基础信息变更事件
fn register_launcher_basic_info_updated_listeners(app: &AppHandle) {
    let app_handle = app.clone();
    EventSystem::register_listener(app, LauncherBasicInfoUpdated, move |_payload| {
        debug!("launcher_basic_info_updated_listeners 处理中");
        refresh_tray(&app_handle);
        debug!("launcher_basic_info_updated_listeners 处理完成");
    });
}

/// 注册应用程序启动完成监听器
fn register_application_startup_complete_listeners(app: &AppHandle) {
    let app_cloned = app.clone();
    EventSystem::register_listener(app, ApplicationStartupComplete, move |payload| {
        debug!("application_startup_complete_listeners 处理中");
        refresh_tray(&app_cloned);
        launch_auto_start_launchers(&app_cloned, &payload);
        debug!("application_startup_complete_listeners 处理完成");
    });
}

/// 注册应用程序设置修改监听器
fn register_setting_updated_listeners(app: &AppHandle) {
    let app_cloned = app.clone();
    EventSystem::register_listener(app, SettingUpdated, move |payload| {
        debug!("setting_updated_listeners 处理中");
        change_theme(&app_cloned, &payload);
        debug!("setting_updated_listeners 处理完成");
    });
}

/// 刷新系统图标
fn refresh_tray(app: &AppHandle) {
    let app_cloned = app.clone();
    tauri::async_runtime::spawn(async move {
        if let Err(e) = window_api::refresh_tray(app_cloned).await {
            error!("刷新系统图标失败.{:?}", e);
        } else {
            debug!("刷新系统图标失败成功.");
        }
    });
}

/// 注册应用程序启动完成监听器: 启动自启动启动器
fn launch_auto_start_launchers(app: &AppHandle, payload: &ApplicationStartupCompletePayload) {
    // 检查启动参数, 当命令包含`--auto`时表示是操作系统触发的自动启动
    if payload.args.contains(&AUTO_START_FLAG) {
        debug!(
            "launch_auto_start_launchers 判断为自动启动, 命令行参数: {:?}",
            payload.args
        );

        let inner_app = app.clone();

        tauri::async_runtime::spawn(async move {
            let app_cloned = inner_app.clone();

            let db_manager = app_cloned.state::<DatabaseManager>();

            if let Ok(Some(settings)) =
                settings::read(&db_manager.pool, "auto_start_launcher_ids").await
            {
                if let Ok(auto_start_launcher_ids) =
                    serde_json::from_str::<Vec<i64>>(&settings.value)
                {
                    if let Ok(launcher_resources) = launcher_resource::query_by_launcher_ids(
                        &db_manager.pool,
                        &auto_start_launcher_ids,
                    )
                    .await
                    {
                        // 如果用户设置的自启启动器为空则不执行启动
                        if !launcher_resources.is_empty() {
                            launcher_api::launch_resources(&app_cloned, &launcher_resources);
                            debug!("自启启动器已启动. 启动器信息: {:?}", launcher_resources);
                            let _ = EventDispatcher::<LauncherLaunched>::send_event(
                                &app_cloned,
                                LauncherLaunchedPayload {
                                    launcher_ids: auto_start_launcher_ids.clone(),
                                },
                            );
                        } else {
                            debug!("开启了自启启动器,但启动器为空");
                        }
                    }
                } else {
                    debug!(
                        "launch_auto_start_launchers 自启动启动器反序列化失败,原始数据: {}",
                        &settings.value
                    );
                }
            } else {
                debug!("launch_auto_start_launchers 没有找到自启动启动器");
            }
        });
    } else {
        debug!(
            "launch_auto_start_launchers 判断为非自动启动, 命令行参数: {:?}",
            payload.args
        );
    }
}

fn launch_then_exit(app: &AppHandle) {
    let app_cloned = app.clone();
    tauri::async_runtime::spawn(async move {
        let db = app_cloned.state::<DatabaseManager>();
        if let Ok(_exit @ true) = check_launch_then_exit(&db.pool).await {
            debug!("launch_then_exit 已设置启动后退出, 正在退出程序.");
            app_cloned.exit(0);
        } else {
            debug!("launch_then_exit 未设置启动后退出");
        }
    });
}

/// 隐藏窗口: 启动器启动隐藏窗口
fn hide_window(app: &AppHandle, payload: &LauncherLaunchedPayload) {
    if !payload.launcher_ids.is_empty() && payload.launcher_ids.len() == 1 {
        let _ = window_api::hide_window(app);
        debug!("hide_window 已隐藏");
    } else {
        debug!("hide_window 启动的启动器数量不是1, 不执行隐藏");
    }
}

/// 隐藏窗口: 启动器启动隐藏窗口
fn change_theme(app: &AppHandle, payload: &SettingUpdatedPayload) {
    if THEME == payload.key {
        match window_api::change_windows_theme(app, &payload.value) {
            Ok(_) => {
                debug!("修改主题成功,新主题: {}", payload.value)
            }
            Err(_) => {
                error!("修改主题失败,新主题: {}", payload.value)
            }
        }
    }
}
