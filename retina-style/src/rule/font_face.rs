// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::ops::Range;

use retina_common::StrTendril;
use strum::{
    AsRefStr,
    EnumIter,
    IntoEnumIterator,
};

use crate::{CssFontStyle, CssDecimal};

#[derive(Clone, Debug, PartialEq)]
pub struct CssFontFaceAtRule {
    pub declarations: Vec<CssFontFaceDeclaration>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CssFontFaceDeclaration {
    FontFamily(StrTendril),
    FontStyle(CssFontStyle),
    FontWeight(CssDecimal),
    Src {
        sources: Vec<CssFontFaceSrc>,
    },
    UnicodeRanges(Vec<Range<u32>>),
}

/// <https://www.w3.org/TR/css-values-3/#value-examples>
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(AsRefStr, EnumIter)]
#[strum(serialize_all = "kebab-case")]
pub enum CssFontFaceProperty {
    FontFamily,
    FontStyle,
    FontWeight,
    Src,
    UnicodeRanges,
}

impl CssFontFaceProperty {
    pub fn parse(input: &str) -> Option<Self> {
        Self::iter().find(|property| property.as_ref() == input)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(AsRefStr, EnumIter, strum::Display)]
#[strum(serialize_all = "kebab-case")]
pub enum CssFontFaceFormat {
    Unknown,

    Collection,
    EmbeddedOpentype,
    Opentype,
    Svg,
    Truetype,
    Woff,
    Woff2,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CssFontFaceSrc {
    WebFont {
        url: StrTendril,
        format: CssFontFaceFormat,
    },
    Local(StrTendril),
}
