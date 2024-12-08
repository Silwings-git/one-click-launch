use crate::error::OCLError;
use crate::program_link::Shortcut;
use lnk::ShellLink;
use std::path::{Path, PathBuf};
use std::{fs, panic};

/// 获取指定目录`base_path`中后缀名与`extension`匹配的文件的路径信息
/// `extension`不需要指定`.`符号
fn list_file(base_path: &str, extension: &str) -> Vec<String> {
    let mut programs = Vec::new();

    // 遍历目录中的所有文件和子文件夹
    if let Ok(entries) = fs::read_dir(base_path) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_dir() {
                // 如果是文件夹，递归遍历
                programs.extend(list_file(path.to_str().unwrap(), extension));
            } else if path.extension().map(|e| e == extension).unwrap_or(false) {
                programs.push(path.display().to_string());
            }
        }
    }

    programs
}

fn build_shortcut_from_path<P: AsRef<Path> + panic::RefUnwindSafe>(
    path: &P,
) -> Result<Shortcut, OCLError> {
    let r = panic::catch_unwind(|| {
        // open方法遇到Windows PowerShell.lnk时会panic
        match ShellLink::open(path) {
            Ok(sl) => Ok(Shortcut::from(ShellLinkAndPath(sl, &path.as_ref()))),
            Err(e) => Err(OCLError::ParseShortcutError(e)),
        }
    });
    match r {
        Ok(res) => res,
        Err(err) => {
            println!("解析失败: {}", path.as_ref().display());
            Err(OCLError::ParseShortcutError1)
        }
    }
    // let shortcut = ShellLink::open(path)?;
    // Ok(Shortcut::from(ShellLinkAndPath(shortcut, &path.as_ref())))
}

struct ShellLinkAndPath<'a, P: AsRef<Path> + ?Sized>(ShellLink, &'a P);

impl From<ShellLinkAndPath<'_, &Path>> for Shortcut {
    fn from(value: ShellLinkAndPath<&Path>) -> Self {
        Shortcut {
            name: value.1.file_stem().unwrap().to_str().unwrap().to_string(),
            working_dir: value.0.working_dir().clone(),
            target: value
                .0
                .link_info()
                .clone()
                .and_then(|l| l.local_base_path().clone()),
            location: Some(value.1.display().to_string()),
            icon_location: value.0.icon_location().clone(),
        }
    }
}

/// 尝试解析快捷方式的icon
fn resolve_icon(shortcut: &Shortcut) -> Result<PathBuf, OCLError> {
    shortcut
        .icon_location
        .as_ref()
        .filter(|l| l.ends_with(".ico"))
        .map(|s| s.clone())
        // 如果没有就尝试到快捷方式指定的工作目录查找第一个icon文件
        .or_else(|| {
            shortcut
                .working_dir
                .as_ref()
                .map(|dir| list_file(dir, "icon"))
                .filter(|v| !v.is_empty())
                .map(|v| v.first().unwrap().clone())
                .clone()
        })
        // 如果还没有找到就到预配置列表中检查是否包含相同程序名称的icon文件
        .or_else(|| {
            find_default_icon_by_program_name(&shortcut.name).map(|path| path.display().to_string())
        })
        .map(|path| PathBuf::from(path))
        .ok_or(OCLError::IconNotFound)
}

/// 按程序名称查找默认icon
fn find_default_icon_by_program_name(program_name: &str) -> Option<PathBuf> {
    match program_name {
        "demo" => Some(PathBuf::from("/dev/null")),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::process::Command;

    #[test]
    fn list_file_should_work() {
        let vec = list_file(
            r"C:\ProgramData\Microsoft\Windows\Start Menu\Programs",
            "lnk",
        );
        println!("{:?}", vec);
        println!("{}", vec.len());
    }

    #[test]
    fn build_shortcut_from_path_should_work() {
        let vec = list_file(
            r"C:\ProgramData\Microsoft\Windows\Start Menu\Programs",
            "lnk",
        );
        vec.iter()
            .map(|s| build_shortcut_from_path(s))
            .for_each(|e| println!("{:?}", e));
    }

    #[test]
    fn resolve_icon_should_work() {
        let mut vec = list_file(
            r"C:\ProgramData\Microsoft\Windows\Start Menu\Programs",
            "lnk",
        );
        vec.extend(list_file(
            r"C:\Users\Silwings\AppData\Roaming\Microsoft\Windows\Start Menu\Programs",
            "lnk",
        ));
        vec.sort();
        vec.dedup();
        println!("{}", vec.len());
        let rv = vec
            .iter()
            .map(|s| build_shortcut_from_path(s))
            .filter(|p| p.is_ok())
            .map(|p| p.unwrap())
            .map(|e| resolve_icon(&e))
            .filter(|p| p.is_ok())
            .map(|p| p.unwrap())
            .collect::<Vec<_>>();
        println!("{:?}", rv.len());
    }

    #[test]
    fn extract_icon_should_work() {
        // 定义PowerShell脚本的路径
        let script_path = Path::new(r"src-tauri/scripts/extract_icon.ps1");
        let source_exe = r"D:\software\develop\jetbrains\RustRover\bin\rustrover64.exe";
        let output_ico =
            r"C:\Users\Silwings\AppData\Roaming\JetBrains\RustRover2024.2\scratches\SaveIcon.png";

        // 检查脚本文件是否存在
        if !script_path.exists() {
            eprintln!("脚本文件不存在: {}", script_path.display());
            return;
        }

        // 使用Command来运行PowerShell脚本
        let output = Command::new("powershell.exe")
            .args(&[
                "-NoProfile",
                "-ExecutionPolicy",
                "Bypass",
                "-File",
                &script_path.to_string_lossy(),
            ])
            .arg("-sourceExe")
            .arg(source_exe)
            .arg("-outputIco")
            .arg(output_ico)
            .output()
            .expect("无法运行PowerShell脚本");

        // 输出脚本的结果
        if output.status.success() {
            println!("脚本成功运行。");
            println!("输出: {}", String::from_utf8_lossy(&output.stdout));
        } else {
            eprintln!("脚本运行失败。");
            eprintln!("错误: {}", String::from_utf8_lossy(&output.stderr));
        }
    }
}
