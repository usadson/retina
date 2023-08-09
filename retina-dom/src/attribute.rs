// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::{collections::HashMap, fmt::Write, str::FromStr};

use html5ever::{Attribute, LocalName};
use retina_common::{StrTendril, DynamicSizeOf};

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
        self.append(attribute.name.local, StrTendril::from_str(attribute.value.as_ref()).unwrap());
    }

    pub fn find(&self, name: &LocalName) -> Option<&str> {
        self.map.get(name).map(|s| s.as_ref())
    }

    pub fn find_by_str(&self, name: &str) -> Option<&str> {
        self.find(&LocalName::from(name))
    }

    /// <https://dom.spec.whatwg.org/#concept-element-attributes-get-value>
    pub fn get(&self, name: &LocalName) -> &str {
        self.find(name).unwrap_or("")
    }

    pub fn set(&mut self, name: &str, value: StrTendril) {
        self.map.insert(LocalName::from(name), value);
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

impl std::fmt::Display for AttributeList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (index, (name, value)) in self.map.iter().enumerate() {
            if index != 0 {
                f.write_char(' ')?;
            }

            f.write_str(name.as_ref())?;
            if !value.is_empty() {
                f.write_fmt(format_args!("=\"{value}\""))?;
            }
        }

        Ok(())
    }
}

impl DynamicSizeOf for AttributeList {
    fn dynamic_size_of(&self) -> usize {
        std::mem::size_of_val(self)
            + self.map.values()
                .map(|tendril| tendril.len())
                .sum::<usize>()
    }
}
