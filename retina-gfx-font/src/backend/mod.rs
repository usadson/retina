// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

//! This crate can use different backends for font loading and rasterization,
//! and this module contains the glue code between the backend and retina-gfx.

mod font_kit;

pub use font_kit::FontKitFont;
