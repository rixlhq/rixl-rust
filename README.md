# rixl-rust

Rust client for the [RIXL](https://rixl.com) API.

## Install

```toml
[dependencies]
rixl = "0.1"
```

Requires Rust 1.75+ and `tokio`. The `bon` builder feature is on by default.

## Quick start

```rust
use rixl::apis::{Api, ApiClient, configuration::{ApiKey, Configuration}, images_api};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ApiClient::new(Arc::new(Configuration {
        base_path: "https://api.rixl.com".into(),
        api_key: Some(ApiKey { prefix: None, key: std::env::var("RIXL_API_KEY")? }),
        ..Configuration::new()
    }));

    let page = client.images_api()
        .list(images_api::ListParams::builder().limit(50).build())
        .await?;

    for img in page.data.unwrap_or_default() {
        if let Some(id) = &img.id { println!("{id}"); }
    }
    Ok(())
}
```

`ApiClient` holds one `Configuration` (auth + base URL) and exposes per-tag clients via `.feeds_api()`, `.images_api()`, `.videos_api()`. Each method takes a single `Params` struct built with the `bon` builder.

## Auth

```rust
// API key
Configuration {
    api_key: Some(ApiKey { prefix: None, key }),
    ..Configuration::new()
}

// Bearer token (e.g. minted via /clientauth/token)
Configuration {
    bearer_access_token: Some(token),
    ..Configuration::new()
}
```

See [examples/src/bin/auth.rs](examples/src/bin/auth.rs) for a full client-credentials JWT flow.

## Feeds

```rust
use rixl::apis::feeds;

let page = client.feeds_api().list(feeds_api::ListParams::builder().feed_id(feed_id).build()).await?;
let post = client.feeds_api().get(feeds_api::GetParams::builder().feed_id(feed_id).post_id(post_id).build()).await?;
```

## Images

```rust
use rixl::apis::images;

let page = client.images_api().list(images_api::ListParams::builder().build()).await?;
let img  = client.images_api().get(images_api::GetParams::builder().image_id(image_id).build()).await?;
client.images_api().delete(images_api::DeleteParams::builder().image_id(image_id).build()).await?;
```

Upload (init → PUT bytes → complete):

```rust
use rixl::apis::images;
use rixl::models::{ImageUploadInitRequest, ImageUploadCompleteRequest};

let init = client.images_api()
    .upload_init(images_api::UploadInitParams::builder()
        .image_upload_init_request(ImageUploadInitRequest {
            name: Some("photo.jpg".into()),
            format: Some("jpeg".into()),
        })
        .build())
    .await?;
// PUT bytes to init.presigned_url, then:
let img = client.images_api()
    .upload_complete(images_api::UploadCompleteParams::builder()
        .image_upload_complete_request(ImageUploadCompleteRequest {
            image_id: init.image_id,
            attached_to_video: Some(false),
        })
        .build())
    .await?;
```

## Videos

```rust
use rixl::apis::videos;

let page  = client.videos_api().list(videos_api::ListParams::builder().build()).await?;
let video = client.videos_api().get(videos_api::GetParams::builder().video_id(video_id).build()).await?;
```

Upload returns presigned URLs for both the video and a poster image — see [examples/src/bin/upload_video.rs](examples/src/bin/upload_video.rs).

## Pagination

```rust
use rixl::apis::images;

let mut offset = 0;
let limit = 50;
loop {
    let page = client.images_api()
        .list(images_api::ListParams::builder().limit(limit).offset(offset).build())
        .await?;
    let data = page.data.unwrap_or_default();
    for img in &data { /* ... */ }
    let total = page.pagination.and_then(|p| p.total).unwrap_or(0);
    if offset + data.len() as i32 >= total { break; }
    offset += limit;
}
```

## Errors

API methods return `Result<T, rixl::apis::Error<E>>`. `Error::ResponseError` carries the parsed error body:

```rust
use rixl::apis::Error;

match client.images_api().get(images_api::GetParams::builder().image_id(id).build()).await {
    Ok(img) => println!("{:?}", img),
    Err(Error::ResponseError(e)) => eprintln!("HTTP {}: {}", e.status, e.content),
    Err(e) => return Err(e.into()),
}
```

## Examples

Runnable demos in [examples/src/bin/](examples/src/bin):

```bash
export RIXL_API_KEY=<key>
cargo run -p rixl-examples --bin images
cargo run -p rixl-examples --bin upload_video
```

Available bins: `images`, `feeds`, `videos`, `posts`, `auth`, `upload_image`, `upload_video`.

## Regenerating the SDK

```bash
brew install openapi-generator openjdk
./gen.sh
```

`gen.sh` fetches the upstream OpenAPI spec, runs `openapi-generator generate` with [config.yaml](config.yaml), and post-processes the output via [post-fix.py](post-fix.py) — which (a) drops the `_api` suffix from per-tag accessors so callers write `client.videos_api()` instead of `client.videos_api()`, and (b) patches a known openapi-generator template bug on required multipart file params.

`--skip-validate-spec` is required because the spec uses tag-namespaced operationIds (`list`/`get`/`delete` repeated across tags), which is per-spec-section unique but not globally unique.

## Issues

[github.com/rixlhq/rixl-rust/issues](https://github.com/rixlhq/rixl-rust/issues)
