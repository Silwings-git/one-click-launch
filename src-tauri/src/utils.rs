use std::os::windows::ffi::OsStrExt;

/// 将 OsStr 转换为宽字符字符串，并确保以 null 结尾。
pub fn wide_string(s: &std::ffi::OsStr) -> Vec<u16> {
    s.encode_wide().chain(std::iter::once(0)).collect()
}
