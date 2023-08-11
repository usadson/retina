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

/// <https://drafts.csswg.org/css-fonts/#font-kerning-prop>
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[derive(AsRefStr, EnumIter)]
#[strum(serialize_all = "kebab-case")]
pub enum CssFontKerning {
    #[default]
    Auto,

    Normal,

    None,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssFontShorthand {
    pub families: Vec<CssFontFamilyName>,
    pub style: Option<CssFontStyle>,
    pub size: CssLength,
    pub line_height: Option<CssLength>,
    pub weight: Option<CssFontWeight>,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[derive(AsRefStr, EnumIter)]
#[strum(serialize_all = "kebab-case")]
pub enum CssFontStyle {
    #[default]
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

/// <https://drafts.csswg.org/css-fonts/#propdef-font-variant-caps>
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[derive(AsRefStr, EnumIter)]
#[strum(serialize_all = "kebab-case")]
pub enum CssFontVariantCaps {
    /// https://drafts.csswg.org/css-fonts/#font-variant-caps-normal-value
    #[default]
    Normal,

    /// https://drafts.csswg.org/css-fonts/#valdef-font-variant-caps-small-caps
    SmallCaps,

    /// https://drafts.csswg.org/css-fonts/#valdef-font-variant-caps-all-small-caps
    AllSmallCaps,

    /// https://drafts.csswg.org/css-fonts/#valdef-font-variant-caps-petite-caps
    PetiteCaps,

    /// https://drafts.csswg.org/css-fonts/#valdef-font-variant-caps-all-petite-caps
    AllPetiteCaps,

    /// https://drafts.csswg.org/css-fonts/#valdef-font-variant-caps-unicase
    Unicase,

    /// https://drafts.csswg.org/css-fonts/#valdef-font-variant-caps-titling-caps
    TitlingCaps,
}

/// <https://drafts.csswg.org/css-fonts/#font-variant-east-asian-prop>
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum CssFontVariantEastAsian {
    #[default]
    Normal,
    Specific {
        values: CssFontVariantEastAsianValues,
        width: CssFontVariantEastAsianWidth,
        ruby: bool,
    },
}

/// <https://drafts.csswg.org/css-fonts/#font-variant-east-asian-prop>
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum CssFontVariantEastAsianValues {
    #[default]
    Normal,
    Jis78,
    Jis83,
    Jis90,
    Jis04,
    Simplified,
    Traditional,
}

/// <https://drafts.csswg.org/css-fonts/#font-variant-east-asian-prop>
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum CssFontVariantEastAsianWidth {
    #[default]
    Normal,
    FullWidth,
    ProportionalWidth,
}

/// <https://drafts.csswg.org/css-fonts/#font-variant-ligatures-prop>
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum CssFontVariantLigatures {
    #[default]
    Normal,
    None,
    Specific {
        common: bool,
        discretionary: bool,
        historical: bool,
        contextual: bool,
    },
}

/// <https://drafts.csswg.org/css-fonts/#propdef-font-variant-position>
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[derive(AsRefStr, EnumIter)]
#[strum(serialize_all = "kebab-case")]
pub enum CssFontVariantPosition {
    #[default]
    Normal,
    Sub,
    Super,
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
