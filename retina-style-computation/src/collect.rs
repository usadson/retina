// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use retina_dom::NodeKind;

use retina_style::{
    Rule,
    Stylesheet,
    StyleRule,
};

use crate::SelectorMatcher;

#[derive(Clone, Debug, PartialEq)]
pub struct CollectedStyles<'stylesheets> {
    applicable_rules: Vec<&'stylesheets StyleRule>,
}

impl<'stylesheets> CollectedStyles<'stylesheets> {
    pub fn new() -> Self {
        Self {
            applicable_rules: Vec::new(),
        }
    }

    /// Get the rules that are applicable to this node.
    pub fn applicable_rules(&self) -> &[&'stylesheets StyleRule] {
        &self.applicable_rules
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
    use retina_style::CascadeOrigin;
    use tendril::StrTendril;

    use super::*;
    use retina_style_parser::CssParsable;

    #[test]
    fn stylesheet_single_rule_single_declaration_text_node() {
        let stylesheets = &[
            Stylesheet::parse(CascadeOrigin::UserAgent, "* {
                color: white;
            }")
        ];

        let node = &NodeKind::Text(Text::new(StrTendril::new()));

        let collected = StyleCollector::new(stylesheets).collect(node);
        assert_eq!(collected, CollectedStyles{
            applicable_rules: vec![stylesheets[0].rules()[0].try_as_style().unwrap()]
        });
    }

}
