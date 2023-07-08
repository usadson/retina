// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

// The contents of the `about:not-found` url.
pub const NOT_FOUND: &str = include_str!("not-found.html");

/// An error occurred whilst
pub const NETWORK_ERROR: &str = include_str!("network-error.html");

/// This page is shown when an unknown URL scheme was entered.
pub const URL_SCHEME_UNKNOWN: &str = include_str!("url-scheme-unknown.html");
