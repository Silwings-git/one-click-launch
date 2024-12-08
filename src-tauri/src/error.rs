use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum OCLError {
    #[error("解析快捷方式失败: 0")]
    ParseShortcutError(lnk::Error),
    #[error("解析快捷方式失败: 1")]
    ParseShortcutError1,
    #[error("找不到icon")]
    IconNotFound,
    #[error(r#"启动{0}失败"#)]
    LaunchFailed(String),
    #[error("{0}")]
    IOError(#[from] std::io::Error),
    #[error("无效的路径")]
    InvalidPath,
    #[error("缺少路径")]
    MissingPath,
}

impl From<lnk::Error> for OCLError {
    fn from(error: lnk::Error) -> Self {
        OCLError::ParseShortcutError(error)
    }
}
