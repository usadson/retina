// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::{sync::Arc, ops::Deref};

use crate::WgpuFont;

#[derive(Clone, Debug)]
pub struct FontHandle {
    pub(crate) font: Arc<WgpuFont>,
}

impl Deref for FontHandle {
    type Target = WgpuFont;

    fn deref(&self) -> &Self::Target {
        &self.font
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
