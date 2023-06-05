// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use retina_style::{ColorValue, CssDisplay, CssWhiteSpace, Property, Value};

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct PropertyMap {
    pub background_color: Option<ColorValue>,
    pub color: Option<ColorValue>,
    pub display: Option<CssDisplay>,
    pub white_space: Option<CssWhiteSpace>,
}

impl PropertyMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn apply_property(&mut self, property: Property, value: Value) -> PropertyMapDidApply {
        match property {
            Property::Invalid => PropertyMapDidApply::NoBecauseOfAnInvalidProperty,

            Property::BackgroundColor => if let Value::Color(color) = value {
                self.color = Some(color);
                PropertyMapDidApply::Yes
            } else {
                PropertyMapDidApply::NoBecauseOfAnInvalidValue
            }

            Property::Color => if let Value::Color(color) = value {
                self.color = Some(color);
                PropertyMapDidApply::Yes
            } else {
                PropertyMapDidApply::NoBecauseOfAnInvalidValue
            }

            Property::Display => if let Value::Display(display) = value {
                self.display = Some(display);
                PropertyMapDidApply::Yes
            } else {
                PropertyMapDidApply::NoBecauseOfAnInvalidValue
            }

            Property::WhiteSpace => if let Value::WhiteSpace(white_space) = value {
                self.white_space = Some(white_space);
                PropertyMapDidApply::Yes
            } else {
                PropertyMapDidApply::NoBecauseOfAnInvalidValue
            }
        }
    }

    pub fn background_color(&self) -> ColorValue {
        self.background_color.unwrap_or(ColorValue::Transparent)
    }

    pub fn color(&self) -> ColorValue {
        // The initial value is implementation-defined.
        self.color.unwrap_or(ColorValue::BasicColorKeyword(retina_style::BasicColorKeyword::Black))
    }

    pub fn display(&self) -> CssDisplay {
        self.display.unwrap_or(CssDisplay::InlineFlow)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PropertyMapDidApply {
    NoBecauseOfAnInvalidProperty,
    NoBecauseOfAnInvalidValue,
    Yes,
}
