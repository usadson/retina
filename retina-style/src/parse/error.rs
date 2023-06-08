// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use cssparser::{Token, CowRcStr};

#[derive(Clone, Debug, PartialEq)]
pub enum RetinaStyleParseError<'i> {
    ExpectedIdentifierAsPropertyValue,

    LengthUnexpectedToken(Token<'i>),
    LengthUnknownIdentifier(CowRcStr<'i>),
    LengthUnknownUnit(CowRcStr<'i>),

    UnexpectedEofBasicColorKeyword,

    UnknownBasicColorKeyword,
    UnknownSelector(Token<'i>),
    UnknownValue(Option<Token<'i>>),
    UnknownWhiteSpaceKeyword,
}
