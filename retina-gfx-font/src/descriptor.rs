// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::hash::Hash;

use retina_common::StrTendril;
use unicase::UniCase;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum FamilyName {
    Title(UniCase<StrTendril>),
    Cursive,
    Emoji,
    Fangsong,
    Fantasy,
    Math,
    Monospace,
    SansSerif,
    Serif,
    SystemUi,
    UiMonospace,
    UiRounded,
    UiSansSerif,
    UiSerif,
}

impl From<StrTendril> for FamilyName {
    fn from(value: StrTendril) -> Self {
        Self::Title(UniCase::new(value))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FontDescriptor {
    pub name: FamilyName,
    pub style: FontStyle,
    pub weight: FontWeight,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FontStyle {
    #[default]
    Normal,

    Italic,

    Oblique,
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

impl Hash for FontWeight {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let raw = self.0.to_bits();
        raw.hash(state);
    }
}

impl std::cmp::Eq for FontWeight {}
