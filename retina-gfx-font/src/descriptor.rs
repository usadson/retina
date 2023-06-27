// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use retina_common::StrTendril;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum FamilyName {
    Title(StrTendril),
    Cursive,
    Fantasy,
    Monospace,
    SansSerif,
    Serif,
}

impl From<FamilyName> for font_kit::family_name::FamilyName {
    fn from(value: FamilyName) -> Self {
        match value {
            FamilyName::Title(name) => font_kit::family_name::FamilyName::Title(name.to_string()),
            FamilyName::Cursive => font_kit::family_name::FamilyName::Cursive,
            FamilyName::Fantasy => font_kit::family_name::FamilyName::Fantasy,
            FamilyName::Monospace => font_kit::family_name::FamilyName::Monospace,
            FamilyName::SansSerif => font_kit::family_name::FamilyName::SansSerif,
            FamilyName::Serif => font_kit::family_name::FamilyName::Serif,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FontDescriptor {
    pub name: FamilyName,
    pub weight: FontWeight,
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct FontWeight(f32);

impl FontWeight {
    pub const THIN: Self = Self(100.0);
    pub const EXTRA_LIGHT: Self = Self(200.0);
    pub const LIGHT: Self = Self(300.0);
    pub const REGULAR: Self = Self(400.0);
    pub const MEDIUM: Self = Self(500.0);
    pub const SEMI_BOLD: Self = Self(600.0);
    pub const BOLD: Self = Self(700.0);
    pub const EXTRA_BOLD: Self = Self(800.0);
    pub const BLACK: Self = Self(900.0);

    pub const fn new(value: f32) -> Self {
        Self(value)
    }

    pub const fn value(&self) -> f32 {
        self.0
    }
}

impl From<FontWeight> for font_kit::properties::Weight {
    fn from(value: FontWeight) -> Self {
        font_kit::properties::Weight(value.value())
    }
}
