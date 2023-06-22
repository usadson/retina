// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use wgpu_glyph::{ab_glyph::{FontVec, PxScale}, GlyphPositioner, SectionGeometry};

use crate::FontDescriptor;

#[derive(Debug)]
pub struct Font {
    pub(crate) descriptor: FontDescriptor,
    pub(crate) font: FontVec,
}

impl Font {
    pub fn calculate_width(&self, size: f32, text: &str) -> f32 {
        0.0
    }
}
