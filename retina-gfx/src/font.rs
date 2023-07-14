// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use euclid::default::{Point2D, Size2D};
use retina_common::Color;

use crate::{Painter, FontDescriptor};

pub trait Font {
    fn calculate_size(&self, size: f32, text: &str) -> Size2D<f32>;

    fn descriptor(&self) -> &FontDescriptor;

    fn paint(
        &self,
        text: &str,
        color: Color,
        position: Point2D<f32>,
        font_size: f32,
        painter: &mut Painter
    );
}
