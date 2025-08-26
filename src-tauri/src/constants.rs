use lazy_static::lazy_static;

lazy_static! {
    pub static ref AUTO_START_FLAG: String = "--auto".to_string();
    pub static ref LAUNCH_SPECIFIED_LAUNCHER_KEY: String = "launch".to_string();
}

pub static APPLICATION_NAME: &str = "一键启动";
pub static MAIN_WINDOW_LABEL: &str = "main";
pub static THEME_KEY: &str = "theme";
pub static CLOSE_MAIN_PANEL_KEY: &str = "close_main_panel";
pub static AUTO_START_LAUNCHER_IDS_KEY: &str = "auto_start_launcher_ids";
pub static HIDE_AFTER_AUTO_START_KEY: &str = "hide_after_auto_start";
pub static CLOSE_MAIN_PANEL_EXIT: &str = "m2";
pub static WINDOW_MIN_WIDTH: u32 = 800;
pub static WINDOW_MIN_HEIGHT: u32 = 600;
