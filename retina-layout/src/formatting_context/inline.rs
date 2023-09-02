// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use euclid::default::Point2D;
use retina_style::{
    CssDecimal,
    CssLength,
    CssReferencePixels,
};

use crate::{LayoutBox, boxes::LineBox};

use super::FormattingContext;

pub struct InlineFormattingContext<'bx> {
    base: FormattingContext<'bx>,
    state: InlineFormattingContextState,
}

pub struct InlineFormattingContextState {
    pub(crate) x_offset: CssDecimal,
    pub(crate) line_boxes: Vec<LineBox>,
    pub(crate) content_position_origin: Point2D<CssDecimal>,
}

impl InlineFormattingContextState {
    pub fn new(content_position_origin: Point2D<CssDecimal>) -> Self {
        Self {
            line_boxes: vec![LineBox::new()],
            x_offset: 0.0,
            content_position_origin,
        }
    }
}

impl<'bx> InlineFormattingContext<'bx> {
    pub fn perform(layout_box: &'bx mut LayoutBox, parent: Option<&FormattingContext>) {
        let content_position_origin = layout_box.dimensions().content_position;

        let mut instance = Self {
            base: FormattingContext::new(parent, layout_box),
            state: InlineFormattingContextState::new(content_position_origin),
        };

        instance.perform_inner()
    }

    fn layout_box(&mut self) -> &mut LayoutBox {
        self.base.layout_box
    }

    fn perform_inner(&mut self) {
        let mut children = std::mem::replace(&mut self.layout_box().children, Vec::new());

        let max_width = self.layout_box().dimensions().width().value();

        for child in &mut children {
            if max_width != 0.0 && self.state.x_offset > max_width {
                self.state.content_position_origin.y += self.state.line_boxes.last().unwrap().height;
                self.state.line_boxes.push(LineBox::new());
                self.state.x_offset = 0.0;
            }
            self.layout_child(child);
        }

        if let CssLength::Auto = self.layout_box().computed_style.height() {
            let height = self.state.line_boxes
                .iter()
                .map(|bx| bx.height)
                .sum();
            self.layout_box().dimensions.height = CssReferencePixels::new(height);
        }

        if let CssLength::Auto = self.layout_box().computed_style.width() {
            self.layout_box().dimensions.width = CssReferencePixels::new(self.state.x_offset);
        }

        self.layout_box().children = children;
    }

    fn layout_child(
        &mut self,
        child: &mut LayoutBox,
    ) {
        child.dimensions = child.actual_value_map.dimensions;
        child.dimensions.set_margin_position(Point2D::new(
            self.state.content_position_origin.x + self.state.x_offset,
            self.state.content_position_origin.y,
        ));

        child.run_layout(Some(&mut self.base), Some(&mut self.state));

        let child_size = child.dimensions.size_margin_box();

        if let Some(last_fragment) = child.line_box_fragments.last() {
            self.state.x_offset = last_fragment.position.x - self.state.content_position_origin.x + last_fragment.size.width;
        } else {
            self.state.x_offset += child_size.width;
        }

        let line_box = self.state.line_boxes.last_mut().unwrap();
        line_box.height = line_box.height.max(child_size.height);
    }
}
