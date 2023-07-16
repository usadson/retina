// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

#[derive(Debug)]
#[derive(thiserror::Error)]
pub enum WoffError {
    #[error("invalid signature (expected: {expected}, found: {found})")]
    InvalidSignature {
        expected: u32,
        found: u32,
    },

    #[error("I/O error whilst reading header: {0}")]
    IoErrorWhilstReadingHeader(#[from] std::io::Error),
}

pub type WoffResult<T> = Result<T, WoffError>;
