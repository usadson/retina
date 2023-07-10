// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::sync::Arc;

use pathfinder_geometry::{line_segment::LineSegment2F, vector::Vector2F};
use wgpu_glyph::ab_glyph::{self, GlyphId, OutlineCurve, Point};

type FontKit = Arc<font_kit::font::Font>;

#[derive(Clone, Debug)]
pub struct FontKitAbGlyphFontBridge {
    inner: FontKit,
    fallback_glyph_id: GlyphId,
    hinting_options: font_kit::hinting::HintingOptions,
}

unsafe impl Send for FontKitAbGlyphFontBridge {}
unsafe impl Sync for FontKitAbGlyphFontBridge {}

/// <windows>
#[cfg(windows)]
impl FontKitAbGlyphFontBridge {
    pub fn new<T>(inner: T) -> Self
            where T: Into<FontKit> {
        let inner = inner.into();
        let fallback_glyph_id = GlyphId(inner.glyph_for_char('?').unwrap_or_default() as _);

        Self {
            inner,
            fallback_glyph_id,
            hinting_options: font_kit::hinting::HintingOptions::None,
        }
    }

    fn kern_unscaled_windows(&self, first: GlyphId, second: GlyphId) -> f32 {
        // TODO https://learn.microsoft.com/en-us/windows/win32/api/dwrite_1/nf-dwrite_1-idwritefontface1-getkerningpairadjustments
        _ = first;
        _ = second;
        0.0
    }
}
/// </windows>

impl ab_glyph::Font for FontKitAbGlyphFontBridge {
    fn units_per_em(&self) -> Option<f32> {
        Some(self.inner.metrics().units_per_em as _)
    }

    fn ascent_unscaled(&self) -> f32 {
        self.inner.metrics().ascent
    }

    fn descent_unscaled(&self) -> f32 {
        self.inner.metrics().descent
    }

    fn line_gap_unscaled(&self) -> f32 {
        self.inner.metrics().line_gap
    }

    fn glyph_id(&self, character: char) -> GlyphId {
        self.inner.glyph_for_char(character)
            .map(|id| GlyphId(id as u16))
            .unwrap_or(self.fallback_glyph_id)
    }

    fn h_advance_unscaled(&self, id: GlyphId) -> f32 {
        self.inner.advance(id.0 as _).map(|v| v.x()).unwrap_or(0.0)
    }

    fn h_side_bearing_unscaled(&self, id: GlyphId) -> f32 {
        self.inner.origin(id.0 as _).map(|v| v.x()).unwrap_or(0.0)
    }

    fn v_advance_unscaled(&self, id: GlyphId) -> f32 {
        self.inner.advance(id.0 as _).map(|v| v.y()).unwrap_or(0.0)
    }

    fn v_side_bearing_unscaled(&self, id: GlyphId) -> f32 {
        self.inner.origin(id.0 as _).map(|v| v.y()).unwrap_or(0.0)
    }

    #[cfg(windows)]
    fn kern_unscaled(&self, first: GlyphId, second: GlyphId) -> f32 {
        self.kern_unscaled_windows(first, second)
    }

    #[cfg(not(windows))]
    fn kern_unscaled(&self, first: GlyphId, second: GlyphId) -> f32 {
        _ = first;
        _ = second;
        0.0
    }

    fn outline(&self, id: GlyphId) -> Option<ab_glyph::Outline> {
        let bounds = self.inner.typographic_bounds(id.0 as _).ok()?;
        let bounds = ab_glyph::Rect {
            min: Point {
                x: bounds.min_x(),
                y: bounds.min_y()
            },
            max: Point {
                x: bounds.max_x(),
                y: bounds.max_y()
            },
        };

        let mut sink = FontKitAbGlyphOutlineBridge {
            inner: ab_glyph::Outline {
                bounds,
                curves: Vec::new(),
            },
            pen: Vector2F::default(),
        };


        match self.inner.outline(id.0 as _, self.hinting_options, &mut sink) {
            Ok(..) => Some(sink.finish()),
            Err(e) => {
                log::error!("Failed to load glyph({}): {e}", id.0);
                None
            }
        }
    }

    fn glyph_count(&self) -> usize {
        self.inner.glyph_count() as _
    }

    fn codepoint_ids(&self) -> ab_glyph::CodepointIdIter<'_> {
        // self.inner.native_font().dwrite_font_face.get_glyph_indices(code_points)
        todo!("We can't implement this because the ab_glyph type is protected")
    }

    fn glyph_raster_image(&self, id: GlyphId, pixel_size: u16) -> Option<ab_glyph::GlyphImage> {
        log::info!("Asked to raster glyph: {} with size {pixel_size}", id.0);
        _ = id;
        _ = pixel_size;
        None
    }
}

struct FontKitAbGlyphOutlineBridge {
    inner: ab_glyph::Outline,
    pen: Vector2F,
}

impl FontKitAbGlyphOutlineBridge {
    fn finish(self) -> ab_glyph::Outline {
        log::warn!("Finished outline with bounds {:?} and {} curves", self.inner.bounds,self.inner.curves.len());
        self.inner
    }

    #[inline]
    fn update_bounds(&mut self, vector: Vector2F) {
        // if self.inner.bounds.max.x < vector.x() {
        //     self.inner.bounds.max.x = vector.x();
        // }

        // if self.inner.bounds.max.y < vector.y() {
        //     self.inner.bounds.max.y = vector.y();
        // }

        // if self.inner.bounds.min.x > vector.x() {
        //     self.inner.bounds.min.x = vector.x();
        // }

        // if self.inner.bounds.min.y > vector.y() {
        //     self.inner.bounds.min.y = vector.y();
        // }
        _ = vector;
    }
}

impl font_kit::outline::OutlineSink for FontKitAbGlyphOutlineBridge {
    fn close(&mut self) {
        let Some(last_curve) = self.inner.curves.last() else {
            self.pen = Default::default();
            return;
        };

        self.pen = match last_curve {
            OutlineCurve::Line(a, ..) => Vector2F::new(a.x, a.y),
            OutlineCurve::Quad(a, ..) => Vector2F::new(a.x, a.y),
            OutlineCurve::Cubic(a, ..) => Vector2F::new(a.x, a.y),
        };
    }

    fn cubic_curve_to(&mut self, ctrl: LineSegment2F, to: Vector2F) {
        self.inner.curves.push(OutlineCurve::Cubic(
            Point { x: self.pen.x(), y: self.pen.y() },
            Point { x: ctrl.from_x(), y: ctrl.from_y() },
            Point { x: ctrl.to_x(), y: ctrl.to_y() },
            Point { x: to.x(), y: to.y() }
        ));

        self.update_bounds(ctrl.from());
        self.update_bounds(ctrl.to());
        self.update_bounds(to);
    }

    fn line_to(&mut self, to: Vector2F) {
        self.inner.curves.push(OutlineCurve::Line(
            Point { x: self.pen.x(), y: self.pen.y() },
            Point { x: to.x(), y: to.y() },
        ));

        self.update_bounds(to);

        self.pen = to;
    }

    fn move_to(&mut self, to: Vector2F) {
        self.pen = to;
    }

    fn quadratic_curve_to(&mut self, ctrl: Vector2F, to: Vector2F) {
        self.inner.curves.push(OutlineCurve::Quad(
            Point { x: self.pen.x(), y: self.pen.y() },
            Point { x: ctrl.x(), y: ctrl.y() },
            Point { x: to.x(), y: to.y() }
        ));

        self.update_bounds(ctrl);
        self.update_bounds(to);

        self.pen = to;
    }
}
