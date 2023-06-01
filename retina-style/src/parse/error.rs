// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum RetinaStyleParseError {
    ExpectedIdentifierAsPropertyValue,

    UnexpectedEofBasicColorKeyword,

    UnknownBasicColorKeyword,
    UnknownSelector,
    UnknownValue,
    UnknownWhiteSpaceKeyword,
}
