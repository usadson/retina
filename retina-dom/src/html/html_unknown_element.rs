// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use html5ever::QualName;

use crate::{
    Element,
    html::HtmlElement,
};

#[derive(Debug)]
pub struct HtmlUnknownElement {
    superclass_html_element: HtmlElement,
}

impl HtmlUnknownElement {
    pub fn new(qualified_name: QualName) -> Self {
        Self {
            superclass_html_element: HtmlElement::new(qualified_name),
        }
    }
}

impl AsRef<Element> for HtmlUnknownElement {
    fn as_ref(&self) -> &Element {
        self.superclass_html_element.as_ref()
    }
}

impl AsMut<Element> for HtmlUnknownElement {
    fn as_mut(&mut self) -> &mut Element {
        self.superclass_html_element.as_mut()
    }
}

impl AsRef<HtmlElement> for HtmlUnknownElement {
    fn as_ref(&self) -> &HtmlElement {
        &self.superclass_html_element
    }
}

impl AsMut<HtmlElement> for HtmlUnknownElement {
    fn as_mut(&mut self) -> &mut HtmlElement {
        &mut self.superclass_html_element
    }
}
