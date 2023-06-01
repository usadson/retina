// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use strum::{AsRefStr, EnumIter};

/// The value enum for the [`white-space`][property spec].
///
/// # References
/// * [CSS Text Module Level 3 § 3. White Space and Wrapping: the `white-space` property][property-spec]
///
/// [property spec]: https://drafts.csswg.org/css-text/#white-space-property
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[derive(AsRefStr, EnumIter)]
#[strum(serialize_all = "kebab-case")]
pub enum CssWhiteSpace {
    /// `normal`
    Normal,

    /// `nowrap`
    Nowrap,

    /// `pre`
    Pre,

    /// `pre-wrap`
    PreWrap,

    /// `pre-line`
    PreLine,

    /// `break-spaces`,
    BreakSpaces,
}
