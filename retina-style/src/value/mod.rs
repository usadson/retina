// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

pub mod color;
pub mod display;
pub mod length;
pub mod reference_pixels;
pub mod white_space;

pub type CssDecimal = f64;

pub use color::{CssColor, CssNamedColor};
pub use display::{CssDisplay, CssDisplayBox, CssDisplayInside, CssDisplayInternal, CssDisplayOutside};
pub use length::CssLength;
pub use reference_pixels::CssReferencePixels;
pub use white_space::CssWhiteSpace;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Color(CssColor),
    Display(CssDisplay),
    Length(CssLength),
    WhiteSpace(CssWhiteSpace),
}
