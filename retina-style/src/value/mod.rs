// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

pub mod color;
pub mod display;
pub mod length;
pub mod reference_pixels;
pub mod white_space;

pub type CssDecimal = f64;

pub use color::{BasicColorKeyword, ColorValue};
pub use display::CssDisplay;
pub use length::CssLength;
pub use reference_pixels::CssReferencePixels;
pub use white_space::CssWhiteSpace;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Value {
    Color(ColorValue),
    Display(CssDisplay),
    Length(CssLength),
    WhiteSpace(CssWhiteSpace),
}
