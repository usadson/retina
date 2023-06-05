// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use cssparser::{Parser, Token};

use crate::{
    Selector,
    SelectorList,
    SimpleSelector,
};

use super::{
    ParseError,
    RetinaStyleParseError,
};

fn parse_selector<'i, 't>(
    input: &mut Parser<'i, 't>
) -> Result<Selector, ParseError<'i>> {
    input.skip_whitespace();
    Ok(match input.next()? {
        Token::Ident(ident) => Selector::Simple(SimpleSelector::TypeSelector(ident.as_ref().into())),
        Token::Delim('*') => Selector::Simple(SimpleSelector::Universal),

        _ => return Err(input.new_custom_error(RetinaStyleParseError::UnknownSelector))
    })
}

pub fn parse_selector_list<'i, 't>(
    input: &mut Parser<'i, 't>
) -> Result<SelectorList, ParseError<'i>> {
    Ok(SelectorList { items: vec![parse_selector(input)?] })
}
#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("*", Selector::Simple(SimpleSelector::Universal))]
    #[case("   *", Selector::Simple(SimpleSelector::Universal))]
    #[case("   *  ", Selector::Simple(SimpleSelector::Universal))]
    #[case("*  ", Selector::Simple(SimpleSelector::Universal))]
    #[case("h1", Selector::Simple(SimpleSelector::TypeSelector("h1".into())))]
    #[case("p", Selector::Simple(SimpleSelector::TypeSelector("p".into())))]
    #[case("style", Selector::Simple(SimpleSelector::TypeSelector("style".into())))]
    #[case("my-custom-element", Selector::Simple(SimpleSelector::TypeSelector("my-custom-element".into())))]
    fn single_selector(#[case] input: &str, #[case] expected: Selector) {
        let mut input = cssparser::ParserInput::new(input);
        let input = &mut cssparser::Parser::new(&mut input);

        let result = parse_selector(input);
        assert_eq!(result, Ok(expected));
    }

}
