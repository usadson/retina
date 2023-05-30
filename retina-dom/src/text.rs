// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

//! The [Interface `Tex`](https://dom.spec.whatwg.org/#interface-text)
//! implementation.

use std::rc::Rc;

use crate::{
    Node,
    NodeKind,
};

/// The [Interface `Text`](https://dom.spec.whatwg.org/#interface-text)
/// implementation.
#[derive(Debug)]
pub struct Text {
    superclass_node: Node,
    data: String,
}

impl Text {
    pub fn new(data: String) -> Self {
        Self {
            superclass_node: Node::new(),
            data,
        }
    }

    pub fn new_handle(data: String) -> Rc<NodeKind> {
        Rc::new(
            NodeKind::Text(
                Self::new(data)
            )
        )
    }

    pub fn as_node(&self) -> &Node {
        &self.superclass_node
    }

    pub fn as_node_mut(&mut self) -> &mut Node {
        &mut self.superclass_node
    }

    pub fn data(&self) -> &str {
        &self.data
    }
}
