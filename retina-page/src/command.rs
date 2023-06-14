// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use retina_gfx::euclid::Size2D;

/// The browser can send commands to the page that the page must act upon.
#[derive(Clone, Debug, PartialEq)]
pub enum PageCommand {
    ResizeCanvas {
        size: Size2D<u32, u32>,
    },

    OpenDomTreeView,
}
