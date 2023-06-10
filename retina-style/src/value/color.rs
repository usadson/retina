// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::Value;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum CssColor {
    Color(retina_common::Color),
}

pub struct CssNamedColor {
    _marker: (),
}

impl CssNamedColor {
    pub const WHITE: CssColor = CssColor::Color(retina_common::Color::rgb(1.0, 1.0, 1.0));
    pub const BLACK: CssColor = CssColor::Color(retina_common::Color::rgb(0.0, 0.0, 0.0));
    pub const TRANSPARENT: CssColor = CssColor::Color(retina_common::Color::TRANSPARENT);

    pub const BLUE: CssColor = CssColor::Color(retina_common::Color::rgb(0.0, 0.0, 1.0));
    pub const GREEN: CssColor = CssColor::Color(retina_common::Color::rgb(0.0, 128.0 / 255.0, 0.0));
    pub const RED: CssColor = CssColor::Color(retina_common::Color::rgb(1.0, 0.0, 0.0));
}

impl From<retina_common::Color> for CssColor {
    fn from(value: retina_common::Color) -> Self {
        CssColor::Color(value)
    }
}

impl From<CssColor> for Value {
    fn from(value: CssColor) -> Self {
        Value::Color(value)
    }
}
