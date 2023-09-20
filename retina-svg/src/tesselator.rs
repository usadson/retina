// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::{
    Geometry,
    GeometrySink,
    GeometrySinkFillType,
    Material,
    Painter,
};
use euclid::default::Box2D;

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
}
