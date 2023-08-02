// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

pub mod descriptor;
pub(crate) mod handle;
pub(crate) mod provider;

pub use self:: {
    descriptor::*,
    handle::FontHandle,
    provider::{
        FontProvider,
        FontProviderBackend,
    },
};
