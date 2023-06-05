// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use retina_style::CssReferencePixels;

use super::LayoutEdge;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct LayoutBoxDimensions {
    pub(crate) width: CssReferencePixels,
    pub(crate) height: CssReferencePixels,

    pub(crate) padding: LayoutEdge,
    pub(crate) border: LayoutEdge,
    pub(crate) margin: LayoutEdge,
}

impl LayoutBoxDimensions {
    pub fn width(&self) -> CssReferencePixels {
        self.width
    }

    pub fn height(&self) -> CssReferencePixels {
        self.height
    }

    pub fn padding(&self) -> LayoutEdge {
        self.padding
    }

    pub fn border(&self) -> LayoutEdge {
        self.border
    }

    pub fn margin(&self) -> LayoutEdge {
        self.margin
    }
}
