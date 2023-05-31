// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use strum::{EnumIter, AsRefStr};

/// # References
/// * [CSS - Color Module Level 3 - 4.1](https://drafts.csswg.org/css-color-3/#html4)
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, EnumIter, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum BasicColorKeyword {
    Black,
    Silver,
    Gray,
    White,
    Maroon,
    Red,
    Purple,
    Fuchsia,
    Green,
    Lime,
    Olive,
    Yellow,
    Navy,
    Blue,
    Teal,
    Aqua,
}

impl From<BasicColorKeyword> for retina_common::Color {
    fn from(value: BasicColorKeyword) -> Self {
        match value {
            BasicColorKeyword::Black => Self::rgb_decimal(0, 0, 0),
            BasicColorKeyword::Silver => Self::rgb_decimal(192, 192, 192),
            BasicColorKeyword::Gray => Self::rgb_decimal(128, 128, 128),
            BasicColorKeyword::White => Self::rgb_decimal(255, 255, 255),
            BasicColorKeyword::Maroon => Self::rgb_decimal(128, 0, 0),
            BasicColorKeyword::Red => Self::rgb_decimal(255, 0, 0),
            BasicColorKeyword::Purple => Self::rgb_decimal(80, 0, 80),
            BasicColorKeyword::Fuchsia => Self::rgb_decimal(255, 0, 255),
            BasicColorKeyword::Green => Self::rgb_decimal(0, 128, 0),
            BasicColorKeyword::Lime => Self::rgb_decimal(0, 255, 0),
            BasicColorKeyword::Olive => Self::rgb_decimal(128, 128, 0),
            BasicColorKeyword::Yellow => Self::rgb_decimal(255, 255, 0),
            BasicColorKeyword::Navy => Self::rgb_decimal(0, 0, 128),
            BasicColorKeyword::Blue => Self::rgb_decimal(0, 0, 255),
            BasicColorKeyword::Teal => Self::rgb_decimal(0, 128, 128),
            BasicColorKeyword::Aqua => Self::rgb_decimal(0, 255, 255),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ColorValue {
    BasicColorKeyword(BasicColorKeyword),

    /// ***transparent***
    /// > Fully transparent. This keyword can be considered a shorthand for
    /// > transparent black, rgba(0,0,0,0), which is its computed value.
    ///
    /// # References
    /// * [CSS - Color Module Level 3 - 4.2.3](https://drafts.csswg.org/css-color-3/#transparent)
    Transparent,
}
