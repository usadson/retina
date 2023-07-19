// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use strum::{
    AsRefStr,
    EnumIter,
};

use crate::CssColor;

/// The [`text-decoration`][spec] property value.
///
/// [spec]: https://drafts.csswg.org/css-text-decor/#text-decoration-property
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct CssTextDecoration {
    pub line: Option<CssTextDecorationLine>,
    pub style: Option<CssTextDecorationStyle>,
    pub color: Option<CssColor>,
}

/// The [`text-decoration-line`][spec] property value.
///
/// [spec]: https://drafts.csswg.org/css-text-decor/#text-decoration-line-property
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[derive(AsRefStr, EnumIter)]
#[strum(serialize_all = "kebab-case")]
pub enum CssTextDecorationLine {
    #[default]
    None,

    Underline,

    Overline,

    LineThrough,

    Blink,
}

/// The [`text-decoration-style`][spec] property value.
///
/// [spec]: https://drafts.csswg.org/css-text-decor/#text-decoration-style-property
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[derive(AsRefStr, EnumIter)]
#[strum(serialize_all = "kebab-case")]
pub enum CssTextDecorationStyle {
    #[default]
    Solid,

    Double,

    Dotted,

    Dashed,

    Wavy,
}
