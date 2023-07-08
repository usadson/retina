// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::sync::{Arc, RwLock, RwLockWriteGuard};

use euclid::default::Size2D;

use wgpu_glyph::{
    ab_glyph::FontVec,
    GlyphBrush,
    GlyphCruncher,
    Section,
    Text,
};

use crate::FontDescriptor;

type Brush = GlyphBrush<(), FontVec>;

#[derive(Debug)]
pub struct Font {
    pub(crate) descriptor: FontDescriptor,
    pub(crate) brush: Arc<RwLock<Brush>>,

    /// The width of the space character.
    pub(crate) space_width: f32,
}

impl Font {
    pub fn brush(&self) -> RwLockWriteGuard<'_, Brush> {
        self.brush.write().unwrap()
    }

    #[inline]
    pub fn calculate_height(&self, size: f32, text: &str) -> f32 {
        self.calculate_size(size, text).height
    }

    pub fn calculate_size(&self, size: f32, text: &str) -> Size2D<f32> {
        let mut brush = self.brush();

        let bounds = brush.glyph_bounds(
            Section::builder()
                .add_text(
                    Text::new(text)
                        .with_scale(size * 1.5)
                )
        ).unwrap_or_default();

        Size2D::new(
            bounds.width(),
            bounds.height()
        )
    }

    #[inline]
    pub fn calculate_width(&self, size: f32, text: &str) -> f32 {
        self.calculate_size(size, text).width
    }

    #[inline]
    pub const fn descriptor(&self) -> &FontDescriptor {
        &self.descriptor
    }

    /// The width of the space character.
    #[inline]
    pub const fn width_of_space_character(&self) -> f32 {
        self.space_width
    }
}
