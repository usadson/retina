// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use strum::{AsRefStr, EnumIter};

/// # References
/// * [CSS Backgrounds and Borders Module Level 3](https://drafts.csswg.org/css-backgrounds/#typedef-repeat-style)
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CssBackgroundRepeat {
    pub horizontal: CssBackgroundRepeatStyle,
    pub vertical: CssBackgroundRepeatStyle,
}

/// # References
/// * [CSS Backgrounds and Borders Module Level 3](https://drafts.csswg.org/css-backgrounds/#typedef-repeat-style)
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[derive(AsRefStr, EnumIter)]
#[strum(serialize_all="kebab-case")]
pub enum CssBackgroundRepeatStyle {
    Repeat,
    Space,
    Round,
    NoRepeat,
}
