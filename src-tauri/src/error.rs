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
    InfoError(#[from] std::io::Error),
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
