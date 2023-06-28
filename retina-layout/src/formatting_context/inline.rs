// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use euclid::default::Point2D;
use retina_style::{
    CssDecimal,
    CssLength,
    CssReferencePixels,
};

use crate::{LayoutBox, boxes::LineBox};

use super::{
    FormattingContext,
    FormattingContextWhitespaceState,
};

pub struct InlineFormattingContext<'bx> {
    base: FormattingContext<'bx>,
    x_offset: CssDecimal,
    line_boxes: Vec<LineBox>,
}

impl<'bx> InlineFormattingContext<'bx> {
    pub fn perform(layout_box: &'bx mut LayoutBox) {
        let mut instance = Self {
            base: FormattingContext {
                layout_box,
                whitespace_state: FormattingContextWhitespaceState::Initial,
            },
            line_boxes: vec![LineBox::new()],
            x_offset: 0.0,
        };

        instance.perform_inner()
    }

    fn layout_box(&mut self) -> &mut LayoutBox {
        self.base.layout_box
    }

    fn perform_inner(&mut self) {
        let mut children = std::mem::replace(&mut self.layout_box().children, Vec::new());

        for child in &mut children {
            self.layout_child(child);
        }

        if let CssLength::Auto = self.layout_box().computed_style.height() {
            let height = self.line_boxes
                .iter()
                .map(|bx| bx.height)
                .sum();
            self.layout_box().dimensions.height = CssReferencePixels::new(height);
        }

        if let CssLength::Auto = self.layout_box().computed_style.width() {
            self.layout_box().dimensions.width = CssReferencePixels::new(self.x_offset);
        }

        self.layout_box().children = children;
    }

    fn layout_child(
        &mut self,
        child: &mut LayoutBox,
    ) {
        let content_position_origin = self.layout_box().dimensions.content_position;

        child.dimensions.set_margin_position(Point2D::new(
            content_position_origin.x + self.x_offset,
            content_position_origin.y,
        ));

        child.run_layout(Some(&mut self.base));

        let child_size = child.dimensions.size_margin_box();

        self.x_offset += child_size.width;

        let line_box = self.line_boxes.last_mut().unwrap();
        line_box.height = line_box.height.max(child_size.height);
    }
}
