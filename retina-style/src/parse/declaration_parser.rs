// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use cssparser::Parser;

use crate::{
    Declaration,
    Property,
};

use super::{
    parse_value,
    ParseError,
    RetinaStyleParseError,
};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct DeclarationParser;

impl<'i> cssparser::DeclarationParser<'i> for DeclarationParser {
    type Declaration = Declaration;
    type Error = RetinaStyleParseError<'i>;

    fn parse_value<'t>(
        &mut self,
        name: cssparser::CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Declaration, cssparser::ParseError<'i, Self::Error>> {
        let property = Property::parse(name.as_ref()).unwrap_or(Property::Invalid);

        parse_value(input).map(|value| Declaration{
            property,
            value,
        })
    }

    fn enable_nesting(&self) -> bool {
        false
    }
}

/// Different from [parse_one_declaration], this function parses a declaration
/// whether or not it should be the only one in the given `input`.
///
/// [parser_one_declaration]: cssparser::parse_one_declaration
pub(crate) fn parse_declaration_one_of_many<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<Declaration, ParseError<'i>> {
    let name = input.expect_ident()?.clone();
    input.expect_colon()?;
    cssparser::DeclarationParser::parse_value(&mut DeclarationParser::default(), name, input)
}

#[cfg(test)]
mod tests {
    use crate::*;
    use super::*;

    #[test]
    fn declaration_background_color_blue() {
        const INPUT: &str = "background-color: blue;";
        let mut input = cssparser::ParserInput::new(INPUT);
        let input = &mut cssparser::Parser::new(&mut input);

        let result = parse_declaration_one_of_many(input);
        let expected = Ok(Declaration{
            property: Property::BackgroundColor,
            value: CssNamedColor::BLUE.into(),
        });
        assert_eq!(result, expected);
    }

    #[test]
    fn declaration_color_red() {
        const INPUT: &str = "color: red;";
        let mut input = cssparser::ParserInput::new(INPUT);
        let input = &mut cssparser::Parser::new(&mut input);

        let result = parse_declaration_one_of_many(input);
        let expected = Ok(Declaration {
            property: Property::Color,
            value: CssNamedColor::RED.into(),
        });
        assert_eq!(result, expected);
    }
}
