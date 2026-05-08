use anyhow::{Context, Result};
use rixl::apis::{Api, ApiClient, configuration::{ApiKey, Configuration}, feeds_api};
use std::{env, sync::Arc};

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = env::var("RIXL_API_KEY").context("missing RIXL_API_KEY")?;
    let feed_id = env::var("RIXL_FEED_ID").context("missing RIXL_FEED_ID")?;

    let client = ApiClient::new(Arc::new(Configuration {
        base_path: "https://api.rixl.com".into(),
        api_key: Some(ApiKey { prefix: None, key: api_key }),
        ..Configuration::new()
    }));

    let page = client.feeds_api()
        .list(feeds_api::ListParams::builder().feed_id(feed_id.clone()).build())
        .await?;
    let data = page.data.unwrap_or_default();
    println!("feed {feed_id} — {} posts", data.len());
    for post in &data {
        if let Some(id) = &post.id {
            println!("  - {id}");
        }
    }

    Ok(())
}
