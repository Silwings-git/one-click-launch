// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use anyhow::Result;
use one_click_start_lib::error::OneClickLaunchError;
#[tokio::main]
async fn main() -> Result<(), OneClickLaunchError> {
    // 初始化日志
    tracing_subscriber::fmt::init();
    one_click_start_lib::run().await?;
    Ok(())
}
