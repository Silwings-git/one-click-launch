#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum OneClickLaunchError {
    #[error("Failed to execute command: {0}")]
    ExecutionError(String),

    #[error("{0}")]
    SystemError(#[from] std::fmt::Error),

    #[error("{0}")]
    DatebaseError(#[from] sqlx::Error),

    #[error("{0}")]
    AnyhowError(#[from] anyhow::Error),

    #[error("{0}")]
    IOError(#[from] std::io::Error),

    #[error("{0}")]
    TauriError(#[from] tauri::Error),

    #[error("Unable to convert from {0} to Event")]
    EventConvertError(String),

    #[error("{0}")]
    WindowsCommonError(String),

    #[error("{0}")]
    WindowsError(#[from] windows::core::Error),
}

// we must manually implement serde::Serialize
impl serde::Serialize for OneClickLaunchError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
