// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

//! The Document Object Model (DOM) implementation of **retina**.
//!
//! # References
//! * [DOM - Living Standard - WHATWG](https://dom.spec.whatwg.org/)

pub mod attribute;
pub mod character_data;
pub mod comment;
pub mod document;
pub mod element;
pub mod element_kind;
pub mod html;
pub mod node;
pub mod parent_node;
pub mod parse;
pub mod text;

use std::{ops::Deref, sync::{Arc, Weak}};

pub use attribute::AttributeList;
pub use character_data::CharacterData;
pub use comment::Comment;
pub use document::Document;
pub use element::Element;
pub use html::*;
pub use node::NodeInterface;
pub use parent_node::ParentNode;
pub use parse::Parser;
use retina_common::DumpableNode;
pub use text::Text;

#[derive(Clone, Debug)]
pub struct Node {
    inner: Arc<NodeKind>,
}

impl Node {
    pub fn new(kind: NodeKind) -> Self {
        Self {
            inner: Arc::new(kind),
        }
    }

    pub fn ptr_eq(this: &Node, other: &Node) -> bool {
        Arc::<NodeKind>::ptr_eq(&this.inner, &other.inner)
    }

    fn downgrade(this: &Node) -> Weak<NodeKind> {
        Arc::downgrade(&this.inner)
    }
}

impl AsRef<NodeKind> for Node {
    fn as_ref(&self) -> &NodeKind {
        &self.inner
    }
}

impl Deref for Node {
    type Target = NodeKind;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[derive(Debug)]
pub enum NodeKind {
    Comment(Comment),
    Document(Document),
    HtmlElement(HtmlElementKind),
    Text(Text),
}

impl NodeKind {
    pub fn as_comment(&self) -> Option<&Comment> {
        if let Self::Comment(comment) = self {
            Some(comment)
        } else {
            None
        }
    }

    pub fn as_comment_mut(&mut self) -> Option<&mut Comment> {
        if let Self::Comment(comment) = self {
            Some(comment)
        } else {
            None
        }
    }

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

    pub fn as_node(&self) -> &NodeInterface {
        match self {
            Self::Comment(comment) => comment.as_node(),
            Self::Document(doc) => doc.as_node(),
            Self::HtmlElement(element) => element.as_node(),
            Self::Text(text) => text.as_node(),
        }
    }

    pub fn as_node_mut(&mut self) -> &mut NodeInterface {
        match self {
            Self::Comment(comment) => comment.as_node_mut(),
            Self::Document(doc) => doc.as_node_mut(),
            Self::HtmlElement(element) => element.as_node_mut(),
            Self::Text(text) => text.as_node_mut(),
        }
    }

    pub fn as_parent_node(&self) -> Option<&ParentNode> {
        match self {
            Self::Comment(..) => None,
            Self::Document(doc) => Some(doc.as_parent_node()),
            Self::HtmlElement(element) => Some(element.as_dom_element().as_parent_node()),
            Self::Text(..) => None,
        }
    }

    pub fn as_parent_node_mut(&mut self) -> Option<&mut ParentNode> {
        match self {
            Self::Comment(..) => None,
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

    pub fn children_count(&self) -> usize {
        self.as_parent_node().map(|p| p.children().len()).unwrap_or(0)
    }

    pub fn for_each_child_node_recursive(&self, callback: &mut dyn FnMut(&NodeKind, usize), depth: usize) {
        if let Some(as_parent) = self.as_parent_node() {
            let children = as_parent.children();
            let children: &Vec<Node> = children.as_ref();
            for child in children {
                callback(child.as_ref(), depth);
                child.for_each_child_node_recursive(callback, depth + 1);
            }
        }
    }

    pub const fn is_comment(&self) -> bool {
        matches!(self, Self::Comment(..))
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

    pub fn is_text_with_only_whitespace(&self) -> bool {
        if let Self::Text(text) = self {
            text.data_as_str().trim().is_empty()
        } else {
            false
        }
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
            Self::Comment(comment) => {
                write!(writer, "<!--{}-->", comment.data_as_str())?;
            }
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
            for child in as_parent.children().iter() {
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
            NodeKind::Comment(..) => f.write_str("#comment"),
            NodeKind::Document(..) => f.write_str("#document"),
            NodeKind::HtmlElement(..) => f.write_fmt(format_args!("<{}>", self.node_kind.tag_name().unwrap_or("element?"))),
            NodeKind::Text(..) => f.write_str("#text"),
        }
    }
}
