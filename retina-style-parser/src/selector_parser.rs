// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use cssparser::{
    BasicParseErrorKind,
    Parser,
    Token,
};

use retina_common::StrTendril;

use retina_style::{
    AttributeSelector,
    AttributeSelectorCaseSensitivity,
    AttributeSelectorKind,
    ComplexSelector,
    CompoundSelector,
    PseudoClassSelectorKind,
    Selector,
    SelectorCombinator,
    SelectorList,
    SimpleSelector,
};

use crate::{
    ParseError,
    RetinaStyleParseError,
};

fn parse_attribute_selector<'i, 't>(
    input: &mut Parser<'i, 't>
) -> Result<SimpleSelector, ParseError<'i>> {
    let attribute = parse_attribute_selector_name(input)?;

    let location = input.current_source_location();
    let case_sensitivity = AttributeSelectorCaseSensitivity::Default;
    let first_token = input.next();

    let kind = match first_token {
        Ok(Token::Delim('=')) => {
            AttributeSelectorKind::Exact(input.expect_ident_or_string()?.as_ref().into())
        }

        Ok(Token::IncludeMatch) => {
            AttributeSelectorKind::OneOfWhitespaceSeparatedList(input.expect_ident_or_string()?.as_ref().into())
        }

        Ok(Token::DashMatch) => {
            AttributeSelectorKind::ExactOrStartsWithAndHyphen(input.expect_ident_or_string()?.as_ref().into())
        }

        Ok(Token::PrefixMatch) => {
            AttributeSelectorKind::BeginsWith(input.expect_ident_or_string()?.as_ref().into())
        }

        Ok(Token::SuffixMatch) => {
            AttributeSelectorKind::EndsWith(input.expect_ident_or_string()?.as_ref().into())
        }

        Ok(Token::SubstringMatch) => {
            AttributeSelectorKind::Contains(input.expect_ident_or_string()?.as_ref().into())
        }

        Err(e) if e.kind == BasicParseErrorKind::EndOfInput => {
            AttributeSelectorKind::RegardlessOfValue
        },

        _ => return Err(ParseError {
            kind: RetinaStyleParseError::AttributeSelectorUnknownOperatorName(first_token.unwrap().clone()).into(),
            location,
        })
    };

    Ok(SimpleSelector::Attribute(AttributeSelector::new(attribute, case_sensitivity, kind)))
}

fn parse_attribute_selector_name<'i, 't>(
    input: &mut Parser<'i, 't>
) -> Result<StrTendril, ParseError<'i>> {
    let location = input.current_source_location();
    let token = input.next()?;

    if let Token::Ident(ident) = token {
        Ok(ident.as_ref().into())
    } else {
        Err(ParseError {
            kind: RetinaStyleParseError::AttributeSelectorExpectedIdentifierAsAttributeName(token.clone()).into(),
            location,
        })
    }
}

fn parse_selector<'i, 't>(
    input: &mut Parser<'i, 't>
) -> Result<Selector, ParseError<'i>> {
    let mut complex = ComplexSelector {
        topmost: parse_compound_selector(input)?,
        combinators: Vec::new(),
    };

    loop {
        input.skip_whitespace();
        if input.is_exhausted() {
            break;
        }

        let state_before_combinator_token = input.state();
        let combinator_token = input.next()?;
        let combinator = match combinator_token {
            Token::Delim('>') => SelectorCombinator::Child,
            Token::Delim('+') => SelectorCombinator::NextSibling,
            Token::Delim('~') => SelectorCombinator::SubsequentSibling,

            _ => {
                input.reset(&state_before_combinator_token);
                SelectorCombinator::Descendant
            }
        };

        // The descendant combinator, which is represented by whitespace, can
        // be mistaken for actual whitespace before the `{` token, so we need
        // to make sure that isn't the case.
        //
        // ```text
        // h1 > h2 { ... }
        //   ^ ^
        //   | |__ but this isn't
        //   |
        //   |_ this is whitespace
        // ```

        input.skip_whitespace();
        if input.is_exhausted() && combinator == SelectorCombinator::Descendant {
            break;
        }

        complex.combinators.push((
            combinator,
            parse_compound_selector(input)?
        ));
    }

    Ok(parse_selector_try_simplify(complex))
}

fn parse_selector_try_simplify(complex_selector: ComplexSelector) -> Selector {
    if !complex_selector.combinators.is_empty() {
        return Selector::Complex(complex_selector);
    }

    let compound = complex_selector.topmost;

    match TryInto::<[SimpleSelector; 1]>::try_into(compound.0) {
        Ok([selector]) => Selector::Simple(selector),

        Err(compound) => Selector::Compound(CompoundSelector(compound)),
    }
}

fn parse_compound_selector<'i, 't>(
    input: &mut Parser<'i, 't>
) -> Result<CompoundSelector, ParseError<'i>> {
    input.skip_whitespace();

    let mut selectors = vec![
        parse_simple_selector(input)?
    ];

    while let Ok(selector) = input.try_parse(parse_simple_selector) {
        selectors.push(selector);
    }

    Ok(CompoundSelector(selectors))
}

fn parse_simple_selector<'i, 't>(
    input: &mut Parser<'i, 't>
) -> Result<SimpleSelector, ParseError<'i>> {
    if input.try_parse(Parser::expect_square_bracket_block).is_ok() {
        return input.parse_nested_block(parse_attribute_selector);
    }

    let first_token = input.next_including_whitespace()?;
    Ok(match first_token {
        Token::Colon => {
            let pseudo_class = input.expect_ident_cloned()?;
            let pseudo = PseudoClassSelectorKind::parse(pseudo_class.as_ref())
                .ok_or_else(|| input.new_custom_error(RetinaStyleParseError::UnknownSelectorPseudoClass(pseudo_class)))?;
            SimpleSelector::PseudoClass(pseudo)
        }

        Token::Delim('*') => SimpleSelector::Universal,

        Token::Ident(ident) => SimpleSelector::TypeSelector(ident.as_ref().into()),

        Token::IDHash(ident) if !ident.is_empty() => SimpleSelector::Id(ident.as_ref().into()),

        Token::Delim('.') => SimpleSelector::Class(input.expect_ident()?.as_ref().into()),

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
    #[case("[attr]", Selector::Simple(SimpleSelector::Attribute(AttributeSelector::new("attr".into(), AttributeSelectorCaseSensitivity::Default, AttributeSelectorKind::RegardlessOfValue))))]
    #[case("[attr=val]", Selector::Simple(SimpleSelector::Attribute(AttributeSelector::new("attr".into(), AttributeSelectorCaseSensitivity::Default, AttributeSelectorKind::Exact("val".into())))))]
    #[case("[attr='my value']", Selector::Simple(SimpleSelector::Attribute(AttributeSelector::new("attr".into(), AttributeSelectorCaseSensitivity::Default, AttributeSelectorKind::Exact("my value".into())))))]
    #[case("[attr=\"my value\"]", Selector::Simple(SimpleSelector::Attribute(AttributeSelector::new("attr".into(), AttributeSelectorCaseSensitivity::Default, AttributeSelectorKind::Exact("my value".into())))))]
    #[case("[attr~=val]", Selector::Simple(SimpleSelector::Attribute(AttributeSelector::new("attr".into(), AttributeSelectorCaseSensitivity::Default, AttributeSelectorKind::OneOfWhitespaceSeparatedList("val".into())))))]
    #[case("[attr~='my value']", Selector::Simple(SimpleSelector::Attribute(AttributeSelector::new("attr".into(), AttributeSelectorCaseSensitivity::Default, AttributeSelectorKind::OneOfWhitespaceSeparatedList("my value".into())))))]
    #[case("[attr~=\"my value\"]", Selector::Simple(SimpleSelector::Attribute(AttributeSelector::new("attr".into(), AttributeSelectorCaseSensitivity::Default, AttributeSelectorKind::OneOfWhitespaceSeparatedList("my value".into())))))]
    #[case("[attr|=val]", Selector::Simple(SimpleSelector::Attribute(AttributeSelector::new("attr".into(), AttributeSelectorCaseSensitivity::Default, AttributeSelectorKind::ExactOrStartsWithAndHyphen("val".into())))))]
    #[case("[attr|='my value']", Selector::Simple(SimpleSelector::Attribute(AttributeSelector::new("attr".into(), AttributeSelectorCaseSensitivity::Default, AttributeSelectorKind::ExactOrStartsWithAndHyphen("my value".into())))))]
    #[case("[attr|=\"my value\"]", Selector::Simple(SimpleSelector::Attribute(AttributeSelector::new("attr".into(), AttributeSelectorCaseSensitivity::Default, AttributeSelectorKind::ExactOrStartsWithAndHyphen("my value".into())))))]
    #[case("[attr^=val]", Selector::Simple(SimpleSelector::Attribute(AttributeSelector::new("attr".into(), AttributeSelectorCaseSensitivity::Default, AttributeSelectorKind::BeginsWith("val".into())))))]
    #[case("[attr^='my value']", Selector::Simple(SimpleSelector::Attribute(AttributeSelector::new("attr".into(), AttributeSelectorCaseSensitivity::Default, AttributeSelectorKind::BeginsWith("my value".into())))))]
    #[case("[attr^=\"my value\"]", Selector::Simple(SimpleSelector::Attribute(AttributeSelector::new("attr".into(), AttributeSelectorCaseSensitivity::Default, AttributeSelectorKind::BeginsWith("my value".into())))))]
    #[case("[attr$=val]", Selector::Simple(SimpleSelector::Attribute(AttributeSelector::new("attr".into(), AttributeSelectorCaseSensitivity::Default, AttributeSelectorKind::EndsWith("val".into())))))]
    #[case("[attr$='my value']", Selector::Simple(SimpleSelector::Attribute(AttributeSelector::new("attr".into(), AttributeSelectorCaseSensitivity::Default, AttributeSelectorKind::EndsWith("my value".into())))))]
    #[case("[attr$=\"my value\"]", Selector::Simple(SimpleSelector::Attribute(AttributeSelector::new("attr".into(), AttributeSelectorCaseSensitivity::Default, AttributeSelectorKind::EndsWith("my value".into())))))]
    #[case("[attr*=val]", Selector::Simple(SimpleSelector::Attribute(AttributeSelector::new("attr".into(), AttributeSelectorCaseSensitivity::Default, AttributeSelectorKind::Contains("val".into())))))]
    #[case("[attr*='my value']", Selector::Simple(SimpleSelector::Attribute(AttributeSelector::new("attr".into(), AttributeSelectorCaseSensitivity::Default, AttributeSelectorKind::Contains("my value".into())))))]
    #[case("[attr*=\"my value\"]", Selector::Simple(SimpleSelector::Attribute(AttributeSelector::new("attr".into(), AttributeSelectorCaseSensitivity::Default, AttributeSelectorKind::Contains("my value".into())))))]
    fn single_selector(#[case] input: &str, #[case] expected: Selector) {
        let mut input = cssparser::ParserInput::new(input);
        let input = &mut cssparser::Parser::new(&mut input);

        let result = parse_selector(input);
        assert_eq!(result, Ok(expected));
        assert!(input.is_exhausted());
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
