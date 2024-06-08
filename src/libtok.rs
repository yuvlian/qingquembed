use serde::Deserialize;
use reqwest::Error;

#[derive(Debug, Deserialize)]
pub struct TikTokApiResponse {
    pub result: Option<TikTokResult>,
}

#[derive(Debug, Deserialize)]
pub struct TikTokResult {
    pub r#type: String,
    pub images: Option<Vec<String>>,
    pub video1: Option<String>,   
}

impl TikTokApiResponse {
    pub async fn from_url(url: &str) -> Result<TikTokApiResponse, Error> {
        let response = reqwest::get(url).await?;
        let json: TikTokApiResponse = response.json().await?;
        Ok(json)
    }

    pub fn get_media_urls(&self) -> Vec<&str> {
        if let Some(result) = &self.result {
            println!("Result type: {}", result.r#type);  
            match result.r#type.as_str() {
                "image" => {
                    if let Some(images) = &result.images {
                        images.iter().map(|s| s.as_str()).collect()
                    } else {
                        Vec::new()
                    }
                }
                "video" => {
                    if let Some(video1) = &result.video1 {
                        vec![video1.as_str()]
                    } else {
                        Vec::new()
                    }
                }
                _ => Vec::new(),
            }
        } else {
            Vec::new()
        }
    }
}
