// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use retina_common::Color;

#[derive(Clone, Debug, PartialEq)]
pub struct ActualValueMap {
    pub text_color: Color,
    pub background_color: Color,
}
