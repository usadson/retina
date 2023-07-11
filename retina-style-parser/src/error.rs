// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use cssparser::{Token, CowRcStr};
use retina_style::Value;

#[derive(Clone, Debug, PartialEq)]
pub enum RetinaStyleParseError<'i> {
    ColorUnknownValue(cssparser::Color),

    ComponentListUnknownKinds(Vec<Value>),

    ExpectedIdentifierAsPropertyValue,

    FloatUnknownKeyword(CowRcStr<'i>),
    FontStyleUnknownKeyword(CowRcStr<'i>),

    ImageUnexpectedFunction(CowRcStr<'i>),
    ImageUnexpectedToken(Token<'i>),

    LengthUnexpectedToken(Token<'i>),
    LengthUnknownIdentifier(CowRcStr<'i>),
    LengthUnknownUnit(CowRcStr<'i>),

    LineStyleExpectedKeyword,
    LineStyleUnexpectedEof,
    LineStyleUnknownKeyword,

    MediaPreludeUnknownType(CowRcStr<'i>),

    UnexpectedEofBasicColorKeyword,

    UnknownAtRule(CowRcStr<'i>),

    UnknownBasicColorKeyword,
    UnknownSelector(Token<'i>),
    UnknownSelectorPseudoClass(CowRcStr<'i>),
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
