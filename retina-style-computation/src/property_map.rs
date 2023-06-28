// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use retina_common::Color;
use retina_style::*;

/// The initial value of the [`border-color`][spec-color].
///
/// [spec-color]: https://drafts.csswg.org/css-backgrounds/#typedef-line-color
pub const INITIAL_BORDER_COLOR: CssColor = CssColor::Color(Color::BLACK);

/// The initial value of the [`border-width`][spec-width].
///
/// [spec-width]: https://drafts.csswg.org/css-backgrounds/#typedef-line-width
pub const INITIAL_BORDER_WIDTH: CssLength = CssLength::Pixels(3 as _);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BorderProperties {
    pub color: CssColor,
    pub style: CssLineStyle,
    pub width: CssLength,
}

impl Default for BorderProperties {
    fn default() -> Self {
        Self {
            color: INITIAL_BORDER_COLOR,
            style: CssLineStyle::None,
            width: INITIAL_BORDER_WIDTH,
        }
    }
}

impl From<CssBorderLonghand> for BorderProperties {
    fn from(value: CssBorderLonghand) -> Self {
        let mut border = Self::default();

        if let Some(color) = value.color { border.color = color }
        if let Some(style) = value.style { border.style = style }
        if let Some(width) = value.width { border.width = width }

        border
    }
}

impl TryFrom<Value> for BorderProperties {
    type Error = ();

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::BorderLonghand(longhand) => Ok(longhand.into()),

            Value::Color(color) => Ok(BorderProperties {
                color,
                ..Default::default()
            }),

            Value::Length(width) => Ok(BorderProperties {
                width,
                ..Default::default()
            }),

            Value::LineStyle(style) => Ok(BorderProperties {
                style,
                ..Default::default()
            }),

            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PropertyMap {
    pub background_color: Option<CssColor>,

    pub border_bottom: BorderProperties,
    pub border_left: BorderProperties,
    pub border_right: BorderProperties,
    pub border_top: BorderProperties,

    pub margin_bottom: Option<CssLength>,
    pub margin_left: Option<CssLength>,
    pub margin_right: Option<CssLength>,
    pub margin_top: Option<CssLength>,

    pub padding_bottom: Option<CssLength>,
    pub padding_left: Option<CssLength>,
    pub padding_right: Option<CssLength>,
    pub padding_top: Option<CssLength>,

    pub color: Option<CssColor>,
    pub display: Option<CssDisplay>,
    pub font_size: Option<CssLength>,
    pub font_family_list: Option<Vec<CssFontFamilyName>>,
    pub font_style: Option<CssFontStyle>,
    pub font_weight: Option<CssFontWeight>,
    pub height: Option<CssLength>,
    pub line_height: Option<CssLength>,
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

            Property::Border => if let Ok(border) = value.try_into() {
                self.border_bottom = border;
                self.border_left = border;
                self.border_right = border;
                self.border_top = border;

                PropertyMapDidApply::Yes
            } else {
                PropertyMapDidApply::NoBecauseOfAnInvalidValue
            }

            Property::BorderBottom => if let Ok(border) = value.try_into() {
                self.border_bottom = border;
                PropertyMapDidApply::Yes
            } else {
                PropertyMapDidApply::NoBecauseOfAnInvalidValue
            }

            Property::BorderBottomColor => if let Value::Color(color) = value {
                self.border_bottom.color = color;
                PropertyMapDidApply::Yes
            } else {
                PropertyMapDidApply::NoBecauseOfAnInvalidValue
            }

            Property::BorderBottomStyle => if let Value::LineStyle(style) = value {
                self.border_bottom.style = style;
                PropertyMapDidApply::Yes
            } else {
                PropertyMapDidApply::NoBecauseOfAnInvalidValue
            }

            Property::BorderBottomWidth => if let Value::Length(width) = value {
                self.border_bottom.width = width;
                PropertyMapDidApply::Yes
            } else {
                PropertyMapDidApply::NoBecauseOfAnInvalidValue
            }

            Property::BorderColor => if let Value::Color(color) = value {
                self.border_bottom.color = color;
                self.border_left.color = color;
                self.border_right.color = color;
                self.border_top.color = color;
                PropertyMapDidApply::Yes
            } else {
                PropertyMapDidApply::NoBecauseOfAnInvalidValue
            }

            Property::BorderLeft => if let Ok(border) = value.try_into() {
                self.border_left = border;
                PropertyMapDidApply::Yes
            } else {
                PropertyMapDidApply::NoBecauseOfAnInvalidValue
            }

            Property::BorderLeftColor => if let Value::Color(color) = value {
                self.border_left.color = color;
                PropertyMapDidApply::Yes
            } else {
                PropertyMapDidApply::NoBecauseOfAnInvalidValue
            }

            Property::BorderLeftStyle => if let Value::LineStyle(style) = value {
                self.border_left.style = style;
                PropertyMapDidApply::Yes
            } else {
                PropertyMapDidApply::NoBecauseOfAnInvalidValue
            }

            Property::BorderLeftWidth => if let Value::Length(width) = value {
                self.border_left.width = width;
                PropertyMapDidApply::Yes
            } else {
                PropertyMapDidApply::NoBecauseOfAnInvalidValue
            }

            Property::BorderRight => if let Ok(border) = value.try_into() {
                self.border_right = border;
                PropertyMapDidApply::Yes
            } else {
                PropertyMapDidApply::NoBecauseOfAnInvalidValue
            }

            Property::BorderRightColor => if let Value::Color(color) = value {
                self.border_right.color = color;
                PropertyMapDidApply::Yes
            } else {
                PropertyMapDidApply::NoBecauseOfAnInvalidValue
            }

            Property::BorderRightStyle => if let Value::LineStyle(style) = value {
                self.border_right.style = style;
                PropertyMapDidApply::Yes
            } else {
                PropertyMapDidApply::NoBecauseOfAnInvalidValue
            }

            Property::BorderRightWidth => if let Value::Length(width) = value {
                self.border_right.width = width;
                PropertyMapDidApply::Yes
            } else {
                PropertyMapDidApply::NoBecauseOfAnInvalidValue
            }

            Property::BorderStyle => if let Value::LineStyle(style) = value {
                self.border_bottom.style = style;
                self.border_left.style = style;
                self.border_right.style = style;
                self.border_top.style = style;
                PropertyMapDidApply::Yes
            } else {
                PropertyMapDidApply::NoBecauseOfAnInvalidValue
            }

            Property::BorderTop => if let Ok(border) = value.try_into() {
                self.border_top = border;
                PropertyMapDidApply::Yes
            } else {
                PropertyMapDidApply::NoBecauseOfAnInvalidValue
            }

            Property::BorderTopColor => if let Value::Color(color) = value {
                self.border_top .color = color;
                PropertyMapDidApply::Yes
            } else {
                PropertyMapDidApply::NoBecauseOfAnInvalidValue
            }

            Property::BorderTopStyle => if let Value::LineStyle(style) = value {
                self.border_top.style = style;
                PropertyMapDidApply::Yes
            } else {
                PropertyMapDidApply::NoBecauseOfAnInvalidValue
            }

            Property::BorderTopWidth => if let Value::Length(width) = value {
                self.border_top.width = width;
                PropertyMapDidApply::Yes
            } else {
                PropertyMapDidApply::NoBecauseOfAnInvalidValue
            }

            Property::BorderWidth => if let Value::Length(width) = value {
                self.border_bottom.width = width;
                self.border_left.width = width;
                self.border_right.width = width;
                self.border_top.width = width;
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

            Property::Font => if let Value::FontShorthand(shorthand) = value {
                self.font_family_list = Some(shorthand.families);
                self.font_size = Some(shorthand.size);
                self.font_style = shorthand.style;
                self.font_weight = shorthand.weight;
                self.line_height = shorthand.line_height;

                PropertyMapDidApply::Yes
            } else {
                PropertyMapDidApply::NoBecauseOfAnInvalidValue
            }

            Property::FontFamily => if let Value::FontFamily(families) = value {
                self.font_family_list = Some(families);
                PropertyMapDidApply::Yes
            } else {
                PropertyMapDidApply::NoBecauseOfAnInvalidValue
            }

            Property::FontStretch => PropertyMapDidApply::NoBecauseOfAnUnsupportedFeature,

            Property::FontStyle => if let Value::FontStyle(style) = value {
                self.font_style = Some(style);
                PropertyMapDidApply::Yes
            } else {
                PropertyMapDidApply::NoBecauseOfAnInvalidValue
            }

            Property::FontWeight => if let Value::FontWeight(weight) = value {
                self.font_weight = Some(weight);
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

            Property::Margin => {
                match value.into_length_percentage_longhand() {
                    Some((bottom, left, right, top)) => {
                        self.margin_bottom = Some(bottom);
                        self.margin_left = Some(left);
                        self.margin_right = Some(right);
                        self.margin_top = Some(top);
                        PropertyMapDidApply::Yes
                    }
                    _ => PropertyMapDidApply::NoBecauseOfAnInvalidValue,
                }
            }

            Property::MarginBottom => if let Value::Length(length) = value {
                self.margin_bottom = Some(length);
                PropertyMapDidApply::Yes
            } else {
                PropertyMapDidApply::NoBecauseOfAnInvalidValue
            }

            Property::MarginLeft => if let Value::Length(length) = value {
                self.margin_left = Some(length);
                PropertyMapDidApply::Yes
            } else {
                PropertyMapDidApply::NoBecauseOfAnInvalidValue
            }

            Property::MarginRight => if let Value::Length(length) = value {
                self.margin_right = Some(length);
                PropertyMapDidApply::Yes
            } else {
                PropertyMapDidApply::NoBecauseOfAnInvalidValue
            }

            Property::MarginTop => if let Value::Length(length) = value {
                self.margin_top = Some(length);
                PropertyMapDidApply::Yes
            } else {
                PropertyMapDidApply::NoBecauseOfAnInvalidValue
            }

            Property::Padding => {
                match value.into_length_percentage_longhand() {
                    Some((bottom, left, right, top)) => {
                        self.padding_bottom = Some(bottom);
                        self.padding_left = Some(left);
                        self.padding_right = Some(right);
                        self.padding_top = Some(top);
                        PropertyMapDidApply::Yes
                    }
                    _ => PropertyMapDidApply::NoBecauseOfAnInvalidValue,
                }
            }

            Property::PaddingBottom => if let Value::Length(length) = value {
                self.padding_bottom = Some(length);
                PropertyMapDidApply::Yes
            } else {
                PropertyMapDidApply::NoBecauseOfAnInvalidValue
            }

            Property::PaddingLeft => if let Value::Length(length) = value {
                self.padding_left = Some(length);
                PropertyMapDidApply::Yes
            } else {
                PropertyMapDidApply::NoBecauseOfAnInvalidValue
            }

            Property::PaddingRight => if let Value::Length(length) = value {
                self.padding_right = Some(length);
                PropertyMapDidApply::Yes
            } else {
                PropertyMapDidApply::NoBecauseOfAnInvalidValue
            }

            Property::PaddingTop => if let Value::Length(length) = value {
                self.padding_top = Some(length);
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

    pub fn display(&self) -> CssDisplay {
        self.display.unwrap_or(CssDisplay::Normal {
            inside: CssDisplayInside::Flow,
            outside: CssDisplayOutside::Inline,
            is_list_item: false
        })
    }

    pub fn font_size(&self) -> CssLength {
        self.font_size.unwrap_or(CssLength::Pixels(16.0))
    }

    pub fn font_weight(&self) -> CssFontWeight {
        self.font_weight.unwrap_or(CssFontWeight::Absolute(400.0))
    }

    pub fn has_same_font_properties(&self, other: &PropertyMap) -> bool {
        self.font_size == other.font_size
            && self.font_family_list == other.font_family_list
            && self.font_style == other.font_style
            && self.font_weight == other.font_weight
    }

    pub fn height(&self) -> CssLength {
        self.height.unwrap_or(CssLength::Auto)
    }

    pub fn margin_bottom(&self) -> CssLength {
        self.margin_bottom.unwrap_or(CssLength::Pixels(0.0))
    }

    pub fn margin_left(&self) -> CssLength {
        self.margin_left.unwrap_or(CssLength::Pixels(0.0))
    }

    pub fn margin_right(&self) -> CssLength {
        self.margin_right.unwrap_or(CssLength::Pixels(0.0))
    }

    pub fn margin_top(&self) -> CssLength {
        self.margin_top.unwrap_or(CssLength::Pixels(0.0))
    }

    pub fn padding_bottom(&self) -> CssLength {
        self.padding_bottom.unwrap_or(CssLength::Pixels(0.0))
    }

    pub fn padding_left(&self) -> CssLength {
        self.padding_left.unwrap_or(CssLength::Pixels(0.0))
    }

    pub fn padding_right(&self) -> CssLength {
        self.padding_right.unwrap_or(CssLength::Pixels(0.0))
    }

    pub fn padding_top(&self) -> CssLength {
        self.padding_top.unwrap_or(CssLength::Pixels(0.0))
    }

    pub fn width(&self) -> CssLength {
        self.width.unwrap_or(CssLength::Auto)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PropertyMapDidApply {
    NoBecauseOfAnInvalidProperty,
    NoBecauseOfAnInvalidValue,
    NoBecauseOfAnUnsupportedFeature,
    Yes,
}
