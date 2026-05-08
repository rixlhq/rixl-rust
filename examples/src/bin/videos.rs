use anyhow::{Context, Result};
use rixl::apis::{configuration::{ApiKey, Configuration}, videos_api};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = env::var("RIXL_API_KEY").context("missing RIXL_API_KEY")?;

    let config = Configuration {
        base_path: "https://api.rixl.com".into(),
        api_key: Some(ApiKey { prefix: None, key: api_key }),
        ..Configuration::new()
    };

    let page = videos_api::list_videos(&config, None, None, None, None).await?;
    let data = page.data.unwrap_or_default();
    println!("listed {} videos", data.len());
    for v in &data {
        if let Some(id) = &v.id {
            println!("  - {id}");
        }
    }

    if let Ok(video_id) = env::var("VIDEO_ID") {
        let v = videos_api::get_video(&config, &video_id).await?;
        if let Some(id) = &v.id {
            println!("video {id}");
        }
    }

    Ok(())
}
