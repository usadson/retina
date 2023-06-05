// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::CssDecimal;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum CssLength {
    Auto,
    Pixels(CssDecimal),
}
