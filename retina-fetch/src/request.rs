// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use hyper::Method;
use url::Url;

use crate::{
    RequestDestination,
    RequestInitiator,
    RequestMode,
    RequestReferrer,
};

/// The [Request][spec] class.
///
/// [spec]: https://fetch.spec.whatwg.org/#request-class
#[derive(Clone, Debug, PartialEq)]
pub struct Request {
    pub(crate) initiator: RequestInitiator,
    pub(crate) destination: RequestDestination,
    pub(crate) mode: RequestMode,
    pub(crate) referrer: RequestReferrer,

    pub(crate) method: hyper::Method,
    pub(crate) url: Url,
}

impl Request {
    /// Create a new [Request][https://fetch.spec.whatwg.org/#request-class].
    pub fn new(
        url: Url,
        initiator: RequestInitiator,
        destination: RequestDestination,
        mode: RequestMode,
        referrer: RequestReferrer,
    ) -> Self {
        Request {
            initiator,
            destination,
            mode,
            referrer,

            method: hyper::Method::GET,
            url,
        }
    }

    /// A helper to create a [Request][spec], specifically for Document
    /// retrieval with navigation (i.e. top-level browser contexts).
    ///
    /// [spec]: https://fetch.spec.whatwg.org/#request-class
    pub fn get_document(url: Url, referrer: RequestReferrer) -> Self {
        Request {
            initiator: RequestInitiator::None,
            destination: RequestDestination::Document,
            mode: RequestMode::Navigate,
            referrer,

            method: Method::GET,
            url,
        }
    }

    /// Compute or get the value of the [`Accept`][spec] header, which specifies
    /// what type of content is acceptable for us to handle.
    ///
    /// [spec]: https://httpwg.org/specs/rfc9110.html#field.accept
    pub fn accept_header_value(&self) -> &str {
        match self.destination {
            RequestDestination::Document => "text/html,*/*;q=0.8",
            RequestDestination::Style => "text/css,*/*;q=0.8",
            _ => "*/*",
        }
    }

    /// Get the [Url] that this request should retrieve.
    pub fn url(&self) -> &Url {
        &self.url
    }
}
