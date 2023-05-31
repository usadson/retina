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

trait Cascade {
    fn cascade(&self) -> PropertyMap;
}

impl<'stylesheets> Cascade for CollectedStyles<'stylesheets> {
    fn cascade(&self) -> PropertyMap {
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

        property_map
    }
}

#[cfg(test)]
mod tests {
    use retina_dom::{NodeKind, Text};
    use retina_style::{BasicColorKeyword, ColorValue, Stylesheet};

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

        let node = &NodeKind::Text(Text::new(String::new()));

        let collected_styles = StyleCollector::new(&stylesheets).collect(node);
        let cascaded_style = collected_styles.cascade();

        let expected = PropertyMap {
            color: Some(ColorValue::BasicColorKeyword(BasicColorKeyword::Blue)),

            ..Default::default()
        };

        assert_eq!(cascaded_style, expected);
    }

}
