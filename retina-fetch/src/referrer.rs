// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use url::Url;

/// Specifies the type of [referrer][spec] a request should be associated with.
/// Most requests have a specific origin, most likely a script or document, that
/// initiated/linked the resource pointed to by this request. For example, a
/// __HTML__ document might have images (`<img>`), and those images should be
/// requested from a server. To follow web security practices, a HTTP
/// [`Referer`][http] header should be associated with the request.
///
/// [spec]: fetch.spec.whatwg.org/#concept-request-referrer
/// [http]: https://httpwg.org/specs/rfc9110.html#field.referer
#[derive(Clone, Debug, Default, PartialEq)]
pub enum RequestReferrer {
    /// No referrer was found or associated with this request (for example,
    /// top-level documents), or the client/server has explicitly disabled
    /// referrer by [policy][policy].
    ///
    /// [policy]: https://w3c.github.io/webappsec-referrer-policy/#referrer-policy-header
    NoReferrer,

    /// The default value, which specifies that [Fetch][crate::Fetch] should
    /// determine the referrer using [the algorithm][algo].
    ///
    /// [algo]: https://w3c.github.io/webappsec-referrer-policy/#determine-requests-referrer
    #[default]
    Client,

    /// The request was referred to by
    Url(Url),
}
