use crate::resource::{ExecutableResource, Resource, ResourceLocation, ResourceType};
use std::path::PathBuf;

pub struct LaunchFile {
    name: String,
    file_path: Option<PathBuf>,
    /// ICON路径
    icon_path: Option<ResourceLocation>,
}

impl LaunchFile {
    pub fn new(
        name: String,
        file_path: Option<PathBuf>,
        icon_path: Option<ResourceLocation>,
    ) -> Self {
        Self {
            name,
            file_path,
            icon_path,
        }
    }
}

impl Resource for LaunchFile {
    fn name(&self) -> &str {
        &self.name
    }

    fn path(&self) -> Option<ResourceLocation> {
        if let Some(file_path) = &self.file_path {
            Some(ResourceLocation::from(file_path))
        } else {
            None
        }
    }

    fn icon(&self) -> Option<ResourceLocation> {
        self.icon_path.clone()
    }

    fn resource_type(&self) -> ResourceType {
        ResourceType::File
    }
}

impl ExecutableResource for LaunchFile {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn executable_should_working() {
        // 快捷方式
        let path: PathBuf =
            r#"C:\ProgramData\Microsoft\Windows\Start Menu\Programs\EditPlus.lnk"#.into();
        // 应用程序
        let path: PathBuf = r#"D:\software\appliation\bilibili\哔哩哔哩.exe"#.into();
        // 网页
        let path: PathBuf = r#"https://www.baidu.com/"#.into();
        // 文件
        let path: PathBuf = r#"C:\Users\Silwings\Downloads\一封信.pdf"#.into();
        let path: PathBuf = r#"C:\Users\Silwings\Downloads\新建 文本文档.txt"#.into();

        let everything = LaunchFile::new("EditPlus".into(), Some(path), None);
        everything.execute().unwrap();
    }
}
