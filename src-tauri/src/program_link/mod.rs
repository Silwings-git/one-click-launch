mod link;

/// 程序链接,包含了应用程序的关联信息
pub struct ProgramLink {
    /// 应用程序名称
    name: String,
    /// 链接类型
    link_type: LinkType,
    /// 启动方式
    start_method: StartMethod,
    /// 工作目录
    working_dir: Option<String>,
    /// 程序位置
    program_location: Option<String>,
    /// 快捷方式文件位置
    shortcut_location: Option<String>,
    /// 应用程序ICON路径
    icon_location: Option<String>,
}

/// 应用程序的链接类型
pub enum LinkType {
    /// 快捷方式
    Shortcut,
    /// 启动程序
    StartProgram,
}

/// 程序链接的启动方式
pub enum StartMethod {
    /// 直接启动
    Start,
    /// 命令行启动
    CMD,
}

/// 快捷方式
#[derive(Debug)]
struct Shortcut {
    /// 应用程序名称
    name: String,
    /// 工作目录
    working_dir: Option<String>,
    /// 程序位置
    target: Option<String>,
    /// 快捷方式文件位置
    location: Option<String>,
    /// 应用程序ICON路径
    icon_location: Option<String>,
}