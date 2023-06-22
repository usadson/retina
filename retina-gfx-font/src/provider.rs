// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use log::error;

use crate::{
    FamilyName,
    Font,
    FontDescriptor,
    FontFamily,
    FontHandle,
};

#[derive(Clone)]
pub struct FontProvider {
    families: Arc<RwLock<HashMap<FamilyName, FontFamily>>>,
}

impl FontProvider {
    pub fn new() -> Self {
        Self {
            families: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn load(&self, descriptor: FontDescriptor, data: Vec<u8>) -> bool {
        let font = match wgpu_glyph::ab_glyph::FontVec::try_from_vec(data) {
            Ok(font) => font,
            Err(e) => {
                error!("Failed to load font ({descriptor:?}): {e}");
                return false;
            }
        };

        let mut families = self.families.write().expect("FontProvider failed to write to `families`");
        families.entry(descriptor.name.clone()).or_insert(Default::default()).entries.push(Arc::new(Font {
            descriptor,
            font,
        }));

        true
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

}
