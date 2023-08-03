// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::hash::Hash;

use retina_common::StrTendril;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum FamilyName {
    Title(StrTendril),
    Cursive,
    Emoji,
    Fantasy,
    Monospace,
    SansSerif,
    Serif,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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

impl Hash for FontWeight {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let raw = self.0.to_bits();
        raw.hash(state);
    }
}

impl std::cmp::Eq for FontWeight {}
