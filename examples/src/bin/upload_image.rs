// Upload flow: init -> PUT bytes to the presigned URL -> complete.
use anyhow::{anyhow, Context, Result};
use rixl::apis::{configuration::{ApiKey, Configuration}, images_api};
use rixl::models::{ImageUploadCompleteRequest, ImageUploadInitRequest};
use std::env;

const SAMPLE_IMAGE_URL: &str = "https://picsum.photos/seed/rixl/800/600.jpg";

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = env::var("RIXL_API_KEY").context("missing RIXL_API_KEY")?;

    let config = Configuration {
        base_path: "https://api.rixl.com".into(),
        api_key: Some(ApiKey { prefix: None, key: api_key }),
        ..Configuration::new()
    };

    let body = reqwest::get(SAMPLE_IMAGE_URL).await?.bytes().await?;

    let init = images_api::init_image_upload(&config, ImageUploadInitRequest {
        name: Some("sample.jpg".into()),
        format: Some("jpeg".into()),
    }).await?;
    let image_id = init.image_id.context("init: missing image_id")?;
    let presigned = init.presigned_url.context("init: missing presigned_url")?;

    put_bytes(&presigned, body.to_vec(), "image/jpeg").await?;

    let img = images_api::complete_image_upload(&config, ImageUploadCompleteRequest {
        image_id: Some(image_id),
        attached_to_video: Some(false),
    }).await?;
    if let (Some(id), Some(w), Some(h)) = (&img.id, img.width, img.height) {
        println!("uploaded: {id} {w}x{h}");
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
