// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::{sync::Arc, ops::Deref};

use crate::Font;

#[derive(Clone, Debug)]
pub struct FontHandle {
    pub(crate) font: Arc<Font>,
}

impl Deref for FontHandle {
    type Target = Font;

    fn deref(&self) -> &Self::Target {
        &self.font
    }
}

impl PartialEq for FontHandle {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.font, &other.font)
    }
}
