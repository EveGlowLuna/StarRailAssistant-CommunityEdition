// 公告获取模块

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Announcement {
    pub title: String,
    pub content: String,
}

// 获取公告列表（根据语言）
pub async fn get_announcements(lang: String) -> Result<Vec<Announcement>, String> {
    // 根据语言确定请求的语言代码
    let lang_code = if lang.starts_with("zh") { "zh" } else { "en" };
    
    let url = format!("https://services.starrailassistant.xyz/get-announcement/{}", lang_code);
    
    let response = reqwest::get(&url)
        .await
        .map_err(|e| format!("Failed to fetch announcements: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("Server returned error: {}", response.status()));
    }
    
    let announcements = response
        .json::<Vec<Announcement>>()
        .await
        .map_err(|e| format!("Failed to parse announcements: {}", e))?;
    
    Ok(announcements)
}


