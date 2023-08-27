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
    SameOrigin,
    Cors,
    #[default]
    NoCors,
    Navigate,
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
