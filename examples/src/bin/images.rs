use anyhow::{Context, Result};
use rixl::apis::{
    configuration::{ApiKey, Configuration},
    images_api, Api, ApiClient,
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
        .images_api()
        .list_images(
            images_api::ListImagesParams::builder()
                .project_id(project_id)
                .build(),
        )
        .await?;
    let data = page.images.unwrap_or_default();
    println!("listed {} images", data.len());
    for img in &data {
        if let Some(id) = &img.id {
            println!("  - {id}");
        }
    }

    if let Ok(image_id) = env::var("IMAGE_ID") {
        let resp = client
            .images_api()
            .get_image(
                images_api::GetImageParams::builder()
                    .image_id(image_id)
                    .build(),
            )
            .await?;
        let img = resp.image.context("response missing image")?;
        if let (Some(id), Some(w), Some(h)) = (&img.id, img.width, img.height) {
            println!("image {id}: {w}x{h}");
        }
    }

    Ok(())
}
