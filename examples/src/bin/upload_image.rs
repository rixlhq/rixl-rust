// Upload flow: create the upload -> PUT bytes to the presigned URL.
use anyhow::{anyhow, Context, Result};
use rixl::apis::{
    configuration::{ApiKey, Configuration},
    images_api, Api, ApiClient,
};
use rixl::models::CreateImageUploadRequest;
use std::{env, sync::Arc};

const SAMPLE_IMAGE_URL: &str = "https://picsum.photos/seed/rixl/800/600.jpg";

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

    let body = reqwest::get(SAMPLE_IMAGE_URL).await?.bytes().await?;

    let upload = client
        .images_api()
        .create_image_upload(
            images_api::CreateImageUploadParams::builder()
                .project_id(project_id)
                .create_image_upload_request(CreateImageUploadRequest {
                    name: Some("sample.jpg".into()),
                    ..CreateImageUploadRequest::new()
                })
                .build(),
        )
        .await?;
    let image_id = upload.image_id.context("upload: missing image_id")?;
    let upload_url = upload.upload_url.context("upload: missing upload_url")?;

    put_bytes(&upload_url, body.to_vec(), "image/jpeg").await?;
    println!("uploaded: {image_id}");

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
