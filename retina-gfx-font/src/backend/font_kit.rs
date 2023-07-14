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

use pathfinder_geometry::{
    rect::RectF,
    transform2d::Transform2F,
    vector::Vector2F,
};

use rayon::prelude::*;

use retina_gfx::{
    Context,
    FontDescriptor,
};

use wgpu::util::DeviceExt;

use font_kit::metrics::Metrics as BackendMetrics;

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
        let metrics = font.metrics();
        Self {
            gfx_context: context.clone(),
            descriptor,
            font: Backend {
                font: Arc::new(font),
            },
            metrics,
            atlases: RwLock::new(Vec::new()),
        }
    }

    fn with_size<F, RetVal>(&self, size: f32, f: F) -> RetVal
            where F: FnOnce(&mut GlyphAtlas) -> RetVal {
        let atlases = self.atlases.read().unwrap();
        for atlas in atlases.iter() {
            return f(&mut atlas.write().unwrap());
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
}

impl retina_gfx::Font for FontKitFont {
    fn calculate_size(&self, point_size: f32, text: &str) -> Size2D<f32> {
        let mut size = Size2D::default();

        for character in text.chars() {
            let Some(glyph_id) = self.font.glyph_for_char(character) else { continue };
            let Ok(advance) = self.font.advance(glyph_id) else { continue };
            let advance = Size2D::new(advance.x(), advance.y());

            size.width += advance.width;
            if size.height < advance.height {
                size.height = advance.height;
            }
        }

        size * point_size
    }

    #[inline]
    fn descriptor(&self) -> &FontDescriptor {
        &self.descriptor
    }

    fn paint(
        &self,
        text: &str,
        color: retina_common::Color,
        mut position: euclid::default::Point2D<f32>,
        font_size: f32,
        painter: &mut retina_gfx::Painter
    ) {
        let ascent = self.metrics.ascent / self.metrics.units_per_em as f32 * font_size;
        let descent = -(self.metrics.descent / self.metrics.units_per_em as f32 * font_size);

        let baseline = ascent - descent;
        position.y += baseline;

        self.with_size(font_size, |atlas| {
            for character in text.chars() {
                let Some(glyph) = atlas.glyph(&self.gfx_context, &self.font, character) else {
                    continue;
                };

                let glyph_rect = Rect::new(
                    Point2D::new(
                        position.x + glyph.origin.x(),
                        position.y - glyph.typographic_bounds.max_y(),
                    ),
                    glyph.size.cast(),
                ).cast();

                _ = color; // TODO!

                // If the texture is absent, this glyph is invisible (e.g. whitespace).
                if let Some(texture_view) = glyph.texture_view.as_ref() {
                    painter.paint_rect_textured(glyph_rect, texture_view);
                }

                position.x += glyph.advance.x();
            }
        });
    }
}

/// TODO: this doesn't actually use a texture atlas, for simplicity reasons,
/// but this can/should be implemented in the future.
struct GlyphAtlas {
    size: f32,
    units_per_em: u32,
    glyphs: HashMap<char, Option<Glyph>>,
}

impl GlyphAtlas {
    pub fn new(size: f32, units_per_em: u32) -> Self {
        Self {
            size,
            units_per_em,
            glyphs: Default::default(),
        }
    }

    pub fn glyph(&mut self, context: &Context, font: &Backend, character: char) -> Option<&Glyph> {
        if !self.glyphs.contains_key(&character) {
            self.glyphs.insert(character, Glyph::new_opt(context, font, self.units_per_em, character, self.size));
        }

        self.glyphs.get(&character).unwrap().as_ref()
    }

    #[inline]
    pub fn prepare_basic_latin(&mut self, context: &Context, font: &Backend) {
        self.prepare_chars(context, font, '!'..='~');
    }

    pub fn prepare_chars(&mut self, context: &Context, font: &Backend, range: RangeInclusive<char>) {
        let glyphs = range.into_par_iter()
            .map(|character| {
                let context = context.clone();
                let font = font.clone();

                let glyph = Glyph::new_opt(&context, &font, self.units_per_em, character, self.size);
                (character, glyph)
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
    advance: Vector2F,

    #[allow(dead_code)]
    texture: Option<wgpu::Texture>,

    texture_view: Option<wgpu::TextureView>,
}

impl Glyph {
    pub fn new(
        context: &Context,
        font: &Backend,
        units_per_em: u32,
        character: char,
        point_size: f32,
    ) -> Result<Self, Error> {
        let glyph_id = font.glyph_for_char(character).ok_or(Error::GlyphNotPresent(character))?;

        let transform = Transform2F::default();
        let hinting_options = font_kit::hinting::HintingOptions::None;
        let rasterization_options = font_kit::canvas::RasterizationOptions::GrayscaleAa;

        let typographic_bounds = font.typographic_bounds(glyph_id)?;
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

        let origin = font.origin(glyph_id)? / typographic_unit_conversion_factor;
        let advance = font.advance(glyph_id)? / typographic_unit_conversion_factor;

        let bounds = font.raster_bounds(glyph_id, point_size, transform, hinting_options, rasterization_options)?;

        if bounds.width() < 0 || bounds.height() < 0 {
            return Err(Error::InvalidGlyphBounds(bounds));
        }

        let mut texture = None;
        let mut texture_view = None;

        if bounds.size().x() != 0 && bounds.size().y() != 0 {
            let mut canvas = font_kit::canvas::Canvas::new(bounds.size(), font_kit::canvas::Format::A8);
            let transform = Transform2F::from_translation(-bounds.origin().to_f32()) * transform;
            font.rasterize_glyph(&mut canvas, glyph_id, point_size, transform, hinting_options, rasterization_options)?;

            let created_texture = context.device().create_texture_with_data(
                context.queue(),
                &wgpu::TextureDescriptor {
                    dimension: wgpu::TextureDimension::D2,
                    format: wgpu::TextureFormat::R8Unorm,
                    label: Some(&format!("FontGlyph[backend=font_kit, char='{character}']")),
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
            advance,
            typographic_bounds,
            texture,
            texture_view,
        })
    }

    pub fn new_opt(
        context: &Context,
        font: &Backend,
        units_per_em: u32,
        character: char,
        point_size: f32,
    ) -> Option<Self> {
        match Self::new(&context, &font, units_per_em, character, point_size) {
            Ok(glyph) => Some(glyph),
            Err(e) => {
                warn!("Failed to load character U+{:0<4x}: {e:?}", character as usize);
                None
            }
        }
    }
}

#[derive(Debug)]
enum Error {
    GlyphNotPresent(char),
    GlyphLoadingError(font_kit::error::GlyphLoadingError),
    InvalidGlyphBounds(pathfinder_geometry::rect::RectI),
}

impl From<font_kit::error::GlyphLoadingError> for Error {
    fn from(value: font_kit::error::GlyphLoadingError) -> Self {
        Self::GlyphLoadingError(value)
    }
}

struct Backend {
    font: Arc<font_kit::font::Font>,
}

unsafe impl Send for Backend {}
unsafe impl Sync for Backend {}

impl Deref for Backend {
    type Target = Arc<font_kit::font::Font>;

    fn deref(&self) -> &Self::Target {
        &self.font
    }
}
