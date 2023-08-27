# Retina-Fetch
This crate provides the [Fetch API](https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API) implementation for the [Retina Browser](https://github.com/usadson/retina), but can be used independently. It currently covers some of the specification, with full intention to conform to all of the [Fetch Standard](https://fetch.spec.whatwg.org/) and [HTTP](https://httpwg.org/specs/rfc9110.html), as well as the W3C Web App.

## Installing
```
cargo add retina-fetch
```

## Example
```rust
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
```

## References
* [Fetch API - MDN](https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API)
* [Fetch Metadata Request Headers - W3C](https://w3c.github.io/webappsec-fetch-metadata/)
* [Fetch Standard](https://fetch.spec.whatwg.org/)
* [HTTP Semantics - RFC 9110 - IETF](https://httpwg.org/specs/rfc9110.html)
* [Referrer Policy - W3C](https://www.w3.org/TR/referrer-policy/)
