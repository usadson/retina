// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use retina_gfx::{euclid::Size2D, MouseScrollDelta};

/// The browser can send commands to the page that the page must act upon.
#[derive(Clone, Debug, PartialEq)]
pub enum PageCommand {
    OpenDomTreeView,

    OpenLayoutTreeView,

    Reload,

    ResizeCanvas {
        size: Size2D<u32, u32>,
    },

    Scroll {
        delta: MouseScrollDelta,
    },
}
