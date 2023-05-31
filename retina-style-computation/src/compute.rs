// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::collections::HashMap;

use retina_style::{
    Property,
    Value,
};

#[derive(Clone, Debug, Default)]
pub struct ComputedStyle {
    pub(crate) values: HashMap<Property, Value>,
}

impl ComputedStyle {
    pub fn new() -> Self {
        Self::default()
    }
}
