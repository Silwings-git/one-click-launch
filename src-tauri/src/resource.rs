//! 资源模块
pub trait Resource {
    /// 获取资源名称
    fn name(&self) -> String;

    /// 资源路径
    fn path(&self) -> String;
}
