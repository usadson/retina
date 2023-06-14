// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use retina_dom::Node;

#[cfg(windows)]
mod win;

pub struct DomTreeViewDescriptor {
    pub page_title: String,
    pub root: Node,
}

pub fn open_dom_tree_view(descriptor: DomTreeViewDescriptor) {
    #[cfg(windows)]
    win::open_dom_tree_view(descriptor);

    #[cfg(not(windows))]
    { _ = descriptor }
}
