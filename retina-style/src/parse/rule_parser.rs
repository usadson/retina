// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use cssparser::Parser;
use log::warn;

use crate::{
    Rule,
    StyleRule,
    SelectorList, CascadeOrigin,
};

use super::{
    parse_declaration_one_of_many,
    RetinaStyleParseError,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct RuleParser {
    cascade_origin: CascadeOrigin,
}

impl RuleParser {
    pub const fn new(cascade_origin: CascadeOrigin) -> Self {
        Self {
            cascade_origin
        }
    }
}

impl<'i> cssparser::AtRuleParser<'i> for RuleParser {
    type Prelude = ();
    type AtRule = Rule;
    type Error = RetinaStyleParseError<'i>;
    // ignored / errors upon
}

impl<'i> cssparser::QualifiedRuleParser<'i> for RuleParser {
    type Error = RetinaStyleParseError<'i>;
    type Prelude = SelectorList;
    type QualifiedRule = Rule;

    fn parse_block<'t>(
        &mut self,
        selector_list: Self::Prelude,
        _start: &cssparser::ParserState,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::QualifiedRule, cssparser::ParseError<'i, Self::Error>> {
        let mut declarations = Vec::new();

        while !input.is_exhausted() {
            match parse_declaration_one_of_many(input) {
                Ok(declaration) => declarations.push(declaration),
                Err(e) => warn!("Failed to parse declaration: {e:#?}"),
            }

            // consume everything up to and including the semicolon.
            while !input.is_exhausted() {
                if input.expect_semicolon().is_ok() {
                    break;
                }
            }
        }

        Ok(Rule::Style(StyleRule {
            cascade_origin: self.cascade_origin,
            selector_list,
            declarations,
        }))
    }

    fn parse_prelude<'t>(
        &mut self,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Prelude, cssparser::ParseError<'i, Self::Error>> {
        super::parse_selector_list(input)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use crate::*;

    fn helper_color(value: BasicColorKeyword) -> Declaration {
        Declaration {
            property: Property::Color,
            value: Value::Color(ColorValue::BasicColorKeyword(value))
        }
    }

    #[rstest]
    /// Normal
    #[case("* { color: red; }", helper_color(BasicColorKeyword::Red))]
    /// Without trailing semicolon
    #[case("* { color: red }", helper_color(BasicColorKeyword::Red))]
    /// Without whitespace
    #[case("*{color:red}", helper_color(BasicColorKeyword::Red))]
    /// [Basic color keywords are ASCII-case insensitive](https://drafts.csswg.org/css-color-3/#html4)
    #[case("*{color:RED}", helper_color(BasicColorKeyword::Red))]
    /// [Basic color keywords are ASCII-case insensitive](https://drafts.csswg.org/css-color-3/#html4)
    #[case("*{color:Red}", helper_color(BasicColorKeyword::Red))]
    /// [Basic color keywords are ASCII-case insensitive](https://drafts.csswg.org/css-color-3/#html4)
    #[case("*{color:rEd}", helper_color(BasicColorKeyword::Red))]
    #[test]
    fn qualified_rule_single_declaration(#[case] input: &str, #[case] expected: Declaration) {
        let stylesheet = Stylesheet::parse(CascadeOrigin::Author, input);

        let rule = Rule::Style(StyleRule {
            cascade_origin: CascadeOrigin::Author,
            selector_list: SelectorList {
                items: vec![
                    Selector::Simple(SimpleSelector::Universal),
                ],
            },
            declarations: vec![
                expected
            ]
        });

        assert_eq!(stylesheet.rules(), &[
            rule
        ]);
    }
}
