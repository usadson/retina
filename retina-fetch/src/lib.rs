// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

//! This API is modeled after the [Fetch API][spec]
//!
//! # References
//! * [Fetch API - WHATWG Specification][spec]
//! * [Fetch API - MDN][mdn]
//!
//! [mdn]: https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API
//! [spec]: https://fetch.spec.whatwg.org

pub(crate) mod destination;
pub(crate) mod error;
pub(crate) mod initiator;
pub(crate) mod fetch;
pub(crate) mod mode;
pub(crate) mod promise;
pub(crate) mod referrer;
pub(crate) mod request;
pub(crate) mod response;
pub(crate) mod status_code;

pub use destination::RequestDestination;
pub use error::{Error, InternalError, NetworkError};
pub use fetch::Fetch;
pub use initiator::RequestInitiator;
pub use mode::RequestMode;
pub use promise::FetchPromise;
pub use referrer::RequestReferrer;
pub use request::Request;
pub use response::Response;
pub use status_code::{StatusCode, StatusCodeClass};

pub use mime;
pub use url::{
    self,
    Url,
};

pub type FetchResponse = Result<Response, Error>;

pub fn parse_page_url(input: &str) -> Result<Url, url::ParseError> {
    if input.len() > 3 && &input[1..3] == ":\\" {
        let url = format!("file:///{}", input.replace('\\', "/"));
        return Url::parse(&url);
    }

    let result = Url::parse(input);

    if result == Err(url::ParseError::RelativeUrlWithoutBase) && !input.starts_with("http") {
        if let Ok(url) = Url::parse(&format!("https://{input}")) {
            return Ok(url);
        }
    }

    result
}
