// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::str::SplitAsciiWhitespace;

use crate::LinkType;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(dead_code)]
enum Kind {
    Anchor,
    Area,
    Form,
    Link,
}

#[derive(Clone, Debug)]
pub struct LinkRelationship<'attribute> {
    tokens: SplitAsciiWhitespace<'attribute>,
    kind: Kind,
}

impl<'attribute> LinkRelationship<'attribute> {
    pub fn contains(mut self, link_type: LinkType) -> bool {
        self.find(|ty| *ty == link_type).is_some()
    }

    pub fn for_link_element(attribute_value: &'attribute str) -> Self {
        Self {
            tokens: attribute_value.split_ascii_whitespace(),
            kind: Kind::Link,
        }
    }
}

impl<'attribute> Iterator for LinkRelationship<'attribute> {
    type Item = LinkType;
    fn next(&mut self) -> Option<Self::Item> {
        if self.kind != Kind::Link {
            todo!();
        }

        self.tokens.find_map(LinkType::from_str_link_element)
    }
}
