// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::LayoutBox;

/// The rectangular area that contains the boxes that form a line is called a
/// line box.
#[derive(Clone, Debug)]
pub struct LineBox {
    pub(crate) boxes: Vec<LayoutBox>,
}

impl LineBox {
    pub fn new(boxes: Vec<LayoutBox>) -> Self {
        Self { boxes }
    }

    pub fn boxes(&self) -> &[LayoutBox] {
        &self.boxes
    }
}
