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
    let first_token = input.next()?;
    Ok(match first_token {
        Token::Delim('*') => Selector::Simple(SimpleSelector::Universal),

        Token::Ident(ident) => Selector::Simple(SimpleSelector::TypeSelector(ident.as_ref().into())),

        Token::IDHash(ident) if !ident.is_empty() => Selector::Simple(SimpleSelector::Id(ident.as_ref().into())),

        Token::Delim('.') => Selector::Simple(SimpleSelector::Class(input.expect_ident()?.as_ref().into())),

        _ => {
            let first_token = first_token.clone();
            return Err(input.new_custom_error(RetinaStyleParseError::UnknownSelector(first_token)))
        }
    })
}

pub fn parse_selector_list<'i, 't>(
    input: &mut Parser<'i, 't>
) -> Result<SelectorList, ParseError<'i>> {
    input.parse_comma_separated(parse_selector)
        .map(|items| SelectorList { items})
}
#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use pretty_assertions::assert_eq;

    #[rstest]
    #[case("*", Selector::Simple(SimpleSelector::Universal))]
    #[case("   *", Selector::Simple(SimpleSelector::Universal))]
    #[case("   *  ", Selector::Simple(SimpleSelector::Universal))]
    #[case("*  ", Selector::Simple(SimpleSelector::Universal))]
    #[case("h1", Selector::Simple(SimpleSelector::TypeSelector("h1".into())))]
    #[case("p", Selector::Simple(SimpleSelector::TypeSelector("p".into())))]
    #[case("style", Selector::Simple(SimpleSelector::TypeSelector("style".into())))]
    #[case("my-custom-element", Selector::Simple(SimpleSelector::TypeSelector("my-custom-element".into())))]
    #[case("#my-id", Selector::Simple(SimpleSelector::Id("my-id".into())))]
    #[case(".class", Selector::Simple(SimpleSelector::Class("class".into())))]
    fn single_selector(#[case] input: &str, #[case] expected: Selector) {
        let mut input = cssparser::ParserInput::new(input);
        let input = &mut cssparser::Parser::new(&mut input);

        let result = parse_selector(input);
        assert_eq!(result, Ok(expected));
    }

    #[rstest]
    #[case("*", vec![Selector::Simple(SimpleSelector::Universal)])]
    #[case("*, *", vec![Selector::Simple(SimpleSelector::Universal), Selector::Simple(SimpleSelector::Universal)])]
    #[case("h1", vec![Selector::Simple(SimpleSelector::TypeSelector("h1".into()))])]
    #[case("h1, h2", vec![
        Selector::Simple(SimpleSelector::TypeSelector("h1".into())),
        Selector::Simple(SimpleSelector::TypeSelector("h2".into())),
    ])]
    fn selector_list(#[case] input: &str, #[case] items: Vec<Selector>) {
        let mut input = cssparser::ParserInput::new(input);
        let input = &mut cssparser::Parser::new(&mut input);

        assert_eq!(parse_selector_list(input), Ok(SelectorList { items }));
    }

}
