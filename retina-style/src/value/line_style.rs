// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[derive(strum::EnumString)]
#[strum(serialize_all = "kebab-case")]
pub enum CssLineStyle {
    None,
    Hidden,
    Dotted,
    Dashed,
    Solid,
    Double,
    Groove,
    Ridge,
    Inset,
    Outset,
}

impl CssLineStyle {
    pub fn from_str(value: &str) -> Option<Self> {
        std::str::FromStr::from_str(value).ok()
    }
}
