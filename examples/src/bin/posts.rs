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
    let post_id = env::var("RIXL_POST_ID").context("missing RIXL_POST_ID")?;

    let client = ApiClient::new(Arc::new(Configuration {
        api_key: Some(ApiKey {
            prefix: None,
            key: api_key,
        }),
        ..Configuration::new()
    }));

    let resp = client
        .posts_api()
        .get_post(
            posts_api::GetPostParams::builder()
                .project_id(project_id)
                .post_id(post_id)
                .feed_id(feed_id)
                .build(),
        )
        .await?;
    let post = resp.post.context("response missing post")?;
    if let Some(id) = &post.id {
        println!("post {id}");
    }

    Ok(())
}
