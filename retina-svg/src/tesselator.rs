// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::{
    Geometry,
    GeometrySink,
    GeometrySinkFillType,
    Material,
    Painter,
    StrokeStyle,
    StrokeStyleProperties,
};
use euclid::default::{Box2D, Rect, Size2D};

use lyon::lyon_tessellation::geometry_builder::simple_builder;
use lyon::math::Point;
use lyon::path::Winding;
use lyon::tessellation::*;

pub struct Tesselator;

impl Painter for Tesselator {
    fn create_geometry(&self, ty: GeometrySinkFillType) -> Box<dyn GeometrySink> {
        _ = ty;
        todo!()
    }

    fn create_stroke_style(&self, properties: StrokeStyleProperties) -> Box<dyn StrokeStyle> {
        _ = properties;
        todo!();
    }

    fn draw_geometry(&mut self, geometry: &dyn Geometry, material: Material) {
        _ = geometry;
        _ = material;
        todo!()
    }

    fn draw_rect(&mut self, rect: Box2D<f32>, material: Material) {
        let mut geometry: VertexBuffers<Point, u16> = VertexBuffers::new();
        let mut geometry_builder = simple_builder(&mut geometry);
        let options = FillOptions::tolerance(0.1);
        let mut tessellator = FillTessellator::new();

        let mut builder = tessellator.builder(
            &options,
            &mut geometry_builder,
        );

        builder.add_rectangle(
            &rect,
            Winding::Positive
        );

        builder.build().expect("failed to build path");

        _ = material;
        todo!();
    }

    fn stroke_geometry(&mut self, geometry: &dyn Geometry, material: Material, width: f32, stroke_style: Option<&dyn StrokeStyle>) {
        _ = geometry;
        _ = material;
        _ = width;
        _ = stroke_style;
        todo!();
    }

    fn stroke_rect(&mut self, rect: Box2D<f32>, material: Material, width: f32, stroke_style: Option<&dyn StrokeStyle>) {
        _ = rect;
        _ = material;
        _ = width;
        _ = stroke_style;
        todo!();
    }

    fn push_view_box(&self, view_box: Rect<f32>) {
        _ = view_box;
        todo!();
    }

    fn set_size(&self, size: Size2D<f32>) {
        _ = size;
        todo!();
    }
}
