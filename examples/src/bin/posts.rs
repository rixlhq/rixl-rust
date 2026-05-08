use anyhow::{Context, Result};
use rixl::apis::{configuration::{ApiKey, Configuration}, feeds_api};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = env::var("RIXL_API_KEY").context("missing RIXL_API_KEY")?;
    let feed_id = env::var("RIXL_FEED_ID").context("missing RIXL_FEED_ID")?;
    let post_id = env::var("RIXL_POST_ID").context("missing RIXL_POST_ID")?;

    let config = Configuration {
        base_path: "https://api.rixl.com".into(),
        api_key: Some(ApiKey { prefix: None, key: api_key }),
        ..Configuration::new()
    };

    let post = feeds_api::get_feed_post(&config, &feed_id, &post_id).await?;
    if let Some(id) = &post.id {
        println!("post {id}");
    }

    Ok(())
}
