// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::ops::Deref;

use crate::Node;

#[derive(Debug)]
pub struct DocumentWrapper(pub Node);

impl DocumentWrapper {
    pub fn html(&self) -> Node {
        self.0.as_ref()
            .as_parent_node()
            .expect("Expected Node to be Document, which is a parent node.")
            .children()
            .get(0)
            .expect("Expected Node to have 1 child of type HTMLHtmlElement")
            .clone()
    }

    pub fn head(&self) -> Node {
        self.html()
            .as_ref()
            .as_parent_node()
            .expect("Expected Node to be HTMLHtmlElement, which is a parent node.")
            .children()
            .get(0)
            .expect("Expected Node to have a first child of type HTMLHeadElement")
            .clone()
    }
}

impl Deref for DocumentWrapper {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
