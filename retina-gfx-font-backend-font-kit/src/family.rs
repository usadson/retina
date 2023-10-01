// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::sync::Arc;

use retina_gfx::font::Font;

#[derive(Default)]
pub(crate) struct FontFamily {
    pub(crate) entries: Vec<Arc<dyn Font>>,
}
