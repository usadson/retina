// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::painter::{Material, Painter};
use euclid::default::Box2D;

use lyon::lyon_tessellation::geometry_builder::simple_builder;
use lyon::math::Point;
use lyon::path::Winding;
use lyon::tessellation::*;

pub struct Tesselator;

impl Painter for Tesselator {
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
