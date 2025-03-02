use lazy_static::lazy_static;

lazy_static! {
    pub static ref AUTO_START_FLAG: String = "--auto".to_string();
}

pub static APPLICATION_NAME: &str = "一键启动";
pub static THEME_KEY: &str = "theme";
pub static CLOSE_MAIN_PANEL_KEY: &str = "close_main_panel";
pub static CLOSE_MAIN_PANEL_EXIT: &str = "m2";
pub static WINDOW_MIN_WIDTH: u32 = 800;
pub static WINDOW_MIN_HEIGHT: u32 = 600;
