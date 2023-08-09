// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::ops::Deref;

use html5ever::{QualName, local_name};
use retina_common::DynamicSizeOf;
use retina_i18n::IetfLanguageSubtag;

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

    /// Resolves the [`lang`][spec] attribute.
    ///
    /// [spec]: https://html.spec.whatwg.org/multipage/dom.html#attr-lang
    pub fn language(&self) -> Option<IetfLanguageSubtag> {
        if let Some(lang) = self.element.attributes().find(&local_name!("lang")) {
            // Even if the language is invalid, we don't have to look at the
            // ancestors:
            //
            // "That attribute specifies the language of the node (regardless of
            // its value)."
            return IetfLanguageSubtag::from_str(lang);
        }

        self.element.as_node()
            .parent()?
            .upgrade()?
            .as_html_element_kind()?
            .as_html_element()
            .language()

    }
}

impl DynamicSizeOf for HtmlElement {
    fn dynamic_size_of(&self) -> usize {
        self.element.dynamic_size_of()
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
