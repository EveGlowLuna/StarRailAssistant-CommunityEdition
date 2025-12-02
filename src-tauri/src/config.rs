// 配置文件管理模块

use std::fs;
use std::path::PathBuf;
use serde_json::json;
use crate::encryption;

// 获取配置目录路径
fn get_config_dir() -> Result<PathBuf, String> {
    let appdata = std::env::var("APPDATA").map_err(|_| "Failed to get APPDATA path")?;
    Ok(PathBuf::from(appdata).join("SRA").join("configs"))
}

// 获取任务顺序文件路径
fn get_task_order_file() -> Result<PathBuf, String> {
    let appdata = std::env::var("APPDATA").map_err(|_| "Failed to get APPDATA path")?;
    Ok(PathBuf::from(appdata).join("SRA").join("task-order.json"))
}

// 将字符串中的非ASCII字符转换为Unicode转义序列
fn escape_unicode_chars(s: &str) -> String {
    let mut result = String::new();
    for ch in s.chars() {
        if ch.is_ascii() {
            result.push(ch);
        } else {
            // 转换为\uXXXX格式
            result.push_str(&format!("\\u{:04x}", ch as u32));
        }
    }
    result
}

// 获取配置列表
pub fn get_config_list() -> Result<Vec<String>, String> {
    let config_dir = get_config_dir()?;

    // 确保目录存在
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)
            .map_err(|e| format!("Failed to create config directory: {}", e))?;
    }

    // 获取所有 .json 文件
    let mut config_names = Vec::new();
    let entries =
        fs::read_dir(&config_dir).map_err(|e| format!("Failed to read config directory: {}", e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let path = entry.path();
        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == "json" {
                    if let Some(file_name) = path.file_stem() {
                        if let Some(name_str) = file_name.to_str() {
                            config_names.push(name_str.to_string());
                        }
                    }
                }
            }
        }
    }

    // 如果没有配置文件，创建默认配置
    if config_names.is_empty() {
        create_default_config()?;
        config_names.push("Default".to_string());
    }

    Ok(config_names)
}

// 创建默认配置
pub fn create_default_config() -> Result<(), String> {
    create_config_with_name("Default")
}

// 创建新配置
pub fn create_new_config(name: String) -> Result<(), String> {
    create_config_with_name(&name)
}

// 使用指定名称创建配置
fn create_config_with_name(name: &str) -> Result<(), String> {
    let config_dir = get_config_dir()?;
    let config_file = config_dir.join(format!("{}.json", name));

    // 确保目录存在
    fs::create_dir_all(&config_dir)
        .map_err(|e| format!("Failed to create config directory: {}", e))?;

    // 默认配置内容
    let default_config = json!({
        "StartGamePassword": "",
        "StartGameUsername": "",
        "Version": 3,
        "AfterExitApp": false,
        "AfterExitGame": false,
        "AfterLogout": false,
        "AfterShutdown": false,
        "AfterSleep": false,
        "EnabledTasks": [
            false,
            false,
            false,
            false,
            false
        ],
        "Name": name,
        "ReceiveRewardRedeemCodes": "",
        "ReceiveRewards": [
            false,
            false,
            false,
            false,
            false,
            false,
            false
        ],
        "SimulatedUniverseEnable": false,
        "SimulatedUniverseMode": 0,
        "SimulatedUniversePolicy": 0,
        "SimulatedUniverseRunTimes": 0,
        "CurrencyWarsEnable": false,
        "CurrencyWarsPolicy": 0,
        "CurrencyWarsRunTimes": 0,
        "CurrencyWarsUsername": "",
        "StartGameAlwaysLogin": false,
        "StartGameAutoLogin": false,
        "StartGameChannel": 0,
        "StartGamePath": "",
        "TrailblazePowerChangeLineup": false,
        "TrailblazePowerLineupCheck": false,
        "TrailblazePowerReplenishStamina": false,
        "TrailblazePowerReplenishTimes": 0,
        "TrailblazePowerReplenishWay": 0,
        "TrailblazePowerTaskList": [],
        "TrailblazePowerUseAssistant": false,
        "TrailblazePowerUseSkill": false
    });

    // 写入文件 - 使用Unicode转义序列，与SRA源程序保持一致
    let config_content = serde_json::to_string_pretty(&default_config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;
    
    // 将中文字符转换为Unicode转义序列
    let config_content_escaped = escape_unicode_chars(&config_content);
    
    fs::write(&config_file, config_content_escaped)
        .map_err(|e| format!("Failed to write config file: {}", e))?;

    Ok(())
}

// 删除配置
pub fn delete_config(name: String) -> Result<(), String> {
    let config_dir = get_config_dir()?;
    let config_file = config_dir.join(format!("{}.json", name));

    // 删除文件
    if config_file.exists() {
        fs::remove_file(&config_file)
            .map_err(|e| format!("Failed to delete config file: {}", e))?;
    }

    // 从任务顺序中移除已删除的配置
    let task_order_file = get_task_order_file()?;
    if task_order_file.exists() {
        let content = fs::read_to_string(&task_order_file)
            .map_err(|e| format!("Failed to read task order file: {}", e))?;
        
        let mut task_order: Vec<String> = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse task order: {}", e))?;
        
        // 移除已删除的配置
        task_order.retain(|config_name| config_name != &name);
        
        // 保存更新后的任务顺序
        let updated_content = serde_json::to_string_pretty(&task_order)
            .map_err(|e| format!("Failed to serialize task order: {}", e))?;
        fs::write(&task_order_file, updated_content)
            .map_err(|e| format!("Failed to write task order file: {}", e))?;
    }

    Ok(())
}

// 加载配置
pub fn load_config(name: String) -> Result<serde_json::Value, String> {
    let config_dir = get_config_dir()?;
    let config_file = config_dir.join(format!("{}.json", name));

    if !config_file.exists() {
        return Err(format!("Config file not found: {}", name));
    }

    let content = fs::read_to_string(&config_file)
        .map_err(|e| format!("Failed to read config file: {}", e))?;
    
    let mut config: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse config: {}", e))?;

    // 解密密码和用户名（如果存在）
    if let Some(encrypted_password) = config.get("StartGamePassword").and_then(|v| v.as_str()) {
        if !encrypted_password.is_empty() {
            match encryption::decrypt_string(encrypted_password) {
                Ok(decrypted) => {
                    config["StartGamePassword"] = serde_json::Value::String(decrypted);
                }
                Err(e) => {
                    // 解密失败，可能是旧的明文密码，保持原样
                    eprintln!("Failed to decrypt password: {}", e);
                }
            }
        }
    }

    if let Some(encrypted_username) = config.get("StartGameUsername").and_then(|v| v.as_str()) {
        if !encrypted_username.is_empty() {
            match encryption::decrypt_string(encrypted_username) {
                Ok(decrypted) => {
                    config["StartGameUsername"] = serde_json::Value::String(decrypted);
                }
                Err(e) => {
                    // 解密失败，可能是旧的明文用户名，保持原样
                    eprintln!("Failed to decrypt username: {}", e);
                }
            }
        }
    }

    Ok(config)
}

// 保存配置
pub fn save_config(config: serde_json::Value) -> Result<(), String> {
    // 从配置中获取名称
    let name = config.get("Name")
        .and_then(|v| v.as_str())
        .ok_or("Config must have a Name field")?;

    let config_dir = get_config_dir()?;
    let config_file = config_dir.join(format!("{}.json", name));

    // 确保目录存在
    fs::create_dir_all(&config_dir)
        .map_err(|e| format!("Failed to create config directory: {}", e))?;

    // 克隆配置以便修改
    let mut config_to_save = config.clone();

    // 加密密码和用户名（如果存在且不为空）
    if let Some(password) = config_to_save.get("StartGamePassword").and_then(|v| v.as_str()) {
        if !password.is_empty() {
            let encrypted = encryption::encrypt_string(password)
                .map_err(|e| format!("Failed to encrypt password: {}", e))?;
            config_to_save["StartGamePassword"] = serde_json::Value::String(encrypted);
        }
    }

    if let Some(username) = config_to_save.get("StartGameUsername").and_then(|v| v.as_str()) {
        if !username.is_empty() {
            let encrypted = encryption::encrypt_string(username)
                .map_err(|e| format!("Failed to encrypt username: {}", e))?;
            config_to_save["StartGameUsername"] = serde_json::Value::String(encrypted);
        }
    }

    // 写入文件 - 使用Unicode转义序列，与SRA源程序保持一致
    let config_content = serde_json::to_string_pretty(&config_to_save)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;
    
    // 将中文字符转换为Unicode转义序列
    let config_content_escaped = escape_unicode_chars(&config_content);
    
    fs::write(&config_file, config_content_escaped)
        .map_err(|e| format!("Failed to write config file: {}", e))?;

    Ok(())
}

// 保存任务顺序
pub fn save_task_order(task_order: Vec<String>) -> Result<(), String> {
    let task_order_file = get_task_order_file()?;

    // 确保目录存在
    if let Some(parent) = task_order_file.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    let content = serde_json::to_string_pretty(&task_order)
        .map_err(|e| format!("Failed to serialize task order: {}", e))?;

    fs::write(&task_order_file, content)
        .map_err(|e| format!("Failed to write task order file: {}", e))?;

    Ok(())
}

// 加载任务顺序
pub fn load_task_order() -> Result<Vec<String>, String> {
    let task_order_file = get_task_order_file()?;

    if !task_order_file.exists() {
        return Ok(Vec::new());
    }

    let content = fs::read_to_string(&task_order_file)
        .map_err(|e| format!("Failed to read task order file: {}", e))?;

    let task_order: Vec<String> = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse task order: {}", e))?;

    Ok(task_order)
}
