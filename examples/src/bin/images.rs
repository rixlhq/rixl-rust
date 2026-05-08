use anyhow::{Context, Result};
use rixl::apis::{configuration::{ApiKey, Configuration}, images_api};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = env::var("RIXL_API_KEY").context("missing RIXL_API_KEY")?;

    let config = Configuration {
        base_path: "https://api.rixl.com".into(),
        api_key: Some(ApiKey { prefix: None, key: api_key }),
        ..Configuration::new()
    };

    let page = images_api::list_images(&config, None, None, None, None).await?;
    let data = page.data.unwrap_or_default();
    println!("listed {} images", data.len());
    for img in &data {
        if let Some(id) = &img.id {
            println!("  - {id}");
        }
    }

    if let Ok(image_id) = env::var("IMAGE_ID") {
        let img = images_api::get_image(&config, &image_id).await?;
        if let (Some(id), Some(w), Some(h)) = (&img.id, img.width, img.height) {
            println!("image {id}: {w}x{h}");
        }
    }

    Ok(())
}
