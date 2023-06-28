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

impl CssWhiteSpace {
    /// Determines whether the `white-space` collapses whitespace or not.
    ///
    /// # References
    /// * [CSS Text Module Level 3 § 4.1.1. Phase I: Collapsing and Transformation][spec]
    ///
    /// [spec]: https://drafts.csswg.org/css-text/#collapse
    pub const fn collapses(&self) -> bool {
        matches!(self, Self::Normal | Self::Nowrap | Self::PreLine)
    }
}
