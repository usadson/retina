// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use retina_style::CssReferencePixels;

#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct LayoutEdge {
    pub(crate) top: CssReferencePixels,
    pub(crate) bottom: CssReferencePixels,
    pub(crate) left: CssReferencePixels,
    pub(crate) right: CssReferencePixels,
}

impl LayoutEdge {
    pub fn top(&self) -> CssReferencePixels {
        self.top
    }

    pub fn bottom(&self) -> CssReferencePixels {
        self.bottom
    }

    pub fn left(&self) -> CssReferencePixels {
        self.left
    }

    pub fn right(&self) -> CssReferencePixels {
        self.right
    }
}
