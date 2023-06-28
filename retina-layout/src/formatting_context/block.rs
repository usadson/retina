// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use euclid::default::Point2D;
use retina_style::{
    CssDecimal,
    CssLength,
    CssReferencePixels,
};

use crate::LayoutBox;

pub struct BlockFormattingContext<'bx> {
    layout_box: &'bx mut LayoutBox,
    y_offset: CssDecimal,
}

impl<'bx> BlockFormattingContext<'bx> {
    pub fn perform(layout_box: &'bx mut LayoutBox) {
        let mut instance = Self {
            layout_box,
            y_offset: 0.0,
        };

        instance.perform_inner()
    }

    fn perform_inner(&mut self) {
        let mut children = std::mem::replace(&mut self.layout_box.children, Vec::new());

        let mut max_container_width: f64 = 0.0;

        let content_position_origin = self.layout_box.dimensions.content_position;

        for child in &mut children {
            child.dimensions.set_margin_position(
                Point2D::new(
                    content_position_origin.x,
                    content_position_origin.y + self.y_offset,
                )
            );

            child.run_layout(Some(self.layout_box));

            let child_size = child.dimensions.size_margin_box();

            self.y_offset += child_size.height;
            max_container_width = max_container_width.max(child_size.width);
        }

        if let CssLength::Auto = self.layout_box.computed_style.height() {
            self.layout_box.dimensions.height = CssReferencePixels::new(self.y_offset);
        }

        if let CssLength::Auto = self.layout_box.computed_style.width() {
            self.layout_box.dimensions.width = CssReferencePixels::new(max_container_width);
        }

        self.layout_box.children = children;
    }
}
