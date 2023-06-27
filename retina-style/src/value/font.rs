// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use retina_common::StrTendril;
use strum::{AsRefStr, EnumIter};

use crate::{CssLength, CssDecimal};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CssFontFamilyName {
    Generic(CssGenericFontFamilyName),
    Name(StrTendril),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssFontShorthand {
    pub families: Vec<CssFontFamilyName>,
    pub style: Option<CssFontStyle>,
    pub size: CssLength,
    pub line_height: Option<CssLength>,
    pub weight: Option<CssFontWeight>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[derive(AsRefStr, EnumIter)]
#[strum(serialize_all = "kebab-case")]
pub enum CssFontStyle {
    Normal,
    Italic,
    Oblique,
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
#[derive(AsRefStr, EnumIter)]
pub enum CssFontWeight {
    Absolute(CssDecimal),
    Bolder,
    Lighter,
}

/// <https://drafts.csswg.org/css-fonts-4/#generic-font-families>
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[derive(AsRefStr, EnumIter)]
#[strum(serialize_all = "kebab-case")]
pub enum CssGenericFontFamilyName {
    Serif,
    SansSerif,
    Cursive,
    Fantasy,
    Monospace,
    SystemUi,
    Emoji,
    Math,
    Fangsong,
    UiSerif,
    UiSansSerif,
    UiMonospace,
    UiRounded,
}
