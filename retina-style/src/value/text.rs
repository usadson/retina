// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use strum::{
    AsRefStr,
    EnumIter,
};

/// The [`text-transform`][spec] property value.
///
/// [spec]: https://drafts.csswg.org/css-text-4/#text-transform-property
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[derive(AsRefStr, EnumIter)]
#[strum(serialize_all = "kebab-case")]
pub enum CssTextTransform {
    #[default]
    None,

    Capitalize,
    Uppercase,
    Lowercase,
}
