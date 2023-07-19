// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

//! This crate is the layout engine of `retina`.
//!
//! # References
//! ## Box Model
//! * [CSS Box Model Module Level 3](https://www.w3.org/TR/css-box-3/)
//! * [CSS Level 2 Revision 2 (CSS 2.2) - Box Model](https://www.w3.org/TR/CSS22/box.html)
//!
//! ## Visual Formatting Model
//! * [CSS Level 2 Revision 2 (CSS 2.2) - Visual Formatting Model (visuren)](https://www.w3.org/TR/CSS22/visuren.html)
//! * [CSS Display Module Level 3](https://www.w3.org/TR/css-display-3/)
//! * [CSS Shapes Module Level 1](https://www.w3.org/TR/css-shapes-1/)
//! * [CSS Writing Modes Level 3](https://www.w3.org/TR/css-writing-modes-3/)

mod actual_values;
mod boxes;
mod formatting_context;
mod generate;
pub(crate) mod text;

pub use self::{
    actual_values::ActualValueMap,
    boxes::{
        LayoutBox,
        LayoutBoxDimensions,
        LayoutBoxKind,
        LayoutEdge,
    },
    generate::LayoutGenerator,
};

pub(crate) type DomNode = retina_dom::Node;
