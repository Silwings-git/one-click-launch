use crate::error::OCLError;
use crate::resource::{ExecutableResource, Resource, ResourceLocation, ResourceType};
use crate::utils::wide_string;
use log::error;
use std::ffi::OsString;
use std::ptr;
use windows::core::PCWSTR;
use windows::Win32::Foundation::{HINSTANCE, HWND};
use windows::Win32::UI::Shell::ShellExecuteW;
use windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;

/// 定义 "open" 操作的宽字符表示作为常量
const OPEN_OPERATION: &[u16] = &[
    'o' as u16, 'p' as u16, 'e' as u16, 'n' as u16, 0, // Null terminator
];

#[derive(Debug)]
pub struct ShellExecuteResource {
    name: String,
    path: Option<ResourceLocation>,
    icon: Option<ResourceLocation>,
    resource_type: ResourceType,
}

impl Resource for ShellExecuteResource {
    fn name(&self) -> &str {
        &self.name
    }

    fn path(&self) -> Option<ResourceLocation> {
        self.path.clone()
    }

    fn icon(&self) -> Option<ResourceLocation> {
        self.icon.clone()
    }

    fn resource_type(&self) -> ResourceType {
        self.resource_type.clone()
    }
}

impl ExecutableResource for ShellExecuteResource {
    fn execute(&self) -> Result<(), OCLError> {
        match self.resource_type() {
            ResourceType::Application
            | ResourceType::Shortcut
            | ResourceType::WebPage
            | ResourceType::Folder
            | ResourceType::File => self.execute_resource(),
            ResourceType::UNKNOWN => Err(OCLError::InvalidOperation(format!(
                "Resource of type {:?} cannot be executed",
                self.resource_type()
            ))),
        }
    }
}

impl ShellExecuteResource {
    // 单独定义执行逻辑，便于复用
    fn execute_resource(&self) -> Result<(), OCLError> {
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
            Err(OCLError::LaunchFailed(shortcut_name.into()))
        }
    }
}
