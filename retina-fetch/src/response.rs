// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::{sync::Arc, io::BufRead};

use futures_core::Stream;
use hyper::body::{Buf, Bytes};
use log::error;
use url::Url;

use crate::{Request, StatusCode};

type Inner = hyper::Response<hyper::Body>;

/// Represents the HTTP response to an [request][Request].
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

    /// Get the [`Content-Type`][spec] header, which specifies which [media type]
    /// this response is represented with.
    ///
    /// [spec]: https://www.rfc-editor.org/rfc/rfc2046.html
    /// [media type]: https://httpwg.org/specs/rfc9110.html#field.content-type
    pub fn content_type(&self) -> mime::Mime {
        let Some(content_type) = self.inner.headers().get(hyper::header::CONTENT_TYPE) else {
            return mime::APPLICATION_OCTET_STREAM;
        };

        let Ok(content_type) = content_type.to_str() else {
            return mime::APPLICATION_OCTET_STREAM;
        };

        content_type.parse().unwrap_or(mime::APPLICATION_OCTET_STREAM)
    }

    /// Gets the redirect location this response points to, if the response is
    /// a redirection.
    pub fn redirect_location(&self) -> Option<&str> {
        if !self.status().is_redirection() {
            return None;
        }

        let location = self.inner.headers().get(hyper::header::LOCATION)?;
        location.to_str().ok()
    }

    /// Gets the redirect location this response points to, if the response is
    /// a redirection.
    pub fn redirect_url(&self) -> Option<Url> {
        let location = self.redirect_location()?;
        match url::Url::parse(location) {
            Ok(url) => Some(url),
            Err(e) => {
                error!("Redirect status ({:?}) with invalid URL: \"{location}\", error: {e}", self.status());
                None
            }
        }
    }

    /// Get the body of this response.
    pub async fn body(&mut self) -> Box<dyn BufRead + '_> {
        Box::new(hyper::body::aggregate(self.inner.body_mut()).await.unwrap().reader())
    }

    /// TODO: this function is not needed if the BufRead can be somehow
    /// [`Seek`][std::io::Seek].
    pub async fn body_bytes(&mut self) -> Bytes {
        hyper::body::to_bytes(self.inner.body_mut()).await.unwrap()
    }

    /// Get the [`Request`] that created this [`Response`].
    pub fn request(&self) -> &Request {
        &self.request
    }

    /// Check if this response is successful.
    pub fn ok(&self) -> bool {
        self.status().is_successful()
    }

    /// Get the [StatusCode] of this response.
    pub fn status(&self) -> StatusCode {
        self.inner.status().into()
    }

    /// Get the [URL][Url] this response was requested with.
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
