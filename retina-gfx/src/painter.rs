// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use euclid::default::{
    Rect,
    Size2D, Point2D,
};

use retina_common::Color;

use crate::{TextureViewId, font::{Font, TextHintingOptions}};

pub trait Painter {
    fn viewport_size(&self) -> Size2D<u32> {
        self.viewport_rect().size.cast()
    }

    fn viewport_rect(&self) -> Rect<f64>;

    /// Returns whether or not the given rect is inside the viewport.
    fn is_rect_inside_viewport<Unit>(&self, rect: euclid::Rect<f32, Unit>) -> bool {
        self.viewport_rect().intersects(&rect.cast().cast_unit())
    }

    fn clear(&mut self, clear_color: Color);

    fn paint_rect_colored(&mut self, rect: Rect<f64>, color: Color);

    fn paint_rect_textured(&mut self, rect: Rect<f64>, texture_view: TextureViewId);

    fn paint_text(
        &self,
        font: &dyn Font,
        text: &str,
        color: Color,
        position: Point2D<f32>,
        font_size: f32,
        hints: TextHintingOptions,
    );

    fn submit_sync(self);

    fn submit_async(self) -> Box<dyn std::future::Future<Output = ()>>;
}
