// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

//! The [Interface `Element`](https://dom.spec.whatwg.org/#interface-element)
//! implementation.

use html5ever::QualName;

use crate::{Node, ParentNode};

/// The [Interface `Element`](https://dom.spec.whatwg.org/#interface-element)
/// implementation.
#[derive(Debug)]
pub struct Element {
    superclass_node: Node,
    mixin_parent_node: ParentNode,
    qualified_name: QualName,
}

impl Element {
    pub fn new(qualified_name: QualName) -> Self {
        Self {
            superclass_node: Node::new(),
            mixin_parent_node: ParentNode::new(),
            qualified_name,
        }
    }

    pub fn as_node(&self) -> &Node {
        &self.superclass_node
    }

    pub fn as_node_mut(&mut self) -> &mut Node {
        &mut self.superclass_node
    }

    pub fn as_parent_node(&self) -> &ParentNode {
        &self.mixin_parent_node
    }

    pub fn as_parent_node_mut(&mut self) -> &mut ParentNode {
        &mut self.mixin_parent_node
    }

    pub fn qualified_name(&self) -> &QualName {
        &self.qualified_name
    }
}
