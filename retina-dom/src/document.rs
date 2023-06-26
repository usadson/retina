// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

//! The [Interface `Document`](https://dom.spec.whatwg.org/#interface-document)
//! implementation.

use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use retina_common::StrTendril;

use crate::{
    Node,
    NodeKind,
    NodeInterface,
    ParentNode,
    PlatformMessenger,
};

/// The [Interface `Document`](https://dom.spec.whatwg.org/#interface-document)
/// implementation.
#[derive(Debug)]
pub struct Document {
    superclass_node: NodeInterface,
    mixin_parent_node: ParentNode,
    data: RwLock<DocumentData>,
}

impl Document {
    pub fn new() -> Self {
        Self {
            superclass_node: NodeInterface::new(),
            mixin_parent_node: ParentNode::new(),
            data: RwLock::new(DocumentData::new()),
        }
    }

    #[must_use]
    pub fn new_handle() -> Node {
        Node::new(NodeKind::Document(Self::new()))
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

    pub fn parent_node(&self) -> &ParentNode {
        &self.mixin_parent_node
    }

    pub fn parent_node_mut(&mut self) -> &mut ParentNode {
        &mut self.mixin_parent_node
    }

    pub fn data(&self) -> RwLockReadGuard<'_, DocumentData> {
        self.data.read().unwrap()
    }

    pub fn data_mut(&self) -> RwLockWriteGuard<'_, DocumentData> {
        self.data.write().unwrap()
    }
}

#[derive(Clone, Debug)]
pub struct DocumentData {
    title: StrTendril,
    platform_messenger: Option<PlatformMessenger>,
}

impl DocumentData {
    pub fn new() -> Self {
        Self {
            title: StrTendril::new(),
            platform_messenger: None,
        }
    }

    pub fn platform_messenger(&self) -> &Option<PlatformMessenger> {
        &self.platform_messenger
    }

    pub fn set_title(&mut self, title: StrTendril) {
        if let Some(messenger) = &self.platform_messenger {
            messenger.send_title(title.clone())
        }

        self.title = title;
    }

    pub fn title(&self) -> &StrTendril {
        &self.title
    }
}
