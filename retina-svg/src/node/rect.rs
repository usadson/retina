// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::{
    Point2D,
    Rect,
    SvgPaint,
    Size2D,
};

/// This [`<rect>`][spec] element specifies a rectangular shape.
///
/// [spec]: https://svgwg.org/svg2-draft/shapes.html#RectElement
pub struct SvgNodeRect {
    coordinates: Point2D,
    size: Size2D,

    fill: SvgPaint,
}
