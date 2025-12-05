// 应用设置管理模块

use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// 主要设置（保存到 settings.json）- 使用PascalCase以匹配原版SRA
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct MainSettings {
    // 原版SRA字段 - 必须保留以保持兼容性
    pub mirror_chyan_cdk: String,
    pub activity_hotkey: String,
    pub chronicle_hotkey: String,
    pub warp_hotkey: String,
    pub guide_hotkey: String,
    pub map_hotkey: String,
    pub technique_hotkey: String,
    pub allow_email_notifications: bool,
    pub allow_notifications: bool,
    pub allow_system_notifications: bool,
    pub app_channel: i32,
    pub background_opacity: f64,
    pub ctrl_panel_opacity: f64,
    pub background_image_path: String,
    pub confidence_threshold: f64,
    pub default_page: i32,
    pub download_channel: i32,
    pub email_auth_code: String,
    pub email_receiver: String,
    pub email_sender: String,
    pub enable_auto_update: bool,
    pub enable_minimize_to_tray: bool,
    pub enable_startup_launch: bool,
    pub language: i32,
    pub proxies: Vec<String>,
    pub smtp_port: i32,
    pub smtp_server: String,
    pub thread_safety: bool,
    pub zoom: f64,
}

impl Default for MainSettings {
    fn default() -> Self {
        Self {
            mirror_chyan_cdk: String::new(),
            activity_hotkey: "F1".to_string(),
            chronicle_hotkey: "F2".to_string(),
            warp_hotkey: "F3".to_string(),
            guide_hotkey: "F4".to_string(),
            map_hotkey: "M".to_string(),
            technique_hotkey: "E".to_string(),
            allow_email_notifications: false,
            allow_notifications: true,
            allow_system_notifications: true,
            app_channel: 0,
            background_opacity: 0.9,
            ctrl_panel_opacity: 0.9,
            background_image_path: String::new(),
            confidence_threshold: 0.9,
            default_page: 0,
            download_channel: 1,
            email_auth_code: String::new(),
            email_receiver: String::new(),
            email_sender: String::new(),
            enable_auto_update: true,
            enable_minimize_to_tray: false,
            enable_startup_launch: false,
            language: 1,
            proxies: vec!["https://tvv.tw/".to_string()],
            smtp_port: 465,
            smtp_server: "smtp.qq.com".to_string(),
            thread_safety: false,
            zoom: 1.25,
        }
    }
}

// CE 专属设置（保存到 SRA-CE-Settings.json）
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
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
    // 界面设置
    pub language: i32,
    pub zoom: f64,
    pub confidence_threshold: f64,
    
    // 通知设置
    pub allow_notifications: bool,
    pub allow_system_notifications: bool,
    pub allow_email_notifications: bool,
    
    // 邮件设置
    pub smtp_server: String,
    pub smtp_port: i32,
    pub email_sender: String,
    pub email_auth_code: String,
    pub email_receiver: String,
    
    // 游戏内快捷键
    pub activity_hotkey: String,
    pub chronicle_hotkey: String,
    pub warp_hotkey: String,
    pub guide_hotkey: String,
    pub map_hotkey: String,
    pub technique_hotkey: String,
    
    // 应用设置
    pub enable_startup_launch: bool,
    pub enable_minimize_to_tray: bool,
    
    // CE专属设置
    pub wallpaper_path: Option<String>,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            language: 1,
            zoom: 125.0,
            confidence_threshold: 0.9,
            allow_notifications: true,
            allow_system_notifications: true,
            allow_email_notifications: false,
            smtp_server: "smtp.qq.com".to_string(),
            smtp_port: 465,
            email_sender: String::new(),
            email_auth_code: String::new(),
            email_receiver: String::new(),
            activity_hotkey: "F1".to_string(),
            chronicle_hotkey: "F2".to_string(),
            warp_hotkey: "F3".to_string(),
            guide_hotkey: "F4".to_string(),
            map_hotkey: "M".to_string(),
            technique_hotkey: "E".to_string(),
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
        let default_settings = MainSettings::default();
        // 创建默认settings.json
        if let Some(parent) = main_settings_file.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let content = serde_json::to_string_pretty(&default_settings)
            .map_err(|e| format!("Failed to serialize default settings: {}", e))?;
        let _ = fs::write(&main_settings_file, content);
        default_settings
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

    // 合并设置 - zoom需要转换为百分比
    Ok(AppSettings {
        language: main_settings.language,
        zoom: main_settings.zoom * 100.0, // 转换为百分比
        confidence_threshold: main_settings.confidence_threshold,
        allow_notifications: main_settings.allow_notifications,
        allow_system_notifications: main_settings.allow_system_notifications,
        allow_email_notifications: main_settings.allow_email_notifications,
        smtp_server: main_settings.smtp_server,
        smtp_port: main_settings.smtp_port,
        email_sender: main_settings.email_sender,
        email_auth_code: main_settings.email_auth_code,
        email_receiver: main_settings.email_receiver,
        activity_hotkey: main_settings.activity_hotkey,
        chronicle_hotkey: main_settings.chronicle_hotkey,
        warp_hotkey: main_settings.warp_hotkey,
        guide_hotkey: main_settings.guide_hotkey,
        map_hotkey: main_settings.map_hotkey,
        technique_hotkey: main_settings.technique_hotkey,
        enable_startup_launch: main_settings.enable_startup_launch,
        enable_minimize_to_tray: main_settings.enable_minimize_to_tray,
        wallpaper_path: ce_settings.wallpaper_path,
    })
}

// 保存设置
pub fn save_settings(settings: AppSettings) -> Result<(), String> {
    // 读取现有的settings.json以保留其他字段
    let main_settings_file = get_main_settings_file()?;
    let mut main_json: Value = if main_settings_file.exists() {
        let content = fs::read_to_string(&main_settings_file)
            .map_err(|e| format!("Failed to read main settings file: {}", e))?;
        serde_json::from_str(&content).unwrap_or(serde_json::json!({}))
    } else {
        serde_json::json!({})
    };

    // 更新CE管理的字段 - zoom需要转换回小数
    if let Some(obj) = main_json.as_object_mut() {
        obj.insert("Language".to_string(), serde_json::json!(settings.language));
        obj.insert("Zoom".to_string(), serde_json::json!(settings.zoom / 100.0)); // 转换回小数
        obj.insert("ConfidenceThreshold".to_string(), serde_json::json!(settings.confidence_threshold));
        obj.insert("AllowNotifications".to_string(), serde_json::json!(settings.allow_notifications));
        obj.insert("AllowSystemNotifications".to_string(), serde_json::json!(settings.allow_system_notifications));
        obj.insert("AllowEmailNotifications".to_string(), serde_json::json!(settings.allow_email_notifications));
        obj.insert("SmtpServer".to_string(), serde_json::json!(settings.smtp_server));
        obj.insert("SmtpPort".to_string(), serde_json::json!(settings.smtp_port));
        obj.insert("EmailSender".to_string(), serde_json::json!(settings.email_sender));
        obj.insert("EmailAuthCode".to_string(), serde_json::json!(settings.email_auth_code));
        obj.insert("EmailReceiver".to_string(), serde_json::json!(settings.email_receiver));
        obj.insert("ActivityHotkey".to_string(), serde_json::json!(settings.activity_hotkey));
        obj.insert("ChronicleHotkey".to_string(), serde_json::json!(settings.chronicle_hotkey));
        obj.insert("WarpHotkey".to_string(), serde_json::json!(settings.warp_hotkey));
        obj.insert("GuideHotkey".to_string(), serde_json::json!(settings.guide_hotkey));
        obj.insert("MapHotkey".to_string(), serde_json::json!(settings.map_hotkey));
        obj.insert("TechniqueHotkey".to_string(), serde_json::json!(settings.technique_hotkey));
        obj.insert("EnableStartupLaunch".to_string(), serde_json::json!(settings.enable_startup_launch));
        obj.insert("EnableMinimizeToTray".to_string(), serde_json::json!(settings.enable_minimize_to_tray));
        
        // 确保必需字段存在（如果不存在则使用默认值）
        if !obj.contains_key("MirrorChyanCdk") {
            obj.insert("MirrorChyanCdk".to_string(), serde_json::json!(""));
        }
        if !obj.contains_key("BackgroundOpacity") {
            obj.insert("BackgroundOpacity".to_string(), serde_json::json!(0.9));
        }
        if !obj.contains_key("CtrlPanelOpacity") {
            obj.insert("CtrlPanelOpacity".to_string(), serde_json::json!(0.9));
        }
        if !obj.contains_key("BackgroundImagePath") {
            obj.insert("BackgroundImagePath".to_string(), serde_json::json!(""));
        }
        if !obj.contains_key("DefaultPage") {
            obj.insert("DefaultPage".to_string(), serde_json::json!(0));
        }
        if !obj.contains_key("DownloadChannel") {
            obj.insert("DownloadChannel".to_string(), serde_json::json!(1));
        }
        if !obj.contains_key("EnableAutoUpdate") {
            obj.insert("EnableAutoUpdate".to_string(), serde_json::json!(true));
        }
        if !obj.contains_key("Proxies") {
            obj.insert("Proxies".to_string(), serde_json::json!(["https://tvv.tw/"]));
        }
        if !obj.contains_key("ThreadSafety") {
            obj.insert("ThreadSafety".to_string(), serde_json::json!(false));
        }
        if !obj.contains_key("AppChannel") {
            obj.insert("AppChannel".to_string(), serde_json::json!(0));
        }
    }

    // 保存主设置到 settings.json
    if let Some(parent) = main_settings_file.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create settings directory: {}", e))?;
    }
    let main_content = serde_json::to_string_pretty(&main_json)
        .map_err(|e| format!("Failed to serialize main settings: {}", e))?;
    fs::write(&main_settings_file, main_content)
        .map_err(|e| format!("Failed to write main settings file: {}", e))?;

    // 分离 CE 设置
    let ce_settings = CESettings {
        wallpaper_path: settings.wallpaper_path,
    };

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
