use anyhow::{Context, Result};
use rixl::apis::{
    configuration::{ApiKey, Configuration},
    videos_api, Api, ApiClient,
};
use std::{env, sync::Arc};

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = env::var("RIXL_API_KEY").context("missing RIXL_API_KEY")?;
    let project_id = env::var("RIXL_PROJECT_ID").context("missing RIXL_PROJECT_ID")?;

    let client = ApiClient::new(Arc::new(Configuration {
        api_key: Some(ApiKey {
            prefix: None,
            key: api_key,
        }),
        ..Configuration::new()
    }));

    let page = client
        .videos_api()
        .list_videos(
            videos_api::ListVideosParams::builder()
                .project_id(project_id)
                .build(),
        )
        .await?;
    let data = page.videos.unwrap_or_default();
    println!("listed {} videos", data.len());
    for v in &data {
        if let Some(id) = &v.id {
            println!("  - {id}");
        }
    }

    if let Ok(video_id) = env::var("VIDEO_ID") {
        let resp = client
            .videos_api()
            .get_video(
                videos_api::GetVideoParams::builder()
                    .video_id(video_id)
                    .build(),
            )
            .await?;
        let v = resp.video.context("response missing video")?;
        if let Some(id) = &v.id {
            println!("video {id}");
        }
    }

    Ok(())
}
