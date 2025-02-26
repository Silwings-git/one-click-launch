use lazy_static::lazy_static;

lazy_static! {
    pub static ref AUTO_START_FLAG: String = "--auto".to_string();
}

pub static APPLICATION_NAME: &str = "一键启动";
pub static THEME: &str = "theme";
