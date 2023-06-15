// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use retina_gfx::{canvas::CanvasPainter, euclid::{Rect, Point2D, Size2D}};
use retina_layout::LayoutBox;
use retina_style::CssColor;

pub struct Compositor {

}

impl Compositor {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn paint(&self, layout_box: &LayoutBox, painter: &mut CanvasPainter) {
        _ = layout_box;

        self.paint_inner(layout_box, painter);

        for child in layout_box.children() {
            self.paint(child, painter);
        }
    }

    fn paint_inner(&self, layout_box: &LayoutBox, painter: &mut CanvasPainter) {
        let position = Point2D::new(0.0, 0.0);

        let width = layout_box.dimensions().width().value();
        let height = layout_box.dimensions().height().value();

        let size = Size2D::new(width, height);

        if size.is_empty() {
            return;
        }

        let CssColor::Color(background_color) = layout_box.computed_style().background_color() else {
            return;
        };

        if background_color.alpha() <= 0.0 {
            return;
        }

        println!("Colored rect: {size:?} @ {position:?} with color {background_color:?}");

        painter.paint_rect_colored(Rect::new(position, size), background_color);
    }
}
