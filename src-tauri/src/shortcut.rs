// 桌面快捷方式管理模块

use std::path::PathBuf;

// 获取桌面目录
fn get_desktop_dir() -> Option<PathBuf> {
    if let Ok(user_profile) = std::env::var("USERPROFILE") {
        let desktop = PathBuf::from(user_profile).join("Desktop");
        if desktop.exists() {
            return Some(desktop);
        }
    }
    None
}

// 检查桌面是否存在
pub fn has_desktop_folder() -> Result<bool, String> {
    Ok(get_desktop_dir().is_some())
}

// 检查桌面是否已有快捷方式
pub fn has_desktop_shortcut() -> Result<bool, String> {
    let desktop = get_desktop_dir().ok_or("Failed to get desktop directory")?;
    
    let shortcut_names = vec![
        "SRA",
        "StarRailAssistant",
        "StarRailAssistant Community Edition",
        "SRA-CE",
        "StarRailAssistant-CE",
        "StarRailAssistant-CommunityEdition",
    ];
    
    for name in shortcut_names {
        let shortcut_path = desktop.join(format!("{}.lnk", name));
        if shortcut_path.exists() {
            return Ok(true);
        }
    }
    
    Ok(false)
}

// 创建桌面快捷方式
#[cfg(target_os = "windows")]
pub fn create_desktop_shortcut() -> Result<(), String> {
    use std::env;
    use std::os::windows::process::CommandExt;
    use std::process::Command;
    
    let desktop = get_desktop_dir().ok_or("Failed to get desktop directory")?;
    let exe_path = env::current_exe().map_err(|e| format!("Failed to get exe path: {}", e))?;
    let shortcut_path = desktop.join("StarRailAssistant.lnk");
    
    // 使用 PowerShell 创建快捷方式
    let ps_script = format!(
        r#"$WshShell = New-Object -ComObject WScript.Shell; $Shortcut = $WshShell.CreateShortcut('{}'); $Shortcut.TargetPath = '{}'; $Shortcut.Save()"#,
        shortcut_path.display(),
        exe_path.display()
    );
    
    const CREATE_NO_WINDOW: u32 = 0x08000000;
    
    let output = Command::new("powershell")
        .args(&["-NoProfile", "-Command", &ps_script])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .map_err(|e| format!("Failed to execute PowerShell: {}", e))?;
    
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to create shortcut: {}", error));
    }
    
    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn create_desktop_shortcut() -> Result<(), String> {
    Err("Desktop shortcut creation is only supported on Windows".to_string())
}

// 保存"不再提示"设置
pub fn save_skip_prompt_setting() -> Result<(), String> {
    use crate::settings::{load_settings, save_settings};
    
    let mut settings = load_settings()?;
    settings.skip_desktop_shortcut_prompt = true;
    save_settings(settings)?;
    
    Ok(())
}
