// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use euclid::default::Box2D;
use retina_common::Color;

pub enum Material {
    Color(Color),
}

pub trait Painter {
    fn draw_rect(&mut self, rect: Box2D<f32>, material: Material);
}
