//! 资源模块

use crate::error::OCLError;
use log::error;
use std::ffi::OsString;
use std::os::windows::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::ptr;
use url::Url;
use windows::core::PCWSTR;
use windows::Win32::Foundation::{HINSTANCE, HWND};
use windows::Win32::UI::Shell::ShellExecuteW;
use windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;

mod application;
mod file;
pub mod shortcut;
mod web_page;

/// 定义 "open" 操作的宽字符表示作为常量
const OPEN_OPERATION: &[u16] = &[
    'o' as u16, 'p' as u16, 'e' as u16, 'n' as u16, 0, // Null terminator
];

pub trait Resource {
    /// 获取资源名称
    fn name(&self) -> &str;

    /// 资源路径
    fn path(&self) -> Option<ResourceLocation>;

    /// 获取资源图标
    fn icon(&self) -> Option<ResourceLocation>;

    /// 获取资源类型
    fn resource_type(&self) -> ResourceType;
}

/// 可执行资源
/// 该 trait 扩展了 `Resource`，为可执行资源（如应用程序、脚本文件等）提供了执行的能力。
pub trait ExecutableResource: Resource {
    /// 执行可执行资源。
    ///
    /// 对于具体的资源类型，该方法将执行相应的操作（如启动应用程序、运行脚本等）。
    /// 默认实现使用 `ShellExecuteW` API 来执行资源。
    fn execute(&self) -> Result<(), OCLError> {
        // 获取资源路径，如果路径不存在则返回错误。
        let lnk_path = self.path().ok_or(OCLError::MissingPath)?;

        let path_str = match lnk_path {
            // 将路径转换为宽字符字符串，确保以 null 结尾。
            ResourceLocation::FilePath(file_path) => wide_string(file_path.as_os_str()),
            ResourceLocation::WebUrl(url) => {
                let os_url = OsString::from(url.as_str());
                wide_string(&os_url)
            }
        };

        // 使用 ShellExecuteW 启动资源。
        // - 窗口句柄设置为 NULL，表示没有父窗口。
        // - 使用预定义的 "open" 操作来打开资源。
        // - 参数和工作目录设置为 NULL，表示默认行为。
        // - 显示方式设置为 SW_SHOWNORMAL，表示以正常大小和位置显示窗口。
        let result: HINSTANCE = unsafe {
            ShellExecuteW(
                HWND(ptr::null_mut()),
                PCWSTR(OPEN_OPERATION.as_ptr()),
                PCWSTR::from_raw(path_str.as_ptr()),
                PCWSTR::null(),
                PCWSTR::null(),
                SW_SHOWNORMAL,
            )
        };

        // 检查 ShellExecuteW 的返回值。
        // 如果返回值大于 32，则表示成功；否则，记录错误并返回错误信息。
        let code = result.0 as isize;
        if code > 32 {
            Ok(())
        } else {
            let shortcut_name = self.name();
            error!(
                "Failed to open shortcut: {} - Status Code: {}",
                shortcut_name, code
            );
            Err(OCLError::LaunchFailed(self.name().into()))
        }
    }
}

/// 将 OsStr 转换为宽字符字符串，并确保以 null 结尾。
fn wide_string(s: &std::ffi::OsStr) -> Vec<u16> {
    s.encode_wide().chain(std::iter::once(0)).collect()
}

pub enum ResourceType {
    /// 应用程序
    Application,
    /// 快捷方式
    Shortcut,
    /// 网页
    WebPage,
    /// 文件夹
    Folder,
    /// 文件
    File,
}

pub enum ResourceLocation {
    FilePath(PathBuf),
    WebUrl(Url),
}

impl From<PathBuf> for ResourceLocation {
    fn from(path: PathBuf) -> Self {
        ResourceLocation::FilePath(path)
    }
}

impl From<&PathBuf> for ResourceLocation {
    fn from(path: &PathBuf) -> Self {
        ResourceLocation::FilePath(path.clone())
    }
}

impl From<Url> for ResourceLocation {
    fn from(url: Url) -> Self {
        ResourceLocation::WebUrl(url)
    }
}

impl From<&Url> for ResourceLocation {
    fn from(url: &Url) -> Self {
        ResourceLocation::WebUrl(url.clone())
    }
}
