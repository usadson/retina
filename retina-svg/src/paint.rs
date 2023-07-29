// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use retina_common::Color;

/// The [`paint`][spec] specification describes a way of putting color onto
/// a specific SVG element.
///
/// [spec]: https://svgwg.org/svg2-draft/painting.html#SpecifyingPaint
pub enum SvgPaint {
    /// No paint is applied
    None,

    /// A solid color.
    ///
    /// **TODO:** this should be a `CssColor`.
    Color(Color),
}
