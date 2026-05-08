use anyhow::{Context, Result};
use rixl::apis::{Api, ApiClient, configuration::{ApiKey, Configuration}, images};
use std::{env, sync::Arc};

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = env::var("RIXL_API_KEY").context("missing RIXL_API_KEY")?;

    let client = ApiClient::new(Arc::new(Configuration {
        base_path: "https://api.rixl.com".into(),
        api_key: Some(ApiKey { prefix: None, key: api_key }),
        ..Configuration::new()
    }));

    let page = client.images().list(images::ListParams::builder().build()).await?;
    let data = page.data.unwrap_or_default();
    println!("listed {} images", data.len());
    for img in &data {
        if let Some(id) = &img.id {
            println!("  - {id}");
        }
    }

    if let Ok(image_id) = env::var("IMAGE_ID") {
        let img = client.images()
            .get(images::GetParams::builder().image_id(image_id).build())
            .await?;
        if let (Some(id), Some(w), Some(h)) = (&img.id, img.width, img.height) {
            println!("image {id}: {w}x{h}");
        }
    }

    Ok(())
}
