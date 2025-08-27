use std::{sync::Mutex, time::Instant};

use tauri::{
    AppHandle, DragDropEvent, Manager, State, Theme,
    menu::{MenuBuilder, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
};
use tracing::{debug, info};

use crate::{
    DatabaseManager, WindowContext,
    constants::{self, WINDOW_MIN_HEIGHT, WINDOW_MIN_WIDTH},
    db::launcher,
    error::OneClickLaunchError,
    events::{
        EventDispatcher,
        types::{DragDropResource, DragDropResourcePaylod},
    },
};

use super::{launcher_api, setting_api};

pub fn hide_window(app: &AppHandle) -> Result<(), OneClickLaunchError> {
    let window = app
        .get_webview_window(constants::MAIN_WINDOW_LABEL)
        .unwrap();
    window.hide()?;
    Ok(())
}

/// 刷新系统图标菜单
pub async fn refresh_tray(app: AppHandle) -> Result<(), OneClickLaunchError> {
    // 获取全局状态
    let window_context: State<'_, WindowContext> = app.state();
    let database_manager: State<'_, DatabaseManager> = app.state();
    let launchers = launcher::query(&database_manager.pool).await?;

    let mut menu_builder = MenuBuilder::new(&app);
    // 创建动态菜单项
    for launcher in &launchers {
        let id = format!("launch_{}", launcher.id);
        let title = format!("启动: {}", launcher.name);
        menu_builder =
            menu_builder.item(&MenuItem::with_id(&app, &id, &title, true, None::<&str>)?);
    }

    if !launchers.is_empty() {
        menu_builder = menu_builder.separator();
    }

    // 添加退出按钮
    let quit_item = MenuItem::with_id(&app, "quit", "退出", true, None::<&str>)?;
    let menu = menu_builder.item(&quit_item).build()?;

    // 设置菜单到托盘图标
    window_context.tray_icon.set_menu(Some(menu))?;
    Ok(())
}

pub fn change_windows_theme(app: &AppHandle, theme: &str) -> Result<(), OneClickLaunchError> {
    if let Some(window) = app.get_webview_window(constants::MAIN_WINDOW_LABEL) {
        window.set_theme(match theme {
            "dark" => Some(Theme::Dark),
            "light" => Some(Theme::Light),
            _ => None,
        })?;
    }
    Ok(())
}

pub fn handle_window_event(window: &tauri::Window, event: &tauri::WindowEvent) {
    match event {
        tauri::WindowEvent::CloseRequested { api, .. } => {
            api.prevent_close();
            let _ = window.hide();

            let app_handle = window.app_handle().clone();
            tauri::async_runtime::spawn(async move {
                let db = app_handle.state::<DatabaseManager>();
                let setting = setting_api::read_setting(db, constants::CLOSE_MAIN_PANEL_KEY).await;

                match setting {
                    Ok(Some(setting)) if constants::CLOSE_MAIN_PANEL_EXIT == setting.value => {
                        app_handle.exit(0);
                    }
                    _ => {}
                }
            });
        }
        tauri::WindowEvent::ScaleFactorChanged {
            scale_factor,
            new_inner_size,
            ..
        } => {
            print!(
                "ScaleFactorChanged scale_factor: {}, new_inner_size: {:?}",
                scale_factor, new_inner_size
            );

            let now = Instant::now();
            let state = window.state::<ScaleFactorChangedState>();
            let mut lock = state.last_reset.try_lock();

            if let Ok(ref mut last_reset) = lock {
                // 500ms防抖间隔
                if last_reset.is_none_or(|t| now.duration_since(t).as_millis() > 500) {
                    if let Ok(physical_size) = window.inner_size() {
                        // 如果窗口大小异常，强制调整到正常大小
                        if physical_size.width != WINDOW_MIN_WIDTH
                            || physical_size.height != WINDOW_MIN_HEIGHT
                        {
                            let _ = window.set_size(tauri::Size::Logical(tauri::LogicalSize {
                                width: WINDOW_MIN_WIDTH as f64,
                                height: WINDOW_MIN_HEIGHT as f64,
                            }));
                            **last_reset = Some(now);
                            debug!("DPI发生变化,触发窗口大小重置");
                        }
                    }
                } else {
                    debug!("DPI重置窗口防抖生效");
                }
            }
        }
        tauri::WindowEvent::DragDrop(drag_drop_event) => match drag_drop_event {
            DragDropEvent::Drop { paths, .. } if !paths.is_empty() => {
                let _ = window.set_focus();
                let _ = EventDispatcher::<DragDropResource>::send_event(
                    window.app_handle(),
                    DragDropResourcePaylod {
                        paths: paths.clone(),
                    },
                );
            }
            _ => {}
        },
        _ => {}
    }
}

/// 初始化窗口
pub fn setup_tray(app: &AppHandle) -> Result<(), OneClickLaunchError> {
    let tray_icon = TrayIconBuilder::new()
        // 设置系统托盘的提示,鼠标悬浮时会显示
        .tooltip(constants::APPLICATION_NAME)
        // 左键系统托盘时不显示菜单
        .show_menu_on_left_click(false)
        // 使用主窗口的icon作为系统托盘的图标
        .icon(app.default_window_icon().unwrap().clone())
        // 系统托盘的点击事件
        .on_tray_icon_event(|tray, event| match event {
            TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } => {
                // 当用户点击系统托盘时使应用程序取消最小化,显示并聚焦
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window(constants::MAIN_WINDOW_LABEL) {
                    let _ = window.unminimize();
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            _ => {
                debug!("unhandled event {event:?}");
            }
        })
        .on_menu_event(|app, event| match event.id.as_ref() {
            "quit" => {
                // 退出应用程序
                info!("quit menu item was clicked");
                app.exit(0);
            }
            id if id.starts_with("launch_") => {
                // 系统托盘的菜单id由"launch_"与启动器id拼接而成, 点击菜单后通过解析id,找到要启动的启动器触发启动
                let id = &id["launch_".len()..];
                if let Ok(launcher_id) = id.parse::<i64>() {
                    let app_cloned = app.clone();
                    tauri::async_runtime::spawn(async move {
                        launcher_api::launch(app_cloned, launcher_id).await
                    });
                }
            }
            _ => {
                tracing::error!("menu item {:?} not handled", event.id);
            }
        })
        .build(app)?;

    let window_context = WindowContext { tray_icon };

    app.manage(window_context);

    Ok(())
}

/// 用于保存上次处理分辨率变更事件的时间
pub struct ScaleFactorChangedState {
    pub last_reset: Mutex<Option<Instant>>,
}

use std::os::windows::ffi::OsStrExt;
use std::path::PathBuf;
use windows::{
    Win32::{
        System::Com::{
            CLSCTX_INPROC_SERVER, COINIT_APARTMENTTHREADED, CoCreateInstance, CoInitializeEx,
            CoUninitialize, IPersistFile,
        },
        UI::Shell::{IShellLinkW, ShellLink},
    },
    core::{HSTRING, Interface, PCWSTR},
};

/// 创建 Windows 快捷方式 (.lnk 文件)
pub fn create_shortcut(
    app_path: &str,
    shortcut_name: &str,
    args: Option<Vec<String>>,
    target_dir: Option<&str>,
) -> Result<PathBuf, OneClickLaunchError> {
    unsafe {
        // 初始化 COM
        CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok()?;

        // 创建 IShellLink 实例
        let shell_link: IShellLinkW = CoCreateInstance(&ShellLink, None, CLSCTX_INPROC_SERVER)?;

        // 设置应用路径
        shell_link.SetPath(&HSTRING::from(dbg!(app_path)))?;

        // 设置参数
        if let Some(arguments) = args {
            let arg_str = arguments.join(" ");
            shell_link.SetArguments(&HSTRING::from(arg_str))?;
        }

        // 设置工作目录（使用 app_path 的父目录）
        if let Some(parent) = std::path::Path::new(app_path).parent() {
            shell_link.SetWorkingDirectory(&HSTRING::from(parent.to_string_lossy().to_string()))?;
        }

        // 获取 IPersistFile 接口
        let persist_file: IPersistFile = shell_link.cast()?;

        // 目标目录（默认桌面）
        let save_dir = if let Some(dir) = target_dir {
            PathBuf::from(dir)
        } else {
            dirs::desktop_dir().ok_or_else(|| anyhow::anyhow!("无法获取桌面路径"))?
        };

        // 拼接快捷方式路径
        let lnk_path = save_dir.join(format!("{}.lnk", shortcut_name));

        // 转换为宽字符串
        let wide: Vec<u16> = lnk_path
            .as_os_str()
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        // 保存
        persist_file.Save(PCWSTR::from_raw(wide.as_ptr()), true)?;

        // 释放 COM
        CoUninitialize();

        Ok(lnk_path)
    }
}
