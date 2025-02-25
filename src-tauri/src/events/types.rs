use std::marker::PhantomData;

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Listener};
use tracing::error;

use crate::error::OneClickLaunchError;

// #[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
// pub enum Event {
//     LauncherLaunched { launcher_id: i64 },
//     LauncherBasicInfoUpdated { launcher_id: i64 },
// }

// impl Event {
//     pub fn name(&self) -> &'static str {
//         match self {
//             Event::LauncherLaunched { .. } => "launcher_launched",
//             Event::LauncherBasicInfoUpdated { .. } => "launcher_basic_info_updated",
//         }
//     }
// }

pub trait Event {
    type Payload: Serialize + for<'a> Deserialize<'a> + Clone;

    fn name() -> &'static str;
}

// 事件发送器
pub struct EventDispatcher<E: Event> {
    _marker: PhantomData<E>,
}

impl<E: Event> EventDispatcher<E> {
    pub fn send_event(app: &AppHandle, payload: E::Payload) -> Result<(), OneClickLaunchError> {
        app.emit(E::name(), payload)?;
        Ok(())
    }
}

// 事件监听器注册系统
pub struct EventSystem;

impl EventSystem {
    pub fn register_listener<E, F>(app: &AppHandle, _event: E, callback: F)
    where
        E: Event + 'static,
        F: Fn(E::Payload) + Send + 'static,
    {
        app.listen(E::name(), move |e| {
            if let Ok(payload) = serde_json::from_str(e.payload()) {
                callback(payload);
            } else {
                error!("{}事件payload反序列化失败.原始数据: {}", "", e.payload());
            }
        });
    }
}
