use anyhow::{Context, Result};
use rixl::apis::{configuration::{ApiKey, Configuration}, feeds_api};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = env::var("RIXL_API_KEY").context("missing RIXL_API_KEY")?;
    let feed_id = env::var("RIXL_FEED_ID").context("missing RIXL_FEED_ID")?;

    let config = Configuration {
        base_path: "https://api.rixl.com".into(),
        api_key: Some(ApiKey { prefix: None, key: api_key }),
        ..Configuration::new()
    };

    let page = feeds_api::list_feed_posts(&config, &feed_id, None, None).await?;
    let data = page.data.unwrap_or_default();
    println!("feed {feed_id} — {} posts", data.len());
    for post in &data {
        if let Some(id) = &post.id {
            println!("  - {id}");
        }
    }

    Ok(())
}
