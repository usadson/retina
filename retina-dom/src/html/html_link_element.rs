// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::ops::{DerefMut, Deref};

use html5ever::{QualName, local_name};
use retina_fetch::mime::{Mime, APPLICATION_OCTET_STREAM};

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

    pub fn type_(&self) -> Mime {
        self.superclass_html_element.as_ref()
            .attributes()
            .find(&local_name!("type"))
            .map(|str| str.parse().ok())
            .flatten()
            .unwrap_or(APPLICATION_OCTET_STREAM)
    }
}

impl Deref for HtmlLinkElement {
    type Target = HtmlElement;

    fn deref(&self) -> &Self::Target {
        &self.superclass_html_element
    }
}

impl DerefMut for HtmlLinkElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.superclass_html_element
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
