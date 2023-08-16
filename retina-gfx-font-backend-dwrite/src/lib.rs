// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

#[cfg(windows)]
mod color_glyph_run_analysis;
#[cfg(windows)]
mod com;

#[cfg(windows)]
pub use self::color_glyph_run_analysis::ColorGlyphRunAnalysis;

#[cfg(windows)]
pub struct DWriteWrapper<'font> {
    font: &'font dwrote::Font,
    font_face: &'font dwrote::FontFace,
}

#[cfg(windows)]
impl<'font> DWriteWrapper<'font> {

    pub fn rasterize(&self) {
        unsafe {
            let font = self.font_face.as_ptr();
            let font = &mut *font;

        }
    }

}
