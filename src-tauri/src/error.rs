use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
#[non_exhaustive]
pub enum OneClickLaunchError {
    #[error("Failed to execute command: {0}")]
    ExecutionError(String),

    #[error("Invalid file path or URL: {0}")]
    InvalidPathError(String),
}
