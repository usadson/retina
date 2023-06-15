// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::CssDecimal;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum CssLength {
    Auto,

    /// The `em` unit.
    ///
    /// <https://drafts.csswg.org/css-values-4/#em>
    FontSize(CssDecimal),

    /// The `rem` unit.
    ///
    /// <https://drafts.csswg.org/css-values-4/#rem>
    FontSizeOfRootElement(CssDecimal),

    Pixels(CssDecimal),

    UaDefaultViewportHeightPercentage(CssDecimal),
    UaDefaultViewportWidthPercentage(CssDecimal),
}
