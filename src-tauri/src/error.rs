use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum OCSError {
    #[error("解析快捷方式失败: 0")]
    ParseShortcutError(lnk::Error),
    #[error("解析快捷方式失败: 1")]
    ParseShortcutError1,
    #[error("找不到icon")]
    IconNotFound,
}

impl From<lnk::Error> for OCSError {
    fn from(error: lnk::Error) -> Self {
        OCSError::ParseShortcutError(error)
    }
}