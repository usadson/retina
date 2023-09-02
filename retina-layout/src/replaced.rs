// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use retina_dom::{Node, qual_name};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum ReplacedElementType {
    Button,
    Checkbox,
    InputButton,
    InputText,
}

impl ReplacedElementType {
    pub fn detect(node: &Node) -> Option<Self> {
        let element = node.as_html_element_kind()?;
        if *element.as_dom_element().qualified_name() == qual_name("button") {
            return Some(Self::Button);
        }

        if *element.as_dom_element().qualified_name().local != qual_name("input").local {
            return None;
        }

        let ty = element.as_dom_element().attributes().find_by_str("type").unwrap_or("text");
        match ty {
            "button" | "submit" => Some(Self::InputButton),
            "checkbox" => Some(Self::Checkbox),
            _ => Some(Self::InputText),
        }
    }
}
