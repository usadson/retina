// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

//! The [Mixin `ParentNode`](https://dom.spec.whatwg.org/#interface-parentnode)
//! implementation.

use std::{rc::Rc, cell::RefCell};

use crate::NodeKind;

/// The [Mixin `ParentNode`](https://dom.spec.whatwg.org/#interface-parentnode)
/// implementation.
#[derive(Debug)]
pub struct ParentNode {
    children: RefCell<Vec<Rc<NodeKind>>>,
}

impl ParentNode {
    pub fn new() -> Self {
        Self {
            children: RefCell::new(Vec::new()),
        }
    }

    pub fn children(&self) -> &RefCell<Vec<Rc<NodeKind>>> {
        &self.children
    }

    pub fn children_mut(&mut self) -> &mut RefCell<Vec<Rc<NodeKind>>> {
        &mut self.children
    }
}
