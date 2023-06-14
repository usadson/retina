// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

//! The [Interface `Element`](https://dom.spec.whatwg.org/#interface-element)
//! implementation.

use std::str::SplitAsciiWhitespace;

use html5ever::{QualName, local_name};

use crate::{NodeInterface, ParentNode, AttributeList};

/// The [Interface `Element`](https://dom.spec.whatwg.org/#interface-element)
/// implementation.
#[derive(Debug)]
pub struct Element {
    superclass_node: NodeInterface,
    mixin_parent_node: ParentNode,
    qualified_name: QualName,
    attribute_list: AttributeList,
}

impl Element {
    pub fn new(qualified_name: QualName) -> Self {
        Self {
            superclass_node: NodeInterface::new(),
            mixin_parent_node: ParentNode::new(),
            qualified_name,
            attribute_list: AttributeList::new(),
        }
    }

    pub fn as_node(&self) -> &NodeInterface {
        &self.superclass_node
    }

    pub fn as_node_mut(&mut self) -> &mut NodeInterface {
        &mut self.superclass_node
    }

    pub fn as_parent_node(&self) -> &ParentNode {
        &self.mixin_parent_node
    }

    pub fn as_parent_node_mut(&mut self) -> &mut ParentNode {
        &mut self.mixin_parent_node
    }

    pub fn attributes(&self) -> &AttributeList {
        &self.attribute_list
    }

    pub fn attributes_mut(&mut self) -> &mut AttributeList {
        &mut self.attribute_list
    }

    pub fn class_list(&self) -> SplitAsciiWhitespace {
        let Some(attribute_value) = self.attribute_list.find(&local_name!("class")) else {
            return "".split_ascii_whitespace();
        };

        attribute_value.split_ascii_whitespace()
    }

    pub fn id(&self) -> &str {
        self.attributes().find(&local_name!("id")).unwrap_or("")
    }

    pub fn qualified_name(&self) -> &QualName {
        &self.qualified_name
    }
}
