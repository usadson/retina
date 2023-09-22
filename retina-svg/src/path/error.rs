// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use nom::error::{ParseError, FromExternalError};

pub type IResult<I, O> = nom::IResult<I, O, PathError<I>>;

#[derive(Debug, PartialEq)]
pub enum PathError<I> {
    Nom(nom::error::Error<I>)
}

impl<I> From<nom::error::Error<I>> for PathError<I> {
    fn from(value: nom::error::Error<I>) -> Self {
        Self::Nom(value)
    }
}

impl<I> ParseError<I> for PathError<I> {
    fn from_error_kind(input: I, kind: nom::error::ErrorKind) -> Self {
        Self::Nom(nom::error::Error::from_error_kind(input, kind))
    }

    fn append(input: I, kind: nom::error::ErrorKind, other: Self) -> Self {
        _ = input;
        _ = kind;
        other
    }
}

impl<I, E> FromExternalError<I, E> for PathError<I> {
    fn from_external_error(input: I, kind: nom::error::ErrorKind, e: E) -> Self {
        _ = e;
        Self::from_error_kind(input, kind)
    }
}
