// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::CssDecimal;

/// The [___reference pixel___][rp] is the pixel unit that is independent of
/// scale by zooming.
///
/// [rp]: https://www.w3.org/TR/css-values-3/#reference-pixel
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct CssReferencePixels {
    value: CssDecimal,
}

impl CssReferencePixels {
    pub fn new(value: CssDecimal) -> Self {
        Self { value }
    }

    pub fn value(&self) -> CssDecimal {
        self.value
    }
}
