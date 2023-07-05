// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::ops::Deref;

use html5ever::QualName;

use crate::Element;

#[derive(Debug)]
pub struct HtmlElement {
    element: Element,
}

impl HtmlElement {
    pub fn new(qualified_name: QualName) -> Self {
        Self {
            element: Element::new(qualified_name),
        }
    }
}

impl Deref for HtmlElement {
    type Target = Element;

    fn deref(&self) -> &Self::Target {
        &self.element
    }
}

impl AsRef<Element> for HtmlElement {
    fn as_ref(&self) -> &Element {
        &self.element
    }
}

impl AsMut<Element> for HtmlElement {
    fn as_mut(&mut self) -> &mut Element {
        &mut self.element
    }
}
