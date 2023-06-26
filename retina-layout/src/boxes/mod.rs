// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

//! # References
//! * [CSS Box Model Module Level 3](https://www.w3.org/TR/css-box-3/)
//! * [CSS Level 2 Revision 2 (CSS 2.2) - Box Model](https://www.w3.org/TR/CSS22/box.html)

mod dimensions;
mod edge;
mod line;

pub use dimensions::LayoutBoxDimensions;
pub use edge::LayoutEdge;
pub use line::LineBox;
use retina_common::DumpableNode;
use retina_gfx_font::FontHandle;
use retina_style::CssReferencePixels;

use super::DomNode;
use retina_style_computation::PropertyMap;

#[derive(Clone, Debug)]
pub struct LayoutBox {
    pub(crate) kind: LayoutBoxKind,
    pub node: DomNode,
    pub(crate) computed_style: PropertyMap,
    pub(crate) dimensions: LayoutBoxDimensions,
    pub(crate) children: Vec<LayoutBox>,
    pub(crate) font: FontHandle,
    pub(crate) font_size: CssReferencePixels,
}

impl LayoutBox {
    pub fn new(
        kind: LayoutBoxKind,
        node: DomNode,
        computed_style: PropertyMap,
        dimensions: LayoutBoxDimensions,
        font: FontHandle,
        font_size: CssReferencePixels,
    ) -> Self {
        Self {
            kind,
            node,
            computed_style,
            dimensions,
            children: Vec::new(),
            font,
            font_size,
        }
    }

    pub fn children(&self) -> &[LayoutBox] {
        &self.children
    }

    pub fn computed_style(&self) -> &PropertyMap {
        &self.computed_style
    }

    pub fn dimensions(&self) -> LayoutBoxDimensions {
        self.dimensions
    }

    pub const fn font(&self) -> &FontHandle {
        &self.font
    }

    pub const fn font_size(&self) -> CssReferencePixels {
        self.font_size
    }

    pub fn kind(&self) -> &LayoutBoxKind {
        &self.kind
    }

    pub fn kind_mut(&mut self) -> &mut LayoutBoxKind {
        &mut self.kind
    }

    #[inline]
    pub fn dump(&self) {
        DumpableNode::dump(self);
    }
}

impl DumpableNode for LayoutBox {
    fn dump_to(&self, depth: usize, writer: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
        writeln!(
            writer,
            "{pad:pad_width$}LayoutBox({kind:?}, {display}), {dom:?}, {width}x{height} @ ({x}, {y})",
            pad = "",
            pad_width = depth * 4,
            kind = self.kind,
            width = self.dimensions.width().value(),
            height = self.dimensions.height().value(),
            dom = self.node.to_short_dumpable(),
            display = self.computed_style.display(),
            x = self.dimensions.position_padding_box().x,
            y = self.dimensions.position_padding_box().y,
        )?;

        for child in &self.children {
            child.dump_to(depth + 1, writer)?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub enum LayoutBoxKind {
    AnonymousBlock,
    AnonymousInline,
    Block,
    Inline,
}
