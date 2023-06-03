// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

/// The page sends messages to the browser to inform it of it's status.
#[derive(Clone, Debug, PartialEq)]
pub enum PageMessage {
    Progress {
        progress: PageProgress,
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
