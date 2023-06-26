// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod color;
mod dumpable;
mod time;
mod transparent_wrapper;

pub use color::Color;
pub use dumpable::DumpableNode;
pub use time::LoadTime;
pub use transparent_wrapper::TransparentWrapper;

/// An atomic UTF-8 string tendril (shared buffer string).
pub type StrTendril = tendril::Tendril<tendril::fmt::UTF8, tendril::Atomic>;
