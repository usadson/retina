// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

/// Specifies the kinds of links in HTML.
///
/// ## References
/// * [HTML Living Standard § 4.6.1.][spec]
///
/// [spec]: https://html.spec.whatwg.org/multipage/links.html#introduction-2
pub enum LinkKind {
    ExternalResource,
    Hyperlink,
}
