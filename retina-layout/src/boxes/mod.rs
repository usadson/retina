// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

//! # References
//! * [CSS Box Model Module Level 3](https://www.w3.org/TR/css-box-3/)
//! * [CSS Level 2 Revision 2 (CSS 2.2) - Box Model](https://www.w3.org/TR/CSS22/box.html)

mod dimensions;
mod edge;
mod line;

use std::borrow::Cow;

pub use dimensions::LayoutBoxDimensions;
pub use edge::LayoutEdge;
pub use line::LineBox;
use log::warn;
use retina_common::{DumpableNode, Color};
use retina_gfx_font::FontHandle;
use retina_style::{CssReferencePixels, CssDecimal};

use crate::formatting_context::{
    BlockFormattingContext,
    FormattingContext,
    FormattingContextKind,
    InlineFormattingContext, FormattingContextWhitespaceState,
};

use super::DomNode;
use retina_style_computation::PropertyMap;

#[derive(Clone, Debug, PartialEq)]
pub struct LayoutBox {
    pub(crate) kind: LayoutBoxKind,
    pub(crate) formatting_context: FormattingContextKind,
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
        formatting_context: FormattingContextKind,
        node: DomNode,
        computed_style: PropertyMap,
        dimensions: LayoutBoxDimensions,
        font: FontHandle,
        font_size: CssReferencePixels,
    ) -> Self {
        Self {
            kind,
            formatting_context,
            node,
            computed_style,
            dimensions,
            children: Vec::new(),
            font,
            font_size,
        }
    }

    /// Get the background-color, applicable for root boxes.
    pub fn background_color_as_root(&self) -> Color {
        debug_assert_eq!(self.kind, LayoutBoxKind::Root);

        let css_color = self.children[0].computed_style().background_color();

        match css_color {
            retina_style::CssColor::Color(color) => {
                color.with_alpha(1.0)
            }
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

    pub fn dimensions_mut(&mut self) -> &mut LayoutBoxDimensions {
        &mut self.dimensions
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

    fn run_anonymous_layout(&mut self, parent: &mut FormattingContext) {
        let Some(text) = self.node.as_text() else {
            warn!("Anonymous layout with a non-Text DOM node: {:#?}", self.node);
            return;
        };

        let mut text = Cow::Borrowed(text.data_as_str());
        if self.computed_style.white_space().collapses() {
            text = crate::white_space::collapse_white_space(text, parent.whitespace_state);
        }

        if text.ends_with(' ') {
            parent.whitespace_state = FormattingContextWhitespaceState::EndedWithWhitespace;
        } else {
            parent.whitespace_state = FormattingContextWhitespaceState::NoWhitespace;
        }

        // TODO this should participate in a inline formatting context.
        _ = parent;

        let size = self.font.calculate_size(self.font_size.value() as _, &text);
        self.dimensions.width = CssReferencePixels::new(size.width as CssDecimal);
        self.dimensions.height = CssReferencePixels::new(size.height as CssDecimal);
    }

    pub fn run_layout(&mut self, parent: Option<&mut FormattingContext>) {
        if let LayoutBoxKind::Anonymous = self.kind {
            if let Some(parent) = parent {
                self.run_anonymous_layout(parent);
            } else {
                warn!("Anonymous layout without a parent node.");
            }

            return;
        }

        match self.formatting_context {
            FormattingContextKind::Block => BlockFormattingContext::perform(self),
            FormattingContextKind::Inline => {
                // TODO
                _ = parent;
                InlineFormattingContext::perform(self)
            }
        }
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

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LayoutBoxKind {
    Root,
    Normal,
    Anonymous,
}
