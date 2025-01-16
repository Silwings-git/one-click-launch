// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use anyhow::Result;
use one_click_start_lib::error::OneClickLaunchError;
use tracing_subscriber::fmt;

#[tokio::main]
async fn main() -> Result<(), OneClickLaunchError> {

    // 只在开发模式下调用日志初始化
    #[cfg(debug_assertions)]
    {
        fmt()
            .with_writer(std::io::stderr) // 将日志发送到标准错误输出
            .with_max_level(tracing::Level::DEBUG) // 设置最大日志级别
            .init();
    }

    one_click_start_lib::run().await?;
    Ok(())
}
