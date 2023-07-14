// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::{
    DomNode,
    Point2D,
    Rect,
};

/// This is the main [`<svg>`][spec] element.
///
/// [spec]: https://svgwg.org/svg2-draft/struct.html#SVGElement
pub struct SvgLayoutNodeSvg {
    dom_node: DomNode,

    coordinates: Point2D,
    view_box: Rect,
}
