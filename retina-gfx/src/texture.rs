// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use euclid::default::Size2D;

#[derive(Clone, Debug)]
pub struct Texture {
    size: Size2D<u32>,
    id: TextureId,
    view_id: TextureViewId,
}

impl Texture {
    pub fn new(size: Size2D<u32>, id: TextureId, view_id: TextureViewId) -> Self {
        Self { size, id, view_id }
    }

    pub fn id(&self) -> TextureId {
        self.id
    }

    pub fn view(&self) -> TextureViewId {
        self.view_id
    }

    pub fn width(&self) -> u32 {
        self.size.width
    }

    pub fn height(&self) -> u32 {
        self.size.height
    }

    pub fn size(&self) -> Size2D<u32> {
        self.size
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TextureId(pub usize);

pub struct TextureViewId(pub usize);
