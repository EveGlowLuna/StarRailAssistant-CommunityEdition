// 应用设置管理模块

use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

// 主要设置（保存到 settings.json）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MainSettings {
    pub language: i32,
    pub zoom: f64,
    pub confidence_threshold: f64,
    pub app_channel: i32,
    pub proxies: Vec<String>,
    pub allow_notifications: bool,
    pub allow_system_notifications: bool,
    pub allow_email_notifications: bool,
    pub enable_startup_launch: bool,
    pub enable_minimize_to_tray: bool,
}

impl Default for MainSettings {
    fn default() -> Self {
        Self {
            language: 1,
            zoom: 1.0,
            confidence_threshold: 0.9,
            app_channel: 0,
            proxies: vec!["https://tvv.tw/".to_string()],
            allow_notifications: true,
            allow_system_notifications: true,
            allow_email_notifications: false,
            enable_startup_launch: false,
            enable_minimize_to_tray: false,
        }
    }
}

// CE 专属设置（保存到 SRA-CE-Settings.json）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CESettings {
    pub wallpaper_path: Option<String>,
}

impl Default for CESettings {
    fn default() -> Self {
        Self {
            wallpaper_path: None,
        }
    }
}

// 完整的应用设置（用于前端交互）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppSettings {
    pub language: i32,
    pub zoom: f64,
    pub confidence_threshold: f64,
    pub app_channel: i32,
    pub proxies: Vec<String>,
    pub allow_notifications: bool,
    pub allow_system_notifications: bool,
    pub allow_email_notifications: bool,
    pub enable_startup_launch: bool,
    pub enable_minimize_to_tray: bool,
    pub wallpaper_path: Option<String>,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            language: 1,
            zoom: 1.0,
            confidence_threshold: 0.9,
            app_channel: 0,
            proxies: vec!["https://tvv.tw/".to_string()],
            allow_notifications: true,
            allow_system_notifications: true,
            allow_email_notifications: false,
            enable_startup_launch: false,
            enable_minimize_to_tray: false,
            wallpaper_path: None,
        }
    }
}

// 获取主设置文件路径（settings.json）
fn get_main_settings_file() -> Result<PathBuf, String> {
    let appdata = std::env::var("APPDATA").map_err(|_| "Failed to get APPDATA path")?;
    Ok(PathBuf::from(appdata).join("SRA").join("settings.json"))
}

// 获取 CE 设置文件路径（SRA-CE-Settings.json）
fn get_ce_settings_file() -> Result<PathBuf, String> {
    let appdata = std::env::var("APPDATA").map_err(|_| "Failed to get APPDATA path")?;
    Ok(PathBuf::from(appdata).join("SRA").join("SRA-CE-Settings.json"))
}

// 加载设置
pub fn load_settings() -> Result<AppSettings, String> {
    // 加载主设置
    let main_settings_file = get_main_settings_file()?;
    let main_settings = if main_settings_file.exists() {
        let content = fs::read_to_string(&main_settings_file)
            .map_err(|e| format!("Failed to read main settings file: {}", e))?;
        serde_json::from_str(&content).unwrap_or_else(|_| MainSettings::default())
    } else {
        MainSettings::default()
    };

    // 加载 CE 设置
    let ce_settings_file = get_ce_settings_file()?;
    let ce_settings = if ce_settings_file.exists() {
        let content = fs::read_to_string(&ce_settings_file)
            .map_err(|e| format!("Failed to read CE settings file: {}", e))?;
        serde_json::from_str(&content).unwrap_or_else(|_| CESettings::default())
    } else {
        CESettings::default()
    };

    // 合并设置
    Ok(AppSettings {
        language: main_settings.language,
        zoom: main_settings.zoom,
        confidence_threshold: main_settings.confidence_threshold,
        app_channel: main_settings.app_channel,
        proxies: main_settings.proxies,
        allow_notifications: main_settings.allow_notifications,
        allow_system_notifications: main_settings.allow_system_notifications,
        allow_email_notifications: main_settings.allow_email_notifications,
        enable_startup_launch: main_settings.enable_startup_launch,
        enable_minimize_to_tray: main_settings.enable_minimize_to_tray,
        wallpaper_path: ce_settings.wallpaper_path,
    })
}

// 保存设置
pub fn save_settings(settings: AppSettings) -> Result<(), String> {
    // 分离主设置
    let main_settings = MainSettings {
        language: settings.language,
        zoom: settings.zoom,
        confidence_threshold: settings.confidence_threshold,
        app_channel: settings.app_channel,
        proxies: settings.proxies,
        allow_notifications: settings.allow_notifications,
        allow_system_notifications: settings.allow_system_notifications,
        allow_email_notifications: settings.allow_email_notifications,
        enable_startup_launch: settings.enable_startup_launch,
        enable_minimize_to_tray: settings.enable_minimize_to_tray,
    };

    // 分离 CE 设置
    let ce_settings = CESettings {
        wallpaper_path: settings.wallpaper_path,
    };

    // 保存主设置到 settings.json
    let main_settings_file = get_main_settings_file()?;
    if let Some(parent) = main_settings_file.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create settings directory: {}", e))?;
    }
    let main_content = serde_json::to_string_pretty(&main_settings)
        .map_err(|e| format!("Failed to serialize main settings: {}", e))?;
    fs::write(&main_settings_file, main_content)
        .map_err(|e| format!("Failed to write main settings file: {}", e))?;

    // 保存 CE 设置到 SRA-CE-Settings.json
    let ce_settings_file = get_ce_settings_file()?;
    if let Some(parent) = ce_settings_file.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create CE settings directory: {}", e))?;
    }
    let ce_content = serde_json::to_string_pretty(&ce_settings)
        .map_err(|e| format!("Failed to serialize CE settings: {}", e))?;
    fs::write(&ce_settings_file, ce_content)
        .map_err(|e| format!("Failed to write CE settings file: {}", e))?;

    Ok(())
}
