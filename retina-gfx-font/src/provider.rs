// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::{
    path::Path,
    sync::Arc,
};

use retina_common::LoadTime;

use crate::{
    FontDescriptor,
    FontHandle,
};

#[derive(Clone)]
pub struct FontProvider {
    backend: Arc<dyn FontProviderBackend>,
}

impl FontProvider {
    #[inline]
    pub const fn new(backend: Arc<dyn FontProviderBackend>) -> Self {
        Self {
            backend,
        }
    }

    #[inline]
    pub fn get(&self, descriptor: &FontDescriptor) -> Option<FontHandle> {
        self.backend.get(descriptor)
    }

    #[inline]
    pub fn load(&self, descriptor: FontDescriptor, data: Vec<u8>) -> bool {
        self.backend.load(descriptor, data)
    }

    #[inline]
    pub fn load_defaults(&self) {
        self.backend.load_defaults();
    }

    #[inline]
    pub fn load_from_file(&self, load_time: LoadTime, path: &Path, descriptor: FontDescriptor) -> bool {
        self.backend.load_from_file(load_time, path, descriptor)
    }

    #[inline]
    pub fn load_from_system(&self, descriptor: FontDescriptor) -> bool {
        self.backend.load_from_system(descriptor)
    }
}

pub trait FontProviderBackend: Send + Sync {
    fn get(&self, descriptor: &FontDescriptor) -> Option<FontHandle>;
    fn load(&self, descriptor: FontDescriptor, data: Vec<u8>) -> bool;
    fn load_defaults(&self);
    fn load_from_system(&self, descriptor: FontDescriptor) -> bool;
    fn load_from_file(&self, load_time: LoadTime, path: &Path, descriptor: FontDescriptor) -> bool;
}
