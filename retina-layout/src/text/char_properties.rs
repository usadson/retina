// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

pub(super) trait CharExt {
    /// <https://unicode.org/charts/PDF/U1F100.pdf>
    fn is_regional_indicator(&self) -> bool;
}

impl CharExt for char {
    fn is_regional_indicator(&self) -> bool {
        const START: char = '\u{1F1E6}';
        const END: char = '\u{1F1FF}';
        *self >= START && *self <= END
    }
}
