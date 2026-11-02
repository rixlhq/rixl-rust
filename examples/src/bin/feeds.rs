use anyhow::{Context, Result};
use rixl::apis::{
    configuration::{ApiKey, Configuration},
    posts_api, Api, ApiClient,
};
use std::{env, sync::Arc};

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = env::var("RIXL_API_KEY").context("missing RIXL_API_KEY")?;
    let project_id = env::var("RIXL_PROJECT_ID").context("missing RIXL_PROJECT_ID")?;
    let feed_id = env::var("RIXL_FEED_ID").context("missing RIXL_FEED_ID")?;

    let client = ApiClient::new(Arc::new(Configuration {
        api_key: Some(ApiKey {
            prefix: None,
            key: api_key,
        }),
        ..Configuration::new()
    }));

    let page = client
        .posts_api()
        .list_posts(
            posts_api::ListPostsParams::builder()
                .project_id(project_id)
                .feed_id(feed_id.clone())
                .build(),
        )
        .await?;
    let data = page.posts.unwrap_or_default();
    println!("feed {feed_id} — {} posts", data.len());
    for post in &data {
        if let Some(id) = &post.id {
            println!("  - {id}");
        }
    }

    Ok(())
}
