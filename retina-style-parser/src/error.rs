// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::borrow::Cow;

use cssparser::{Token, CowRcStr, BasicParseErrorKind, ParseErrorKind};
use log::warn;
use retina_style::Value;

use crate::ParseError;

/// If the line is too long - which is often the case for minified CSS files -
/// we should only log so many characters
const MAX_LINE_LENGTH_FOR_ERROR_LOGGING: usize = 80;

const REDUCE_CONTEXT_PREFIX: usize = 20;
const REDUCE_CONTEXT_SUFFIX: usize = 20;

#[derive(Clone, Debug, PartialEq)]
pub enum RetinaStyleParseError<'i> {
    ColorUnknownValue(cssparser::Color),

    ComponentListUnknownKinds(Vec<Value>),

    ExpectedIdentifierAsPropertyValue,

    FloatUnknownKeyword(CowRcStr<'i>),

    AtFontFaceInvalidProperty(CowRcStr<'i>),

    FontKerningUnknownKeyword(CowRcStr<'i>),
    FontSizeUnknownKeyword(CowRcStr<'i>),
    FontStyleUnknownKeyword(CowRcStr<'i>),
    FontVariantLigaturesUnknownKeyword(CowRcStr<'i>),

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
    UnknownKeyword(CowRcStr<'i>),
    UnknownProperty(CowRcStr<'i>),
    UnknownSelector(Token<'i>),
    UnknownSelectorPseudoClass(CowRcStr<'i>),
    UnknownValue(Option<Token<'i>>),
    UnknownWhiteSpaceKeyword,

    WideKeywordsNotYetSupported,

    AttributeSelectorExpectedIdentifierAsAttributeName(Token<'i>),
    AttributeSelectorUnknownOperatorName(Token<'i>),
}

impl<'i> From<RetinaStyleParseError<'i>> for cssparser::ParseErrorKind<'i, RetinaStyleParseError<'i>> {
    fn from(value: RetinaStyleParseError<'i>) -> Self {
        cssparser::ParseErrorKind::Custom(value)
    }
}

pub fn display_parse_error<'i, 't>(
    parser: &cssparser::Parser<'i, 't>,
    constituent: &str,
    error: (ParseError<'i>, &str),
) {
    let (error, line) = error;

    warn!("Error whilst parsing {constituent} at line {}, column {}",
        error.location.line,
        error.location.column
    );

    // The `line` parameter often has indentation removed, meaning our
    // column offset is incorrect. Therefore, we might be in luck if the
    // parser is at the same line as it was when it encountered the error,
    // because we can use the parser to get the whole line including
    // leading whitespace (indentation).
    let mut line = if error.location.line == parser.current_source_location().line {
        let line = parser.current_line();
        line
    } else {
        line
    };

    let mut space_count = if error.location.column == 0 {
        0
    } else {
        (error.location.column - 1) as usize
    };

    let mut caret_count = match &error.kind {
        ParseErrorKind::Basic(BasicParseErrorKind::UnexpectedToken(token)) => token.length(),
        _ => 1,
    };

    if let Some((new_space_count, new_caret_count)) = improve_caret_location(space_count, caret_count, line, &error) {
        space_count = new_space_count;
        caret_count = new_caret_count;
    }

    if line.len() > MAX_LINE_LENGTH_FOR_ERROR_LOGGING {
        (space_count, line) = reduce_line_length(space_count, caret_count, line);
    }

    let message = translate_error_to_friendly_error_description(&error);

    warn!("{line}");
    warn!("{spaces}^{tildes} {message}",
        spaces = " ".repeat(space_count),
        tildes = "~".repeat(caret_count.saturating_sub(1)),
    );
    warn!("");
}

fn improve_caret_location<'i>(
    space_count: usize,
    caret_count: usize,
    line: &str,
    error: &ParseError<'i>,
) -> Option<(usize, usize)> {
    let ParseErrorKind::Custom(custom) = &error.kind else { return None };

    _ = line;

    match custom {
        RetinaStyleParseError::UnknownValue(Some(Token::Function(function))) => {
            let len = function.length();

            Some((space_count - len - 1, caret_count + len - 1))
        }

        RetinaStyleParseError::UnknownValue(Some(token)) => {
            let len = token.length();

            Some((space_count - len, caret_count + len - 1))
        }

        _ => None
    }
}

fn reduce_line_length(
    space_count: usize,
    caret_count: usize,
    line: &str,
) -> (usize, &str) {
    if space_count == 0 {
        return (space_count, line);
    }

    let prefix = space_count.checked_sub(REDUCE_CONTEXT_PREFIX).unwrap_or(space_count);
    let new_space_count = REDUCE_CONTEXT_PREFIX;
    let range = prefix..space_count + caret_count + REDUCE_CONTEXT_SUFFIX;

    if let Some(line) = line.get(range) {
        (new_space_count, line)
    } else {
        (space_count, line)
    }
}

fn translate_error_to_friendly_error_description<'i>(error: &ParseError<'i>) -> Cow<'static, str> {
    match &error.kind {
        ParseErrorKind::Custom(RetinaStyleParseError::WideKeywordsNotYetSupported) => "CSS-wide keywords are not yet supported.".into(),

        kind => format!("{kind:?}").into(),
    }
}

trait Length {
    fn length(&self) -> usize;
}

impl Length for str {
    #[inline]
    fn length(&self) -> usize {
        self.len()
    }
}

impl<'a> Length for CowRcStr<'a> {
    fn length(&self) -> usize {
        self.as_ref().length()
    }
}

impl<'a> Length for Token<'a> {
    fn length(&self) -> usize {
        match self {
            Self::AtKeyword(value) => value.length(),
            Self::BadString(value) => value.length(),
            Self::BadUrl(value) => value.length(),
            Self::CDC => 3,
            Self::CDO => 3,
            Self::CloseCurlyBracket => 1,
            Self::CloseParenthesis => 1,
            Self::CloseSquareBracket => 1,
            Self::Colon => 1,
            Self::Comma => 1,
            Self::Comment(value) => value.length(),
            Self::CurlyBracketBlock => 1,
            Self::DashMatch => 2,
            Self::Delim(_ch) => 1,
            Self::Dimension { unit, .. } => unit.length(), // TODO
            Self::Function(value) => value.length(),
            Self::Hash(value) => value.length(),
            Self::IDHash(value) => value.length(),
            Self::Ident(value) => value.length(),
            Self::IncludeMatch => 2,
            Self::Number { .. } => 1, // TODO
            Self::ParenthesisBlock => 1,
            Self::Percentage { .. } => 1, // TODO
            Self::PrefixMatch => 2,
            Self::QuotedString(value) => value.length(),
            Self::Semicolon => 1,
            Self::SquareBracketBlock => 1,
            Self::SubstringMatch => 2,
            Self::SuffixMatch => 2,
            Self::UnquotedUrl(value) => value.length(),
            Self::WhiteSpace(value) => value.length(),
        }
    }
}
