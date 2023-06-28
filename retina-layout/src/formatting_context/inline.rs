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

pub struct InlineFormattingContext<'bx> {
    base: FormattingContext<'bx>,
    x_offset: CssDecimal,
}

impl<'bx> InlineFormattingContext<'bx> {
    pub fn perform(layout_box: &'bx mut LayoutBox) {
        let mut instance = Self {
            base: FormattingContext {
                ended_with_whitespace: false,
                layout_box,
            },
            x_offset: 0.0,
        };

        instance.perform_inner()
    }

    fn layout_box(&mut self) -> &mut LayoutBox {
        self.base.layout_box
    }

    fn perform_inner(&mut self) {
        let mut children = std::mem::replace(&mut self.layout_box().children, Vec::new());

        let mut max_container_height: f64 = 0.0;

        let content_position_origin = self.layout_box().dimensions.content_position;

        for child in &mut children {
            child.dimensions.set_margin_position(Point2D::new(
                content_position_origin.x + self.x_offset,
                content_position_origin.y,
            ));

            child.run_layout(Some(&mut self.base));

            let child_size = child.dimensions.size_margin_box();

            self.x_offset += child_size.width;
            max_container_height = max_container_height.max(child_size.height);
        }

        if let CssLength::Auto = self.layout_box().computed_style.height() {
            self.layout_box().dimensions.height = CssReferencePixels::new(max_container_height);
        }

        if let CssLength::Auto = self.layout_box().computed_style.width() {
            self.layout_box().dimensions.width = CssReferencePixels::new(self.x_offset);
        }

        self.layout_box().children = children;
    }
}
