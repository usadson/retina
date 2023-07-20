// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::{
    collections::HashMap,
    ops::{
        Deref,
        RangeInclusive,
    },
    sync::{
        Arc,
        RwLock,
    },
};

use euclid::default::{
    Point2D,
    Rect,
    Size2D,
};

use log::warn;

use ouroboros::self_referencing;

use pathfinder_geometry::{
    rect::RectF,
    transform2d::Transform2F,
    vector::Vector2F,
};

use rayon::prelude::*;

use retina_gfx::{
    CapitalLetterMode,
    EastAsianGlyphForm,
    EastAsianGlyphWidth,
    Context,
    FontDescriptor,
    LigatureMode,
    TextHintingOptions,
};

use wgpu::util::DeviceExt;

use font_kit::metrics::Metrics as BackendMetrics;

use crate::renderer::FontTextureMaterialRenderer;

pub struct FontKitFont {
    gfx_context: Context,
    descriptor: FontDescriptor,
    font: Backend,
    metrics: BackendMetrics,
    atlases: RwLock<Vec<RwLock<GlyphAtlas>>>,
}

unsafe impl Send for FontKitFont {}
unsafe impl Sync for FontKitFont {}

impl FontKitFont {
    pub fn new(
        context: &Context,
        descriptor: FontDescriptor,
        font: font_kit::font::Font,
    ) -> Self {
        // Initialize the renderer here to parallelize the material render setup
        // code, which costs some CPU and GPU time.
        _ = FontTextureMaterialRenderer::get(context);

        let metrics = font.metrics();
        let font = Backend::new(font);

        Self {
            gfx_context: context.clone(),
            descriptor,
            font,
            metrics,
            atlases: RwLock::new(Vec::new()),
        }
    }

    fn with_size<F, RetVal>(&self, size: f32, f: F) -> RetVal
            where F: FnOnce(&mut GlyphAtlas) -> RetVal {
        let atlases = self.atlases.read().unwrap();
        for atlas in atlases.iter() {
            if atlas.read().unwrap().is_size(size) {
                return f(&mut atlas.write().unwrap());
            }
        }

        drop(atlases);

        let mut atlas = GlyphAtlas::new(size, self.metrics.units_per_em);
        atlas.prepare_basic_latin(&self.gfx_context, &self.font);

        let mut atlases = self.atlases.write().unwrap();

        // TODO: dropping the mutable lock of `atlases` here can improve
        // concurrent read performance, but it is probably nicer to use a
        // concurrent HashMap for that instead.
        let return_value = f(&mut atlas);

        atlases.push(RwLock::new(atlas));

        return_value
    }

    fn glyph_iter<F>(
        &self,
        point_size: f32,
        text: &str,
        hints: TextHintingOptions,
        mut f: F
    )
            where F: FnMut(harfbuzz_rs::GlyphPosition, &Glyph) {
        self.with_size(point_size, |atlas| {
            self.font.with_harfbuzz_font(|font| {
                let unicode_buffer = harfbuzz_rs::UnicodeBuffer::new()
                    .add_str(text)
                    .guess_segment_properties();

                let features = resolve_hints_to_harfbuzz(hints);

                let glyph_buffer = harfbuzz_rs::shape(font, unicode_buffer, &features);
                let positions = glyph_buffer.get_glyph_positions();
                let infos = glyph_buffer.get_glyph_infos();

                for (position, info) in positions.iter().zip(infos) {
                    let glyph_id = GlyphId(info.codepoint);

                    let glyph = atlas.glyph(&self.gfx_context, &self.font, glyph_id)
                        .expect(&format!("Failed to lookup Glyph that HarfBuzz _did_ find: {glyph_id:?}"));

                    f(*position, glyph);
                }
            });
        });
    }
}

fn resolve_hints_to_harfbuzz(hints: TextHintingOptions) -> Vec<harfbuzz_rs::Feature> {
    use harfbuzz_rs::Feature;
    use crate::harfbuzz_util::*;

    let mut features = Vec::new();

    match hints.capitals {
        // https://drafts.csswg.org/css-fonts/#font-variant-caps-normal-value
        CapitalLetterMode::Normal => (),

        // https://drafts.csswg.org/css-fonts/#valdef-font-variant-caps-small-caps
        CapitalLetterMode::SmallCaps => {
            features.push(Feature::new(TAG_SMALL_CAPITALS, 1, ..));
        }

        // https://drafts.csswg.org/css-fonts/#valdef-font-variant-caps-all-small-caps
        CapitalLetterMode::AllSmallCaps => {
            features.push(Feature::new(TAG_SMALL_CAPITALS, 1, ..));
            features.push(Feature::new(TAG_SMALL_CAPITALS_FROM_CAPITALS, 1, ..));
        }

        // https://drafts.csswg.org/css-fonts/#valdef-font-variant-caps-petite-caps
        CapitalLetterMode::PetiteCaps => {
            features.push(Feature::new(TAG_PETITE_CAPITALS, 1, ..));
        }

        // https://drafts.csswg.org/css-fonts/#valdef-font-variant-caps-all-petite-caps
        CapitalLetterMode::AllPetiteCaps => {
            features.push(Feature::new(TAG_PETITE_CAPITALS, 1, ..));
            features.push(Feature::new(TAG_PETITE_CAPITALS_FROM_CAPITALS, 1, ..));
        }

        // https://drafts.csswg.org/css-fonts/#valdef-font-variant-caps-unicase
        CapitalLetterMode::Unicase => {
            features.push(Feature::new(TAG_UNICASE, 1, ..));
        }

        // https://drafts.csswg.org/css-fonts/#valdef-font-variant-caps-titling-caps
        CapitalLetterMode::TitlingCaps => {
            features.push(Feature::new(TAG_TITLING, 1, ..));
        }
    }

    match hints.east_asian_form {
        EastAsianGlyphForm::Normal => (),
        EastAsianGlyphForm::Jis78 => features.push(Feature::new(TAG_JP78, 1, ..)),
        EastAsianGlyphForm::Jis83 => features.push(Feature::new(TAG_JP83, 1, ..)),
        EastAsianGlyphForm::Jis90 => features.push(Feature::new(TAG_JP90, 1, ..)),
        EastAsianGlyphForm::Jis04 => features.push(Feature::new(TAG_JP04, 1, ..)),
        EastAsianGlyphForm::Simplified => features.push(Feature::new(TAG_SIMPLIFIED_FORMS, 1, ..)),
        EastAsianGlyphForm::Traditional => features.push(Feature::new(TAG_TRADITIONAL_FORMS, 1, ..)),
    }

    match hints.east_asian_width {
        EastAsianGlyphWidth::Normal => (),
        EastAsianGlyphWidth::FullWidth => features.push(Feature::new(TAG_FULL_WIDTHS, 1, ..)),
        EastAsianGlyphWidth::ProportionalWidth => features.push(Feature::new(TAG_PROPORTIONAL_WIDTHS, 1, ..)),
    }

    if !hints.kerning {
        features.push(Feature::new(TAG_KERN, 0, ..));
    }

    match hints.ligatures {
        // https://drafts.csswg.org/css-fonts/#font-variant-ligatures-none-value
        LigatureMode::None => {
            features.push(Feature::new(TAG_CONTEXTUAL_ALTERNATIVES, 0, ..));
            features.push(Feature::new(TAG_CONTEXTUAL_LIGATURES, 0, ..));
            features.push(Feature::new(TAG_DISCRETIONARY_LIGATURES, 0, ..));
            features.push(Feature::new(TAG_HISTORICAL_LIGATURES, 0, ..));
            features.push(Feature::new(TAG_STANDARD_LIGATURES, 0, ..));

            // We must not disable required ligatures (rlig), see
            // https://drafts.csswg.org/css-fonts/#rlig-unaffected%22
        }

        // https://drafts.csswg.org/css-fonts/#font-variant-ligatures-normal-value
        // See also: https://drafts.csswg.org/css-fonts/#default-features
        LigatureMode::Normal => {
            // Harfbuzz already complies :)
            // https://harfbuzz.github.io/shaping-opentype-features.html
        }

        LigatureMode::Specific {
            common,
            contextual,
            discretionary,
            historical,
        } => {
            // https://drafts.csswg.org/css-fonts/#valdef-font-variant-ligatures-common-ligatures
            let common = common as u32;
            features.push(Feature::new(TAG_CONTEXTUAL_LIGATURES, common, ..));
            features.push(Feature::new(TAG_STANDARD_LIGATURES, common, ..));


            // https://drafts.csswg.org/css-fonts/#valdef-font-variant-ligatures-contextual
            let contextual = contextual as u32;
            features.push(Feature::new(TAG_CONTEXTUAL_ALTERNATIVES, contextual, ..));


            // https://drafts.csswg.org/css-fonts/#valdef-font-variant-ligatures-discretionary-ligatures
            let discretionary = discretionary as u32;
            features.push(Feature::new(TAG_DISCRETIONARY_LIGATURES, discretionary, ..));


            // https://drafts.csswg.org/css-fonts/#valdef-font-variant-ligatures-historical-ligatures
            let historical = historical as u32;
            features.push(Feature::new(TAG_HISTORICAL_LIGATURES, historical, ..));
        }
    }

    if !hints.kerning {
        features.push(Feature::new(TAG_RUBY, 1, ..));
    }

    features
}

impl retina_gfx::Font for FontKitFont {
    fn calculate_size(&self, point_size: f32, text: &str, hints: TextHintingOptions) -> Size2D<f32> {
        let typographic_unit_conversion_factor = self.metrics.units_per_em as f32 / point_size;

        let height = (self.metrics.ascent - self.metrics.descent)
            / typographic_unit_conversion_factor;
        let mut size = Size2D::new(
            0.0,
            height,
        );

        self.glyph_iter(point_size, text, hints, |position, _glyph| {
            size.width += position.x_advance as f32 / typographic_unit_conversion_factor;
        });

        size
    }

    fn baseline_offset(&self, point_size: f32) -> f32 {
        let typographic_unit_conversion_factor = self.metrics.units_per_em as f32 / point_size;
        (self.metrics.ascent - self.metrics.descent) / typographic_unit_conversion_factor
    }

    #[inline]
    fn descriptor(&self) -> &FontDescriptor {
        &self.descriptor
    }

    fn underline_position(&self, point_size: f32) -> f32 {
        let typographic_unit_conversion_factor = self.metrics.units_per_em as f32 / point_size;
        self.metrics.underline_position / typographic_unit_conversion_factor
    }

    fn underline_thickness(&self, point_size: f32) -> f32 {
        let typographic_unit_conversion_factor = self.metrics.units_per_em as f32 / point_size;
        self.metrics.underline_thickness / typographic_unit_conversion_factor
    }

    fn paint(
        &self,
        text: &str,
        color: retina_common::Color,
        mut position: euclid::default::Point2D<f32>,
        font_size: f32,
        hints: TextHintingOptions,
        painter: &mut retina_gfx::Painter,
    ) {
        let typographic_unit_conversion_factor = self.metrics.units_per_em as f32 / font_size;

        // Offset the position to the baseline.
        position.y += self.metrics.ascent / typographic_unit_conversion_factor;

        self.glyph_iter(font_size, text, hints, |glyph_position, glyph| {
            let x_offset = glyph_position.x_offset as f32 / typographic_unit_conversion_factor;
            let y_offset = glyph_position.y_offset as f32 / typographic_unit_conversion_factor;
            let glyph_rect = Rect::new(
                Point2D::new(
                    position.x + glyph.origin.x() + x_offset,
                    position.y - glyph.typographic_bounds.max_y() + y_offset,
                ),
                glyph.size.cast(),
            ).cast();

            // If the texture is absent, this glyph is invisible (e.g. whitespace).
            if let Some(texture_view) = glyph.texture_view.as_ref() {
                let renderer = FontTextureMaterialRenderer::get(&painter.artwork().context);
                let bind_group_entry = wgpu::BindGroupEntry {
                    binding: 3,
                    resource: renderer.uniform_buffer.as_entire_binding(),
                };

                renderer.prepare(painter, color);

                painter.paint_rect_textured_with(
                    glyph_rect,
                    texture_view,
                    Some(&renderer.renderer),
                    Some(bind_group_entry),
                );
            }

            position.x += glyph_position.x_advance as f32 / typographic_unit_conversion_factor;
        });
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct GlyphId(pub u32);

/// TODO: this doesn't actually use a texture atlas, for simplicity reasons,
/// but this can/should be implemented in the future.
struct GlyphAtlas {
    size: f32,
    units_per_em: u32,
    glyphs: HashMap<GlyphId, Option<Glyph>>,
}

impl GlyphAtlas {
    pub fn new(size: f32, units_per_em: u32) -> Self {
        Self {
            size,
            units_per_em,
            glyphs: Default::default(),
        }
    }

    pub fn glyph(&mut self, context: &Context, font: &Backend, glyph_id: GlyphId) -> Option<&Glyph> {
        if !self.glyphs.contains_key(&glyph_id) {
            self.glyphs.insert(glyph_id, Glyph::new_opt(context, font, self.units_per_em, glyph_id, self.size));
        }

        self.glyphs.get(&glyph_id).unwrap().as_ref()
    }

    #[inline]
    pub fn is_size(&self, size: f32) -> bool {
        let convert = |val| (val * 10.0) as i32;
        convert(self.size) == convert(size)
    }

    #[inline]
    pub fn prepare_basic_latin(&mut self, context: &Context, font: &Backend) {
        self.prepare_chars(context, font, '!'..='~');
    }

    pub fn prepare_chars(&mut self, context: &Context, font: &Backend, range: RangeInclusive<char>) {
        let glyphs = range.into_par_iter()
            .filter_map(|character| {
                let context = context.clone();
                let font = font.clone();

                let glyph_id = GlyphId(font.borrow_font().glyph_for_char(character)?);

                let glyph = Glyph::new_opt(&context, &font, self.units_per_em, glyph_id, self.size);
                Some((glyph_id, glyph))
            });

        self.glyphs.par_extend(
            glyphs
        );
    }
}

/// TODO: this should obviously be in a texture atlas
struct Glyph {
    size: Size2D<u32>,
    typographic_bounds: RectF,
    origin: Vector2F,

    #[allow(dead_code)]
    texture: Option<wgpu::Texture>,

    texture_view: Option<wgpu::TextureView>,
}

impl Glyph {
    pub fn new(
        context: &Context,
        font: &Backend,
        units_per_em: u32,
        glyph_id: GlyphId,
        point_size: f32,
    ) -> Result<Self, Error> {
        let glyph_id = glyph_id.0;
        let transform = Transform2F::default();
        let hinting_options = font_kit::hinting::HintingOptions::None;
        let rasterization_options = font_kit::canvas::RasterizationOptions::SubpixelAa;

        let typographic_bounds = font.borrow_font().typographic_bounds(glyph_id)?;
        let typographic_unit_conversion_factor = units_per_em as f32 / point_size;
        let typographic_bounds = RectF::new(
            Vector2F::new(
                typographic_bounds.origin_x() / typographic_unit_conversion_factor,
                typographic_bounds.origin_y() / typographic_unit_conversion_factor
            ),
            Vector2F::new(
                typographic_bounds.width() / typographic_unit_conversion_factor,
                typographic_bounds.height() / typographic_unit_conversion_factor
            )
        );

        let origin = font.borrow_font().origin(glyph_id)? / typographic_unit_conversion_factor;

        let bounds = font.borrow_font().raster_bounds(glyph_id, point_size, transform, hinting_options, rasterization_options)?;

        if bounds.width() < 0 || bounds.height() < 0 {
            return Err(Error::InvalidGlyphBounds(bounds));
        }

        let mut texture = None;
        let mut texture_view = None;

        if bounds.size().x() != 0 && bounds.size().y() != 0 {
            let mut canvas = font_kit::canvas::Canvas::new(bounds.size(), font_kit::canvas::Format::Rgba32);
            let transform = Transform2F::from_translation(-bounds.origin().to_f32()) * transform;
            font.borrow_font().rasterize_glyph(&mut canvas, glyph_id, point_size, transform, hinting_options, rasterization_options)?;

            let created_texture = context.device().create_texture_with_data(
                context.queue(),
                &wgpu::TextureDescriptor {
                    dimension: wgpu::TextureDimension::D2,
                    format: wgpu::TextureFormat::Rgba8Unorm,
                    label: Some(&format!("FontGlyph[backend=font_kit, glyph_id={glyph_id}]")),
                    mip_level_count: 1,
                    sample_count: 1,
                    size: wgpu::Extent3d {
                        width: canvas.size.x() as _,
                        height: canvas.size.y() as _,
                        ..Default::default()
                    },
                    usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING,
                    view_formats: &[],
                },
                &canvas.pixels
            );

            texture_view = Some(created_texture.create_view(&wgpu::TextureViewDescriptor {
                ..Default::default()
            }));

            texture = Some(created_texture);
        }

        Ok(Self {
            size: Size2D::new(bounds.width() as _, bounds.height() as _),
            origin,
            typographic_bounds,
            texture,
            texture_view,
        })
    }

    pub fn new_opt(
        context: &Context,
        font: &Backend,
        units_per_em: u32,
        glyph_id: GlyphId,
        point_size: f32,
    ) -> Option<Self> {
        match Self::new(&context, &font, units_per_em, glyph_id, point_size) {
            Ok(glyph) => Some(glyph),
            Err(e) => {
                warn!("Failed to load glyph {glyph_id:?}: {e:?}");
                None
            }
        }
    }
}

#[derive(Debug)]
enum Error {
    GlyphLoadingError(font_kit::error::GlyphLoadingError),
    InvalidGlyphBounds(pathfinder_geometry::rect::RectI),
}

impl From<font_kit::error::GlyphLoadingError> for Error {
    fn from(value: font_kit::error::GlyphLoadingError) -> Self {
        Self::GlyphLoadingError(value)
    }
}

#[derive(Clone)]
struct Backend {
    inner: Arc<BackendInner>,
}

impl Backend {
    pub fn new(font: font_kit::font::Font) -> Self {
        let font_data = font.copy_font_data().unwrap();
        let inner = BackendInnerBuilder {
            font,
            font_data,
            harfbuzz_font_builder: |font_data: &Arc<Vec<u8>>| {
                let face = harfbuzz_rs::Face::from_bytes(&font_data, 0);
                let font = harfbuzz_rs::Font::new(face);
                font
            }
        };

        Self {
            inner: Arc::new(inner.build()),
        }
    }
}

impl Deref for Backend {
    type Target = Arc<BackendInner>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

unsafe impl Send for Backend {}
unsafe impl Sync for Backend {}

#[self_referencing]
struct BackendInner {
    font: font_kit::font::Font,

    font_data: Arc<Vec<u8>>,

    #[borrows(font_data)]
    #[not_covariant]
    harfbuzz_font: harfbuzz_rs::Owned<harfbuzz_rs::Font<'this>>,
}

unsafe impl Send for BackendInner {}
unsafe impl Sync for BackendInner {}
