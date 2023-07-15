// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

pub(crate) mod backend;
pub(crate) mod descriptor;
pub(crate) mod family;
pub(crate) mod handle;
pub(crate) mod harfbuzz_util;
pub(crate) mod provider;
pub(crate) mod renderer;

pub(crate) use family::FontFamily;

pub use self:: {
    descriptor::{
        FamilyName,
        FontDescriptor,
        FontWeight,
        LigatureMode,
        TextHintingOptions,
    },
    handle::FontHandle,
    provider::FontProvider,
};
