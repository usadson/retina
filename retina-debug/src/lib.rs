// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::rc::Rc;

use retina_dom::NodeKind;

#[cfg(windows)]
mod win;

pub struct DomTreeViewDescriptor {
    pub page_title: String,
    pub root: Rc<NodeKind>,
}

pub fn open_dom_tree_view(descriptor: DomTreeViewDescriptor) {
    if cfg!(windows) {
        win::open_dom_tree_view(descriptor);
    } else {
        _ = descriptor;
    }
}
