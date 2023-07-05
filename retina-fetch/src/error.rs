// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::fmt::Display;

use crate::FetchResponse;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InternalError {
    HyperError,
    SynchronizationFault,
}

impl Display for InternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        core::fmt::Debug::fmt(&self, f)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    NetworkError(NetworkError),
    InternalError(InternalError),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        core::fmt::Debug::fmt(&self, f)
    }
}

impl std::error::Error for Error {
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NetworkError {
    Generic,

    /// A `file://` URL was not found.
    LocalFileNotFound,
}

impl Display for NetworkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        core::fmt::Debug::fmt(&self, f)
    }
}

impl From<InternalError> for FetchResponse {
    fn from(value: InternalError) -> Self {
        Err(value.into())
    }
}

impl From<hyper::Error> for Error {
    fn from(value: hyper::Error) -> Self {
        // TODO
        _ = value;
        Error::NetworkError(NetworkError::Generic)
    }
}

impl From<InternalError> for Error {
    fn from(value: InternalError) -> Self {
        Self::InternalError(value)
    }
}

impl From<Error> for FetchResponse {
    fn from(value: Error) -> Self {
        Err(value)
    }
}
