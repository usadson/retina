// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::{
    Declaration,
    SelectorList, cascade_origin::CascadeOrigin,
};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rule {
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
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct StyleRule {
    pub cascade_origin: CascadeOrigin,
    pub selector_list: SelectorList,
    pub declarations: Vec<Declaration>,
}
