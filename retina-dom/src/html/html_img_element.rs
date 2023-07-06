// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::ops::Deref;

use html5ever::{QualName, local_name};
use crate::html::LazyLoadingKind;

use crate::{
    Element,
    HtmlElement,
    ImageData,
};

/// [spec]: https://html.spec.whatwg.org/multipage/embedded-content.html#the-img-element
#[derive(Debug)]
pub struct HtmlImgElement {
    superclass_html_element: HtmlElement,
    image_data: ImageData,
}

impl HtmlImgElement {
    pub fn new(qualified_name: QualName) -> Self {
        Self {
            superclass_html_element: HtmlElement::new(qualified_name),
            image_data: ImageData::new(),
        }
    }

    pub fn alt(&self) -> &str {
        self.superclass_html_element.as_ref()
            .attributes()
            .get(&local_name!("alt"))
    }

    pub fn lazy_loading(&self) -> LazyLoadingKind {
        let attribute_value = self.superclass_html_element.as_ref()
            .attributes()
            .find_by_str("loading")
            .unwrap_or_default();

        if attribute_value.eq_ignore_ascii_case("lazy") {
            LazyLoadingKind::Lazy
        } else {
            LazyLoadingKind::Eager
        }
    }

    pub fn src(&self) -> &str {
        self.superclass_html_element.as_ref()
            .attributes()
            .get(&local_name!("src"))
    }

    pub fn data(&self) -> ImageData {
        self.image_data.clone()
    }

    pub fn data_ref(&self) -> &ImageData {
        &self.image_data
    }
}

impl AsRef<Element> for HtmlImgElement {
    fn as_ref(&self) -> &Element {
        self.superclass_html_element.as_ref()
    }
}

impl AsMut<Element> for HtmlImgElement {
    fn as_mut(&mut self) -> &mut Element {
        self.superclass_html_element.as_mut()
    }
}

impl Deref for HtmlImgElement {
    type Target = HtmlElement;

    fn deref(&self) -> &Self::Target {
        &self.superclass_html_element
    }
}

impl AsRef<HtmlElement> for HtmlImgElement {
    fn as_ref(&self) -> &HtmlElement {
        &self.superclass_html_element
    }
}

impl AsMut<HtmlElement> for HtmlImgElement {
    fn as_mut(&mut self) -> &mut HtmlElement {
        &mut self.superclass_html_element
    }
}
