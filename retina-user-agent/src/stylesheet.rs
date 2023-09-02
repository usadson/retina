// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

//! This file contains the User-Agent stylesheet which defines the foundation
//! upon which author stylesheets are built on.
//!
//! This stylesheet is largely (if not entirely) part of the [_HTML Living
//! Standard CSS User Agent style sheet_][html].
//!
//! [html]: https://html.spec.whatwg.org/multipage/rendering.html#rendering

/// *See the [module-level documentation][self] for details.*
pub const USER_AGENT_STYLESHEET_CODE: &str = include_str!("stylesheet.css");

#[cfg(test)]
mod tests {
    use retina_style::{
        CascadeOrigin,
        CssDisplay,
        CssDisplayInside,
        CssDisplayOutside,
        Declaration,
        Property,
        Selector,
        SelectorList,
        SimpleSelector,
        Stylesheet,
        Value,
    };

    use retina_style_parser::CssParsable;

    use super::USER_AGENT_STYLESHEET_CODE;
    use pretty_assertions::assert_eq;

    #[test]
    fn parse_basic() {
        let stylesheet = Stylesheet::parse(CascadeOrigin::UserAgent, USER_AGENT_STYLESHEET_CODE);
        assert!(stylesheet.rules().len() >= 40);

        let style_rule = stylesheet.rules()[1].try_as_style().expect("not a style rule");
        assert_eq!(
            style_rule.selector_list,
            SelectorList{ items: vec![
                Selector::Simple(SimpleSelector::TypeSelector("html".into())),
                Selector::Simple(SimpleSelector::TypeSelector("body".into())),
            ]}
        );

        assert_eq!(style_rule.declarations, vec![
            Declaration::new(Property::Display, Value::Display(CssDisplay::Normal {
                inside: CssDisplayInside::Flow,
                outside: CssDisplayOutside::Block,
                is_list_item: false
            }))
        ])
    }

}
