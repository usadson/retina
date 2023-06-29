// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use strum::{AsRefStr, EnumIter};

/// # References
/// * [CSS 2.2 § 9.5.1](https://drafts.csswg.org/css2/#propdef-float)
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[derive(AsRefStr, EnumIter)]
pub enum CssFloatValue {
    None,
    Left,
    Right,
    InlineStart,
    InlineEnd,
}
