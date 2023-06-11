// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::{sync::Arc, io::BufRead};

use futures_core::Stream;
use hyper::body::{Buf, Bytes};
use url::Url;

use crate::{Request, StatusCode};

type Inner = hyper::Response<hyper::Body>;

#[derive(Debug)]
pub struct Response {
    request: Arc<Request>,
    inner: Inner,
}

impl Response {
    pub(crate) fn new_about(request: Arc<Request>, body: &'static str) -> Self {
        Self {
            request,
            inner: Inner::new(body.into()),
        }
    }

    pub(crate) fn new_file<S, O, E>(
        request: Arc<Request>,
        stream: S,
    ) -> Self
        where S: Stream<Item = Result<O, E>> + Send + 'static,
            O: Into<Bytes> + 'static,
            E: Into<Box<dyn std::error::Error + Send + Sync>> + 'static,
        {
        Self {
            request,
            inner: Inner::new(hyper::Body::wrap_stream(stream))
        }
    }

    pub async fn body(&mut self) -> Box<dyn BufRead + '_> {
        Box::new(hyper::body::aggregate(self.inner.body_mut()).await.unwrap().reader())
    }

    /// Get the [`Request`] that created this [`Response`].
    pub fn request(&self) -> &Request {
        &self.request
    }

    pub fn ok(&self) -> bool {
        self.status().is_successful()
    }

    pub fn status(&self) -> StatusCode {
        self.inner.status().into()
    }

    pub fn url(&self) -> &Url {
        &self.request.url
    }
}

impl From<(Arc<Request>, Inner)> for Response {
    fn from(value: (Arc<Request>, Inner)) -> Self {
        let (request, inner) = value;
        Self { request, inner }
    }
}
