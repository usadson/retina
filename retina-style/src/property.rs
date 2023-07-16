// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use strum::{AsRefStr, EnumIter, IntoEnumIterator};

/// <https://www.w3.org/TR/css-values-3/#value-examples>
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(AsRefStr, EnumIter)]
#[strum(serialize_all = "kebab-case")]
pub enum Property {
    #[strum(disabled)]
    Invalid,

    BackgroundColor,
    BackgroundImage,

    Border,
    BorderColor,
    BorderWidth,
    BorderStyle,

    BorderBottom,
    BorderBottomColor,
    BorderBottomStyle,
    BorderBottomWidth,

    BorderLeft,
    BorderLeftColor,
    BorderLeftStyle,
    BorderLeftWidth,

    BorderRight,
    BorderRightColor,
    BorderRightStyle,
    BorderRightWidth,

    BorderTop,
    BorderTopColor,
    BorderTopStyle,
    BorderTopWidth,

    Color,
    Display,

    Float,
    Font,
    FontFamily,
    FontKerning,
    /// <https://drafts.csswg.org/css-fonts-4/#font-size-prop>
    FontSize,
    FontStretch,
    FontStyle,
    FontVariantCaps,
    FontVariantLigatures,
    FontWeight,

    Height,

    Margin,
    MarginBlock,
    MarginBlockEnd,
    MarginBlockStart,
    MarginBottom,
    MarginInline,
    MarginInlineEnd,
    MarginInlineStart,
    MarginLeft,
    MarginRight,
    MarginTop,

    Padding,
    PaddingBottom,
    PaddingLeft,
    PaddingRight,
    PaddingTop,

    Width,
    WhiteSpace,
}

impl Property {
    pub fn parse(input: &str) -> Option<Self> {
        Self::iter().find(|property| property.as_ref() == input)
    }
}
