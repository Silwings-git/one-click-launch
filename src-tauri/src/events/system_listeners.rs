use tauri::{AppHandle, Manager};
use tracing::{debug, error};

use crate::{
    DatabaseManager,
    api::{
        launcher_api,
        setting_api::{self, check_launch_then_exit},
        window_api,
    },
    constants::{
        self, AUTO_START_FLAG, AUTO_START_LAUNCHER_IDS_KEY, HIDE_AFTER_AUTO_START_KEY,
        LAUNCH_SPECIFIED_LAUNCHER_KEY, THEME_KEY,
    },
    db::{launcher_resource, settings},
    events::EventDispatcher,
    extract_arg_value,
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
        hide_after_auto_start(&app_cloned, &payload);
        refresh_tray(&app_cloned);
        launch_auto_start_launchers(&app_cloned, &payload);
        launch_specified_launcher(&app_cloned, &payload);
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
            debug!("刷新系统图标成功.");
        }
    });
}

/// 应用程序自动启动后隐藏
fn hide_after_auto_start(app: &AppHandle, payload: &ApplicationStartupCompletePayload) {
    if payload.args.contains(&AUTO_START_FLAG) {
        debug!(
            "hide_after_auto_start 判断为自动启动, 命令行参数: {:?}",
            payload.args
        );

        let inner_app = app.clone();

        tauri::async_runtime::spawn(async move {
            let app_cloned = inner_app.clone();

            let db_manager = app_cloned.state::<DatabaseManager>();

            let window = app_cloned
                .get_webview_window(constants::MAIN_WINDOW_LABEL)
                .unwrap();

            match settings::read(&db_manager.pool, HIDE_AFTER_AUTO_START_KEY)
                .await
                .ok()
                .flatten()
            {
                Some(settings) if setting_api::string_to_bool(&settings.value) => {
                    let _ = window.hide();
                }
                _ => {
                    let _ = window.show();
                }
            }
        });
    } else {
        let window = app
            .get_webview_window(constants::MAIN_WINDOW_LABEL)
            .unwrap();
        let _ = window.show();
    }
}

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
                settings::read(&db_manager.pool, AUTO_START_LAUNCHER_IDS_KEY).await
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

fn launch_specified_launcher(app: &AppHandle, payload: &ApplicationStartupCompletePayload) {
    // 如果启动命令指定了LAUNCH_SPECIFIED_LAUNCHER_KEY, 需要启动指定的编组
    if let Some(Ok(launcher_id)) = extract_arg_value(&payload.args, &LAUNCH_SPECIFIED_LAUNCHER_KEY)
        .map(|value| value.parse::<i64>())
    {
        let app_cloned = app.clone();
        tokio::spawn(async move {
            if let Err(e) = launcher_api::launch(app_cloned, launcher_id).await {
                tracing::error!("launcher launch fail: {}", e);
            }
        });
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
    if THEME_KEY == payload.key {
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
