// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

pub(crate) mod family;
pub(crate) mod font;
pub(crate) mod harfbuzz_util;
pub(crate) mod provider;

pub use self::provider::FontProvider;

use retina_gfx_font::{
    FamilyName,
    FontWeight
};

pub(crate) fn convert_font_kit_name(value: FamilyName) -> font_kit::family_name::FamilyName {
    match value {
        FamilyName::Title(name) => font_kit::family_name::FamilyName::Title(name.to_string()),
        FamilyName::Cursive => font_kit::family_name::FamilyName::Cursive,
        FamilyName::Fantasy => font_kit::family_name::FamilyName::Fantasy,
        FamilyName::Monospace => font_kit::family_name::FamilyName::Monospace,
        FamilyName::SansSerif => font_kit::family_name::FamilyName::SansSerif,
        FamilyName::Serif => font_kit::family_name::FamilyName::Serif,
    }
}

#[inline]
pub(crate) const fn convert_font_kit_weight(value: FontWeight) -> font_kit::properties::Weight {
    font_kit::properties::Weight(value.value())
}
