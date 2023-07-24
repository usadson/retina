// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use log::warn;

use retina_dom::{
    AttributeList,
    HtmlElementKind,
};

use retina_style::{
    CascadeOrigin,
    CssLength,
    Rule,
};

use retina_style_parser::CssAttributeStrExtensions;

use crate::{
    ApplicableRule,
    CollectedStyles,
    PropertyMap,
};

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

fn cascade_styles_from_presentational_hints(
    property_map: &mut PropertyMap,
    element: &HtmlElementKind,
 ) {
    let attributes = element.as_dom_element().attributes();
    match element.as_dom_element().qualified_name().local.as_ref() {
        "body" => cascade_styles_from_presentational_hints_body(property_map, attributes),
        _ => (),
    }
}

/// <https://html.spec.whatwg.org/multipage/rendering.html#the-page>
fn cascade_styles_from_presentational_hints_body(
    property_map: &mut PropertyMap,
    attributes: &AttributeList,
 ) {
    if let Some(background_color) = attributes.find_by_str("bgcolor").and_then(CssAttributeStrExtensions::parse_legacy_color_value) {
        property_map.background_color = Some(background_color);
    }

    if let Some(text_color) = attributes.find_by_str("text").and_then(CssAttributeStrExtensions::parse_legacy_color_value) {
        property_map.color = Some(text_color);
    }

    property_map.margin_top = Some(cascade_styles_from_presentational_hints_body_margin(
        attributes,
        "marginheight",
        "topmargin",
    ));

    property_map.margin_right = Some(cascade_styles_from_presentational_hints_body_margin(
        attributes,
        "marginwidth",
        "rightmargin",
    ));

    property_map.margin_bottom = Some(cascade_styles_from_presentational_hints_body_margin(
        attributes,
        "marginheight",
        "bottommargin",
    ));

    property_map.margin_left = Some(cascade_styles_from_presentational_hints_body_margin(
        attributes,
        "marginwidth",
        "leftmargin",
    ));
}

/// <https://html.spec.whatwg.org/multipage/rendering.html#the-page>
fn cascade_styles_from_presentational_hints_body_margin(
    attributes: &AttributeList,
    attr_a: &str,
    attr_b: &str,
) -> CssLength {
    if let Some(attr_a) = attributes.find_by_str(attr_a) {
        if let Some(pixels) = attr_a.html_map_to_the_pixel_length_property() {
            return CssLength::Pixels(pixels as _);
        }
    } else if let Some(attr_b) = attributes.find_by_str(attr_b) {
        if let Some(pixels) = attr_b.html_map_to_the_pixel_length_property() {
            return CssLength::Pixels(pixels as _);
        }
    }

    // Default value of 8 pixels
    CssLength::Pixels(8.0)
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
    inherit_property(&mut property_map.font_kerning, &parent.font_kerning);
    inherit_property(&mut property_map.font_size, &parent.font_size);
    inherit_property(&mut property_map.font_style, &parent.font_style);
    inherit_property(&mut property_map.font_variant_caps, &parent.font_variant_caps);
    inherit_property(&mut property_map.font_variant_ligatures, &parent.font_variant_ligatures);
    inherit_property(&mut property_map.font_variant_position, &parent.font_variant_position);
    inherit_property(&mut property_map.font_weight, &parent.font_weight);
    inherit_property(&mut property_map.text_transform, &parent.text_transform);
    inherit_property(&mut property_map.white_space, &parent.white_space);

    // This is incorrect, but I'm not sure what the spec means by inheritance
    // through the box tree...
    // <https://drafts.csswg.org/css-text-decor-4/#ref-for-propdef-display>
    // Probably something along the lines of "at box generation time, only
    // boxes that are generated by an HTML element, but not pseudo-elements"?
    inherit_property(&mut property_map.text_decoration_color, &parent.text_decoration_color);
    inherit_property(&mut property_map.text_decoration_line, &parent.text_decoration_line);
    inherit_property(&mut property_map.text_decoration_style, &parent.text_decoration_style);
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
        if let Some(node) = node.and_then(|node| node.as_html_element_kind()) {
            cascade_styles_from_presentational_hints(&mut property_map, node);
        }

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
