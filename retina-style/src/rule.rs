// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::{
    Declaration,
    SelectorList,
};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rule {
    Style(StyleRule),
}

/// # References
/// * [CSS - Syntax Level 3 - 9.1](https://www.w3.org/TR/css-syntax-3/#style-rules)
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct StyleRule {
    pub selector_list: SelectorList,
    pub declarations: Vec<Declaration>,
}
