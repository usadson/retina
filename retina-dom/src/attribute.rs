// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::collections::HashMap;

use html5ever::{Attribute, LocalName};
use tendril::StrTendril;

pub type AttributeName = html5ever::LocalName;

/// Elements have an [attribute list], which contains the name and value of
/// the attributes for that element.
/// [attribute list]: https://dom.spec.whatwg.org/#concept-element-attribute
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AttributeList {
    map: HashMap<AttributeName, StrTendril>,
}

impl AttributeList {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn append(&mut self, name: AttributeName, value: StrTendril) {
        self.map.insert(name, value);
    }

    pub(crate) fn append_attribute(&mut self, attribute: Attribute) {
        self.append(attribute.name.local, attribute.value);
    }

    pub fn find(&self, name: &LocalName) -> Option<&str> {
        self.map.get(name).map(|s| s.as_ref())
    }

    /// <https://dom.spec.whatwg.org/#concept-element-attributes-get-value>
    pub fn get(&self, name: &LocalName) -> &str {
        self.find(name).unwrap_or("")
    }
}

impl<'a> IntoIterator for &'a AttributeList {
    type Item = (&'a AttributeName, &'a StrTendril);
    type IntoIter = std::collections::hash_map::Iter<'a, AttributeName, StrTendril>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.iter()
    }
}

impl IntoIterator for AttributeList {
    type Item = (AttributeName, StrTendril);
    type IntoIter = std::collections::hash_map::IntoIter<AttributeName, StrTendril>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.into_iter()
    }
}
