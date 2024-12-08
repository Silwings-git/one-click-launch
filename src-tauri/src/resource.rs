//! 资源模块

use crate::error::OCLError;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericResource {
    name: String,
    path: Option<ResourceLocation>,
    icon: Option<ResourceLocation>,
    resource_type: ResourceType,
}

impl Resource for GenericResource {
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
    fn execute(&self) -> Result<(), OCLError>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    /// 未知
    UNKNOWN,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
