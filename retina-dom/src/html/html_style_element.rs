// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use html5ever::QualName;
use tendril::StrTendril;

use crate::{
    Element,
    html::HtmlElement, NodeKind,
};

#[derive(Debug)]
pub struct HtmlStyleElement {
    superclass_html_element: HtmlElement,
}

impl HtmlStyleElement {
    pub fn new(qualified_name: QualName) -> Self {
        Self {
            superclass_html_element: HtmlElement::new(qualified_name),
        }
    }

    pub fn style_content(&self) -> StrTendril {
        let children = self.superclass_html_element.as_ref().as_parent_node().children().borrow();
        if let NodeKind::Text(text) = &children[0].as_ref() {
            debug_assert_eq!(children.len(), 1, "The HTML parser should've concatenated adjacent text nodes!");
            text.data().clone()
        } else {
            StrTendril::new()
        }
    }
}

impl AsRef<Element> for HtmlStyleElement {
    fn as_ref(&self) -> &Element {
        self.superclass_html_element.as_ref()
    }
}

impl AsMut<Element> for HtmlStyleElement {
    fn as_mut(&mut self) -> &mut Element {
        self.superclass_html_element.as_mut()
    }
}

impl AsRef<HtmlElement> for HtmlStyleElement {
    fn as_ref(&self) -> &HtmlElement {
        &self.superclass_html_element
    }
}

impl AsMut<HtmlElement> for HtmlStyleElement {
    fn as_mut(&mut self) -> &mut HtmlElement {
        &mut self.superclass_html_element
    }
}
