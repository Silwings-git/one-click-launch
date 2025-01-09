use crate::error::OneClickLaunchError;
use anyhow::Result;
use std::ffi::{OsStr, OsString};
use std::os::windows::ffi::OsStrExt;
use std::path::PathBuf;
use std::ptr;
use std::str::FromStr;
use tracing::error;
use url::Url;
use windows::core::PCWSTR;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::Shell::ShellExecuteW;
use windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;

pub trait LaunchAble {
    fn launch(&self) -> Result<()>;
}

// 注意以空结尾
const OPEN_OPERATION: &[u16] = &['o' as u16, 'p' as u16, 'e' as u16, 'n' as u16, 0];

impl LaunchAble for str {
    fn launch(&self) -> Result<()> {
        match Url::parse(self) {
            Ok(url) => url.launch(),
            Err(_) => {
                let file_path = PathBuf::from_str(self).map_err(|e| {
                    error!("Failed to parse path: {}", e);
                    OneClickLaunchError::InvalidPathError(self.to_string())
                })?;
                file_path.launch()
            }
        }
    }
}

impl LaunchAble for Url {
    fn launch(&self) -> Result<()> {
        let os_url = OsString::from(self.as_str());
        execute(&os_url)
    }
}

impl LaunchAble for PathBuf {
    fn launch(&self) -> Result<()> {
        execute(self.as_os_str())
    }
}

fn execute(path: &OsStr) -> Result<()> {
    let path_str = wide_string(path);

    unsafe {
        // 调用 ShellExecuteW 来打开文件、程序或网址
        let result = ShellExecuteW(
            HWND(ptr::null_mut()),
            PCWSTR(OPEN_OPERATION.as_ptr()),
            PCWSTR::from_raw(path_str.as_ptr()),
            PCWSTR::null(),
            PCWSTR::null(),
            SW_SHOWNORMAL,
        );

        if result.0 as isize <= 32 {
            error!("ShellExecute failed for: {:?}, result: {:?}", path, result);
            Err(
                OneClickLaunchError::ExecutionError(format!("ShellExecute failed for: {:?}", path))
                    .into(),
            )
        } else {
            Ok(())
        }
    }
}

/// 将 OsStr 转换为宽字符字符串，并确保以 null 结尾。
pub fn wide_string(s: &OsStr) -> Vec<u16> {
    s.encode_wide().chain(std::iter::once(0)).collect()
}
