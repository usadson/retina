// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use euclid::default::{Point2D, Size2D};
use retina_common::Color;
use crate::Painter;

use super::{
    FontDescriptor,
    features::TextHintingOptions,
};

pub trait Font {
    fn calculate_size(&self, size: f32, text: &str, hints: TextHintingOptions) -> Size2D<f32>;

    fn descriptor(&self) -> &FontDescriptor;

    fn baseline_offset(&self, point_size: f32) -> f32;
    fn underline_position(&self, point_size: f32) -> f32;
    fn underline_thickness(&self, point_size: f32) -> f32;

    fn paint(
        &self,
        text: &str,
        color: Color,
        position: Point2D<f32>,
        font_size: f32,
        hints: TextHintingOptions,
        painter: &mut Painter
    );
}
