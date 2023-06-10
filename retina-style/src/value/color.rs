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

impl TryFrom<cssparser::Color> for CssColor {
    type Error = String;

    fn try_from(value: cssparser::Color) -> Result<Self, Self::Error> {
        match value {
            cssparser::Color::Rgba(rgba) => Ok(rgba.into()),
            _ => Err(format!("failed to convert value: {value:?}")),
        }
    }
}

impl From<cssparser::RGBA> for CssColor {
    fn from(value: cssparser::RGBA) -> Self {
        let mut color = retina_common::Color::rgb_bytes(
            value.red.unwrap_or(0),
            value.green.unwrap_or(0),
            value.blue.unwrap_or(0),
        );

        if let Some(alpha) = value.alpha {
            color = color.with_alpha(alpha as _);
        }

        Self::Color(color)
    }
}

impl From<CssColor> for Value {
    fn from(value: CssColor) -> Self {
        Value::Color(value)
    }
}
