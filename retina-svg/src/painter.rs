// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::any::Any;

use euclid::default::{Box2D, Point2D, Rect, Size2D};
use retina_common::Color;

use crate::path::{
    SvgPathCoordinatePair,
    SvgPathCoordinatePairDouble,
    SvgPathCoordinatePairDoubleSequence,
    SvgPathCoordinatePairTripletSequence,
    SvgPathCoordinateSequence,
    SvgPathEllipticArcArgument,
    SvgPathType,
};

#[derive(Debug, Clone)]
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
    fn set_size(&self, size: Size2D<f32>);
    fn create_geometry(&self, fill_type: GeometrySinkFillType) -> Box<dyn GeometrySink>;
    fn create_stroke_style(&self, properties: StrokeStyleProperties) -> Box<dyn StrokeStyle>;

    fn push_view_box(&self, view_box: Rect<f32>);

    fn draw_ellipse(&mut self, center: Point2D<f32>, radius: Point2D<f32>, material: Material);
    fn draw_geometry(&mut self, geometry: &dyn Geometry, material: Material);
    fn draw_rect(&mut self, rect: Box2D<f32>, material: Material, radius: Point2D<f32>);

    fn stroke_ellipse(&mut self, center: Point2D<f32>, radius: Point2D<f32>, material: Material, width: f32, stroke_style: Option<&dyn StrokeStyle>);
    fn stroke_geometry(&mut self, geometry: &dyn Geometry, material: Material, width: f32, stroke_style: Option<&dyn StrokeStyle>);
    fn stroke_line(&mut self, start: Point2D<f32>, end: Point2D<f32>, material: Material, width: f32, stroke_style: Option<&dyn StrokeStyle>);
    fn stroke_rect(&mut self, rect: Box2D<f32>, material: Material, radius: Point2D<f32>, width: f32, stroke_style: Option<&dyn StrokeStyle>);
}

pub trait StrokeStyle {
    fn as_any(&self) -> &dyn Any;
}

#[derive(Clone, Default)]
pub struct StrokeStyleProperties {
    pub cap_style_dash: CapStyle,
    pub cap_style_start: CapStyle,
    pub cap_style_end: CapStyle,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CapStyle {
    #[default]
    Butt,
    Square,
    Round,
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
    fn smooth_curve_to(&mut self, ty: SvgPathType, double: SvgPathCoordinatePairDouble);

    fn quadratic_beziers_curve_to(&mut self, ty: SvgPathType, sequence: SvgPathCoordinatePairDoubleSequence);
    fn smooth_quadratic_bezier_curve_to(&mut self, ty: SvgPathType, coords: SvgPathCoordinatePair);
    fn elliptic_arc(&mut self, ty: SvgPathType, argument: SvgPathEllipticArcArgument);

    fn finish(&mut self) -> Box<dyn Geometry>;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GeometrySinkFillType {
    Hollow,
    Filled,
}
