// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use html5ever::{QualName, local_name};

use crate::{
    Element,
    HtmlElement,
    LinkRelationship,
};

#[derive(Debug)]
pub struct HtmlLinkElement {
    superclass_html_element: HtmlElement,
}

impl HtmlLinkElement {
    pub fn new(qualified_name: QualName) -> Self {
        Self {
            superclass_html_element: HtmlElement::new(qualified_name),
        }
    }

    pub fn href(&self) -> &str {
        self.superclass_html_element.as_ref()
            .attributes()
            .find(&local_name!("href"))
            .unwrap_or("")
    }

    pub fn rel(&self) -> &str {
        self.superclass_html_element.as_ref()
            .attributes()
            .find(&local_name!("rel"))
            .unwrap_or("")
    }

    pub fn relationship(&self) -> LinkRelationship {
        LinkRelationship::for_link_element(self.rel())
    }
}

impl AsRef<Element> for HtmlLinkElement {
    fn as_ref(&self) -> &Element {
        self.superclass_html_element.as_ref()
    }
}

impl AsMut<Element> for HtmlLinkElement {
    fn as_mut(&mut self) -> &mut Element {
        self.superclass_html_element.as_mut()
    }
}

impl AsRef<HtmlElement> for HtmlLinkElement {
    fn as_ref(&self) -> &HtmlElement {
        &self.superclass_html_element
    }
}

impl AsMut<HtmlElement> for HtmlLinkElement {
    fn as_mut(&mut self) -> &mut HtmlElement {
        &mut self.superclass_html_element
    }
}
