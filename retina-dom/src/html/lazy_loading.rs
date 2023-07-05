// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

/// Specifies when the resource references by the element should be loaded.
///
/// ## References
/// * [HTML Living Standard § 2.3.3. Keywords and enumerated attributes][enumerated-attribute]
/// * [HTML Living Standard § 2.5.7. Lazy loading attributes][spec]
///
/// [spec]: https://html.spec.whatwg.org/multipage/urls-and-fetching.html#lazy-loading-attribute
/// [enumerated-attribute]: https://html.spec.whatwg.org/multipage/common-microsyntaxes.html#enumerated-attribute
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[derive(Default)]
pub enum LazyLoadingKind {
    /// Used to fetch a resource immediately; the default state.
    #[default]
    Eager,

    /// Used to defer fetching a resource until some conditions are met.
    Lazy,
}
