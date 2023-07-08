// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use euclid::default::{Point2D, Size2D};
use retina_common::StrTendril;
use retina_style::CssDecimal;

/// The rectangular area that contains the boxes that form a line is called a
/// line box.
#[derive(Clone, Debug)]
pub struct LineBox {
    pub(crate) height: CssDecimal,
}

impl LineBox {
    pub fn new() -> Self {
        Self {
            height: Default::default(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LineBoxFragment {
    pub(crate) position: Point2D<CssDecimal>,
    pub(crate) text: StrTendril,
    pub(crate) size: Size2D<CssDecimal>,
}

impl LineBoxFragment {
    #[inline]
    pub const fn position(&self) -> Point2D<CssDecimal> {
        self.position
    }

    #[inline]
    pub fn text(&self) -> &str {
        &self.text
    }
}
