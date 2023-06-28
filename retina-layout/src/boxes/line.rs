// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use retina_style::CssDecimal;

/// The rectangular area that contains the boxes that form a line is called a
/// line box.
#[derive(Clone, Debug)]
pub struct LineBox {
    pub(crate) height: CssDecimal,
}

impl LineBox {
    pub fn new() -> Self {
        Self {
            height: Default::default(),
        }
    }
}
