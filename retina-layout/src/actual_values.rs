// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use retina_common::Color;
use retina_gfx_font::TextHintingOptions;

#[derive(Clone, Debug, PartialEq)]
pub struct ActualValueMap {
    pub text_color: Color,
    pub background_color: Color,
    pub text_hinting_options: TextHintingOptions,
}
