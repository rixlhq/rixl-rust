# rixl

[![Crates.io](https://img.shields.io/crates/v/rixl.svg)](https://crates.io/crates/rixl)
[![Documentation](https://docs.rs/rixl/badge.svg)](https://docs.rs/rixl)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Official Rust SDK for the [RIXL](https://rixl.com) API platform.

## Install

Add `rixl` to your `Cargo.toml`:

```toml
[dependencies]
rixl = "1.0"
```

Requires Rust 1.75+ and `tokio`. The `bon` builder and `native-tls` features are enabled by default.

### Features

- `default = ["native-tls", "bon"]`
- `rustls`: Use `rustls` instead of `native-tls` for TLS connections.
- `bon`: Fluent builder pattern for all API requests.

To use `rustls`:
```toml
[dependencies]
rixl = { version = "1.0", default-features = false, features = ["rustls", "bon"] }
```

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
        .list_images(images_api::ListImagesParams::builder()
            .project_id(std::env::var("RIXL_PROJECT_ID")?)
            .pagination_limit(50)
            .build())
        .await?;

    for img in page.images.unwrap_or_default() {
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

// Bearer token (e.g. from `client_credentials_api().mint_client_token`)
Configuration {
    bearer_access_token: Some(token),
    ..Configuration::new()
}
```

See [examples/src/bin/auth.rs](examples/src/bin/auth.rs) for a full client-credentials JWT flow.

## Posts

```rust
use rixl::apis::posts_api;

let page = client.posts_api()
    .list_posts(posts_api::ListPostsParams::builder().project_id(project_id).feed_id(feed_id).build())
    .await?;
let resp = client.posts_api()
    .get_post(posts_api::GetPostParams::builder().project_id(project_id).post_id(post_id).build())
    .await?;
```

## Images

```rust
use rixl::apis::images_api;

let page = client.images_api()
    .list_images(images_api::ListImagesParams::builder().project_id(project_id).build())
    .await?;
let resp = client.images_api()
    .get_image(images_api::GetImageParams::builder().image_id(image_id).build())
    .await?;
client.images_api()
    .delete_image(images_api::DeleteImageParams::builder().image_id(image_id).build())
    .await?;
```

Upload (create the upload, then PUT the bytes):

```rust
use rixl::apis::images_api;
use rixl::models::CreateImageUploadRequest;

let upload = client.images_api()
    .create_image_upload(images_api::CreateImageUploadParams::builder()
        .project_id(project_id)
        .create_image_upload_request(CreateImageUploadRequest {
            name: Some("photo.jpg".into()),
            ..CreateImageUploadRequest::new()
        })
        .build())
    .await?;
// PUT bytes to upload.upload_url
```

## Videos

```rust
use rixl::apis::videos_api;

let page = client.videos_api()
    .list_videos(videos_api::ListVideosParams::builder().project_id(project_id).build())
    .await?;
let resp = client.videos_api()
    .get_video(videos_api::GetVideoParams::builder().video_id(video_id).build())
    .await?;
```

Upload returns presigned URLs for both the video and a poster image — see [examples/src/bin/upload_video.rs](examples/src/bin/upload_video.rs).

## Pagination

```rust
use rixl::apis::images_api;

let mut offset = 0;
let limit = 50;
loop {
    let page = client.images_api()
        .list_images(images_api::ListImagesParams::builder()
            .project_id(project_id.clone())
            .pagination_limit(limit)
            .pagination_offset(offset)
            .build())
        .await?;
    let data = page.images.unwrap_or_default();
    for img in &data { /* ... */ }
    if (data.len() as i32) < limit { break; }
    offset += limit;
}
```

## Errors

API methods return `Result<T, rixl::apis::Error<E>>`. `Error::ResponseError` carries the parsed error body:

```rust
use rixl::apis::Error;

match client.images_api().get_image(images_api::GetImageParams::builder().image_id(id).build()).await {
    Ok(img) => println!("{:?}", img),
    Err(Error::ResponseError(e)) => eprintln!("HTTP {}: {}", e.status, e.content),
    Err(e) => return Err(e.into()),
}
```

## Examples

Runnable demos in [examples/src/bin/](examples/src/bin):

```bash
export RIXL_API_KEY=<key>
export RIXL_PROJECT_ID=<project>
cargo run -p rixl-examples --bin images
cargo run -p rixl-examples --bin upload_video
```

Available bins: `images`, `feeds`, `videos`, `posts`, `auth`, `upload_image`, `upload_video`.

## Regenerating the SDK

The SDK regenerates automatically when `openapi.yaml` changes upstream. To run it locally:

```bash
brew install openapi-generator openjdk yq
./gen.sh
```

`gen.sh` fetches the upstream OpenAPI spec, adapts the shapes `openapi-generator`'s Rust target can't handle, and generates `sdk/` with [config.yaml](config.yaml). Set `OPENAPI_GENERATOR` to use a different generator command.

## Issues

[github.com/rixlhq/rixl-rust/issues](https://github.com/rixlhq/rixl-rust/issues)
