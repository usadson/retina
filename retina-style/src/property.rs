// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use strum::{AsRefStr, EnumIter, IntoEnumIterator};

/// <https://www.w3.org/TR/css-values-3/#value-examples>
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(AsRefStr, EnumIter)]
#[strum(serialize_all = "kebab-case")]
pub enum Property {
    #[strum(disabled)]
    Invalid,

    BackgroundColor,
    Color,
    Display,
    Height,
    Width,
    WhiteSpace,
}

impl Property {
    pub fn parse(input: &str) -> Option<Self> {
        Self::iter().find(|property| property.as_ref() == input)
    }
}
