use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum OCLError {
    #[error(r#"启动{0}失败"#)]
    LaunchFailed(String),
    #[error("缺少路径")]
    MissingPath,
    #[error("{0}")]
    InvalidOperation(String),
}
