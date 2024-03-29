// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

//! The [Mixin `ParentNode`](https://dom.spec.whatwg.org/#interface-parentnode)
//! implementation.

use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use retina_common::DynamicSizeOf;

use crate::Node;

/// The [Mixin `ParentNode`](https://dom.spec.whatwg.org/#interface-parentnode)
/// implementation.
#[derive(Debug)]
pub struct ParentNode {
    children: RwLock<Vec<Node>>,
}

impl ParentNode {
    pub fn new() -> Self {
        Self {
            children: RwLock::new(Vec::new()),
        }
    }

    pub fn children(&self) -> RwLockReadGuard<Vec<Node>> {
        self.children.read().unwrap()
    }

    pub fn children_mut(&self) -> RwLockWriteGuard<Vec<Node>> {
        self.children.write().unwrap()
    }
}

impl DynamicSizeOf for ParentNode {
    fn dynamic_size_of(&self) -> usize {
        let mut size = std::mem::size_of_val(self);

        size += self.children.read().unwrap().dynamic_size_of();

        size
    }
}
