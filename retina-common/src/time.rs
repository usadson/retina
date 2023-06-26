// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

/// Specifies when a certain item should be loaded.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LoadTime {
    /// Load the item in the background, minimally blocking the call.
    Background,

    /// Load the item now, blocking the call.
    Now,
}
