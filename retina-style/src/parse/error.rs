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

    AttributeSelectorExpectedIdentifierAsAttributeName(Token<'i>),
    AttributeSelectorUnknownOperatorName(Token<'i>),
}

impl<'i> From<RetinaStyleParseError<'i>> for cssparser::ParseErrorKind<'i, RetinaStyleParseError<'i>> {
    fn from(value: RetinaStyleParseError<'i>) -> Self {
        cssparser::ParseErrorKind::Custom(value)
    }
}
