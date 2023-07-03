// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

#[derive(Clone, Debug, PartialEq)]
#[derive(thiserror::Error)]
pub enum GuiAttachError {
    #[error("Platform is unsupported")]
    UnsupportedPlatform,

    #[error("Window and display handles don't match the same platform")]
    WindowAndDisplayNotMatchPlatform,
}
