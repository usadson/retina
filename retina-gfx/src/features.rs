// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum CapitalLetterMode {
    #[default]
    Normal,

    SmallCaps,

    AllSmallCaps,

    PetiteCaps,

    AllPetiteCaps,

    Unicase,

    TitlingCaps,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum EastAsianGlyphForm {
    #[default]
    Normal,
    Jis78,
    Jis83,
    Jis90,
    Jis04,
    Simplified,
    Traditional,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum EastAsianGlyphWidth {
    #[default]
    Normal,
    FullWidth,
    ProportionalWidth,
}


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
    pub capitals: CapitalLetterMode,
    pub east_asian_form: EastAsianGlyphForm,
    pub east_asian_width: EastAsianGlyphWidth,
    pub ligatures: LigatureMode,
    pub kerning: bool,
    pub ruby: bool,
}

impl Default for TextHintingOptions {
    fn default() -> Self {
        Self {
            capitals: CapitalLetterMode::default(),
            east_asian_form: EastAsianGlyphForm::default(),
            east_asian_width: EastAsianGlyphWidth::default(),
            ligatures: LigatureMode::default(),
            kerning: false,
            ruby: false,
        }
    }
}
