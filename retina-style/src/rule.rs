// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::{
    cascade_origin::CascadeOrigin,
    Declaration,
    MediaQuery,
    SelectorList,
    Stylesheet,
};

pub use self::font_face::*;

mod font_face;

#[derive(Clone, Debug, PartialEq)]
pub enum Rule {
    AtFontFace(CssFontFaceAtRule),

    /// `@rule`
    AtMedia(AtMediaRule),
    Style(StyleRule),
}

impl Rule {
    pub fn try_as_style(&self) -> Option<&StyleRule> {
        match self {
            Rule::Style(style) => Some(style),
            _ => None
        }
    }
}

/// # References
/// * [CSS - Syntax Level 3 - 9.1](https://www.w3.org/TR/css-syntax-3/#style-rules)
#[derive(Clone, Debug, PartialEq)]
pub struct StyleRule {
    pub cascade_origin: CascadeOrigin,
    pub selector_list: SelectorList,
    pub declarations: Vec<Declaration>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AtMediaRule {
    pub media_query_list: Vec<MediaQuery>,
    pub stylesheet: Stylesheet,
}
