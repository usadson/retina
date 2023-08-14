// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod color_glyph_run_analysis;
mod com;

pub use self::color_glyph_run_analysis::ColorGlyphRunAnalysis;

pub struct DWriteWrapper<'font> {
    font: &'font dwrote::Font,
    font_face: &'font dwrote::FontFace,
}

impl<'font> DWriteWrapper<'font> {

    pub fn rasterize(&self) {
        unsafe {
            let font = self.font_face.as_ptr();
            let font = &mut *font;

        }
    }

}
