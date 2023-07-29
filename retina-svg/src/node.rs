// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod svg;
mod rect;

pub use self::{
    svg::SvgNodeSvg,
    rect::SvgNodeRect,
};

pub struct SvgNode {
    kind: SvgNodeKind,
}

pub enum SvgNodeKind {
    /// The `<svg>` element.
    Svg(SvgNodeSvg),

    /// The `<rect>` element.
    Rect(SvgNodeRect),
}
