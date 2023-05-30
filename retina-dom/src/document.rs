// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

//! The [Interface `Document`](https://dom.spec.whatwg.org/#interface-document)
//! implementation.

use std::rc::Rc;

use crate::{
    Node,
    NodeKind,
    ParentNode,
};

/// The [Interface `Document`](https://dom.spec.whatwg.org/#interface-document)
/// implementation.
#[derive(Debug)]
pub struct Document {
    superclass_node: Node,
    mixin_parent_node: ParentNode,
}

impl Document {
    pub fn new() -> Self {
        Self {
            superclass_node: Node::new(),
            mixin_parent_node: ParentNode::new(),
        }
    }

    #[must_use]
    pub fn new_handle() -> Rc<NodeKind> {
        Rc::new(NodeKind::Document(Self::new()))
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

    pub fn parent_node(&self) -> &ParentNode {
        &self.mixin_parent_node
    }

    pub fn parent_node_mut(&mut self) -> &mut ParentNode {
        &mut self.mixin_parent_node
    }
}
