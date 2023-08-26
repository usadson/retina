// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use hyper::Method;
use url::Url;

use crate::{
    RequestDestination,
    RequestInitiator,
    RequestMode,
};

/// The [Request][spec] class.
///
/// [spec]: https://fetch.spec.whatwg.org/#request-class
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Request {
    pub(crate) initiator: RequestInitiator,
    pub(crate) destination: RequestDestination,
    pub(crate) mode: RequestMode,

    pub(crate) method: hyper::Method,
    pub(crate) url: Url,
}

impl Request {
    pub fn new(
        url: Url,
        initiator: RequestInitiator,
        destination: RequestDestination,
        mode: RequestMode,
    ) -> Self {
        Request {
            initiator,
            destination,
            mode,

            method: hyper::Method::GET,
            url,
        }
    }

    pub fn get_document(url: Url) -> Self {
        Request {
            initiator: RequestInitiator::None,
            destination: RequestDestination::Document,
            mode: RequestMode::Navigate,
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

    pub fn url(&self) -> &Url {
        &self.url
    }
}
