// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod constants;
mod error;
mod header;

use std::io::Read;

pub use self::{
    error::{
        WoffError,
        WoffResult,
    },
    header::Woff2Header,
};

/// This crate is a [WOFF2][spec] parser.
///
/// [spec]: https://www.w3.org/TR/WOFF2

pub fn decompress<R>(mut reader: R) -> WoffResult<()>
        where R: Read {
    let header = Woff2Header::parse(&mut reader)?;

    Ok(())
}
