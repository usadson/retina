// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

//! The Document Object Model (DOM) implementation of **retina**.
//!
//! # References
//! * [DOM - Living Standard - WHATWG](https://dom.spec.whatwg.org/)

pub mod attribute;
pub mod document;
pub mod element;
pub mod element_kind;
pub mod html;
pub mod node;
pub mod parent_node;
pub mod parse;
pub mod text;

use std::rc::Rc;

pub use attribute::AttributeList;
pub use document::Document;
pub use element::Element;
pub use html::*;
pub use node::Node;
pub use parent_node::ParentNode;
pub use parse::Parser;
use retina_common::DumpableNode;
pub use text::Text;

#[derive(Debug)]
pub enum NodeKind {
    Document(Document),
    Text(Text),
    HtmlElement(HtmlElementKind),
}

impl NodeKind {
    pub fn as_dom_element(&self) -> Option<&Element> {
        self.as_html_element_kind().map(|e| e.as_dom_element())
    }

    pub fn as_dom_element_mut(&mut self) -> Option<&mut Element> {
        self.as_html_element_kind_mut().map(|e| e.as_dom_element_mut())
    }

    pub fn as_html_element_kind(&self) -> Option<&HtmlElementKind> {
        if let Self::HtmlElement(element) = self {
            Some(element)
        } else {
            None
        }
    }

    pub fn as_html_element_kind_mut(&mut self) -> Option<&mut HtmlElementKind> {
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

    pub fn as_node_mut(&mut self) -> &mut Node {
        match self {
            Self::Document(doc) => doc.as_node_mut(),
            Self::HtmlElement(element) => element.as_node_mut(),
            Self::Text(text) => text.as_node_mut(),
        }
    }

    pub fn as_parent_node(&self) -> Option<&ParentNode> {
        match self {
            Self::Document(doc) => Some(doc.as_parent_node()),
            Self::HtmlElement(element) => Some(element.as_dom_element().as_parent_node()),
            Self::Text(..) => None,
        }
    }

    pub fn as_parent_node_mut(&mut self) -> Option<&mut ParentNode> {
        match self {
            Self::Document(doc) => Some(doc.as_parent_node_mut()),
            Self::HtmlElement(element) => Some(element.as_dom_element_mut().as_parent_node_mut()),
            Self::Text(..) => None,
        }
    }

    pub fn as_text(&self) -> Option<&Text> {
        match self {
            Self::Text(text) => Some(text),
            _ => None,
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

    /// Get the name name of this Node, if it is an HTML element.
    pub fn tag_name(&self) -> Option<&str> {
        if let Self::HtmlElement(element) = &self {
            Some(element.as_dom_element().qualified_name().local.as_ref())
        } else {
            None
        }
    }

    pub fn to_short_dumpable(&self) -> ShortDumpable {
        ShortDumpable { node_kind: self }
    }
}

impl DumpableNode for NodeKind {
    fn dump_to(&self, depth: usize, writer: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
        write!(
            writer,
            "{pad:pad_width$}{prelude:?} ",
            pad = "",
            pad_width = depth * 4,
            prelude = self.to_short_dumpable()
        )?;

        match self {
            Self::Document(..) => (),
            Self::HtmlElement(element) => {
                let id = element.as_dom_element().id();
                if !id.is_empty() {
                    write!(writer, "#{id}")?;
                }

                for class in element.as_dom_element().class_list() {
                    write!(writer, ".{class}")?;
                }
            }
            Self::Text(text) => {
                let text = text.data_as_str().replace('\n', "\\n");
                write!(writer, "\"{}\"", text)?
            }
        }

        writeln!(writer)?;

        if let Some(as_parent) = self.as_parent_node() {
            for child in as_parent.children().borrow().iter() {
                child.dump_to(depth + 1, writer)?;
            }
        }

        Ok(())
    }
}

pub struct ShortDumpable<'node_kind> {
    node_kind: &'node_kind NodeKind,
}

impl<'node_kind> core::fmt::Debug for ShortDumpable<'node_kind> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.node_kind {
            NodeKind::Document(..) => f.write_str("#document"),
            NodeKind::HtmlElement(..) => f.write_fmt(format_args!("<{}>", self.node_kind.tag_name().unwrap_or("element?"))),
            NodeKind::Text(..) => f.write_str("#text"),
        }
    }
}
