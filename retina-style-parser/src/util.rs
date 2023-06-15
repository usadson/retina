// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use retina_style::{CascadeOrigin, Stylesheet, CssColor};

pub trait CssParsable {
    fn parse(cascade_origin: CascadeOrigin, input: &str) -> Self;
}

impl CssParsable for Stylesheet {
    fn parse(cascade_origin: CascadeOrigin, input: &str) -> Self {
        crate::parse_stylesheet(cascade_origin, input)
    }
}

pub trait FromCssParser<T> {
    type Output;

    fn into(self) -> Self::Output;
}

pub fn convert_color(value: cssparser::Color) -> Option<CssColor> {
    match value {
        cssparser::Color::Rgba(rgba) => Some(convert_rgba(rgba)),
        _ => None,
    }
}

pub fn convert_rgba(value: cssparser::RGBA) -> retina_style::CssColor {
    let mut color = retina_common::Color::rgb_bytes(
        value.red.unwrap_or(0),
        value.green.unwrap_or(0),
        value.blue.unwrap_or(0),
    );

    if let Some(alpha) = value.alpha {
        color = color.with_alpha(alpha as _);
    }

    retina_style::CssColor::Color(color)
}
