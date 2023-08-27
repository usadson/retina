// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use strum::AsRefStr;

/// The request has an associated [mode][spec], which is an important concept
/// in [Cross Origin Resource Sharing][cors].
///
/// [spec]: https://fetch.spec.whatwg.org/#concept-request-mode
/// [cors]: https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[derive(AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum RequestMode {
    /// ["same-origin"](https://fetch.spec.whatwg.org/#concept-request-mode)
    /// >   _Used to ensure requests are made to same-origin URLs. Fetch will
    /// >   return a network error if the request is not made to a same-origin
    /// >   URL._
    SameOrigin,

    /// ["cors"](https://fetch.spec.whatwg.org/#concept-request-mode)
    /// >   _For requests whose response tainting gets set to "cors", makes the
    /// >   request a CORS request — in which case, fetch will return a network
    /// >   error if the requested resource does not understand the CORS
    /// >   protocol, or if the requested resource is one that intentionally
    /// >   does not participate in the CORS protocol._
    Cors,

    /// ["no-cors"](https://fetch.spec.whatwg.org/#concept-request-mode)
    /// >   _Restricts requests to using CORS-safelisted methods and
    /// >   CORS-safelisted request-headers. Upon success, fetch will return an
    /// >   opaque filtered response._
    #[default]
    NoCors,

    /// ["navigate"](https://fetch.spec.whatwg.org/#concept-request-mode)
    /// >   _This is a special mode used only when navigating between
    /// >   documents._
    Navigate,

    /// ["websocket"](https://fetch.spec.whatwg.org/#concept-request-mode)
    /// >   _This is a special mode used only when establishing a WebSocket
    /// >   connection._
    WebSocket,
}

impl RequestMode {
    /// Get the normative string representation, as per [Fetch][spec].
    ///
    /// [spec]: https://fetch.spec.whatwg.org/#concept-request-mode
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }
}
