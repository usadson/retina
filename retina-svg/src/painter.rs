// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::any::Any;

use euclid::default::Box2D;
use retina_common::Color;

use crate::path::{SvgPathCoordinatePair, SvgPathType};

pub enum Material {
    Color(Color),
}

pub trait Painter {
    fn create_geometry(&self, fill_type: GeometrySinkFillType) -> Box<dyn GeometrySink>;

    fn draw_rect(&mut self, rect: Box2D<f32>, material: Material);
    fn draw_geometry(&mut self, geometry: &dyn Geometry, material: Material);
}

pub trait Geometry {
    fn as_any(&self) -> &dyn Any;
}

pub trait GeometrySink {
    fn close_path(&mut self);
    fn line_to(&mut self, ty: SvgPathType, coords: SvgPathCoordinatePair);
    fn move_to(&mut self, ty: SvgPathType, coords: SvgPathCoordinatePair);

    fn finish(&mut self) -> Box<dyn Geometry>;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GeometrySinkFillType {
    Hollow,
    Filled,
}
