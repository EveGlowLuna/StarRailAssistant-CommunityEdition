// 壁纸管理模块

use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::logger::{log, LogSource, LogLevel};

#[derive(Debug, Serialize, Deserialize)]
pub struct WallpaperSettings {
    pub wallpaper_path: Option<String>,
}

// 获取设置文件路径
fn get_settings_file() -> Result<PathBuf, String> {
    let appdata = std::env::var("APPDATA").map_err(|_| "Failed to get APPDATA path")?;
    Ok(PathBuf::from(appdata).join("SRA").join("SRA-CE-Settings.json"))
}

// 获取缓存目录路径
fn get_cache_dir() -> Result<PathBuf, String> {
    let appdata = std::env::var("APPDATA").map_err(|_| "Failed to get APPDATA path")?;
    Ok(PathBuf::from(appdata).join("SRA").join("SRA-CE-Cache"))
}



// 加载壁纸设置
pub fn load_wallpaper_settings() -> Result<WallpaperSettings, String> {
    let settings_file = get_settings_file()?;
    
    if !settings_file.exists() {
        // 如果设置文件不存在，返回默认设置
        return Ok(WallpaperSettings {
            wallpaper_path: None,
        });
    }
    
    let content = fs::read_to_string(&settings_file)
        .map_err(|e| format!("Failed to read settings file: {}", e))?;
    
    let settings: WallpaperSettings = serde_json::from_str(&content)
        .unwrap_or(WallpaperSettings {
            wallpaper_path: None,
        });
    
    Ok(settings)
}

// 保存壁纸设置
pub fn save_wallpaper_settings(settings: &WallpaperSettings) -> Result<(), String> {
    let settings_file = get_settings_file()?;
    
    // 确保目录存在
    if let Some(parent) = settings_file.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create settings directory: {}", e))?;
    }
    
    let content = serde_json::to_string_pretty(&settings)
        .map_err(|e| format!("Failed to serialize settings: {}", e))?;
    
    fs::write(&settings_file, content)
        .map_err(|e| format!("Failed to write settings file: {}", e))?;
    
    Ok(())
}

// 设置壁纸（复制文件到缓存目录）
pub fn set_wallpaper(source_path: String) -> Result<String, String> {
    let _ = log(LogSource::Backend, LogLevel::DEBUG, format!("设置壁纸: {}", source_path));
    
    let source = PathBuf::from(&source_path);
    
    // 检查源文件是否存在
    if !source.exists() {
        let _ = log(LogSource::Backend, LogLevel::ERR, "源文件不存在".to_string());
        return Err("Source wallpaper file does not exist".to_string());
    }
    
    // 获取文件扩展名
    let extension = source.extension()
        .and_then(|e| e.to_str())
        .ok_or("Failed to get file extension")?;
    
    let _ = log(LogSource::Backend, LogLevel::DEBUG, format!("文件扩展名: {}", extension));
    
    // 获取缓存目录
    let cache_dir = get_cache_dir()?;
    let _ = log(LogSource::Backend, LogLevel::DEBUG, format!("缓存目录: {:?}", cache_dir));
    
    fs::create_dir_all(&cache_dir)
        .map_err(|e| format!("Failed to create cache directory: {}", e))?;
    
    // 目标文件路径
    let cached_path = cache_dir.join(format!("wallpaper.{}", extension));
    let _ = log(LogSource::Backend, LogLevel::DEBUG, format!("目标路径: {:?}", cached_path));
    
    // 复制文件
    fs::copy(&source, &cached_path)
        .map_err(|e| {
            let _ = log(LogSource::Backend, LogLevel::ERR, format!("复制文件失败: {}", e));
            format!("Failed to copy wallpaper file: {}", e)
        })?;
    
    let _ = log(LogSource::Backend, LogLevel::SUCCESS, "文件复制成功".to_string());
    
    // 保存设置
    let settings = WallpaperSettings {
        wallpaper_path: Some(cached_path.to_string_lossy().to_string()),
    };
    save_wallpaper_settings(&settings)?;
    
    let _ = log(LogSource::Backend, LogLevel::SUCCESS, "壁纸设置已保存".to_string());
    
    Ok(cached_path.to_string_lossy().to_string())
}

// 重置壁纸（删除缓存的壁纸文件）
pub fn reset_wallpaper() -> Result<(), String> {
    // 删除缓存的壁纸文件
    let cache_dir = get_cache_dir()?;
    if cache_dir.exists() {
        // 删除所有以 wallpaper 开头的文件
        let entries = fs::read_dir(&cache_dir)
            .map_err(|e| format!("Failed to read cache directory: {}", e))?;
        
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(file_name) = path.file_name() {
                    if let Some(name_str) = file_name.to_str() {
                        if name_str.starts_with("wallpaper") {
                            let _ = fs::remove_file(&path);
                        }
                    }
                }
            }
        }
    }
    
    // 清空设置
    let settings = WallpaperSettings {
        wallpaper_path: None,
    };
    save_wallpaper_settings(&settings)?;
    
    Ok(())
}

// 获取当前壁纸路径
pub fn get_current_wallpaper() -> Result<Option<String>, String> {
    let settings = load_wallpaper_settings()?;
    
    // 如果有壁纸路径，检查文件是否存在
    if let Some(ref path) = settings.wallpaper_path {
        let wallpaper_path = PathBuf::from(path);
        if wallpaper_path.exists() {
            return Ok(Some(path.clone()));
        } else {
            // 文件不存在，重置壁纸
            reset_wallpaper()?;
            return Ok(None);
        }
    }
    
    Ok(None)
}

// 获取壁纸的base64编码
pub fn get_wallpaper_base64() -> Result<Option<String>, String> {
    let _ = log(LogSource::Backend, LogLevel::DEBUG, "开始获取壁纸 base64".to_string());
    
    let settings = load_wallpaper_settings()?;
    
    // 如果有壁纸路径，读取文件并转换为base64
    if let Some(ref path) = settings.wallpaper_path {
        let wallpaper_path = PathBuf::from(path);
        
        let _ = log(LogSource::Backend, LogLevel::DEBUG, format!("壁纸路径: {:?}", wallpaper_path));
        let _ = log(LogSource::Backend, LogLevel::DEBUG, format!("文件存在: {}", wallpaper_path.exists()));
        
        if wallpaper_path.exists() {
            // 读取文件
            let image_data = fs::read(&wallpaper_path)
                .map_err(|e| {
                    let _ = log(LogSource::Backend, LogLevel::ERR, format!("读取文件失败: {}", e));
                    format!("Failed to read wallpaper file: {}", e)
                })?;
            
            let _ = log(LogSource::Backend, LogLevel::DEBUG, format!("成功读取 {} 字节", image_data.len()));
            
            // 转换为base64
            let base64_data = base64_encode(&image_data);
            
            // 获取文件扩展名以确定MIME类型
            let mime_type = match wallpaper_path.extension().and_then(|e| e.to_str()) {
                Some("png") => "image/png",
                Some("jpg") | Some("jpeg") => "image/jpeg",
                Some("webp") => "image/webp",
                Some("gif") => "image/gif",
                _ => "image/jpeg", // 默认
            };
            
            let _ = log(LogSource::Backend, LogLevel::DEBUG, format!("MIME 类型: {}", mime_type));
            let _ = log(LogSource::Backend, LogLevel::SUCCESS, format!("返回 base64 数据，长度: {}", base64_data.len()));
            
            // 返回data URL格式
            return Ok(Some(format!("data:{};base64,{}", mime_type, base64_data)));
        } else {
            let _ = log(LogSource::Backend, LogLevel::WARN, "文件不存在，重置壁纸".to_string());
            // 文件不存在，重置壁纸
            reset_wallpaper()?;
            return Ok(None);
        }
    }
    
    let _ = log(LogSource::Backend, LogLevel::DEBUG, "没有配置壁纸路径".to_string());
    Ok(None)
}

// Base64编码函数
fn base64_encode(data: &[u8]) -> String {
    use base64::{Engine as _, engine::general_purpose};
    general_purpose::STANDARD.encode(data)
}
