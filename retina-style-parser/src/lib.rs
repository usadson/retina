// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod color_parser;
mod declaration_parser;
mod error;
mod rule_parser;
mod selector_parser;
mod util;
mod value_parser;

pub use self::error::RetinaStyleParseError;
pub use self::util::CssParsable;

pub(crate) use self::rule_parser::RuleParser;
pub(crate) use self::selector_parser::parse_selector_list;
pub(crate) use self::value_parser::parse_value;

pub(self) type ParseError<'i> = cssparser::ParseError<'i, RetinaStyleParseError<'i>>;

use cssparser::{
    Parser,
    ParserInput,
    RuleListParser,
};
use log::warn;

use retina_style::{CascadeOrigin, Rule, Stylesheet};

pub fn parse_stylesheet(cascade_origin: CascadeOrigin, input: &str) -> Stylesheet {
    let mut input = ParserInput::new(input);
    let mut parser = Parser::new(&mut input);
    let mut rule_list_parser = RuleListParser::new_for_stylesheet(&mut parser, RuleParser::new(cascade_origin));

    let mut stylesheet = Stylesheet::new();

    while !rule_list_parser.input.is_exhausted() {
        let Some(rule) = rule_list_parser.next() else { continue };
        match rule {
            Ok(rule) => {
                if let Rule::Style(style_rule) = &rule {
                    if style_rule.declarations.is_empty() {
                        if cfg!(test) && cascade_origin == CascadeOrigin::UserAgent {
                            panic!("[CssParser] Declaration is empty: {:#?}", style_rule);
                        }

                        warn!("[CssParser] Declaration is empty: {:#?}", style_rule);
                    }
                }

                stylesheet.push(rule);
            }
            Err(err) => warn!("[CssParser] CSS parse error: {:#?}", err.0),
        }
    }

    stylesheet
}

#[cfg(test)]
mod tests {
    use retina_style::*;
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
                Declaration::new(Property::Color, CssNamedColor::GREEN.into()),
                Declaration::new(Property::Color, CssNamedColor::RED.into()),
            ]
        });

        assert_eq!(stylesheet.rules(), &[
            rule
        ]);
    }
}
