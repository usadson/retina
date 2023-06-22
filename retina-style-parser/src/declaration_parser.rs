// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use cssparser::Parser;

use retina_style::{
    Declaration,
    Property,
};

use super::{
    parse_value,
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

        parse_value(input).map(|value| Declaration::new(property, value))
    }
}

impl<'i> cssparser::RuleBodyItemParser<'i, Declaration, RetinaStyleParseError<'i>> for DeclarationParser {
    fn parse_declarations(&self) -> bool {
        true
    }

    fn parse_qualified(&self) -> bool {
        false
    }
}

impl<'i> cssparser::AtRuleParser<'i> for DeclarationParser {
    type Prelude = ();
    type AtRule = Declaration;
    type Error = RetinaStyleParseError<'i>;
    // ignored / errors upon
}

impl<'i> cssparser::QualifiedRuleParser<'i> for DeclarationParser {
    type Prelude = ();
    type QualifiedRule = Declaration;
    type Error = RetinaStyleParseError<'i>;
    // ignored / errors upon
}
