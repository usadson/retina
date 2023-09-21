// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::any::Any;

use euclid::default::{Box2D, Rect};
use retina_common::Color;

use crate::path::{
    SvgPathCoordinatePair,
    SvgPathCoordinatePairDoubleSequence,
    SvgPathCoordinatePairTripletSequence,
    SvgPathCoordinateSequence,
    SvgPathType,
};

pub enum Material {
    Color(Color),
}

impl Material {
    pub fn is_transparent(&self) -> bool {
        match self {
            Self::Color(color) => color.alpha() <= 0.0,
        }
    }
}

pub trait Painter {
    fn create_geometry(&self, fill_type: GeometrySinkFillType) -> Box<dyn GeometrySink>;

    fn push_view_box(&self, view_box: Rect<f32>);

    fn draw_geometry(&mut self, geometry: &dyn Geometry, material: Material);
    fn draw_rect(&mut self, rect: Box2D<f32>, material: Material);

    fn stroke_geometry(&mut self, geometry: &dyn Geometry, material: Material, width: f32);
    fn stroke_rect(&mut self, rect: Box2D<f32>, material: Material, width: f32);
}

pub trait Geometry {
    fn as_any(&self) -> &dyn Any;
}

pub trait GeometrySink {
    fn close_path(&mut self);
    fn line_to(&mut self, ty: SvgPathType, coords: SvgPathCoordinatePair);
    fn horizontal_lines_to(&mut self, ty: SvgPathType, lines: SvgPathCoordinateSequence);
    fn vertical_lines_to(&mut self, ty: SvgPathType, lines: SvgPathCoordinateSequence);

    fn move_to(&mut self, ty: SvgPathType, coords: SvgPathCoordinatePair);

    fn curve_to(&mut self, ty: SvgPathType, sequence: SvgPathCoordinatePairTripletSequence);

    fn quadratic_beziers_curve_to(&mut self, ty: SvgPathType, sequence: SvgPathCoordinatePairDoubleSequence);
    fn smooth_quadratic_bezier_curve_to(&mut self, ty: SvgPathType, coords: SvgPathCoordinatePair);

    fn finish(&mut self) -> Box<dyn Geometry>;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GeometrySinkFillType {
    Hollow,
    Filled,
}
