// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::FetchResponse;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InternalError {
    SynchronizationFault,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    NetworkError(NetworkError),
    InternalError(InternalError),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NetworkError {
    Generic,
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
