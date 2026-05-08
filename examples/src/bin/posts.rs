use anyhow::{Context, Result};
use rixl::apis::{
    Api, ApiClient,
    configuration::{ApiKey, Configuration},
    feeds_api::GetFeedPostParams,
};
use std::{env, sync::Arc};

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = env::var("RIXL_API_KEY").context("missing RIXL_API_KEY")?;
    let feed_id = env::var("RIXL_FEED_ID").context("missing RIXL_FEED_ID")?;
    let post_id = env::var("RIXL_POST_ID").context("missing RIXL_POST_ID")?;

    let client = ApiClient::new(Arc::new(Configuration {
        base_path: "https://api.rixl.com".into(),
        api_key: Some(ApiKey { prefix: None, key: api_key }),
        ..Configuration::new()
    }));

    let post = client.feeds_api()
        .get_feed_post(GetFeedPostParams::builder().feed_id(feed_id).post_id(post_id).build())
        .await?;
    if let Some(id) = &post.id {
        println!("post {id}");
    }

    Ok(())
}
