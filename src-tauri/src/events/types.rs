use serde::{Deserialize, Serialize};

use super::Event;

/// 启动器被启动的事件
pub struct LauncherLaunched;

/// 启动器被启动的事件载荷
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LauncherLaunchedPayload {
    /// 启动器id集
    pub launcher_ids: Vec<i64>,
}

impl Event for LauncherLaunched {
    type Payload = LauncherLaunchedPayload;

    fn name() -> &'static str {
        "launcher:launched"
    }
}

/// 应用程序启动完成的事件
pub struct ApplicationStartupComplete;

/// 应用程序启动完成的事件载荷
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApplicationStartupCompletePayload {
    /// 应用程序启动参数
    pub args: Vec<String>,
}

impl Event for ApplicationStartupComplete {
    type Payload = ApplicationStartupCompletePayload;

    fn name() -> &'static str {
        "application:startup-complete"
    }
}

/// 启动器基础信息编辑完成事件
pub struct LauncherBasicInfoUpdated;

/// 启动器基础信息编辑完成事件载荷
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LauncherBasicInfoUpdatedPayload {
    /// 启动器id集
    pub launcher_ids: Vec<i64>,
}

impl Event for LauncherBasicInfoUpdated {
    type Payload = LauncherBasicInfoUpdatedPayload;

    fn name() -> &'static str {
        "launcher:basic_info_updated"
    }
}

/// 设置被更新
pub struct SettingUpdated;

/// 设置被更新载荷
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SettingUpdatedPayload {
    /// 设置项
    pub key: String,
    /// 设置值
    pub value: String,
}

impl Event for SettingUpdated {
    type Payload = SettingUpdatedPayload;

    fn name() -> &'static str {
        "setting:updated"
    }
}
