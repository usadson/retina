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
    font::FontKitFont,
    family::FontFamily,
    convert_font_kit_style,
    convert_font_kit_weight,
};

use retina_gfx_font::{
    descriptor::{
        FamilyName,
        FontDescriptor,
        FontWeight,
    },
    FontHandle,
    FontProviderBackend,
    FontStyle,
};

use retina_gfx::Context as GfxContext;

#[derive(Clone)]
pub struct FontProvider {
    gfx_context: GfxContext,
    families: Arc<RwLock<HashMap<FamilyName, FontFamily>>>,
    static_aliases: Arc<[(FamilyName, String)]>,
}

unsafe impl Send for FontProvider{}

impl FontProvider {
    pub fn new(gfx_context: GfxContext, static_aliases: Arc<[(FamilyName, String)]>) -> Self {
        Self {
            gfx_context,
            families: Arc::new(RwLock::new(HashMap::new())),
            static_aliases,
        }
    }

    fn convert_family_name(&self, value: FamilyName) -> font_kit::family_name::FamilyName {
        use font_kit::family_name::FamilyName as FkName;
        match value {
            FamilyName::Title(name) => FkName::Title(name.to_string()),
            FamilyName::Cursive => FkName::Cursive,
            FamilyName::Fantasy => FkName::Fantasy,
            FamilyName::Monospace => FkName::Monospace,
            FamilyName::SansSerif => FkName::SansSerif,
            FamilyName::Serif => FkName::Serif,
            _ => {
                self.static_aliases
                    .iter()
                    .find(|(alias, _)| alias == &value)
                    .map(|(_, actual)| FkName::Title(actual.into()))
                    .unwrap_or(FkName::Serif)
            }
        }
    }

    fn load_gfx_font(&self, family_name: FamilyName, font: Arc<dyn retina_gfx_font::Font>) {
        let mut families = self.families.write()
            .expect("FontProvider failed to write to `families`");

        families.entry(family_name)
            .or_insert(Default::default())
            .entries
            .push(font);
    }

    fn load_default_in_background(&self, descriptor: FontDescriptor) {
        let provider = self.clone();
        std::thread::spawn(move || {
            let source = font_kit::source::SystemSource::new();
            let desc = descriptor.clone();

            let handle = source.select_best_match(
                &[provider.convert_family_name(desc.name)],
                &font_kit::properties::Properties {
                    weight: convert_font_kit_weight(desc.weight),
                    ..Default::default()
                }
            );

            let handle = match handle {
                Ok(handle) => handle,

                Err(e) => {
                    error!("Failed to find default font! Error: {e}, descriptor: {descriptor:#?}");
                    return false;
                }
            };

            provider.load_from_font_kit_handle(
                handle,
                descriptor,
            )
        });
    }

    fn load_from_file_impl(&self, path: &Path, descriptor: FontDescriptor, font_index: u32) -> bool {
        let mut file = std::fs::File::open(path)
            .expect(&format!("failed to load file from path: {}", path.display()));

        let mut data = Vec::new();
        file.read_to_end(&mut data)
            .expect(&format!("failed to load file from path: {}", path.display()));

        self.load(descriptor, data, font_index)
    }

    fn load_from_font_kit_handle(&self, handle: font_kit::handle::Handle, descriptor: FontDescriptor) -> bool {
        use font_kit::handle::Handle;

        match handle {
            Handle::Memory { bytes, font_index } => {
                println!("load_from_font_kit_handle, font_index: {font_index}");
                self.load(descriptor, Vec::clone(&bytes), font_index)
            }
            Handle::Path { path, font_index } => {
                println!("load_from_font_kit_handle, font_index: {font_index}");
                self.load_from_file(LoadTime::Now, &path, descriptor, font_index)
            }
        }
    }
}

unsafe impl Sync for FontProvider {}

impl FontProviderBackend for FontProvider {
    fn get(&self, descriptor: &FontDescriptor) -> Option<FontHandle> {
        let Ok(families) = self.families.read() else {
            return None;
        };

        let Some(family) = families.get(&descriptor.name) else {
            return None;
        };

        for font in &family.entries {
            if font.descriptor().weight == descriptor.weight && font.descriptor().style == descriptor.style {
                return Some(FontHandle::new(Arc::clone(font)));
            }
        }

        None
    }

    fn load(&self, descriptor: FontDescriptor, data: Vec<u8>, font_index: u32) -> bool {
        let font = match font_kit::font::Font::from_bytes(Arc::new(data), font_index) {
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

    fn load_defaults(&self) {
        use font_kit::properties::Properties;

        let source = font_kit::source::SystemSource::new();
        self.load_from_font_kit_handle(
            source.select_best_match(
                &[font_kit::family_name::FamilyName::SansSerif],
                &Properties::new()
            ).unwrap(),
            FontDescriptor {
                name: FamilyName::SansSerif,
                style: FontStyle::Normal,
                weight: FontWeight::REGULAR,
            }
        );

        self.load_default_in_background(FontDescriptor {
            name: FamilyName::SansSerif,
            style: FontStyle::Normal,
            weight: FontWeight::BOLD,
        });

        self.load_default_in_background(FontDescriptor {
            name: FamilyName::Emoji,
            style: FontStyle::Normal,
            weight: FontWeight::REGULAR,
        });

        self.load_default_in_background(FontDescriptor {
            name: FamilyName::Serif,
            style: FontStyle::Normal,
            weight: FontWeight::REGULAR,
        });

        self.load_default_in_background(FontDescriptor {
            name: FamilyName::Serif,
            style: FontStyle::Normal,
            weight: FontWeight::BOLD,
        });

        self.load_default_in_background(FontDescriptor {
            name: FamilyName::Monospace,
            style: FontStyle::Normal,
            weight: FontWeight::REGULAR,
        });
    }

    fn load_from_file(&self, load_time: LoadTime, path: &Path, descriptor: FontDescriptor, font_index: u32) -> bool {
        match load_time {
            LoadTime::Background => {
                let provider = self.clone();
                let path = path.to_owned();
                std::thread::spawn(move || {
                    provider.load_from_file_impl(&path, descriptor, font_index);
                });
                true
            }

            LoadTime::Now => {
                self.load_from_file_impl(path, descriptor, font_index)
            }
        }
    }

    fn load_from_system(&self, descriptor: FontDescriptor) -> bool {
        let provider = self.clone();
        let source = font_kit::source::SystemSource::new();
        let desc = descriptor.clone();

        let result = source.select_best_match(
            &[self.convert_family_name(desc.name)],
            &font_kit::properties::Properties {
                weight: convert_font_kit_weight(desc.weight),
                style: convert_font_kit_style(desc.style),
                ..Default::default()
            }
        );

        let Ok(handle) = result else {
            return false;
        };

        provider.load_from_font_kit_handle(
            handle,
            descriptor,
        );

        true
    }
}
