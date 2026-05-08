use anyhow::{Context, Result};
use rixl::apis::{Api, ApiClient, configuration::{ApiKey, Configuration}, videos_api};
use std::{env, sync::Arc};

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = env::var("RIXL_API_KEY").context("missing RIXL_API_KEY")?;

    let client = ApiClient::new(Arc::new(Configuration {
        base_path: "https://api.rixl.com".into(),
        api_key: Some(ApiKey { prefix: None, key: api_key }),
        ..Configuration::new()
    }));

    let page = client.videos_api().list(videos_api::ListParams::builder().build()).await?;
    let data = page.data.unwrap_or_default();
    println!("listed {} videos", data.len());
    for v in &data {
        if let Some(id) = &v.id {
            println!("  - {id}");
        }
    }

    if let Ok(video_id) = env::var("VIDEO_ID") {
        let v = client.videos_api()
            .get(videos_api::GetParams::builder().video_id(video_id).build())
            .await?;
        if let Some(id) = &v.id {
            println!("video {id}");
        }
    }

    Ok(())
}
