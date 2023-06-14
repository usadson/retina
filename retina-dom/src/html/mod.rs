// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

pub mod html_element;
pub mod html_style_element;
pub mod html_unknown_element;

use html5ever::{LocalName, Namespace, QualName};
pub use html_element::HtmlElement;
pub use html_unknown_element::HtmlUnknownElement;
pub use self::html_style_element::HtmlStyleElement;

use crate::{Element, NodeInterface};

#[derive(Debug)]
pub enum HtmlElementKind {
    Style(HtmlStyleElement),
    Unknown(HtmlUnknownElement),
}

impl HtmlElementKind {
    pub fn as_dom_element(&self) -> &Element {
        match self {
            Self::Style(element) => element.as_ref(),
            Self::Unknown(element) => element.as_ref(),
        }
    }

    pub fn as_dom_element_mut(&mut self) -> &mut Element {
        match self {
            Self::Style(element) => element.as_mut(),
            Self::Unknown(element) => element.as_mut(),
        }
    }

    pub fn as_style_element(&self) -> Option<&HtmlStyleElement> {
        if let Self::Style(element) = self {
            Some(element)
        } else {
            None
        }
    }

    pub fn as_node(&self) -> &NodeInterface {
        self.as_dom_element().as_node()
    }

    pub fn as_node_mut(&mut self) -> &mut NodeInterface {
        self.as_dom_element_mut().as_node_mut()
    }
}

pub fn qual_name(name: &str) -> QualName {
    QualName {
        prefix: None,
        ns: Namespace::default(),
        local: LocalName::from(name),
    }
}
