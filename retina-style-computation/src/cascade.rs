// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use retina_style::{StyleRule, CascadeOrigin};

use crate::{CollectedStyles, PropertyMap};

fn cascade_normal_declarations_for_origin(
    property_map: &mut PropertyMap,
    rules: &[&StyleRule],
    origin: CascadeOrigin,
) {
    for rule in rules {
        if rule.cascade_origin != origin {
            continue;
        }

        for declaration in &rule.declarations {
            property_map.apply_property(declaration.property(), declaration.value().clone());
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
    // `display` is not inherited
    inherit_property(&mut property_map.white_space, &parent.white_space);
}

pub trait Cascade {
    fn cascade(&self, parent: Option<&PropertyMap>) -> PropertyMap;
}

impl<'stylesheets> Cascade for CollectedStyles<'stylesheets> {
    fn cascade(&self, parent: Option<&PropertyMap>) -> PropertyMap {
        let mut property_map = PropertyMap::new();

        // Declarations from origins earlier in this list win over declarations
        // from later origins.

        // 8. Normal user-agent declarations
        cascade_normal_declarations_for_origin(&mut property_map, self.applicable_rules(), CascadeOrigin::UserAgent);

        // 7. Normal user declarations
        cascade_normal_declarations_for_origin(&mut property_map, self.applicable_rules(), CascadeOrigin::User);

        // 6. Normal author declarations
        cascade_normal_declarations_for_origin(&mut property_map, self.applicable_rules(), CascadeOrigin::Author);

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

        if let Some(parent) = parent {
            inherit_properties(&mut property_map, parent);
        }

        property_map
    }
}

#[cfg(test)]
mod tests {


    use retina_dom::{NodeKind, Text, Document, Node};
    use retina_style::*;
    use retina_style_parser::CssParsable;
    use tendril::StrTendril;

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
        let cascaded_style = collected_styles.cascade(None);

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
        parent_node.as_parent_node_mut().children().borrow_mut().push(
            Node::clone(&node)
        );

        let parent_node = NodeKind::Document(parent_node);

        let parent_collected_styles = StyleCollector::new(&stylesheets).collect(&parent_node);
        let parent_cascaded_styles = parent_collected_styles.cascade(None);

        assert_eq!(parent_cascaded_styles, PropertyMap {
            color: Some(CssNamedColor::BLUE),
            display: Some(CssDisplay::Normal { inside: CssDisplayInside::Flow, outside: CssDisplayOutside::Block, is_list_item: false }),
            ..Default::default()
        });

        let node_collected_styles = CollectedStyles::new();
        let node_cascaded_style = node_collected_styles.cascade(Some(&parent_cascaded_styles));

        assert_eq!(node_cascaded_style, PropertyMap {
            color: Some(CssNamedColor::BLUE),
            display: None,

            ..Default::default()
        });
    }

}
