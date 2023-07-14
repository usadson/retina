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
        RwLock, Mutex,
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
    vector::Vector2F, transform2d::Transform2F,
};

static WARN_LOCK: Mutex<()> = Mutex::new(());

use rayon::prelude::*;
use retina_gfx::{Context, FontDescriptor};
use wgpu::util::DeviceExt;

pub struct FontKitFont {
    gfx_context: Context,
    descriptor: FontDescriptor,
    font: Backend,
    atlases: RwLock<Vec<RwLock<GlyphAtlas>>>,
}

unsafe impl Send for FontKitFont {}
unsafe impl Sync for FontKitFont {}

impl FontKitFont {
    // fn create_cache(&self) -> GlyphCache {

    // }

    pub fn new(
        context: &Context,
        descriptor: FontDescriptor,
        font: font_kit::font::Font,
    ) -> Self {
        Self {
            gfx_context: context.clone(),
            descriptor,
            font: Backend {
                font: Arc::new(font),
            },
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

        let mut atlas = GlyphAtlas::new(size);
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
        self.with_size(font_size, |atlas| {
            for character in text.chars() {
                let Some(glyph) = atlas.glyph(&self.gfx_context, &self.font, character) else {
                    continue;
                };

                let glyph_rect = Rect::new(
                    Point2D::new(
                        position.x + glyph.typographic_bounds.origin_x(),
                        position.y + glyph.typographic_bounds.origin_y(),
                    ),
                    glyph.size.cast(),
                ).cast();

                _ = color; // TODO!

                // If the texture is absent, this glyph is invisible (e.g. whitespace).
                if let Some(texture_view) = glyph.texture_view.as_ref() {
                    painter.paint_rect_textured(glyph_rect, texture_view);
                }

                position.x += glyph.typographic_bounds.width();
            }
        });
    }
}

/// TODO: this doesn't actually use a texture atlas, for simplicity reasons,
/// but this can/should be implemented in the future.
struct GlyphAtlas {
    size: f32,
    glyphs: HashMap<char, Option<Glyph>>,
}

impl GlyphAtlas {
    // pub fn new(context: &Context, size: f32, font: &Backend) -> Self {
    //     font.gl
    // }

    pub fn new(size: f32) -> Self {
        Self {
            size,
            glyphs: Default::default(),
        }
    }

    pub fn glyph(&mut self, context: &Context, font: &Backend, character: char) -> Option<&Glyph> {
        if !self.glyphs.contains_key(&character) {
            self.glyphs.insert(character, Glyph::new_opt(context, font, character, self.size));
        }

        self.glyphs.get(&character).unwrap().as_ref()
    }

    #[inline]
    pub fn prepare_basic_latin(&mut self, context: &Context, font: &Backend) {
        self.prepare_chars(context, font, '!'..='~');
    }

    pub fn prepare_chars(&mut self, context: &Context, font: &Backend, range: RangeInclusive<char>)  {
        let glyphs = range.into_iter()
            .map(|character| {
                let context = context.clone();
                let font = font.clone();

                let glyph = Glyph::new_opt(&context, &font, character, self.size);
                (character, glyph)
            });

        self.glyphs.extend(
            glyphs
        );
    }
}

/// TODO: this should obviously be in a texture atlas
struct Glyph {
    size: Size2D<u32>,
    typographic_bounds: RectF,

    #[allow(dead_code)]
    texture: Option<wgpu::Texture>,

    texture_view: Option<wgpu::TextureView>,
}

impl Glyph {
    pub fn new(
        context: &Context,
        font: &Backend,
        character: char,
        point_size: f32,
    ) -> Result<Self, Error> {
        let glyph_id = font.glyph_for_char(character).ok_or(Error::GlyphNotPresent(character))?;

        let transform = Transform2F::default();
        let hinting_options = font_kit::hinting::HintingOptions::None;
        let rasterization_options = font_kit::canvas::RasterizationOptions::GrayscaleAa;

        let typographic_bounds = font.typographic_bounds(glyph_id)?;
        let typographic_bounds = RectF::new(
            Vector2F::new(
                typographic_bounds.origin_x() * point_size,
                typographic_bounds.origin_y() * point_size
            ),
            Vector2F::new(
                typographic_bounds.width() * point_size,
                typographic_bounds.height() * point_size
            )
        );

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

            println!("Character: {character} with size {point_size}");
            println!("Canvas stride: {:?}", canvas.stride);
            println!("Canvas size: {:?}", canvas.size);
            println!("Canvas pixel count: {}", canvas.pixels.len());
            println!("Typo bounds: {:?}", typographic_bounds);
            println!("Raster bounds: {:?}", bounds);
            println!("Actual format: {:?}", canvas.format);

            println!("Pixels: {:?}", canvas.pixels);

            fn shade(value: u8) -> char {
                match value {
                    0 => ' ',
                    1..=84 => '░',
                    85..=169 => '▒',
                    170..=254 => '▓',
                    _ => '█',
                }
            }

            for y in 0..bounds.height() {
                let mut line = String::new();
                let (row_start, row_end) = (y as usize * canvas.stride, (y + 1) as usize * canvas.stride);
                let row = &canvas.pixels[row_start..row_end];
                for x in 0..bounds.width() {
                    match canvas.format {
                        font_kit::canvas::Format::Rgba32 => unimplemented!(),
                        font_kit::canvas::Format::Rgb24 => {
                            use std::fmt::Write;
                            write!(
                                &mut line,
                                "{}{}{}",
                                shade(row[x as usize * 3 + 0]),
                                shade(row[x as usize * 3 + 1]),
                                shade(row[x as usize * 3 + 2]),
                            )
                            .unwrap();
                        }
                        font_kit::canvas::Format::A8 => {
                            let shade = shade(row[x as usize]);
                            line.push(shade);
                            line.push(shade);
                        }
                    }
                }
                println!("{}", line);
            }

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
            typographic_bounds,
            texture,
            texture_view,
        })
    }

    pub fn new_opt(
        context: &Context,
        font: &Backend,
        character: char,
        point_size: f32,
    ) -> Option<Self> {
        match Self::new(&context, &font, character, point_size) {
            Ok(glyph) => Some(glyph),
            Err(e) => {
                let _guard = WARN_LOCK.lock();
                warn!("Failed to load character U+{:0<4x}: {e:?}", character as usize);
                drop(_guard);
                std::process::abort();
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
