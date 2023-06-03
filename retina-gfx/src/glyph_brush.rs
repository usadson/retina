// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use super::GfxResult;

use wgpu_glyph::{
    ab_glyph,
    GlyphBrushBuilder,
};

pub(crate) struct GlyphBrush {
    pub(crate) inner: wgpu_glyph::GlyphBrush<()>,
}

impl GlyphBrush {
    pub(crate)fn new_noto_serif(
        device: &wgpu::Device,
        render_format: wgpu::TextureFormat,
    ) -> GfxResult<Self> {
        let noto_serif = ab_glyph::FontArc::try_from_slice(include_bytes!(
            concat!(env!("CARGO_MANIFEST_DIR"), "/../resources/fonts/noto-serif/NotoSerif-Regular.ttf")
        ))?;

        let inner = GlyphBrushBuilder::using_font(noto_serif)
            .build(&device, render_format);

        Ok(Self { inner })
    }
}
