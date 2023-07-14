// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::sync::{Arc, RwLock, RwLockWriteGuard};

use euclid::default::{
    Point2D,
    Size2D,
};

use retina_common::Color;
use retina_gfx::Painter;
use wgpu_glyph::{
    ab_glyph::FontArc,
    GlyphBrush,
    GlyphCruncher,
    Section,
    Text,
};

use crate::FontDescriptor;

type Brush = GlyphBrush<(), FontArc>;

#[derive(Debug)]
pub struct WgpuFont {
    pub(crate) descriptor: FontDescriptor,
    pub(crate) brush: Arc<RwLock<Brush>>,

    /// The width of the space character.
    pub(crate) space_width: f32,
}

impl retina_gfx::Font for WgpuFont {
    fn calculate_size(&self, size: f32, text: &str) -> Size2D<f32> {
        WgpuFont::calculate_size(&self, size, text)
    }

    fn descriptor(&self) -> &FontDescriptor {
        &self.descriptor
    }

    fn paint(
        &self,
        text: &str,
        color: Color,
        position: Point2D<f32>,
        font_size: f32,
        painter: &mut Painter
    ) {
        let color = [color.red() as f32, color.green() as f32, color.blue() as f32, color.alpha() as f32];

        let viewport_rect = painter.viewport_rect();
        let mut glyph_brush = self.brush.write().unwrap();

        glyph_brush.queue(wgpu_glyph::Section {
            screen_position: (
                position.x - viewport_rect.origin.x as f32,
                position.y - viewport_rect.origin.y as f32,
            ),
            text: vec![wgpu_glyph::Text::new(text)
                .with_color(color)
                .with_scale(font_size * 1.5)],
            ..Default::default()
        });

        let (artwork, command_encoder) = painter.artwork_and_command_encoder();

        glyph_brush
            .draw_queued(
                artwork.context.device(),
                &mut artwork.staging_belt,
                command_encoder,
                &artwork.texture_view,
                viewport_rect.width() as _,
                viewport_rect.height() as _,
            )
            .expect("Draw queued");
    }
}

impl WgpuFont {
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
