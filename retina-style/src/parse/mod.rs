// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod declaration_parser;
mod error;
mod rule_parser;
mod selector_parser;
mod value_parser;

pub use self::error::RetinaStyleParseError;

pub(crate) use self::declaration_parser::parse_declaration_one_of_many;
pub(crate) use self::rule_parser::RuleParser;
pub(crate) use self::selector_parser::parse_selector_list;
pub(crate) use self::value_parser::parse_value;

pub(self) type ParseError<'i> = cssparser::ParseError<'i, RetinaStyleParseError>;

use cssparser::{
    Parser,
    ParserInput,
    RuleListParser,
};

use crate::{CascadeOrigin, Stylesheet};

pub fn parse_stylesheet(cascade_origin: CascadeOrigin, input: &str) -> Stylesheet {
    let mut input = ParserInput::new(input);
    let mut parser = Parser::new(&mut input);
    let mut rule_list_parser = RuleListParser::new_for_stylesheet(&mut parser, RuleParser::new(cascade_origin));

    let mut stylesheet = Stylesheet::new();

    while !rule_list_parser.input.is_exhausted() {
        let Some(rule) = rule_list_parser.next() else { continue };
        match rule {
            Ok(rule) => stylesheet.push(rule),
            Err(err) => println!("[style] CSS parse error: {:#?}", err.0),
        }
    }

    stylesheet
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn basically_everything_red() {
        let input = "
            * {
                color: green; /* ignored */
                color: red
            }
        ";

        let stylesheet = Stylesheet::parse(CascadeOrigin::Author, input);

        let rule = Rule::Style(StyleRule {
            cascade_origin: CascadeOrigin::Author,
            selector_list: SelectorList {
                items: vec![
                    Selector::Simple(SimpleSelector::Universal),
                ],
            },
            declarations: vec![
                Declaration {
                    property: Property::Color,
                    value: Value::Color(ColorValue::BasicColorKeyword(BasicColorKeyword::Green))
                },
                Declaration {
                    property: Property::Color,
                    value: Value::Color(ColorValue::BasicColorKeyword(BasicColorKeyword::Red))
                }
            ]
        });

        assert_eq!(stylesheet.rules(), &[
            rule
        ]);
    }
}
