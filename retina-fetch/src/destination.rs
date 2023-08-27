// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use strum::AsRefStr;

/// The [Request Destination][spec] specifies what the destination of this
/// `fetch` is.
///
/// [spec]: https://fetch.spec.whatwg.org/#concept-request-destination
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[derive(AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum RequestDestination {
    /// HTML `<audio>`
    Audio,

    /// `audioScript.addModule()`
    AudioWorklet,

    /// A website is loaded, and this is e.g. the HTML document.
    Document,

    /// HTML `<embed>`
    Embed,

    /// CSS `@font-face`
    Font,

    /// HTML `<frame>`
    Frame,

    /// HTML `<iframe>`
    IFrame,

    /// # `""` initiator
    /// * `<img src>`
    /// * `/favicon` resource
    /// * SVG `<image>`
    /// * CSS `background-image`
    /// * CSS `cursor`
    /// * CSS `list-style-image`
    ///
    /// # `imageset` initiator
    /// * HTML `<img srcset>`
    /// * HTML `<picture>`
    Image,

    /// `<link rel=manifest>`
    Manifest,

    /// The empty string `""`. This can come from one of:
    ///
    /// # `""` initiator
    /// 1. `navigator.sendBeacon()`
    /// 2. `EventSource`
    /// 3. `<a ping="">`
    /// 4. `<area ping="">`
    /// 5. `fetch()`
    /// 6. `XMLHttpRequest`
    /// 7. `WebSocket`
    /// 8. _Cache API_
    ///
    /// # `download` initiator
    /// * HTML `download` attribute
    /// * _Save Link As..._ in the browser UI
    ///
    /// # `prefetch` initiator
    /// * HTML `<link rel=prefetch>`
    ///
    /// # `prerender` initiator
    /// * HTML `<link rel=prerender>`
    #[default]
    None,

    /// HTML `<object>`
    Object,

    /// `CSS.paintWorklet.addModule()`
    PaintWorklet,

    /// CSP, NEL reports.
    Report,

    /// * `<script>`
    /// * `importScripts()`
    Script,

    /// `navigator.serviceWorker.register()`
    ServiceWorker,

    /// `SharedWorker`
    SharedWorker,

    /// * HTML `<link rel=stylesheet>`
    /// * CSS `@import`
    Style,

    /// HTML `<track>`
    Track,

    /// HTML `<video>`
    Video,

    /// `Federated Credential Management requests`
    WebIdentity,

    /// `Worker`
    Worker,

    /// `<?xml-stylesheet>`
    Xslt,
}

impl RequestDestination {
    /// Get the normative string representation, as per [Fetch][spec].
    ///
    /// [spec]: https://fetch.spec.whatwg.org/#concept-request-destination
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }

    /// Specifies if the request is initiated by a script load, or activated by
    /// some features in JavaScript.
    ///
    /// # Specification Warning
    /// ___Algorithms that use script-like should also consider "`xslt`" as that
    /// too can cause script execution. It is not included in the list as it is
    /// not always relevant and might require different behavior.___
    ///
    /// # Specifications
    /// * [Fetch API][spec]
    ///
    /// [spec]: https://fetch.spec.whatwg.org/#request-destination-script-like
    pub fn is_script_like(&self) -> bool {
        matches!(
            self,
            Self::AudioWorklet
                | Self::PaintWorklet
                | Self::Script
                | Self::ServiceWorker
                | Self::SharedWorker
                | Self::Worker
        )
    }
}
