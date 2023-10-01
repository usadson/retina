// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use retina_common::Color;
use retina_gfx::font::TextHintingOptions;

use crate::LayoutBoxDimensions;

#[derive(Clone, Debug, PartialEq)]
pub struct ActualValueMap {
    pub text_color: Color,
    pub background_color: Color,
    pub text_hinting_options: TextHintingOptions,
    pub dimensions: LayoutBoxDimensions,
}
