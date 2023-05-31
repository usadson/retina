// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use retina_dom::NodeKind;

use retina_style::{
    Declaration,
    Rule,
    Stylesheet,
    StyleRule,
};

use crate::selector_match::{matches_selector, SelectorMatcher};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CollectedStyles<'stylesheets> {
    applicable_rules: Vec<&'stylesheets StyleRule>,
}

impl<'stylesheets> CollectedStyles<'stylesheets> {
    pub fn new() -> Self {
        Self {
            applicable_rules: Vec::new(),
        }
    }
}

pub struct StyleCollector<'stylesheets> {
    stylesheets: &'stylesheets [Stylesheet],
}

impl<'stylesheets> StyleCollector<'stylesheets> {
    pub fn new(stylesheets: &'stylesheets [Stylesheet]) -> Self {
        Self {
            stylesheets
        }
    }

    pub fn collect(&self, node: &NodeKind) -> CollectedStyles<'stylesheets> {
        let mut collected_styles = CollectedStyles::new();

        for sheet in self.stylesheets {
            for rule in sheet.rules() {
                if let Rule::Style(rule) = rule {
                    if rule.selector_list.matches(node) {
                        collected_styles.applicable_rules.push(rule);
                        continue;
                    }
                }
            }
        }

        collected_styles
    }
}

#[cfg(test)]
mod tests {
    use retina_dom::Text;

    use super::*;

    #[test]
    fn stylesheet_single_rule_single_declaration_text_node() {
        let stylesheets = &[
            Stylesheet::parse("* {
                color: white;
            }")
        ];

        let node = &NodeKind::Text(Text::new(String::new()));

        let collected = StyleCollector::new(stylesheets).collect(node);
        assert_eq!(collected, CollectedStyles{
            applicable_rules: vec![stylesheets[0].rules()[0].try_as_style().unwrap()]
        });
    }

}
