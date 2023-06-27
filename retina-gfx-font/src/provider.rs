// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::{
    collections::HashMap,
    sync::{Arc, RwLock}, path::Path, io::Read,
};

use log::error;
use retina_common::LoadTime;

use crate::{
    FamilyName,
    Font,
    FontDescriptor,
    FontFamily,
    FontHandle,
    FontWeight,
};

use retina_gfx::Context as GfxContext;

#[derive(Clone)]
pub struct FontProvider {
    gfx_context: GfxContext,
    families: Arc<RwLock<HashMap<FamilyName, FontFamily>>>,
}

unsafe impl Send for FontProvider{}

impl FontProvider {
    pub fn new(gfx_context: GfxContext) -> Self {
        Self {
            gfx_context,
            families: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn get(&self, descriptor: FontDescriptor) -> Option<FontHandle> {
        let Ok(families) = self.families.read() else {
            return None;
        };

        let Some(family) = families.get(&descriptor.name) else {
            return None;
        };

        for font in &family.entries {
            if font.descriptor.weight == descriptor.weight {
                return Some(FontHandle {
                    font: Arc::clone(font)
                });
            }
        }

        None
    }

    pub fn load(&self, descriptor: FontDescriptor, data: Vec<u8>) -> bool {
        let font = match wgpu_glyph::ab_glyph::FontVec::try_from_vec(data) {
            Ok(font) => font,
            Err(e) => {
                error!("Failed to load font ({descriptor:?}): {e}");
                return false;
            }
        };

        let brush = wgpu_glyph::GlyphBrushBuilder::using_font(font)
            .build(self.gfx_context.device(), wgpu::TextureFormat::Bgra8UnormSrgb);
        let brush = Arc::new(RwLock::new(brush));

        let mut families = self.families.write().expect("FontProvider failed to write to `families`");
        families.entry(descriptor.name.clone()).or_insert(Default::default()).entries.push(Arc::new(Font {
            descriptor,
            brush,
        }));

        true
    }

    pub fn load_defaults(&self) {
        use font_kit::properties::Properties;

        let source = font_kit::source::SystemSource::new();
        self.load_from_font_kit_handle(
            source.select_best_match(
                &[font_kit::family_name::FamilyName::SansSerif],
                &Properties::new()
            ).unwrap(),
            FontDescriptor {
                name: FamilyName::SansSerif,
                weight: FontWeight::REGULAR,
            }
        );

        self.load_default_in_background(FontDescriptor {
            name: FamilyName::SansSerif,
            weight: FontWeight::BOLD,
        });

        self.load_default_in_background(FontDescriptor {
            name: FamilyName::Serif,
            weight: FontWeight::REGULAR,
        });

        self.load_default_in_background(FontDescriptor {
            name: FamilyName::Serif,
            weight: FontWeight::BOLD,
        });
    }

    fn load_default_in_background(&self, descriptor: FontDescriptor) {
        let provider = self.clone();
        std::thread::spawn(move || {
            let source = font_kit::source::SystemSource::new();
            let desc = descriptor.clone();

            let handle = source.select_best_match(
                &[desc.name.into()],
                &font_kit::properties::Properties {
                    weight: desc.weight.into(),
                    ..Default::default()
                }
            ).unwrap();

            provider.load_from_font_kit_handle(
                handle,
                descriptor,
            )
        });
    }

    pub fn load_from_file(&self, load_time: LoadTime, path: &Path, descriptor: FontDescriptor) {
        match load_time {
            LoadTime::Background => {
                let provider = self.clone();
                let path = path.to_owned();
                std::thread::spawn(move || {
                    provider.load_from_file_impl(&path, descriptor);
                });
            }

            LoadTime::Now => {
                self.load_from_file_impl(path, descriptor);
            }
        }
    }

    fn load_from_file_impl(&self, path: &Path, descriptor: FontDescriptor) {
        let mut file = std::fs::File::open(path)
            .expect(&format!("failed to load file from path: {}", path.display()));

        let mut data = Vec::new();
        file.read_to_end(&mut data)
            .expect(&format!("failed to load file from path: {}", path.display()));

        _ = self.load(descriptor, data);
    }

    fn load_from_font_kit_handle(&self, handle: font_kit::handle::Handle, descriptor: FontDescriptor) {
        use font_kit::handle::Handle;

        match handle {
            Handle::Memory { bytes, font_index } => {
                _ = font_index;
                self.load(descriptor, Vec::clone(&bytes));
            }
            Handle::Path { path, font_index } => {
                _ = font_index;
                self.load_from_file(LoadTime::Now, &path, descriptor)
            }
        }
    }
}
