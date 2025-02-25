use serde::{Deserialize, Serialize};

use super::types::Event;

pub struct LauncherLaunched;
#[derive(Serialize, Deserialize, Clone)]
pub struct LauncherLaunchedPayload {
    pub launcher_id: i64,
}

impl Event for LauncherLaunched {
    type Payload = LauncherLaunchedPayload;

    fn name() -> &'static str {
        "launcher_launched"
    }
}
