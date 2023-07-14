// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod svg;
mod rect;

pub use self::{
    svg::SvgLayoutNodeSvg,
    rect::SvgLayoutNodeRect,
};

pub struct SvgLayoutNode {
    kind: SvgLayoutNodeKind,
}

pub enum SvgLayoutNodeKind {
    /// The `<svg>` element.
    Svg(SvgLayoutNodeSvg),

    /// The `<rect>` element.
    Rect(SvgLayoutNodeRect),
}
