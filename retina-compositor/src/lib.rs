// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use retina_gfx::{canvas::CanvasPainter, Color};
use retina_layout::LayoutBox;

pub struct Compositor {

}

impl Compositor {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn paint(&self, layout_box: &LayoutBox, painter: &mut CanvasPainter) {
        _ = layout_box;
        painter.paint_text("Compositor", Color::WHITE, retina_gfx::euclid::Point2D::new(0.0, 0.0));
    }
}
