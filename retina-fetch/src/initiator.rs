// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

/// The [Request Initiator][spec] specifies which component started the request.
///
/// [spec]: https://fetch.spec.whatwg.org/#concept-request-initiator
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum RequestInitiator {
    Download,

    ImageSet,

    Manifest,

    #[default]
    None,

    Prefetch,

    Prerender,

    Xslt,
}
