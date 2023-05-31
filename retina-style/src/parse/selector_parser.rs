// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use cssparser::Parser;

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

    if input.expect_delim('*').is_ok() {
        return Ok(Selector::Simple(SimpleSelector::Universal));
    }

    Err(input.new_custom_error(RetinaStyleParseError::UnknownSelector))
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
    fn single_selector(#[case] input: &str, #[case] expected: Selector) {
        let mut input = cssparser::ParserInput::new(input);
        let input = &mut cssparser::Parser::new(&mut input);

        let result = parse_selector(input);
        assert_eq!(result, Ok(expected));
    }

}
