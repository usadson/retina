// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum LigatureMode {
    None,

    #[default]
    Normal,

    Specific {
        /// OpenType: liga, clig
        common: bool,

        // OpenType: dlig
        discretionary: bool,

        // OpenType: hlig
        historical: bool,

        // OpenType: calt
        contextual: bool,
    }
}

/// This struct allows for hinting options for text rendering. It is
/// important to note that these are hints, and may or may not be honored
/// by the backing implementation.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TextHintingOptions {
    pub ligatures: LigatureMode,
    pub kerning: bool,
}

impl Default for TextHintingOptions {
    fn default() -> Self {
        Self {
            ligatures: LigatureMode::default(),
            kerning: false,
        }
    }
}
