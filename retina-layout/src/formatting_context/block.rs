// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use euclid::default::Point2D;
use retina_style::{
    CssDecimal,
    CssLength,
    CssReferencePixels,
};

use crate::LayoutBox;

use super::FormattingContext;

pub struct BlockFormattingContext<'bx> {
    base: FormattingContext<'bx>,
    y_offset: CssDecimal,
}

impl<'bx> BlockFormattingContext<'bx> {
    pub fn perform(layout_box: &'bx mut LayoutBox) {
        let mut instance = Self {
            base: FormattingContext {
                ended_with_whitespace: false,
                layout_box,
            },
            y_offset: 0.0,
        };

        instance.perform_inner()
    }

    fn perform_inner(&mut self) {
        let layout_box = &mut self.base.layout_box;

        let mut children = std::mem::replace(&mut layout_box.children, Vec::new());

        let mut max_container_width: f64 = 0.0;

        let content_position_origin = layout_box.dimensions.content_position;

        for child in &mut children {
            child.dimensions.set_margin_position(
                Point2D::new(
                    content_position_origin.x,
                    content_position_origin.y + self.y_offset,
                )
            );

            child.run_layout(Some(layout_box));

            let child_size = child.dimensions.size_margin_box();

            self.y_offset += child_size.height;
            max_container_width = max_container_width.max(child_size.width);
        }

        if let CssLength::Auto = layout_box.computed_style.height() {
            layout_box.dimensions.height = CssReferencePixels::new(self.y_offset);
        }

        if let CssLength::Auto = layout_box.computed_style.width() {
            layout_box.dimensions.width = CssReferencePixels::new(max_container_width);
        }

        layout_box.children = children;
    }
}
