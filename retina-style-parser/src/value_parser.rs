// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use cssparser::{
    Parser,
    ParseErrorKind,
    Token, Color,
};

use strum::IntoEnumIterator;

use retina_style::*;

use crate::{ParseError, RetinaStyleParseError, util::convert_color};

pub(crate) fn parse_display<'i, 't>(
    input: &mut Parser<'i, 't>
) -> Result<CssDisplay, ParseError<'i>> {
    let token = input.next()
        .cloned()
        .map_err(|_| input.new_custom_error(RetinaStyleParseError::UnexpectedEofBasicColorKeyword))?;

    let Token::Ident(ident) = token else {
        return Err(input.new_custom_error(RetinaStyleParseError::ExpectedIdentifierAsPropertyValue));
    };

    Ok(match ident.as_ref() {
        "block" => CssDisplay::Normal {
            inside: CssDisplayInside::Flow,
            outside: CssDisplayOutside::Block,
            is_list_item: false,
        },
        "inline" => CssDisplay::Normal {
            inside: CssDisplayInside::Flow,
            outside: CssDisplayOutside::Inline,
            is_list_item: false,
        },
        "inline-block" => CssDisplay::Normal {
            inside: CssDisplayInside::FlowRoot,
            outside: CssDisplayOutside::Inline,
            is_list_item: false,
        },
        "none" => CssDisplay::Box(CssDisplayBox::None),
        _ => return Err(input.new_custom_error(RetinaStyleParseError::UnknownBasicColorKeyword)),
    })
}

pub(crate) fn parse_length<'i, 't>(
    input: &mut Parser<'i, 't>
) -> Result<CssLength, ParseError<'i>> {
    let token_location = input.current_source_location();
    let token = input.next()
        .cloned()
        .map_err(|_| input.new_custom_error(RetinaStyleParseError::UnexpectedEofBasicColorKeyword))?;

    match token {
        Token::Ident(ident) => {
            if ident == "auto" {
                Ok(CssLength::Auto)
            } else {
                Err(ParseError {
                    kind: ParseErrorKind::Custom(RetinaStyleParseError::LengthUnknownIdentifier(ident)),
                    location: token_location,
                })
            }
        }

        Token::Dimension { value, unit, .. } => {
            match unit.as_ref() {
                "px" => Ok(CssLength::Pixels(value as _)),
                _ => Err(ParseError {
                    kind: ParseErrorKind::Custom(RetinaStyleParseError::LengthUnknownUnit(unit)),
                    location: token_location,
                }),
            }
        }

        Token::Number { int_value, .. } if int_value == Some(0) => {
            Ok(CssLength::Pixels(0.0))
        }

        _ => Err(ParseError {
            kind: ParseErrorKind::Custom(RetinaStyleParseError::LengthUnexpectedToken(token)),
            location: token_location,
        })
    }
}

pub(crate) fn parse_value<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Value, ParseError<'i>> {
    if let Ok(color) = input.try_parse(Color::parse) {
        return Ok(Value::Color(convert_color(color).unwrap()));
    }

    if let Ok(display) = input.try_parse(parse_display) {
        return Ok(Value::Display(display));
    }

    if let Ok(length) = input.try_parse(parse_length) {
        return Ok(Value::Length(length));
    }

    if let Ok(white_space) = input.try_parse(parse_white_space) {
        return Ok(Value::WhiteSpace(white_space));
    }

    let token = input.next().ok().cloned();
    Err(input.new_custom_error(RetinaStyleParseError::UnknownValue(token)))
}
pub(crate) fn parse_white_space<'i, 't>(
    input: &mut Parser<'i, 't>
) -> Result<CssWhiteSpace, ParseError<'i>> {
    let token = input.next()
        .cloned()
        .map_err(|_| input.new_custom_error(RetinaStyleParseError::UnexpectedEofBasicColorKeyword))?;

    let Token::Ident(ident) = token else {
        return Err(input.new_custom_error(RetinaStyleParseError::ExpectedIdentifierAsPropertyValue));
    };

    CssWhiteSpace::iter()
        .find(|keyword| keyword.as_ref().eq_ignore_ascii_case(ident.as_ref()))
        .ok_or_else(|| input.new_custom_error(RetinaStyleParseError::UnknownWhiteSpaceKeyword))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use retina_style::{CssColor, CssNamedColor};

    use super::*;

    #[rstest]
    #[case("red", CssNamedColor::RED)]
    #[case("RED", CssNamedColor::RED)]
    #[case("rEd", CssNamedColor::RED)]
    #[case("Red", CssNamedColor::RED)]
    #[case("green", CssNamedColor::GREEN)]
    #[case("greeN", CssNamedColor::GREEN)]
    #[case("blue", CssNamedColor::BLUE)]
    fn value_color(#[case] input: &str, #[case] color: CssColor) {
        let mut input = cssparser::ParserInput::new(input);
        let input = &mut cssparser::Parser::new(&mut input);

        let result = parse_value(input);
        let expected = Ok(color.into());
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("none", CssDisplay::Box(CssDisplayBox::None))]
    #[case("inline", CssDisplay::Normal { inside: CssDisplayInside::Flow, outside: CssDisplayOutside::Inline, is_list_item: false })]
    #[case("block", CssDisplay::Normal { inside: CssDisplayInside::Flow, outside: CssDisplayOutside::Block, is_list_item: false })]
    #[case("inline-block", CssDisplay::Normal { inside: CssDisplayInside::FlowRoot, outside: CssDisplayOutside::Inline, is_list_item: false })]
    fn value_display(#[case] input: &str, #[case] display: CssDisplay) {
        let mut input = cssparser::ParserInput::new(input);
        let input = &mut cssparser::Parser::new(&mut input);

        let result = parse_value(input);
        let expected = Ok(Value::Display(display));
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("auto", CssLength::Auto)]
    #[case("0", CssLength::Pixels(0.0))]
    #[case("0px", CssLength::Pixels(0.0))]
    #[case("616px", CssLength::Pixels(616.0))]
    fn value_length(#[case] input: &str, #[case] display: CssLength) {
        let mut input = cssparser::ParserInput::new(input);
        let input = &mut cssparser::Parser::new(&mut input);

        let result = parse_value(input);
        let expected = Ok(Value::Length(display));
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("normal", CssWhiteSpace::Normal)]
    #[case("nowrap", CssWhiteSpace::Nowrap)]
    #[case("pre", CssWhiteSpace::Pre)]
    #[case("pre-wrap", CssWhiteSpace::PreWrap)]
    #[case("pre-line", CssWhiteSpace::PreLine)]
    #[case("break-spaces", CssWhiteSpace::BreakSpaces)]
    fn value_white_space(#[case] input: &str, #[case] white_space: CssWhiteSpace) {
        let mut input = cssparser::ParserInput::new(input);
        let input = &mut cssparser::Parser::new(&mut input);

        let result = parse_value(input);
        let expected = Ok(Value::WhiteSpace(white_space));
        assert_eq!(result, expected);
    }

}
