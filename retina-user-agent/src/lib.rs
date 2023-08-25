// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

//! This crate provides the User-Agent specific components that aren't
//! necessarily governed by specifications.

pub mod stylesheet;
pub mod url_scheme;

/// The value of the `User-Agent` HTTP header, colloquially know as just
/// “User Agent”.
pub const USER_AGENT_HEADER_VALUE: &str = "Mozilla/5.0 (like Gecko, WebKit and Chrome) Retina";
