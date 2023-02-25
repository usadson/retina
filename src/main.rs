// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::http::download_resource;

pub mod http;

pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    let result = String::from_utf8(download_resource("http://theoldnet.com/").await?)?;

    println!("Resource: {}", result);

    Ok(())
}
