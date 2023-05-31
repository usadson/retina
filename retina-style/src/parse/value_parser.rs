// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use cssparser::{Parser, Token};
use strum::IntoEnumIterator;

use crate::{value::color::BasicColorKeyword, Value, ColorValue};
use super::{ParseError, RetinaStyleParseError};

pub(crate) fn parse_basic_color_keyword<'i, 't>(
    input: &mut Parser<'i, 't>
) -> Result<BasicColorKeyword, ParseError<'i>> {
    let token = input.next()
        .cloned()
        .map_err(|_| input.new_custom_error(RetinaStyleParseError::UnexpectedEofBasicColorKeyword))?;

    let Token::Ident(ident) = token else {
        return Err(input.new_custom_error(RetinaStyleParseError::ExpectedIdentifierAsPropertyName));
    };

    BasicColorKeyword::iter()
        .find(|keyword| keyword.as_ref().eq_ignore_ascii_case(ident.as_ref()))
        .ok_or_else(|| input.new_custom_error(RetinaStyleParseError::UnknownBasicColorKeyword))
}

pub(crate) fn parse_value<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Value, ParseError<'i>> {
    if let Ok(basic_color_keyword) = input.try_parse(parse_basic_color_keyword) {
        return Ok(Value::Color(ColorValue::BasicColorKeyword(basic_color_keyword)));
    }

    Err(input.new_custom_error(RetinaStyleParseError::UnknownValue))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("red", BasicColorKeyword::Red)]
    #[case("RED", BasicColorKeyword::Red)]
    #[case("rEd", BasicColorKeyword::Red)]
    #[case("Red", BasicColorKeyword::Red)]
    #[case("green", BasicColorKeyword::Green)]
    #[case("greeN", BasicColorKeyword::Green)]
    #[case("blue", BasicColorKeyword::Blue)]
    fn value_color_basic_color_keyword(#[case] input: &str, #[case] keyword: BasicColorKeyword) {
        let mut input = cssparser::ParserInput::new(input);
        let input = &mut cssparser::Parser::new(&mut input);

        let result = parse_value(input);
        let expected = Ok(Value::Color(
            ColorValue::BasicColorKeyword(keyword)
        ));
        assert_eq!(result, expected);
    }

}
