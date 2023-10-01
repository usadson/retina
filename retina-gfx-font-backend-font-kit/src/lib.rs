// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

pub(crate) mod family;
pub(crate) mod font;
pub(crate) mod harfbuzz_util;
pub(crate) mod provider;

pub use self::provider::FontProvider;

use retina_gfx::font::{
    FontStyle,
    FontWeight,
};

#[inline]
pub(crate) const fn convert_font_kit_style(value: FontStyle) -> font_kit::properties::Style {
    use font_kit::properties::Style;
    match value {
        FontStyle::Normal => Style::Normal,
        FontStyle::Italic => Style::Italic,
        FontStyle::Oblique => Style::Oblique,
    }
}

#[inline]
pub(crate) const fn convert_font_kit_weight(value: FontWeight) -> font_kit::properties::Weight {
    font_kit::properties::Weight(value.value())
}
