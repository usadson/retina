// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use retina_gfx::euclid::Size2D;

/// The page sends messages to the browser to inform it of it's status.
#[derive(Debug)]
pub enum PageMessage {
    PaintReceived {
        texture_view: wgpu::TextureView,
        texture_size: Size2D<u32, u32>,
    },

    Progress {
        progress: PageProgress,
    },

    /// The title of the page.
    Title {
        title: String,
    },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PageProgress {
    Initial,

    Fetched,

    ParsedHtml,
    ParsedCss,

    LayoutGenerated,
    LayoutPerformed,

    Painted,

    Ready,
}
