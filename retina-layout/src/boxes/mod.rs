// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

//! # References
//! * [CSS Box Model Module Level 3](https://www.w3.org/TR/css-box-3/)
//! * [CSS Level 2 Revision 2 (CSS 2.2) - Box Model](https://www.w3.org/TR/CSS22/box.html)

mod dimensions;
mod edge;

pub use dimensions::LayoutBoxDimensions;
pub use edge::LayoutEdge;

use super::DomNode;
use retina_style_computation::PropertyMap;

#[derive(Clone, Debug)]
pub struct LayoutBox {
    pub(crate) kind: LayoutBoxKind,
    pub(crate) node: DomNode,
    pub(crate) computed_style: PropertyMap,
    pub(crate) dimensions: LayoutBoxDimensions,
    pub(crate) children: Vec<LayoutBox>,
}

impl LayoutBox {
    pub fn new(
        kind: LayoutBoxKind,
        node: DomNode,
        computed_style: PropertyMap,
        dimensions: LayoutBoxDimensions,
    ) -> Self {
        Self {
            kind,
            node,
            computed_style,
            dimensions,
            children: Vec::new(),
        }
    }

    pub fn dump(&self) {
        _ = self.dump_to(0, &mut std::io::stdout());
    }

    pub fn dump_to(&self, depth: usize, writer: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
        writeln!(
            writer,
            "{pad:pad_width$} LayoutBox({kind:?}, {width}x{height})",
            pad = "",
            pad_width = depth * 4,
            kind = self.kind,
            width = self.dimensions.width().value(),
            height = self.dimensions.height().value(),
        )?;

        for child in &self.children {
            child.dump_to(depth + 1, writer)?;
        }

        Ok(())
    }

    pub fn computed_style(&self) -> &PropertyMap {
        &self.computed_style
    }

    pub fn dimensions(&self) -> LayoutBoxDimensions {
        self.dimensions
    }

    pub fn kind(&self) -> &LayoutBoxKind {
        &self.kind
    }

    pub fn kind_mut(&mut self) -> &mut LayoutBoxKind {
        &mut self.kind
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum LayoutBoxKind {
    AnonymousBlock,
    AnonymousInline,
    Block,
    Inline,
}
