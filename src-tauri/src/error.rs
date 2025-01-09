use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum OneClickLaunchError {
    #[error("Failed to execute command: {0}")]
    ExecutionError(String),

    #[error("Invalid file path or URL: {0}")]
    InvalidPathError(String),
}
