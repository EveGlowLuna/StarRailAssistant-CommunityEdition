// 公告获取模块

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Announcement {
    pub name: String,
    pub content: String,
}

// 获取公告列表（根据语言）
pub async fn get_announcements(lang: String) -> Result<Vec<Announcement>, String> {
    let url = "https://services.starrailassistant.xyz/get-announcement";
    
    let response = reqwest::get(url)
        .await
        .map_err(|e| format!("Failed to fetch announcements: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("Server returned error: {}", response.status()));
    }
    
    let announcements_map = response
        .json::<HashMap<String, Vec<Announcement>>>()
        .await
        .map_err(|e| format!("Failed to parse announcements: {}", e))?;
    
    // 根据语言获取对应的公告，如果不存在则返回空数组
    let lang_key = if lang.starts_with("zh") { "zh" } else { "en" };
    
    Ok(announcements_map
        .get(lang_key)
        .cloned()
        .unwrap_or_else(Vec::new))
}


