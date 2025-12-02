// 公告获取模块

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Announcement {
    pub name: String,
    pub content: String,
}

// 获取公告列表
pub async fn get_announcements() -> Result<Vec<Announcement>, String> {
    let url = "https://services.starrailassistant.xyz/get-announcement";
    
    let response = reqwest::get(url)
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


