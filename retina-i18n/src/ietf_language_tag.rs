// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

/// <https://www.iana.org/assignments/language-subtag-registry/language-subtag-registry>
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum IetfLanguageSubtag {
    Dutch,
    English,
}

impl IetfLanguageSubtag {
    pub fn from_str(value: &str) -> Option<Self> {
        if value.len() != 2 {
            return None;
        }

        // At all times, language tags and their subtags, including private use
        // and extensions, are to be treated as case insensitive: there exist
        // conventions for the capitalization of some of the subtags, but these
        // MUST NOT be taken to carry meaning.
        if value.eq_ignore_ascii_case("en") {
            return Some(Self::English);
        }

        if value.eq_ignore_ascii_case("nl") {
            return Some(Self::Dutch);
        }

        None
    }
}
