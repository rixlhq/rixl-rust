// Init returns two presigned URLs — one for the video, one for the poster image.
use anyhow::{anyhow, Context, Result};
use rixl::apis::{Api, ApiClient, configuration::{ApiKey, Configuration}, videos_api};
use rixl::models::{VideoUploadCompleteRequest, VideoUploadInitRequest};
use std::{env, sync::Arc};

const SAMPLE_VIDEO_URL: &str = "https://download.samplelib.com/mp4/sample-5s.mp4";
const SAMPLE_POSTER_URL: &str = "https://picsum.photos/seed/rixl/800/600.jpg";

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = env::var("RIXL_API_KEY").context("missing RIXL_API_KEY")?;

    let client = ApiClient::new(Arc::new(Configuration {
        base_path: "https://api.rixl.com".into(),
        api_key: Some(ApiKey { prefix: None, key: api_key }),
        ..Configuration::new()
    }));

    let video = reqwest::get(SAMPLE_VIDEO_URL).await?.bytes().await?;
    let poster = reqwest::get(SAMPLE_POSTER_URL).await?.bytes().await?;

    let init = client.videos_api()
        .upload_init(
            videos_api::UploadInitParams::builder()
                .video_upload_init_request(VideoUploadInitRequest {
                    file_name: "sample.mp4".into(),
                    image_format: Some("jpeg".into()),
                    video_quality: None,
                })
                .build(),
        )
        .await?;
    let video_id = init.video_id.clone().context("init: missing video_id")?;
    let video_url = init.video_presigned_url.context("init: missing video_presigned_url")?;
    let poster_url = init.poster_presigned_url.context("init: missing poster_presigned_url")?;

    put_bytes(&video_url, video.to_vec(), "video/mp4").await?;
    put_bytes(&poster_url, poster.to_vec(), "image/jpeg").await?;

    let v = client.videos_api()
        .upload_complete(
            videos_api::UploadCompleteParams::builder()
                .video_upload_complete_request(VideoUploadCompleteRequest {
                    video_id: Some(video_id),
                })
                .build(),
        )
        .await?;
    if let Some(id) = &v.id {
        println!("uploaded: {id}");
    }

    Ok(())
}

async fn put_bytes(url: &str, body: Vec<u8>, content_type: &str) -> Result<()> {
    let resp = reqwest::Client::new()
        .put(url)
        .header("Content-Type", content_type)
        .body(body)
        .send()
        .await?;
    if !resp.status().is_success() {
        return Err(anyhow!("PUT: {}: {}", resp.status(), resp.text().await.unwrap_or_default()));
    }
    Ok(())
}
