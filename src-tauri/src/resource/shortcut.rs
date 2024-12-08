use crate::resource::{ExecutableResource, Resource, ResourceLocation, ResourceType};
use std::path::PathBuf;

/// 快捷方式
#[derive(Debug)]
pub struct Shortcut {
    /// 应用程序名称
    name: String,
    /// 快捷方式文件位置
    shortcut_path: Option<PathBuf>,
    /// 程序位置
    target_path: Option<PathBuf>,
    /// 工作目录
    working_dir_path: Option<PathBuf>,
    /// 应用程序ICON路径
    icon_path: Option<PathBuf>,
}

impl Shortcut {
    pub fn new(
        name: String,
        shortcut_path: Option<PathBuf>,
        target_path: Option<PathBuf>,
        working_dir_path: Option<PathBuf>,
        icon_path: Option<PathBuf>,
    ) -> Self {
        Self {
            name,
            shortcut_path,
            target_path,
            working_dir_path,
            icon_path,
        }
    }
}

impl Resource for Shortcut {
    fn name(&self) -> &str {
        &self.name
    }

    fn path(&self) -> Option<ResourceLocation> {
        if let Some(shortcut_path) = &self.shortcut_path {
            Some(ResourceLocation::from(shortcut_path))
        } else {
            None
        }
    }

    fn icon(&self) -> Option<ResourceLocation> {
        if let Some(icon_path) = &self.icon_path {
            Some(ResourceLocation::from(icon_path))
        } else {
            None
        }
    }

    fn resource_type(&self) -> ResourceType {
        ResourceType::Shortcut
    }
}

impl ExecutableResource for Shortcut {}

#[cfg(test)]
mod test {
    use crate::resource::shortcut::Shortcut;
    use crate::resource::ExecutableResource;
    use std::path::PathBuf;

    #[test]
    fn executable_should_working() {
        // 快捷方式
        let path: PathBuf =
            r#"C:\ProgramData\Microsoft\Windows\Start Menu\Programs\EditPlus.lnk"#.into();
        let everything = Shortcut::new("EditPlus".into(), Some(path), None, None, None);
        everything.execute().unwrap();
    }
}
