// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use http::{Request, HeaderValue};
use hyper::{
    body::HttpBody,
    Client, Body,
};

use tokio::io::AsyncWriteExt;

use crate::Result;

/// https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/User-Agent
pub const USER_AGENT: &str = "Mozilla 5.0 (Unknown; x64) Retina/0.1.0 (KHTML, like Gecko)";

pub async fn download_resource(uri: &str) -> Result<Vec<u8>> {
    let mut request = Request::new(Body::default());
    *request.uri_mut() = uri.parse()?;
    request.headers_mut().append("User-Agent", HeaderValue::from_str(USER_AGENT)?);

    let client = Client::new();
    let mut response = client.request(request).await?;

    let mut body = Vec::new();
    while let Some(chunk) = response.body_mut().data().await {
        body.write_all(&chunk?).await?;
    }

    Ok(body)
}
