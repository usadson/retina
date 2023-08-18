// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use retina_gfx::{
    euclid::Size2D,
    MouseMoveEvent,
    MouseScrollDelta,
};

/// The browser can send commands to the page that the page must act upon.
#[derive(Clone, Debug, PartialEq)]
pub enum PageCommand {
    Action(PageCommandAction),

    MouseMove {
        event: MouseMoveEvent,
    },

    OpenDomTreeView,

    OpenLayoutTreeView,

    OpenMemoryView,

    /// Show/dump the stylesheets.
    OpenStyleView,

    // Open the URL in this page.
    OpenUrl(String),

    Reload,

    ResizeCanvas {
        size: Size2D<u32, u32>,
    },

    Scroll {
        delta: MouseScrollDelta,
    },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PageCommandAction {
    Click,
    RightClick,
    PageUp,
    PageDown,
    ScrollToTop,
    ScrollToBottom,
}
