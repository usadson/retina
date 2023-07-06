// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

/// The [media query][spec] in an `@media` rule.
///
/// [spec]: https://www.w3.org/TR/mediaqueries-5/#typedef-media-query
#[derive(Clone, Debug, PartialEq)]
pub enum MediaQuery {
    Type(MediaType),
}

/// The [media type][spec] in an `@media` rule, e.g. `screen` or `print`.
///
/// [spec]: https://www.w3.org/TR/mediaqueries-5/#media-types
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum MediaType {
    All,
    Print,
    Screen,
}
