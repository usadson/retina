// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

//! The Document Object Model (DOM) implementation of **retina**.
//!
//! # References
//! * [DOM - Living Standard - WHATWG](https://dom.spec.whatwg.org/)

pub mod document;
pub mod element;
pub mod element_kind;
pub mod html;
pub mod node;
pub mod parent_node;
pub mod parse;
pub mod text;

use std::rc::Rc;

pub use document::Document;
pub use element::Element;
pub use html::*;
pub use node::Node;
pub use parent_node::ParentNode;
pub use parse::Parser;
pub use text::Text;

#[derive(Debug)]
pub enum NodeKind {
    Document(Document),
    Text(Text),
    HtmlElement(HtmlElementKind),
}

impl NodeKind {
    pub fn as_html_element_kind(&self) -> Option<&HtmlElementKind> {
        if let Self::HtmlElement(element) = self {
            Some(element)
        } else {
            None
        }
    }

    pub fn as_node(&self) -> &Node {
        match self {
            Self::Document(doc) => doc.as_node(),
            Self::HtmlElement(element) => element.as_node(),
            Self::Text(text) => text.as_node(),
        }
    }

    pub fn as_parent_node(&self) -> Option<&ParentNode> {
        match self {
            Self::Document(doc) => Some(doc.as_parent_node()),
            Self::HtmlElement(element) => Some(element.as_dom_element().as_parent_node()),
            Self::Text(..) => None,
        }
    }

    pub fn for_each_child_node_recursive(&self, callback: &mut dyn FnMut(&NodeKind, usize), depth: usize) {
        if let Some(as_parent) = self.as_parent_node() {
            let children = as_parent.children().borrow();
            let children: &Vec<Rc<NodeKind>> = children.as_ref();
            for child in children {
                callback(child.as_ref(), depth);
                child.for_each_child_node_recursive(callback, depth + 1);
            }
        }
    }

    pub fn is_document(&self) -> bool {
        matches!(self, Self::Document(..))
    }

    pub fn is_element(&self) -> bool {
        matches!(self, Self::HtmlElement(..))
    }

    pub fn is_text(&self) -> bool {
        matches!(self, Self::Text(..))
    }
}
