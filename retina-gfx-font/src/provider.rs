// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::{
    collections::HashMap,
    io::Read,
    sync::{
        Arc,
        RwLock,
    },
    path::Path,
};

use log::error;
use retina_common::LoadTime;

use crate::{
    backend::FontKitFont,
    descriptor::{
        convert_font_kit_name,
        convert_font_kit_weight,
    },
    FamilyName,
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
    implementation_kind: FontProviderImplementationKind,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum FontProviderImplementationKind {
    FontKit,
}

unsafe impl Send for FontProvider{}

impl FontProvider {
    pub fn new(gfx_context: GfxContext) -> Self {
        Self {
            gfx_context,
            families: Arc::new(RwLock::new(HashMap::new())),
            implementation_kind: FontProviderImplementationKind::FontKit,
        }
    }

    pub fn get(&self, descriptor: FontDescriptor) -> Option<FontHandle> {
        let Ok(families) = self.families.read() else {
            self.load_default_in_background(descriptor);
            return None;
        };

        let Some(family) = families.get(&descriptor.name) else {
            self.load_default_in_background(descriptor);
            return None;
        };

        for font in &family.entries {
            if font.descriptor().weight == descriptor.weight {
                return Some(FontHandle {
                    font: Arc::clone(font)
                });
            }
        }

        self.load_default_in_background(descriptor);

        None
    }

    pub fn load(&self, descriptor: FontDescriptor, data: Vec<u8>) -> bool {
        match self.implementation_kind {
            FontProviderImplementationKind::FontKit => {
                let font = match font_kit::font::Font::from_bytes(Arc::new(data), 0) {
                    Ok(font) => font,
                    Err(e) => {
                        error!("Failed to load font ({descriptor:?}): {e}");
                        return false;
                    }
                };

                let family_name = descriptor.name.clone();

                let font = FontKitFont::new(
                    &self.gfx_context,
                    descriptor,
                    font
                );

                self.load_gfx_font(family_name, Arc::new(font));

                true
            }
        }
    }

    fn load_gfx_font(&self, family_name: FamilyName, font: Arc<dyn retina_gfx::Font>) {
        let mut families = self.families.write()
            .expect("FontProvider failed to write to `families`");

        families.entry(family_name)
            .or_insert(Default::default())
            .entries
            .push(font);
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
                &[convert_font_kit_name(desc.name)],
                &font_kit::properties::Properties {
                    weight: convert_font_kit_weight(desc.weight),
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
