// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use retina_dom::NodeKind;
use retina_style::{Selector, SimpleSelector, SelectorList};

/// Checks whether or not the given node matches the selector.
pub fn matches_selector(selector: &Selector, node: &NodeKind) -> bool {
    _ = node;
    match selector {
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
}
