// Pick one auth flow:
//   API key:    RIXL_API_KEY=..., RIXL_PROJECT_ID=...
//   Client JWT: RIXL_CLIENT_ID=..., RIXL_CLIENT_SECRET=..., RIXL_PROJECT_ID=..., RIXL_SUBJECT=...
use anyhow::{Context, Result};
use rixl::apis::{
    client_credentials_api,
    configuration::{ApiKey, Configuration},
    images_api, Api, ApiClient,
};
use rixl::models::ClientauthV1MintClientTokenRequest;
use std::{env, sync::Arc};

#[tokio::main]
async fn main() -> Result<()> {
    let project_id = env::var("RIXL_PROJECT_ID").context("missing RIXL_PROJECT_ID")?;
    let client = ApiClient::new(Arc::new(build_config(&project_id).await?));

    let page = client
        .images_api()
        .list_images(
            images_api::ListImagesParams::builder()
                .project_id(project_id)
                .build(),
        )
        .await?;
    let n = page.images.unwrap_or_default().len();
    println!("auth ok — listed {n} images");

    Ok(())
}

async fn build_config(project_id: &str) -> Result<Configuration> {
    if let Ok(key) = env::var("RIXL_API_KEY") {
        return Ok(Configuration {
            api_key: Some(ApiKey { prefix: None, key }),
            ..Configuration::new()
        });
    }

    let client_id = env::var("RIXL_CLIENT_ID")
        .context("set RIXL_API_KEY, or RIXL_CLIENT_ID + RIXL_CLIENT_SECRET + RIXL_SUBJECT")?;
    let client_secret = env::var("RIXL_CLIENT_SECRET").context("missing RIXL_CLIENT_SECRET")?;
    let subject = env::var("RIXL_SUBJECT").context("missing RIXL_SUBJECT")?;

    let token = mint_token(client_id, client_secret, subject, project_id.to_owned()).await?;
    Ok(Configuration {
        bearer_access_token: Some(token),
        ..Configuration::new()
    })
}

async fn mint_token(
    client_id: String,
    client_secret: String,
    subject: String,
    project_id: String,
) -> Result<String> {
    let mut request = ClientauthV1MintClientTokenRequest::new(client_id, client_secret, subject);
    request.project_id = Some(project_id);

    let client = ApiClient::new(Arc::new(Configuration::new()));
    let resp = client
        .client_credentials_api()
        .mint_client_token(
            client_credentials_api::MintClientTokenParams::builder()
                .clientauth_v1_mint_client_token_request(request)
                .build(),
        )
        .await?;
    resp.access_token.context("response missing access_token")
}
