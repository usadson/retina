// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use euclid::default::Point2D;
use retina_common::Color;

use crate::Painter;

pub trait Font {
    fn paint(
        &self,
        text: &str,
        color: Color,
        position: Point2D<f32>,
        font_size: f32,
        painter: &mut Painter
    );
}
