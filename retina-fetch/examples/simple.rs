// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use retina_fetch::{Fetch, Request};
use url::Url;

#[tokio::main]
async fn main() {
    let fetch = Fetch::new();

    let url = Url::parse("https://example.org/").unwrap();
    let request = Request::get_document(url, Default::default());

    let response = fetch.fetch(request).await.unwrap();
    println!("Response {response:#?}");
}
