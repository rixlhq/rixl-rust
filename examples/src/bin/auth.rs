// Pick one auth flow:
//   API key:    RIXL_API_KEY=...
//   Client JWT: RIXL_CLIENT_ID=..., RIXL_CLIENT_SECRET=..., RIXL_PROJECT_ID=..., RIXL_SUBJECT=...
use anyhow::{anyhow, Context, Result};
use rixl::apis::{Api, ApiClient, configuration::{ApiKey, Configuration}, images_api};
use serde_json::json;
use std::{env, sync::Arc};

#[tokio::main]
async fn main() -> Result<()> {
    let client = ApiClient::new(Arc::new(build_config().await?));

    let page = client.images_api().list(images_api::ListParams::builder().build()).await?;
    let n = page.data.unwrap_or_default().len();
    println!("auth ok — listed {n} images");

    Ok(())
}

async fn build_config() -> Result<Configuration> {
    if let Ok(key) = env::var("RIXL_API_KEY") {
        return Ok(Configuration {
            api_key: Some(ApiKey { prefix: None, key }),
            ..Configuration::new()
        });
    }

    let client_id = env::var("RIXL_CLIENT_ID")
        .context("set RIXL_API_KEY, or RIXL_CLIENT_ID + RIXL_CLIENT_SECRET + RIXL_PROJECT_ID + RIXL_SUBJECT")?;
    let client_secret = env::var("RIXL_CLIENT_SECRET").context("missing RIXL_CLIENT_SECRET")?;
    let subject = env::var("RIXL_SUBJECT").context("missing RIXL_SUBJECT")?;
    let project_id = env::var("RIXL_PROJECT_ID").context("missing RIXL_PROJECT_ID")?;

    let token = mint_token(&client_id, &client_secret, &subject, &project_id).await?;
    Ok(Configuration {
        bearer_access_token: Some(token),
        ..Configuration::new()
    })
}

async fn mint_token(client_id: &str, client_secret: &str, subject: &str, project_id: &str) -> Result<String> {
    let body = json!({
        "client_id": client_id,
        "client_secret": client_secret,
        "subject": subject,
        "project_id": project_id,
    });
    let resp = reqwest::Client::new()
        .post("https://api.rixl.com/clientauth/token")
        .json(&body)
        .send()
        .await?;
    if !resp.status().is_success() {
        return Err(anyhow!("mint token: {}: {}", resp.status(), resp.text().await.unwrap_or_default()));
    }
    let payload: serde_json::Value = resp.json().await?;
    payload["access_token"]
        .as_str()
        .map(String::from)
        .ok_or_else(|| anyhow!("response missing access_token"))
}
