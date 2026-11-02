// Creating the upload returns two presigned URLs — one for the video, one for the poster image.
use anyhow::{anyhow, Context, Result};
use rixl::apis::{
    configuration::{ApiKey, Configuration},
    videos_api, Api, ApiClient,
};
use rixl::models::CreateVideoUploadRequest;
use std::{env, sync::Arc};

const SAMPLE_VIDEO_URL: &str = "https://download.samplelib.com/mp4/sample-5s.mp4";
const SAMPLE_POSTER_URL: &str = "https://picsum.photos/seed/rixl/800/600.jpg";

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

    let video = reqwest::get(SAMPLE_VIDEO_URL).await?.bytes().await?;
    let poster = reqwest::get(SAMPLE_POSTER_URL).await?.bytes().await?;

    let upload = client
        .videos_api()
        .create_video_upload(
            videos_api::CreateVideoUploadParams::builder()
                .project_id(project_id)
                .create_video_upload_request(CreateVideoUploadRequest {
                    name: Some("sample.mp4".into()),
                    ..CreateVideoUploadRequest::new()
                })
                .build(),
        )
        .await?;
    let video_id = upload.video_id.context("upload: missing video_id")?;
    let video_url = upload
        .video_upload_url
        .context("upload: missing video_upload_url")?;
    let poster_url = upload
        .poster_upload_url
        .context("upload: missing poster_upload_url")?;

    put_bytes(&video_url, video.to_vec(), "video/mp4").await?;
    put_bytes(&poster_url, poster.to_vec(), "image/jpeg").await?;
    println!("uploaded: {video_id}");

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
        return Err(anyhow!(
            "PUT: {}: {}",
            resp.status(),
            resp.text().await.unwrap_or_default()
        ));
    }
    Ok(())
}
