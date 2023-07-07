// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use log::warn;
use retina_style::{CascadeOrigin, Rule};

use crate::{CollectedStyles, PropertyMap, collect::ApplicableRule};

fn cascade_normal_declarations_for_origin(
    property_map: &mut PropertyMap,
    applicable_rules: &[ApplicableRule],
    origin: CascadeOrigin,
) {
    let mut applicable_rules: Vec<_> = applicable_rules.to_vec();
    applicable_rules.sort_by(|a, b| a.specificity.cmp(&b.specificity));

    for applicable_rule in applicable_rules.iter() {
        if applicable_rule.rule.cascade_origin != origin {
            continue;
        }

        for declaration in &applicable_rule.rule.declarations {
            property_map.apply_property(declaration.property(), declaration.value().clone());
        }
    }
}

fn cascade_styles_from_attribute(
    property_map: &mut PropertyMap,
    node: &retina_dom::NodeKind,
 ) {
    let Some(element) = node.as_dom_element() else { return };
    let Some(style_attribute) = element.attributes().find_by_str("style") else { return };

    if style_attribute.trim().is_empty() {
        return;
    }

    match retina_style_parser::parse_style_attribute(style_attribute) {
        Ok(Rule::Style(style_rule)) => {
            cascade_normal_declarations_for_origin(
                property_map,
                &[
                    ApplicableRule {
                        specificity: crate::SelectorSpecificity::new_for_style_attribute(),
                        rule: &style_rule,
                    }
                ],
                CascadeOrigin::Author,
            );
        }

        Ok(..) => unreachable!(),

        Err(e) => {
            warn!("Failed to parse style attribute: {e:#?}");
            warn!("Style attribute contents: {style_attribute}");
        }
    }
}

fn inherit_property<T>(target: &mut Option<T>, source: &Option<T>)
        where T: Clone {
    if target.is_none() {
        *target = source.clone();
    }
}

fn inherit_properties(property_map: &mut PropertyMap, parent: &PropertyMap) {
    inherit_property(&mut property_map.color, &parent.color);
    inherit_property(&mut property_map.font_family_list, &parent.font_family_list);
    inherit_property(&mut property_map.font_size, &parent.font_size);
    inherit_property(&mut property_map.font_style, &parent.font_style);
    // `display` is not inherited
    inherit_property(&mut property_map.white_space, &parent.white_space);
}

pub trait Cascade {
    fn cascade(
        &self,
        node: Option<&retina_dom::NodeKind>,
        parent: Option<&PropertyMap>,
    ) -> PropertyMap;
}

impl<'stylesheets> Cascade for CollectedStyles<'stylesheets> {
    fn cascade(
        &self,
        node: Option<&retina_dom::NodeKind>,
        parent: Option<&PropertyMap>,
    ) -> PropertyMap {
        let mut property_map = PropertyMap::new();

        if let Some(parent) = parent {
            inherit_properties(&mut property_map, parent);
        }

        // Declarations from origins earlier in this list win over declarations
        // from later origins.

        // 8. Normal user-agent declarations
        cascade_normal_declarations_for_origin(&mut property_map, self.applicable_rules(), CascadeOrigin::UserAgent);

        // 7. Normal user declarations
        cascade_normal_declarations_for_origin(&mut property_map, self.applicable_rules(), CascadeOrigin::User);

        // 6. Normal author declarations
        cascade_normal_declarations_for_origin(&mut property_map, self.applicable_rules(), CascadeOrigin::Author);

        if let Some(node) = node {
            cascade_styles_from_attribute(&mut property_map, node);
        }

        // 5. Animation declarations [css-animations-1]
        // TODO

        // 4. Important author declarations
        // TODO

        // 3. Important user declarations
        // TODO

        // 2. Important user agent declarations
        // TODO

        // 1. Transition declarations [css-transitions-1]
        // TODO

        property_map
    }
}

#[cfg(test)]
mod tests {


    use retina_dom::{NodeKind, Text, Document, Node};
    use retina_style::*;
    use retina_style_parser::CssParsable;
    use retina_common::StrTendril;

    use crate::*;
    use super::*;

    #[test]
    fn normal_importance() {
        let stylesheets = [
            Stylesheet::parse(CascadeOrigin::User, "
                * {
                    color: green;
                }
            "),
            Stylesheet::parse(CascadeOrigin::Author, "
                * {
                    color: blue;
                }
            "),
            Stylesheet::parse(CascadeOrigin::UserAgent, "
                * {
                    color: yellow;
                }
            "),
        ];

        let node = &NodeKind::Text(Text::new(StrTendril::new()));

        let collected_styles = StyleCollector::new(&stylesheets).collect(node);
        let cascaded_style = collected_styles.cascade(None, None);

        let expected = PropertyMap {
            color: Some(CssNamedColor::BLUE),

            ..Default::default()
        };

        assert_eq!(cascaded_style, expected);
    }

    #[test]
    fn inherit_test() {
        let stylesheets = [
            Stylesheet::parse(CascadeOrigin::User, "
                * {
                    color: green;
                    display: block;
                }
            "),
            Stylesheet::parse(CascadeOrigin::Author, "
                * {
                    color: blue;
                }
            "),
            Stylesheet::parse(CascadeOrigin::UserAgent, "
                * {
                    color: yellow;
                }
            "),
        ];

        let node = Node::new(
            NodeKind::Text(Text::new(StrTendril::new()))
        );

        let mut parent_node = Document::new();
        parent_node.as_parent_node_mut().children_mut().push(
            Node::clone(&node)
        );

        let parent_node = NodeKind::Document(parent_node);

        let parent_collected_styles = StyleCollector::new(&stylesheets).collect(&parent_node);
        let parent_cascaded_styles = parent_collected_styles.cascade(None, None);

        assert_eq!(parent_cascaded_styles, PropertyMap {
            color: Some(CssNamedColor::BLUE),
            display: Some(CssDisplay::Normal { inside: CssDisplayInside::Flow, outside: CssDisplayOutside::Block, is_list_item: false }),
            ..Default::default()
        });

        let node_collected_styles = CollectedStyles::new();
        let node_cascaded_style = node_collected_styles.cascade(None, Some(&parent_cascaded_styles));

        assert_eq!(node_cascaded_style, PropertyMap {
            color: Some(CssNamedColor::BLUE),
            display: None,

            ..Default::default()
        });
    }

}
