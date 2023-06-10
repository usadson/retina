// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use cssparser::{
    DeclarationListParser,
    Parser,
};
use log::warn;

use retina_style::{
    CascadeOrigin,
    Rule,
    StyleRule,
    SelectorList,
};

use super::{
    RetinaStyleParseError,
    declaration_parser::DeclarationParser,
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

        let mut declaration_parser = DeclarationListParser::new(input, DeclarationParser{});

        while let Some(result) = declaration_parser.next() {
            match result {
                Ok(declaration) => declarations.push(declaration),
                Err(e) => warn!("Failed to parse declaration: {e:#?}"),
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
    use crate::CssParsable;
    use retina_style::*;

    #[rstest]
    /// Normal
    #[case("* { color: red; }", CssNamedColor::RED)]
    /// Without trailing semicolon
    #[case("* { color: red }", CssNamedColor::RED)]
    /// Without whitespace
    #[case("*{color:red}", CssNamedColor::RED)]
    /// [Basic color keywords are ASCII-case insensitive](https://drafts.csswg.org/css-color-3/#html4)
    #[case("*{color:RED}", CssNamedColor::RED)]
    /// [Basic color keywords are ASCII-case insensitive](https://drafts.csswg.org/css-color-3/#html4)
    #[case("*{color:Red}", CssNamedColor::RED)]
    /// [Basic color keywords are ASCII-case insensitive](https://drafts.csswg.org/css-color-3/#html4)
    #[case("*{color:rEd}", CssNamedColor::RED)]
    #[test]
    fn qualified_rule_single_declaration(#[case] input: &str, #[case] expected: CssColor) {
        let stylesheet: Stylesheet = Stylesheet::parse(CascadeOrigin::Author, input);

        let rule = Rule::Style(StyleRule {
            cascade_origin: CascadeOrigin::Author,
            selector_list: SelectorList {
                items: vec![
                    Selector::Simple(SimpleSelector::Universal),
                ],
            },
            declarations: vec![
                Declaration::new(Property::Color, expected.into()),
            ]
        });

        assert_eq!(stylesheet.rules(), &[
            rule
        ]);
    }
}
