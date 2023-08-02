// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::{
    fmt::Debug,
    ops::Deref,
    sync::Arc,
};

use retina_gfx::Font;

#[derive(Clone)]
pub struct FontHandle {
    pub(crate) font: Arc<dyn Font>,
}

impl FontHandle {
    #[inline]
    pub const fn new(font: Arc<dyn Font>) -> Self {
        Self {
            font,
        }
    }
}

impl Debug for FontHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FontHandle")
            .finish_non_exhaustive()
    }
}

impl Deref for FontHandle {
    type Target = dyn Font;

    fn deref(&self) -> &Self::Target {
        self.font.deref()
    }
}

impl AsRef<dyn retina_gfx::Font + 'static> for FontHandle {
    fn as_ref(&self) -> &(dyn retina_gfx::Font + 'static) {
        &*self.font
    }
}

impl PartialEq for FontHandle {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.font, &other.font)
    }
}
