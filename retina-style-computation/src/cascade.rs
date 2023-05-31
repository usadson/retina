// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use retina_style::{StyleRule, CascadeOrigin};

use crate::{ComputedStyle, CollectedStyles};

fn cascade_normal_declarations_for_origin(
    computed_style: &mut ComputedStyle,
    rules: &[&StyleRule],
    origin: CascadeOrigin,
) {
    for rule in rules {
        if rule.cascade_origin != origin {
            continue;
        }

        for declaration in &rule.declarations {
            computed_style.values.insert(declaration.property(), declaration.value().clone());
        }
    }
}

trait Cascade {
    fn cascade(&self) -> ComputedStyle;
}

impl<'stylesheets> Cascade for CollectedStyles<'stylesheets> {
    fn cascade(&self) -> ComputedStyle {
        let mut computed_style = ComputedStyle::new();

        // Declarations from origins earlier in this list win over declarations
        // from later origins.

        // 8. Normal user-agent declarations
        cascade_normal_declarations_for_origin(&mut computed_style, self.applicable_rules(), CascadeOrigin::UserAgent);

        // 7. Normal user declarations
        cascade_normal_declarations_for_origin(&mut computed_style, self.applicable_rules(), CascadeOrigin::User);

        // 6. Normal author declarations
        cascade_normal_declarations_for_origin(&mut computed_style, self.applicable_rules(), CascadeOrigin::Author);

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

        computed_style
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use retina_dom::{NodeKind, Text};
    use retina_style::{BasicColorKeyword, ColorValue, Property, Value, Stylesheet};

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

        let mut expected = HashMap::new();
        expected.insert(Property::Color, Value::Color(ColorValue::BasicColorKeyword(BasicColorKeyword::Blue)));

        assert_eq!(cascaded_style.values, expected);
    }

}
