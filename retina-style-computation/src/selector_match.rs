// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use retina_dom::{NodeKind, Element};
use retina_style::{
    AttributeSelector,
    Selector,
    SelectorList,
    SimpleSelector, AttributeSelectorKind, AttributeSelectorCaseSensitivity,
};

fn matches_attribute_value(
    selector_value: &str,
    actual_value: &str,
    case_sensitivity: AttributeSelectorCaseSensitivity
) -> bool {
    match case_sensitivity {
        AttributeSelectorCaseSensitivity::Default => {
            // TODO is this correct for HTML?
            selector_value.eq_ignore_ascii_case(actual_value)
        },

        AttributeSelectorCaseSensitivity::AsciiCaseInsensitive => {
            selector_value.eq_ignore_ascii_case(actual_value)
        },

        AttributeSelectorCaseSensitivity::Identical => {
            selector_value == actual_value
        },
    }
}

fn matches_attribute_selector(
    attribute_selector: &AttributeSelector,
    element: &Element,
) -> bool {
    let Some(actual_value) = element.attributes().find_by_str(attribute_selector.attribute_name()) else {
        return false;
    };

    match attribute_selector.kind() {
        AttributeSelectorKind::RegardlessOfValue => true,

        AttributeSelectorKind::Exact(selector_value) => {
            matches_attribute_value(selector_value, actual_value, attribute_selector.case_sensitivity())
        },

        AttributeSelectorKind::OneOfWhitespaceSeparatedList(selector_value) => {
            actual_value.split_ascii_whitespace()
                .find(|item| matches_attribute_value(selector_value, item, attribute_selector.case_sensitivity()))
                .is_some()
        }

        AttributeSelectorKind::ExactOrStartsWithAndHyphen(selector_value) => {
            if actual_value.len() < selector_value.len() {
                return false;
            }

            let actual_prefix = &actual_value[0..selector_value.len()];
            if !matches_attribute_value(selector_value, actual_prefix, attribute_selector.case_sensitivity()) {
                return false;
            }

            if actual_value.len() == selector_value.len() {
                return true;
            }

            actual_value.bytes().nth(selector_value.len()) == Some(b'-')
        }

        // <https://www.w3.org/TR/selectors-4/#attribute-substrings>
        // [attr^=val]
        AttributeSelectorKind::BeginsWith(selector_value) => {
            if selector_value.is_empty() || actual_value.len() < selector_value.len() {
                return false;
            }

            let actual_prefix = &actual_value[0..selector_value.len()];
            matches_attribute_value(selector_value, actual_prefix, attribute_selector.case_sensitivity())
        }

        // <https://www.w3.org/TR/selectors-4/#attribute-substrings>
        // [attr$=val]
        AttributeSelectorKind::EndsWith(selector_value) => {
            if selector_value.is_empty() || actual_value.len() < selector_value.len() {
                return false;
            }

            let actual_suffix = &actual_value[actual_value.len() - selector_value.len()..];
            matches_attribute_value(selector_value, actual_suffix, attribute_selector.case_sensitivity())
        }

        // <https://www.w3.org/TR/selectors-4/#attribute-substrings>
        // [attr$=val]
        AttributeSelectorKind::Contains(selector_value) => {
            if selector_value.is_empty() || actual_value.len() < selector_value.len() {
                return false;
            }

            match attribute_selector.case_sensitivity() {
                AttributeSelectorCaseSensitivity::Default // TODO <-- is this correct?
                    | AttributeSelectorCaseSensitivity::AsciiCaseInsensitive => {
                    actual_value.to_ascii_lowercase().contains(&selector_value.to_ascii_lowercase())
                }

                AttributeSelectorCaseSensitivity::Identical => {
                    actual_value.contains(selector_value.as_ref())
                }
            }
        }
    }
}

/// Checks whether or not the given node matches the selector.
pub fn matches_selector(selector: &Selector, node: &NodeKind) -> bool {
    _ = node;
    match selector {
        Selector::Simple(SimpleSelector::Attribute(attribute_selector)) => {
            node.as_dom_element().is_some_and(|element| matches_attribute_selector(attribute_selector, element))
        }

        Selector::Simple(SimpleSelector::Class(class_to_find)) => {
            let class_to_find = class_to_find.as_ref();
            node.as_dom_element()
                .is_some_and(|element| {
                    element.class_list()
                        .find(|actual_class| actual_class == &class_to_find)
                        .is_some()
                })
        }

        Selector::Simple(SimpleSelector::Id(id)) => {
            // TODO in quirks mode <https://www.w3.org/TR/selectors-4/#ref-for-concept-document-quirks%E2%91%A0>
            node.as_dom_element().is_some_and(|element| {
                let element_id = element.id();
                !element_id.is_empty() && element_id == id.as_ref()
            })
        }

        Selector::Simple(SimpleSelector::TypeSelector(ty)) => {
            node.tag_name().is_some_and(|name| name.eq_ignore_ascii_case(ty))
        }

        Selector::Simple(SimpleSelector::Universal) => true,
    }
}

/// A simple extension trait to be able to call `Selector::matches`.
pub trait SelectorMatcher {
    /// Checks whether or not the given node matches the selector.
    fn matches(&self, node: &NodeKind) -> bool;
}

impl SelectorMatcher for Selector {
    fn matches(&self, node: &NodeKind) -> bool {
        matches_selector(self, node)
    }
}

impl SelectorMatcher for SelectorList {
    fn matches(&self, node: &NodeKind) -> bool {
        self.items.iter().any(|selector| selector.matches(node))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use retina_dom::*;
    use rstest::rstest;
    use tendril::StrTendril;

    #[rstest]
    #[case("who", "me", AttributeSelector::new("who".into(), AttributeSelectorCaseSensitivity::Identical, AttributeSelectorKind::RegardlessOfValue), true)]
    #[case("who", "me", AttributeSelector::new("me".into(), AttributeSelectorCaseSensitivity::Identical, AttributeSelectorKind::RegardlessOfValue), false)]
    fn attribute_regardless_of_value(
        #[case] attr_name: &str,
        #[case] attr_value: &str,
        #[case] attribute_selector: AttributeSelector,
        #[case] should_match: bool,
    ) {
        impl_matches_attribute_selector_for_element(attr_name, attr_value, attribute_selector, should_match)
    }

    #[rstest]
    #[case("who", "me", AttributeSelector::new("who".into(), AttributeSelectorCaseSensitivity::Identical, AttributeSelectorKind::Exact("me".into())), true)]
    #[case("who", "you", AttributeSelector::new("who".into(), AttributeSelectorCaseSensitivity::Identical, AttributeSelectorKind::Exact("me".into())), false)]
    #[case("who", "you", AttributeSelector::new("who".into(), AttributeSelectorCaseSensitivity::Identical, AttributeSelectorKind::Exact("me".into())), false)]
    #[case("who", "ME", AttributeSelector::new("who".into(), AttributeSelectorCaseSensitivity::Identical, AttributeSelectorKind::Exact("ME".into())), true)]
    #[case("who", "ME", AttributeSelector::new("who".into(), AttributeSelectorCaseSensitivity::AsciiCaseInsensitive, AttributeSelectorKind::Exact("me".into())), true)]
    #[case("who", "ME", AttributeSelector::new("who".into(), AttributeSelectorCaseSensitivity::AsciiCaseInsensitive, AttributeSelectorKind::Exact("ME".into())), true)]
    fn attribute_exact(
        #[case] attr_name: &str,
        #[case] attr_value: &str,
        #[case] attribute_selector: AttributeSelector,
        #[case] should_match: bool,
    ) {
        impl_matches_attribute_selector_for_element(attr_name, attr_value, attribute_selector, should_match)
    }

    #[rstest]
    #[case("class", "a b c d e", AttributeSelector::new("class".into(), AttributeSelectorCaseSensitivity::Identical, AttributeSelectorKind::OneOfWhitespaceSeparatedList("a".into())), true)]
    #[case("class", "a b c d e", AttributeSelector::new("class".into(), AttributeSelectorCaseSensitivity::Identical, AttributeSelectorKind::OneOfWhitespaceSeparatedList("b".into())), true)]
    #[case("class", "a b c d e", AttributeSelector::new("class".into(), AttributeSelectorCaseSensitivity::Identical, AttributeSelectorKind::OneOfWhitespaceSeparatedList("c".into())), true)]
    #[case("class", "a b c d e", AttributeSelector::new("class".into(), AttributeSelectorCaseSensitivity::Identical, AttributeSelectorKind::OneOfWhitespaceSeparatedList("d".into())), true)]
    #[case("class", "a b c d e", AttributeSelector::new("class".into(), AttributeSelectorCaseSensitivity::Identical, AttributeSelectorKind::OneOfWhitespaceSeparatedList("e".into())), true)]
    #[case("class", "a b c d e", AttributeSelector::new("class".into(), AttributeSelectorCaseSensitivity::Identical, AttributeSelectorKind::OneOfWhitespaceSeparatedList("f".into())), false)]
    #[case("class", "a b c d e", AttributeSelector::new("class".into(), AttributeSelectorCaseSensitivity::Identical, AttributeSelectorKind::OneOfWhitespaceSeparatedList("a b".into())), false)]
    #[case("class", "a b c d e", AttributeSelector::new("class".into(), AttributeSelectorCaseSensitivity::Identical, AttributeSelectorKind::OneOfWhitespaceSeparatedList("a b c d e".into())), false)]
    #[case("class", "a b c d e", AttributeSelector::new("not-class".into(), AttributeSelectorCaseSensitivity::Identical, AttributeSelectorKind::OneOfWhitespaceSeparatedList("c".into())), false)]
    #[case("class", "a b c d e", AttributeSelector::new("class".into(), AttributeSelectorCaseSensitivity::Default, AttributeSelectorKind::OneOfWhitespaceSeparatedList("".into())), false)]
    #[case("class", "a b c d e", AttributeSelector::new("class".into(), AttributeSelectorCaseSensitivity::Identical, AttributeSelectorKind::OneOfWhitespaceSeparatedList("".into())), false)]
    #[case("class", "a b c d e", AttributeSelector::new("class".into(), AttributeSelectorCaseSensitivity::AsciiCaseInsensitive, AttributeSelectorKind::OneOfWhitespaceSeparatedList("".into())), false)]
    #[case("class", "", AttributeSelector::new("class".into(), AttributeSelectorCaseSensitivity::AsciiCaseInsensitive, AttributeSelectorKind::OneOfWhitespaceSeparatedList("".into())), false)]
    fn attribute_one_of_whitespace_separated_list(
        #[case] attr_name: &str,
        #[case] attr_value: &str,
        #[case] attribute_selector: AttributeSelector,
        #[case] should_match: bool,
    ) {
        impl_matches_attribute_selector_for_element(attr_name, attr_value, attribute_selector, should_match)
    }

    #[rstest]
    #[case("data-user", "my-data", AttributeSelector::new("data-user".into(), AttributeSelectorCaseSensitivity::Identical, AttributeSelectorKind::ExactOrStartsWithAndHyphen("my-data".into())), true)]
    #[case("data-user", "my-data", AttributeSelector::new("data-user".into(), AttributeSelectorCaseSensitivity::AsciiCaseInsensitive, AttributeSelectorKind::ExactOrStartsWithAndHyphen("my-data".into())), true)]
    #[case("data-user", "my-data", AttributeSelector::new("data-user".into(), AttributeSelectorCaseSensitivity::AsciiCaseInsensitive, AttributeSelectorKind::ExactOrStartsWithAndHyphen("my-DATA".into())), true)]
    #[case("data-user", "my-data", AttributeSelector::new("data-user".into(), AttributeSelectorCaseSensitivity::Identical, AttributeSelectorKind::ExactOrStartsWithAndHyphen("my".into())), true)]
    #[case("data-user", "my-data", AttributeSelector::new("data-user".into(), AttributeSelectorCaseSensitivity::AsciiCaseInsensitive, AttributeSelectorKind::ExactOrStartsWithAndHyphen("my".into())), true)]
    #[case("data-user", "my-data", AttributeSelector::new("data-user".into(), AttributeSelectorCaseSensitivity::AsciiCaseInsensitive, AttributeSelectorKind::ExactOrStartsWithAndHyphen("MY".into())), true)]
    fn attribute_exact_or_starts_with_hyphen(
        #[case] attr_name: &str,
        #[case] attr_value: &str,
        #[case] attribute_selector: AttributeSelector,
        #[case] should_match: bool,
    ) {
        impl_matches_attribute_selector_for_element(attr_name, attr_value, attribute_selector, should_match)
    }

    #[rstest]
    #[case("lang", "en-us", AttributeSelector::new("lang".into(), AttributeSelectorCaseSensitivity::Default, AttributeSelectorKind::BeginsWith("".into())), false)]
    #[case("lang", "en-us", AttributeSelector::new("lang".into(), AttributeSelectorCaseSensitivity::Identical, AttributeSelectorKind::BeginsWith("".into())), false)]
    #[case("lang", "en-us", AttributeSelector::new("lang".into(), AttributeSelectorCaseSensitivity::AsciiCaseInsensitive, AttributeSelectorKind::BeginsWith("".into())), false)]
    #[case("lang", "en-us", AttributeSelector::new("lang".into(), AttributeSelectorCaseSensitivity::Default, AttributeSelectorKind::BeginsWith("en-gb".into())), false)]
    #[case("lang", "en-us", AttributeSelector::new("lang".into(), AttributeSelectorCaseSensitivity::Identical, AttributeSelectorKind::BeginsWith("en-gb".into())), false)]
    #[case("lang", "en-us", AttributeSelector::new("lang".into(), AttributeSelectorCaseSensitivity::AsciiCaseInsensitive, AttributeSelectorKind::BeginsWith("en-gb".into())), false)]
    #[case("lang", "en-us", AttributeSelector::new("lang".into(), AttributeSelectorCaseSensitivity::Default, AttributeSelectorKind::BeginsWith("us".into())), false)]
    #[case("lang", "en-us", AttributeSelector::new("lang".into(), AttributeSelectorCaseSensitivity::Identical, AttributeSelectorKind::BeginsWith("us".into())), false)]
    #[case("lang", "en-us", AttributeSelector::new("lang".into(), AttributeSelectorCaseSensitivity::AsciiCaseInsensitive, AttributeSelectorKind::BeginsWith("us".into())), false)]
    #[case("lang", "en-us", AttributeSelector::new("lang".into(), AttributeSelectorCaseSensitivity::Default, AttributeSelectorKind::BeginsWith("en-us".into())), true)]
    #[case("lang", "en-us", AttributeSelector::new("lang".into(), AttributeSelectorCaseSensitivity::Identical, AttributeSelectorKind::BeginsWith("en-us".into())), true)]
    #[case("lang", "en-us", AttributeSelector::new("lang".into(), AttributeSelectorCaseSensitivity::AsciiCaseInsensitive, AttributeSelectorKind::BeginsWith("en-us".into())), true)]
    #[case("lang", "en-us", AttributeSelector::new("lang".into(), AttributeSelectorCaseSensitivity::Default, AttributeSelectorKind::BeginsWith("en".into())), true)]
    #[case("lang", "en-us", AttributeSelector::new("lang".into(), AttributeSelectorCaseSensitivity::Identical, AttributeSelectorKind::BeginsWith("en".into())), true)]
    #[case("lang", "en-us", AttributeSelector::new("lang".into(), AttributeSelectorCaseSensitivity::AsciiCaseInsensitive, AttributeSelectorKind::BeginsWith("en".into())), true)]
    #[case("lang", "en-us", AttributeSelector::new("lang".into(), AttributeSelectorCaseSensitivity::Default, AttributeSelectorKind::BeginsWith("e".into())), true)]
    #[case("lang", "en-us", AttributeSelector::new("lang".into(), AttributeSelectorCaseSensitivity::Identical, AttributeSelectorKind::BeginsWith("e".into())), true)]
    #[case("lang", "en-us", AttributeSelector::new("lang".into(), AttributeSelectorCaseSensitivity::AsciiCaseInsensitive, AttributeSelectorKind::BeginsWith("e".into())), true)]
    #[case("lang", "en-us", AttributeSelector::new("lang".into(), AttributeSelectorCaseSensitivity::AsciiCaseInsensitive, AttributeSelectorKind::BeginsWith("EN".into())), true)]
    #[case("lang", "en-us", AttributeSelector::new("lang".into(), AttributeSelectorCaseSensitivity::AsciiCaseInsensitive, AttributeSelectorKind::BeginsWith("EN-US".into())), true)]
    #[case("lang", "en-us", AttributeSelector::new("lang".into(), AttributeSelectorCaseSensitivity::AsciiCaseInsensitive, AttributeSelectorKind::BeginsWith("en-US".into())), true)]
    #[case("lang", "en-us", AttributeSelector::new("lang".into(), AttributeSelectorCaseSensitivity::AsciiCaseInsensitive, AttributeSelectorKind::BeginsWith("EN-us".into())), true)]
    fn attribute_begins_with(
        #[case] attr_name: &str,
        #[case] attr_value: &str,
        #[case] attribute_selector: AttributeSelector,
        #[case] should_match: bool,
    ) {
        impl_matches_attribute_selector_for_element(attr_name, attr_value, attribute_selector, should_match)
    }

    #[rstest]
    #[case("lang", "en-us", AttributeSelector::new("lang".into(), AttributeSelectorCaseSensitivity::Default, AttributeSelectorKind::EndsWith("".into())), false)]
    #[case("lang", "en-us", AttributeSelector::new("lang".into(), AttributeSelectorCaseSensitivity::Identical, AttributeSelectorKind::EndsWith("".into())), false)]
    #[case("lang", "en-us", AttributeSelector::new("lang".into(), AttributeSelectorCaseSensitivity::AsciiCaseInsensitive, AttributeSelectorKind::EndsWith("".into())), false)]
    #[case("lang", "en-us", AttributeSelector::new("lang".into(), AttributeSelectorCaseSensitivity::Default, AttributeSelectorKind::EndsWith("en-gb".into())), false)]
    #[case("lang", "en-us", AttributeSelector::new("lang".into(), AttributeSelectorCaseSensitivity::Identical, AttributeSelectorKind::EndsWith("en-gb".into())), false)]
    #[case("lang", "en-us", AttributeSelector::new("lang".into(), AttributeSelectorCaseSensitivity::AsciiCaseInsensitive, AttributeSelectorKind::EndsWith("en-gb".into())), false)]
    #[case("lang", "en-us", AttributeSelector::new("lang".into(), AttributeSelectorCaseSensitivity::Default, AttributeSelectorKind::EndsWith("en".into())), false)]
    #[case("lang", "en-us", AttributeSelector::new("lang".into(), AttributeSelectorCaseSensitivity::Identical, AttributeSelectorKind::EndsWith("en".into())), false)]
    #[case("lang", "en-us", AttributeSelector::new("lang".into(), AttributeSelectorCaseSensitivity::AsciiCaseInsensitive, AttributeSelectorKind::EndsWith("en".into())), false)]
    #[case("lang", "en-us", AttributeSelector::new("lang".into(), AttributeSelectorCaseSensitivity::Default, AttributeSelectorKind::EndsWith("en-us".into())), true)]
    #[case("lang", "en-us", AttributeSelector::new("lang".into(), AttributeSelectorCaseSensitivity::Identical, AttributeSelectorKind::EndsWith("en-us".into())), true)]
    #[case("lang", "en-us", AttributeSelector::new("lang".into(), AttributeSelectorCaseSensitivity::AsciiCaseInsensitive, AttributeSelectorKind::EndsWith("en-us".into())), true)]
    #[case("lang", "en-us", AttributeSelector::new("lang".into(), AttributeSelectorCaseSensitivity::Default, AttributeSelectorKind::EndsWith("us".into())), true)]
    #[case("lang", "en-us", AttributeSelector::new("lang".into(), AttributeSelectorCaseSensitivity::Identical, AttributeSelectorKind::EndsWith("us".into())), true)]
    #[case("lang", "en-us", AttributeSelector::new("lang".into(), AttributeSelectorCaseSensitivity::AsciiCaseInsensitive, AttributeSelectorKind::EndsWith("us".into())), true)]
    #[case("lang", "en-us", AttributeSelector::new("lang".into(), AttributeSelectorCaseSensitivity::Default, AttributeSelectorKind::EndsWith("s".into())), true)]
    #[case("lang", "en-us", AttributeSelector::new("lang".into(), AttributeSelectorCaseSensitivity::Identical, AttributeSelectorKind::EndsWith("s".into())), true)]
    #[case("lang", "en-us", AttributeSelector::new("lang".into(), AttributeSelectorCaseSensitivity::AsciiCaseInsensitive, AttributeSelectorKind::EndsWith("s".into())), true)]
    #[case("lang", "en-us", AttributeSelector::new("lang".into(), AttributeSelectorCaseSensitivity::AsciiCaseInsensitive, AttributeSelectorKind::EndsWith("US".into())), true)]
    #[case("lang", "en-us", AttributeSelector::new("lang".into(), AttributeSelectorCaseSensitivity::AsciiCaseInsensitive, AttributeSelectorKind::EndsWith("EN-US".into())), true)]
    #[case("lang", "en-us", AttributeSelector::new("lang".into(), AttributeSelectorCaseSensitivity::AsciiCaseInsensitive, AttributeSelectorKind::EndsWith("en-US".into())), true)]
    #[case("lang", "en-us", AttributeSelector::new("lang".into(), AttributeSelectorCaseSensitivity::AsciiCaseInsensitive, AttributeSelectorKind::EndsWith("EN-us".into())), true)]
    fn attribute_ends_with(
        #[case] attr_name: &str,
        #[case] attr_value: &str,
        #[case] attribute_selector: AttributeSelector,
        #[case] should_match: bool,
    ) {
        impl_matches_attribute_selector_for_element(attr_name, attr_value, attribute_selector, should_match)
    }

    #[rstest]
    #[case("data-string", "Lorem ipsum!", AttributeSelector::new("data-string".into(), AttributeSelectorCaseSensitivity::Default, AttributeSelectorKind::Contains("".into())), false)]
    #[case("data-string", "Lorem ipsum!", AttributeSelector::new("data-string".into(), AttributeSelectorCaseSensitivity::Identical, AttributeSelectorKind::Contains("".into())), false)]
    #[case("data-string", "Lorem ipsum!", AttributeSelector::new("data-string".into(), AttributeSelectorCaseSensitivity::AsciiCaseInsensitive, AttributeSelectorKind::Contains("".into())), false)]
    #[case("data-string", "Lorem ipsum!", AttributeSelector::new("data-string".into(), AttributeSelectorCaseSensitivity::Default, AttributeSelectorKind::Contains("a".into())), false)]
    #[case("data-string", "Lorem ipsum!", AttributeSelector::new("data-string".into(), AttributeSelectorCaseSensitivity::Identical, AttributeSelectorKind::Contains("a".into())), false)]
    #[case("data-string", "Lorem ipsum!", AttributeSelector::new("data-string".into(), AttributeSelectorCaseSensitivity::AsciiCaseInsensitive, AttributeSelectorKind::Contains("a".into())), false)]
    #[case("data-string", "Lorem ipsum!", AttributeSelector::new("data-string".into(), AttributeSelectorCaseSensitivity::Default, AttributeSelectorKind::Contains("\0".into())), false)]
    #[case("data-string", "Lorem ipsum!", AttributeSelector::new("data-string".into(), AttributeSelectorCaseSensitivity::Identical, AttributeSelectorKind::Contains("\0".into())), false)]
    #[case("data-string", "Lorem ipsum!", AttributeSelector::new("data-string".into(), AttributeSelectorCaseSensitivity::AsciiCaseInsensitive, AttributeSelectorKind::Contains("\0".into())), false)]
    #[case("data-string", "Lorem ipsum!", AttributeSelector::new("data-string".into(), AttributeSelectorCaseSensitivity::Default, AttributeSelectorKind::Contains("dolor".into())), false)]
    #[case("data-string", "Lorem ipsum!", AttributeSelector::new("data-string".into(), AttributeSelectorCaseSensitivity::Identical, AttributeSelectorKind::Contains("dolor".into())), false)]
    #[case("data-string", "Lorem ipsum!", AttributeSelector::new("data-string".into(), AttributeSelectorCaseSensitivity::AsciiCaseInsensitive, AttributeSelectorKind::Contains("dolor".into())), false)]
    #[case("data-string", "Lorem ipsum!", AttributeSelector::new("data-string".into(), AttributeSelectorCaseSensitivity::Default, AttributeSelectorKind::Contains("Lorem".into())), true)]
    #[case("data-string", "Lorem ipsum!", AttributeSelector::new("data-string".into(), AttributeSelectorCaseSensitivity::Identical, AttributeSelectorKind::Contains("Lorem".into())), true)]
    #[case("data-string", "Lorem ipsum!", AttributeSelector::new("data-string".into(), AttributeSelectorCaseSensitivity::AsciiCaseInsensitive, AttributeSelectorKind::Contains("Lorem".into())), true)]
    #[case("data-string", "Lorem ipsum!", AttributeSelector::new("data-string".into(), AttributeSelectorCaseSensitivity::Default, AttributeSelectorKind::Contains("ipsum".into())), true)]
    #[case("data-string", "Lorem ipsum!", AttributeSelector::new("data-string".into(), AttributeSelectorCaseSensitivity::Identical, AttributeSelectorKind::Contains("ipsum".into())), true)]
    #[case("data-string", "Lorem ipsum!", AttributeSelector::new("data-string".into(), AttributeSelectorCaseSensitivity::AsciiCaseInsensitive, AttributeSelectorKind::Contains("ipsum".into())), true)]
    #[case("data-string", "Lorem ipsum!", AttributeSelector::new("data-string".into(), AttributeSelectorCaseSensitivity::Default, AttributeSelectorKind::Contains("Lorem ipsum!".into())), true)]
    #[case("data-string", "Lorem ipsum!", AttributeSelector::new("data-string".into(), AttributeSelectorCaseSensitivity::Identical, AttributeSelectorKind::Contains("Lorem ipsum!".into())), true)]
    #[case("data-string", "Lorem ipsum!", AttributeSelector::new("data-string".into(), AttributeSelectorCaseSensitivity::AsciiCaseInsensitive, AttributeSelectorKind::Contains("Lorem ipsum!".into())), true)]
    #[case("data-string", "Lorem ipsum!", AttributeSelector::new("data-string".into(), AttributeSelectorCaseSensitivity::Default, AttributeSelectorKind::Contains("!".into())), true)]
    #[case("data-string", "Lorem ipsum!", AttributeSelector::new("data-string".into(), AttributeSelectorCaseSensitivity::Identical, AttributeSelectorKind::Contains("!".into())), true)]
    #[case("data-string", "Lorem ipsum!", AttributeSelector::new("data-string".into(), AttributeSelectorCaseSensitivity::AsciiCaseInsensitive, AttributeSelectorKind::Contains("!".into())), true)]
    #[case("data-string", "Lorem ipsum!", AttributeSelector::new("data-string".into(), AttributeSelectorCaseSensitivity::Default, AttributeSelectorKind::Contains("orem ipsu".into())), true)]
    #[case("data-string", "Lorem ipsum!", AttributeSelector::new("data-string".into(), AttributeSelectorCaseSensitivity::Identical, AttributeSelectorKind::Contains("orem ipsu".into())), true)]
    #[case("data-string", "Lorem ipsum!", AttributeSelector::new("data-string".into(), AttributeSelectorCaseSensitivity::AsciiCaseInsensitive, AttributeSelectorKind::Contains("orem ipsu".into())), true)]
    #[case("data-string", "Lorem ipsum!", AttributeSelector::new("data-string".into(), AttributeSelectorCaseSensitivity::AsciiCaseInsensitive, AttributeSelectorKind::Contains("LOREM IPSUM!".into())), true)]
    #[case("data-string", "Lorem ipsum!", AttributeSelector::new("data-string".into(), AttributeSelectorCaseSensitivity::AsciiCaseInsensitive, AttributeSelectorKind::Contains("IPSUM".into())), true)]
    #[case("data-string", "Lorem ipsum!", AttributeSelector::new("data-string".into(), AttributeSelectorCaseSensitivity::AsciiCaseInsensitive, AttributeSelectorKind::Contains("IPSUM!".into())), true)]
    #[case("data-string", "Lorem ipsum!", AttributeSelector::new("data-string".into(), AttributeSelectorCaseSensitivity::AsciiCaseInsensitive, AttributeSelectorKind::Contains("lorem ipsum".into())), true)]
    #[case("data-string", "Lorem ipsum!", AttributeSelector::new("data-string".into(), AttributeSelectorCaseSensitivity::AsciiCaseInsensitive, AttributeSelectorKind::Contains("lorem ipsum!".into())), true)]
    #[case("data-string", "Lorem ipsum!", AttributeSelector::new("data-string".into(), AttributeSelectorCaseSensitivity::AsciiCaseInsensitive, AttributeSelectorKind::Contains("IpSum!".into())), true)]
    fn attribute_contains(
        #[case] attr_name: &str,
        #[case] attr_value: &str,
        #[case] attribute_selector: AttributeSelector,
        #[case] should_match: bool,
    ) {
        impl_matches_attribute_selector_for_element(attr_name, attr_value, attribute_selector, should_match)
    }

    #[rstest]
    #[case("", "", false)]
    #[case("", "active", false)]
    #[case("active", "", false)]
    #[case("black active bg-red", "", false)]
    #[case("active", "active", true)]
    #[case("active", "ACTIVE", false)]
    #[case("active", "list-item", false)]
    #[case("list-item active", "active", true)]
    #[case("list-item active", "list-item active", false)]
    #[case("list-item active", "active list-item", false)]
    #[case("list-item active", "list-item", true)]
    fn class_test(#[case] haystack: &str, #[case] needle: &str, #[case] should_match: bool) {
        let mut element = HtmlElementKind::Unknown(HtmlUnknownElement::new(qual_name("p")));
        element.as_dom_element_mut().attributes_mut().set("class", haystack.into());

        assert!(matches_selector(&Selector::Simple(SimpleSelector::Class(needle.into())), &NodeKind::HtmlElement(element)) == should_match);
    }

    #[rstest]
    #[case("", "", false)]
    #[case("my-form", "", false)]
    #[case("", "my-form", false)]
    #[case("my-form", "my-form", true)]
    #[case("my-form", "my-other-form", false)]
    #[case("my-other-form", "my-form", false)]
    fn id_test(#[case] haystack: &str, #[case] needle: &str, #[case] should_match: bool) {
        let mut element = HtmlElementKind::Unknown(HtmlUnknownElement::new(qual_name("p")));
        element.as_dom_element_mut().attributes_mut().set("id", haystack.into());

        assert!(matches_selector(&Selector::Simple(SimpleSelector::Id(needle.into())), &NodeKind::HtmlElement(element)) == should_match);
    }

    #[test]
    fn text_node() {
        let node = &NodeKind::Text(Text::new(StrTendril::new()));

        let universal_selector = Selector::Simple(SimpleSelector::Universal);
        assert!(universal_selector.matches(node));

        let universal_selector_in_selector_list = SelectorList{ items: vec![universal_selector] };
        assert!(universal_selector_in_selector_list.matches(node));
    }

    #[rstest]
    #[case(NodeKind::HtmlElement(HtmlElementKind::Style(HtmlStyleElement::new(qual_name("style")))), true)]
    #[case(NodeKind::HtmlElement(HtmlElementKind::Unknown(HtmlUnknownElement::new(qual_name("br")))), false)]
    #[case(NodeKind::HtmlElement(HtmlElementKind::Unknown(HtmlUnknownElement::new(qual_name("p")))), false)]
    #[case(NodeKind::Text(Text::new(StrTendril::new())), false)]
    fn element_with_type(#[case] node: NodeKind, #[case] should_match: bool) {
        let universal_selector = Selector::Simple(SimpleSelector::TypeSelector("style".into()));
        assert!(universal_selector.matches(&node) == should_match);

        let universal_selector_in_selector_list = SelectorList{ items: vec![universal_selector] };
        assert!(universal_selector_in_selector_list.matches(&node) == should_match);
    }

    fn impl_matches_attribute_selector_for_element(
        attr_name: &str,
        attr_value: &str,
        attribute_selector: AttributeSelector,
        should_match: bool,
    ) {
        let mut element = HtmlElementKind::Unknown(HtmlUnknownElement::new(qual_name("p")));
        let element = element.as_dom_element_mut();
        element.attributes_mut().set(attr_name, attr_value.into());

        assert!(matches_attribute_selector(&attribute_selector, element) == should_match);
    }
}
