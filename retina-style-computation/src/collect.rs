// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use retina_dom::NodeKind;

use retina_style::{
    MediaQuery,
    MediaType,
    Rule,
    Stylesheet,
    StyleRule,
};

use crate::{SelectorMatcher, SelectorSpecificity, selector_specificity::CalculateSpecificity};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ApplicableRule<'stylesheet> {
    pub(crate) rule: &'stylesheet StyleRule,
    pub(crate) specificity: SelectorSpecificity,
}

impl<'stylesheet> PartialOrd for ApplicableRule<'stylesheet> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.specificity.partial_cmp(&other.specificity)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CollectedStyles<'stylesheets> {
    applicable_rules: Vec<ApplicableRule<'stylesheets>>,
}

impl<'stylesheets> CollectedStyles<'stylesheets> {
    pub fn new() -> Self {
        Self {
            applicable_rules: Vec::new(),
        }
    }

    /// Get the rules that are applicable to this node.
    pub fn applicable_rules(&self) -> &[ApplicableRule] {
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
            self.collect_for_style_sheet(node, sheet, &mut collected_styles);
        }

        collected_styles
    }

    fn collect_for_style_sheet(
        &self,
        node: &NodeKind,
        stylesheet: &'stylesheets Stylesheet,
        collected_styles: &mut CollectedStyles<'stylesheets>
    ) {
        for rule in stylesheet.rules() {
            match rule {
                Rule::AtMedia(media) => {
                    if media.media_query_list[0] != MediaQuery::Type(MediaType::Print) {
                        self.collect_for_style_sheet(node, &media.stylesheet, collected_styles);
                    }
                }

                Rule::Style(rule) => {
                    if let Some(selector) = rule.selector_list.most_specific_match(node) {
                        collected_styles.applicable_rules.push(ApplicableRule {
                            rule,
                            specificity: selector.calculate_specificity()
                        });
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use retina_dom::Text;
    use retina_style::CascadeOrigin;
    use retina_common::StrTendril;

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
            applicable_rules: vec![
                ApplicableRule {
                    rule: stylesheets[0].rules()[0].try_as_style().unwrap(),
                    specificity: Default::default(),
                }
            ]
        });
    }

}
