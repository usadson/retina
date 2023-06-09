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

    AttributeSelectorExpectedIdentifierAsAttributeName(Token<'i>),
    AttributeSelectorUnknownOperatorName(Token<'i>),
}

impl<'i> From<RetinaStyleParseError<'i>> for cssparser::ParseErrorKind<'i, RetinaStyleParseError<'i>> {
    fn from(value: RetinaStyleParseError<'i>) -> Self {
        cssparser::ParseErrorKind::Custom(value)
    }
}
