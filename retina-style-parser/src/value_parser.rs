// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use cssparser::{
    Parser,
    ParseErrorKind,
    Token, Color,
};

use retina_common::StrTendril;
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

pub(crate) fn parse_float<'i, 't>(
    input: &mut Parser<'i, 't>
) -> Result<CssFloatValue, ParseError<'i>> {
    let token = input.expect_ident()?;

    let float = CssFloatValue::iter()
        .find(|float| float.as_ref().eq_ignore_ascii_case(&token));

    match float {
        Some(float) => Ok(float),
        None => {
            let token = token.clone();
            Err(input.new_custom_error(RetinaStyleParseError::FloatUnknownKeyword(token)))
        }
    }
}

pub(crate) fn parse_font_families<'i, 't>(
    input: &mut Parser<'i, 't>
) -> Result<Vec<CssFontFamilyName>, ParseError<'i>> {
    let mut families = Vec::new();
    while !input.is_exhausted() {
        input.skip_whitespace();

        let location = input.current_source_location();

        match input.next() {
            Ok(Token::Ident(ident)) => {
                if let Some(generic) = CssGenericFontFamilyName::iter().find(|generic| ident.eq_ignore_ascii_case(generic.as_ref())) {
                    families.push(CssFontFamilyName::Generic(generic));
                } else {
                    families.push(CssFontFamilyName::Name(StrTendril::from(&ident[..])));
                }
            }

            Ok(Token::QuotedString(str)) => families.push(CssFontFamilyName::Name(StrTendril::from(&str[..]))),

            Ok(token) => return Err(ParseError {
                kind: ParseErrorKind::Basic(cssparser::BasicParseErrorKind::UnexpectedToken(token.clone())),
                location,
            }),

            Err(..) => break,
        }

        input.skip_whitespace();
        if input.is_exhausted() {
            break;
        }

        input.expect_comma()?;
    }

    Ok(families)
}

pub(crate) fn parse_font_shorthand<'i, 't>(
    input: &mut Parser<'i, 't>
) -> Result<CssFontShorthand, ParseError<'i>> {
    let size = parse_length(input)?;
    let line_height = input.try_parse(parse_font_shorthand_line_height).ok();
    let families = parse_font_families(input)?;

    Ok(CssFontShorthand {
        families,
        style: None,
        size,
        line_height,
        weight: None,
    })
}

pub(crate) fn parse_font_shorthand_line_height<'i, 't>(
    input: &mut Parser<'i, 't>
) -> Result<CssLength, ParseError<'i>> {
    input.skip_whitespace();
    input.expect_delim('/')?;
    input.skip_whitespace();

    parse_line_height(input)
}

pub(crate) fn parse_font_style<'i, 't>(
    input: &mut Parser<'i, 't>
) -> Result<CssFontStyle, ParseError<'i>> {
    let location = input.current_source_location();
    let keyword = input.expect_ident()?;
    CssFontStyle::iter()
        .find(|style| {
            style.as_ref().eq_ignore_ascii_case(&keyword)
        })
        .ok_or_else(|| ParseError {
            kind: ParseErrorKind::Custom(
                RetinaStyleParseError::FontStyleUnknownKeyword(keyword.clone())
            ),
            location,
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
                "em" => Ok(CssLength::FontSize(value as _)),
                "px" => Ok(CssLength::Pixels(value as _)),
                "rem" => Ok(CssLength::FontSizeOfRootElement(value as _)),
                "vh" => Ok(CssLength::UaDefaultViewportHeightPercentage(value as _)),
                "vw" => Ok(CssLength::UaDefaultViewportWidthPercentage(value as _)),
                _ => Err(ParseError {
                    kind: ParseErrorKind::Custom(RetinaStyleParseError::LengthUnknownUnit(unit)),
                    location: token_location,
                }),
            }
        }

        Token::Number { int_value, .. } if int_value == Some(0) => {
            Ok(CssLength::Pixels(0.0))
        }

        Token::Percentage { unit_value, .. } => Ok(CssLength::Percentage(unit_value as _)),

        _ => Err(ParseError {
            kind: ParseErrorKind::Custom(RetinaStyleParseError::LengthUnexpectedToken(token)),
            location: token_location,
        })
    }
}

pub(crate) fn parse_line_height<'i, 't>(
    input: &mut Parser<'i, 't>
) -> Result<CssLength, ParseError<'i>> {
    let result = input.try_parse(parse_length);

    if result.is_err() {
        if let Ok(number) = input.expect_number() {
            return Ok(CssLength::FontSize(number as _));
        }
    }

    result
}

pub(crate) fn parse_line_style<'i, 't>(
    input: &mut Parser<'i, 't>
) -> Result<CssLineStyle, ParseError<'i>> {
    let token = input.next()
        .cloned()
        .map_err(|_| input.new_custom_error(RetinaStyleParseError::LineStyleUnknownKeyword))?;

    let Token::Ident(ident) = token else {
        return Err(input.new_custom_error(RetinaStyleParseError::LineStyleExpectedKeyword));
    };

    CssLineStyle::from_str(ident.as_ref())
        .ok_or_else(|| input.new_custom_error(RetinaStyleParseError::LineStyleUnknownKeyword))
}


pub(crate) fn parse_single_value<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Value, ParseError<'i>> {
    let location = input.current_source_location();

    if let Ok(color) = input.try_parse(Color::parse) {
        if let Some(color) = convert_color(color) {
            return Ok(Value::Color(color));
        }

        return Err(ParseError {
            kind: ParseErrorKind::Custom(
                RetinaStyleParseError::ColorUnknownValue(color)
            ),
            location,
        });
    }

    if let Ok(display) = input.try_parse(parse_display) {
        return Ok(Value::Display(display));
    }

    if let Ok(length) = input.try_parse(parse_length) {
        return Ok(Value::Length(length));
    }

    if let Ok(line_style) = input.try_parse(parse_line_style) {
        return Ok(Value::LineStyle(line_style));
    }

    if let Ok(white_space) = input.try_parse(parse_white_space) {
        return Ok(Value::WhiteSpace(white_space));
    }

    let token = input.next().ok().cloned();
    Err(input.new_custom_error(RetinaStyleParseError::UnknownValue(token)))
}

fn parse_specific_value<'i, 't>(
    input: &mut Parser<'i, 't>,
    property: Property,
) -> Option<Result<Value, ParseError<'i>>> {
    match property {
        Property::Float => Some(parse_float(input).map(|float| Value::Float(float))),
        Property::Font => Some(parse_font_shorthand(input).map(|shorthand| Value::FontShorthand(shorthand))),
        Property::FontFamily => Some(parse_font_families(input).map(|families| Value::FontFamily(families))),
        Property::FontStyle => Some(parse_font_style(input).map(|style| Value::FontStyle(style))),

        _ => None,
    }
}

pub(crate) fn parse_value<'i, 't>(input: &mut Parser<'i, 't>, property: Property) -> Result<Value, ParseError<'i>> {
    if let Some(result) = parse_specific_value(input, property) {
        return result;
    }

    let value = parse_single_value(input)?;
    if input.is_exhausted() {
        return Ok(value);
    }

    let mut values = Vec::with_capacity(4);
    values.push(value);

    while !input.is_exhausted() {
        values.push(parse_single_value(input)?);
    }

    assert_ne!(values.len(), 1);

    match &values[..] {
        //
        // Border
        //
        &[Value::Length(length), Value::LineStyle(style)] => Ok(Value::BorderLonghand(
            CssBorderLonghand {
                width: Some(length),
                style: Some(style),
                color: None,
            }
        )),

        &[Value::Length(length), Value::LineStyle(style), Value::Color(color)] => Ok(Value::BorderLonghand(
            CssBorderLonghand {
                width: Some(length),
                style: Some(style),
                color: Some(color),
            }
        )),

        &[Value::LineStyle(style), Value::Color(color)] => Ok(Value::BorderLonghand(
            CssBorderLonghand {
                width: None,
                style: Some(style),
                color: Some(color),
            }
        )),

        //
        // Colors
        //
        &[Value::Color(a), Value::Color(b)] => Ok(Value::ComponentList(
            ValueComponentList::TwoColors([a, b]),
        )),

        &[Value::Color(a), Value::Color(b), Value::Color(c)] => Ok(Value::ComponentList(
            ValueComponentList::ThreeColors([a, b, c]),
        )),

        &[Value::Color(a), Value::Color(b), Value::Color(c), Value::Color(d)] => Ok(Value::ComponentList(
            ValueComponentList::FourColors([a, b, c, d]),
        )),

        //
        // Lengths
        //
        &[Value::Length(a), Value::Length(b)] => Ok(Value::ComponentList(
            ValueComponentList::TwoLengths([a, b]),
        )),

        &[Value::Length(a), Value::Length(b), Value::Length(c)] => Ok(Value::ComponentList(
            ValueComponentList::ThreeLengths([a, b, c]),
        )),

        &[Value::Length(a), Value::Length(b), Value::Length(c), Value::Length(d)] => Ok(Value::ComponentList(
            ValueComponentList::FourLengths([a, b, c, d]),
        )),

        _ => Err(input.new_custom_error(RetinaStyleParseError::ComponentListUnknownKinds(values))),
    }
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

        let result = parse_value(input, Property::Color);
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

        let result = parse_value(input, Property::Display);
        let expected = Ok(Value::Display(display));
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("10px/1 Verdana, sans-serif", CssFontShorthand {
        families: vec![CssFontFamilyName::Name("Verdana".into()), CssFontFamilyName::Generic(CssGenericFontFamilyName::SansSerif)],
        style: None,
        size: CssLength::Pixels(10.0),
        line_height: Some(CssLength::FontSize(1.0)),
        weight: None,
    })]
    fn value_font_shorthand(#[case] input: &str, #[case] shorthand: CssFontShorthand) {
        let mut input = cssparser::ParserInput::new(input);
        let input = &mut cssparser::Parser::new(&mut input);

        let result = parse_value(input, Property::Font);
        let expected = Ok(Value::FontShorthand(shorthand));
        pretty_assertions::assert_eq!(result, expected);
    }

    #[rstest]
    #[case("/1", Some(CssLength::FontSize(1.0)))]
    fn value_font_shorthand_line_height(#[case] input: &str, #[case] expected: Option<CssLength>) {
        let mut input = cssparser::ParserInput::new(input);
        let input = &mut cssparser::Parser::new(&mut input);

        let result = parse_font_shorthand_line_height(input);
        pretty_assertions::assert_eq!(result.as_ref().ok(), expected.as_ref(), "result was: {result:#?}");
    }

    #[rstest]
    #[case("auto", CssLength::Auto)]
    #[case("0", CssLength::Pixels(0.0))]
    #[case("0px", CssLength::Pixels(0.0))]
    #[case("616px", CssLength::Pixels(616.0))]
    fn value_length(#[case] input: &str, #[case] display: CssLength) {
        let mut input = cssparser::ParserInput::new(input);
        let input = &mut cssparser::Parser::new(&mut input);

        let result = parse_value(input, Property::Width);
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

        let result = parse_value(input, Property::WhiteSpace);
        let expected = Ok(Value::WhiteSpace(white_space));
        assert_eq!(result, expected);
    }

}
