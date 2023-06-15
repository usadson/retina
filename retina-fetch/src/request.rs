// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use hyper::Method;
use url::Url;

use crate::{
    RequestDestination,
    RequestInitiator,
};

/// The [Request][spec] class.
///
/// [spec]: https://fetch.spec.whatwg.org/#request-class
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Request {
    pub(crate) initiator: RequestInitiator,
    pub(crate) destination: RequestDestination,

    pub(crate) method: hyper::Method,
    pub(crate) url: Url,
}

impl Request {
    pub fn new(url: Url, initiator: RequestInitiator, destination: RequestDestination) -> Self {
        Request {
            initiator,
            destination,
            method: hyper::Method::GET,
            url,
        }
    }

    pub fn get_document(url: Url) -> Self {
        Request {
            initiator: RequestInitiator::None,
            destination: RequestDestination::Document,
            method: Method::GET,
            url,
        }
    }

    pub fn url(&self) -> &Url {
        &self.url
    }
}
