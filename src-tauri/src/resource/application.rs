use crate::resource::{ExecutableResource, Resource, ResourceLocation, ResourceType};
use std::path::PathBuf;

/// 应用程序
#[derive(Debug)]
pub struct Application {
    name: String,
    app_path: Option<PathBuf>,
    icon_path: Option<PathBuf>,
}

impl Application {
    /// 创建一个新的应用程序实例
    ///
    /// # 参数
    /// - `name`: 应用程序的名称
    /// - `path`: 可选的应用程序路径
    /// - `icon_path`: 可选的应用程序图标路径
    pub fn new(name: String, app_path: Option<PathBuf>, icon_path: Option<PathBuf>) -> Self {
        Application {
            name,
            app_path,
            icon_path,
        }
    }
}

impl Resource for Application {
    fn name(&self) -> &str {
        &self.name
    }

    fn path(&self) -> Option<ResourceLocation> {
        if let Some(app_path) = &self.app_path {
            Some(ResourceLocation::from(app_path))
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
        ResourceType::Application
    }
}

impl ExecutableResource for Application {}

#[cfg(test)]
mod test {
    use crate::resource::application::Application;
    use crate::resource::ExecutableResource;

    #[test]
    fn executable_should_working() {
        let everything = Application::new(
            "Everything".into(),
            Some(r#"D:\software\appliation\Everything-1.4.1.1024.x64\Everything.exe"#.into()),
            None,
        );
        everything.execute().unwrap();
    }
}
