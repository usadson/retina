// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use cssparser::{
    Parser,
    RuleBodyParser, ParseError, ParseErrorKind,
};

use log::warn;

use retina_style::{
    AtMediaRule,
    CascadeOrigin,
    MediaQuery,
    MediaType,
    Rule,
    SelectorList,
    StyleRule,
};

use super::{
    RetinaStyleParseError,
    declaration_parser::DeclarationParser,
};

pub enum AtRulePrelude {
    Media(Vec<MediaQuery>),
}

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

    fn parse_at_media_block<'i, 't>(
        &mut self,
        media_query_list: Vec<MediaQuery>,
        input: &mut Parser<'i, 't>
    ) -> Result<Rule, ParseError<'i, RetinaStyleParseError<'i>>> {
        Ok(Rule::AtMedia(AtMediaRule {
            media_query_list,
            stylesheet: crate::parse_stylesheet_contents(self.cascade_origin, input),
        }))
    }

    fn parse_at_media_prelude<'i, 't>(
        &mut self,
        input: &mut Parser<'i, 't>
    ) -> Result<AtRulePrelude, ParseError<'i, RetinaStyleParseError<'i>>> {
        let location = input.current_source_location();

        let ty = input.expect_ident().map_err(|e| ParseError {
            location,
            kind: ParseErrorKind::Basic(e.kind),
        })?;

        if ty.eq_ignore_ascii_case("all") {
            let query = MediaQuery::Type(MediaType::All);
            return Ok(AtRulePrelude::Media(vec![query]));
        }

        if ty.eq_ignore_ascii_case("print") {
            let query = MediaQuery::Type(MediaType::Print);
            return Ok(AtRulePrelude::Media(vec![query]));
        }

        if ty.eq_ignore_ascii_case("screen") {
            let query = MediaQuery::Type(MediaType::Screen);
            return Ok(AtRulePrelude::Media(vec![query]));
        }

        Err(ParseError {
            location,
            kind: ParseErrorKind::Custom(RetinaStyleParseError::MediaPreludeUnknownType(ty.clone())),
        })
    }
}

impl<'i> cssparser::AtRuleParser<'i> for RuleParser {
    type Prelude = AtRulePrelude;
    type AtRule = Rule;
    type Error = RetinaStyleParseError<'i>;

    fn parse_prelude<'t>(
        &mut self,
        name: cssparser::CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Prelude, cssparser::ParseError<'i, Self::Error>> {
        if name.eq_ignore_ascii_case("media") {
            self.parse_at_media_prelude(input)
        } else {
            Err(ParseError {
                location: input.current_source_location(),
                kind: ParseErrorKind::Custom(RetinaStyleParseError::UnknownAtRule(name)),
            })
        }
    }

    fn parse_block<'t>(
        &mut self,
        prelude: Self::Prelude,
        _start: &cssparser::ParserState,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::AtRule, ParseError<'i, Self::Error>> {
        match prelude {
            AtRulePrelude::Media(media) => self.parse_at_media_block(media, input),
        }
    }
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

        let mut declaration_parser = DeclarationParser{};
        let mut declaration_parser = RuleBodyParser::new(input, &mut declaration_parser);

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
