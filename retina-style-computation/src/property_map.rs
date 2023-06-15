// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use retina_style::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PropertyMap {
    pub background_color: Option<CssColor>,
    pub color: Option<CssColor>,
    pub display: Option<CssDisplay>,
    pub font_size: Option<CssLength>,
    pub height: Option<CssLength>,
    pub width: Option<CssLength>,
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
                self.background_color = Some(color);
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

            Property::FontSize => if let Value::Length(length) = value {
                self.font_size = Some(length);
                PropertyMapDidApply::Yes
            } else {
                PropertyMapDidApply::NoBecauseOfAnInvalidValue
            }

            Property::Height => if let Value::Length(length) = value {
                self.height = Some(length);
                PropertyMapDidApply::Yes
            } else {
                PropertyMapDidApply::NoBecauseOfAnInvalidValue
            }

            Property::Width => if let Value::Length(length) = value {
                self.width = Some(length);
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

    pub fn background_color(&self) -> CssColor {
        self.background_color.unwrap_or(CssNamedColor::TRANSPARENT)
    }

    pub fn color(&self) -> CssColor {
        // The initial value is implementation-defined.
        self.color.unwrap_or(CssNamedColor::BLACK)
    }

    pub fn height(&self) -> CssLength {
        self.height.unwrap_or(CssLength::Auto)
    }

    pub fn width(&self) -> CssLength {
        self.width.unwrap_or(CssLength::Auto)
    }

    pub fn display(&self) -> CssDisplay {
        self.display.unwrap_or(CssDisplay::Normal {
            inside: CssDisplayInside::Flow,
            outside: CssDisplayOutside::Block,
            is_list_item: false
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PropertyMapDidApply {
    NoBecauseOfAnInvalidProperty,
    NoBecauseOfAnInvalidValue,
    Yes,
}
