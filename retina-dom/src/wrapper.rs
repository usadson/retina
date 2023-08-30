// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::ops::Deref;

use crate::Node;

#[derive(Debug)]
pub struct DocumentWrapper(pub Node);

impl DocumentWrapper {
    pub fn html(&self) -> Node {
        debug_assert!(matches!(self.0.as_ref(), crate::NodeKind::Document(..)));

        let children = self.0.as_ref()
            .as_parent_node()
            .expect("Expected Node to be Document, which is a parent node.")
            .children();

        for child in children.iter() {
            if child.tag_name() == Some("html") {
                return child.clone();
            }
        }

        panic!("No <html> found in Document: {:#?}", self.0);
    }

    pub fn head(&self) -> Node {
        let html = self.html();
        let children = html
            .as_ref()
            .as_parent_node()
            .expect("Expected Node to be HTMLHtmlElement, which is a parent node.")
            .children();

        for child in children.iter() {
            if child.tag_name() == Some("head") {
                return child.clone();
            }
        }

        panic!("No <head> found in <html>: {html:#?}");
    }
}

impl Deref for DocumentWrapper {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
