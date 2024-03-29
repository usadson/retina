// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

pub mod background;
pub mod color;
pub mod cursor;
pub mod display;
pub mod float;
pub mod font;
pub mod image;
pub mod length;
pub mod line_style;
pub mod reference_pixels;
pub mod text;
pub mod text_decoration;
pub mod white_space;

pub type CssDecimal = f64;

pub use self::{
    background::{CssBackgroundRepeat, CssBackgroundRepeatStyle},
    color::{CssColor, CssNamedColor},
    cursor::CssCursor,
    display::{CssDisplay, CssDisplayBox, CssDisplayInside, CssDisplayInternal, CssDisplayOutside},
    float::CssFloatValue,
    font::{
        CssFontFamilyName,
        CssFontKerning,
        CssFontShorthand,
        CssFontStyle,
        CssFontVariantCaps,
        CssFontVariantEastAsian,
        CssFontVariantEastAsianValues,
        CssFontVariantEastAsianWidth,
        CssFontVariantLigatures,
        CssFontVariantPosition,
        CssFontWeight,
        CssGenericFontFamilyName,
    },
    image::CssImage,
    length::CssLength,
    line_style::CssLineStyle,
    reference_pixels::CssReferencePixels,
    text::CssTextTransform,
    text_decoration::{
        CssTextDecoration,
        CssTextDecorationLine,
        CssTextDecorationStyle,
    },
    white_space::CssWhiteSpace,
};

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct CssBorderLonghand {
    pub width: Option<CssLength>,
    pub style: Option<CssLineStyle>,
    pub color: Option<CssColor>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    BackgroundRepeat(CssBackgroundRepeat),
    BorderLonghand(CssBorderLonghand),
    Cursor(CssCursor),
    Color(CssColor),
    ComponentList(ValueComponentList),
    Display(CssDisplay),
    Float(CssFloatValue),
    FontFamily(Vec<CssFontFamilyName>),
    FontKerning(CssFontKerning),
    FontShorthand(CssFontShorthand),
    FontStyle(CssFontStyle),
    FontVariantCaps(CssFontVariantCaps),
    FontVariantEastAsian(CssFontVariantEastAsian),
    FontVariantLigatures(CssFontVariantLigatures),
    FontVariantPosition(CssFontVariantPosition),
    FontWeight(CssFontWeight),
    Image(CssImage),
    Length(CssLength),
    LineStyle(CssLineStyle),
    TextDecoration(CssTextDecoration),
    TextDecorationLine(CssTextDecorationLine),
    TextDecorationStyle(CssTextDecorationStyle),
    TextTransform(CssTextTransform),
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
