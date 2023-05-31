// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use retina_style::{ColorValue, Property, Value};

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct PropertyMap {
    pub color: Option<ColorValue>,
}

impl PropertyMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn apply_property(&mut self, property: Property, value: Value) -> PropertyMapDidApply {
        match property {
            Property::Invalid => PropertyMapDidApply::NoBecauseOfAnInvalidProperty,

            Property::Color => match value {
                Value::Color(color) => {
                    self.color = Some(color);
                    PropertyMapDidApply::Yes
                }

                #[allow(unreachable_patterns)] // TODO
                _ => PropertyMapDidApply::NoBecauseOfAnInvalidValue
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PropertyMapDidApply {
    NoBecauseOfAnInvalidProperty,
    NoBecauseOfAnInvalidValue,
    Yes,
}
