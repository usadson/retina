// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

//! The [Interface `Node`](https://dom.spec.whatwg.org/#interface-node)
//! implementation.

use core::fmt;
use std::{cell::RefCell, sync::Weak};

use crate::NodeKind;

/// The [Interface `Node`](https://dom.spec.whatwg.org/#interface-node)
/// implementation.
pub struct NodeInterface {
    parent: RefCell<Option<Weak<NodeKind>>>,
}

impl NodeInterface {
    pub fn new() -> Self {
        Self {
            parent: None.into()
        }
    }

    pub fn parent(&self) -> Option<Weak<NodeKind>> {
        self.parent.borrow().clone()
    }

    pub fn set_parent(&self, parent: Option<Weak<NodeKind>>) {
        self.parent.replace(parent);
    }
}

impl fmt::Debug for NodeInterface {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Node")
            // .field("data", &self.parent)
            .finish()
    }
}
