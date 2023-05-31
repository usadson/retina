// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

pub mod color;
pub mod display;

pub use color::{BasicColorKeyword, ColorValue};
pub use display::CssDisplay;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {
    Color(ColorValue),
    Display(CssDisplay),
}
