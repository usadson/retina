// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::{
    FamilyName,
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

    pub fn load(&mut self, descriptor: FontDescriptor, data: &[u8]) -> bool {
        false
    }

    pub fn get(&mut self, descriptor: FontDescriptor) -> Option<FontHandle> {
        None
    }

}
