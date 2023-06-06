// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use cssparser::Token;

#[derive(Clone, Debug, PartialEq)]
pub enum RetinaStyleParseError<'i> {
    ExpectedIdentifierAsPropertyValue,

    UnexpectedEofBasicColorKeyword,

    UnknownBasicColorKeyword,
    UnknownSelector(Token<'i>),
    UnknownValue,
    UnknownWhiteSpaceKeyword,
}
