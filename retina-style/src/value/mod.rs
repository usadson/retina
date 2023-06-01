// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

pub mod color;
pub mod display;
pub mod white_space;

pub use color::{BasicColorKeyword, ColorValue};
pub use display::CssDisplay;
pub use white_space::CssWhiteSpace;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {
    Color(ColorValue),
    Display(CssDisplay),
    WhiteSpace(CssWhiteSpace),
}
