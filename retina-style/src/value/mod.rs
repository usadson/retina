// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

pub mod color;
pub mod display;
pub mod float;
pub mod font;
pub mod length;
pub mod line_style;
pub mod reference_pixels;
pub mod white_space;

pub type CssDecimal = f64;

pub use color::{CssColor, CssNamedColor};
pub use display::{CssDisplay, CssDisplayBox, CssDisplayInside, CssDisplayInternal, CssDisplayOutside};
pub use float::CssFloatValue;
pub use font::{CssFontFamilyName, CssFontShorthand, CssFontStyle, CssFontWeight, CssGenericFontFamilyName};
pub use length::CssLength;
pub use line_style::CssLineStyle;
pub use reference_pixels::CssReferencePixels;
pub use white_space::CssWhiteSpace;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct CssBorderLonghand {
    pub width: Option<CssLength>,
    pub style: Option<CssLineStyle>,
    pub color: Option<CssColor>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    BorderLonghand(CssBorderLonghand),
    Color(CssColor),
    ComponentList(ValueComponentList),
    Display(CssDisplay),
    Float(CssFloatValue),
    FontFamily(Vec<CssFontFamilyName>),
    FontShorthand(CssFontShorthand),
    FontStyle(CssFontStyle),
    FontWeight(CssFontWeight),
    Length(CssLength),
    LineStyle(CssLineStyle),
    WhiteSpace(CssWhiteSpace),
}

impl Value {
    pub fn into_length_percentage_longhand(self) -> Option<(CssLength, CssLength, CssLength, CssLength)> {
        match self {
            Self::ComponentList(ValueComponentList::TwoLengths([vertical, horizontal])) =>
                Some((vertical, horizontal, horizontal, vertical)),

            Self::ComponentList(ValueComponentList::ThreeLengths([top, horizontal, bottom])) =>
                Some((bottom, horizontal, horizontal, top)),

            Self::ComponentList(ValueComponentList::FourLengths([top, right, bottom, left])) =>
                Some((bottom, left, right, top)),

            Self::Length(length) =>
                Some((length, length, length, length)),

            _ => None
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ValueComponentList {
    TwoColors([CssColor; 2]),
    TwoLengths([CssLength; 2]),

    ThreeColors([CssColor; 3]),
    ThreeLengths([CssLength; 3]),

    FourColors([CssColor; 4]),
    FourLengths([CssLength; 4]),
}
