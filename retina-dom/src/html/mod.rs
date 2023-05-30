// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

pub mod html_element;
pub mod html_unknown_element;

pub use html_element::HtmlElement;
pub use html_unknown_element::HtmlUnknownElement;

use crate::{Element, Node};

#[derive(Debug)]
pub enum HtmlElementKind {
    Unknown(HtmlUnknownElement),
}

impl HtmlElementKind {
    pub fn as_dom_element(&self) -> &Element {
        match self {
            Self::Unknown(element) => element.as_ref(),
        }
    }

    pub fn as_dom_element_mut(&mut self) -> &mut Element {
        match self {
            Self::Unknown(element) => element.as_mut(),
        }
    }

    pub fn as_node(&self) -> &Node {
        self.as_dom_element().as_node()
    }

    pub fn as_node_mut(&mut self) -> &mut Node {
        self.as_dom_element_mut().as_node_mut()
    }
}
