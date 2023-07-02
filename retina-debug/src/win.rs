// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use retina_dom::{Node, NodeKind};
use winsafe::{
    prelude::*,
    co,
    gui::{WindowMain, WindowMainOpts, TreeView, TreeViewOpts, spec::TreeViewItem, Horz, Vert}
};

use crate::DomTreeViewDescriptor;

#[derive(Clone)]
struct DomTreeView {
    window: WindowMain,
    tree_view: TreeView,
    dom_root: Node,
}

impl DomTreeView {
    fn setup(&self) {
        self.setup_on_window_create();
    }

    fn setup_on_window_create(&self) {
        let dom_tree_view2 = self.clone();
        self.window.on().wm_create(move |_| {

            // dom_tree_view2.tree_view.on()
            let root = dom_tree_view2.tree_view.items().add_root("#document", None);
            tree_add_children(&root, dom_tree_view2.dom_root.as_parent_node().unwrap().children().as_slice());
            Ok(0)
        });
    }
}

pub(crate) fn open_dom_tree_view(descriptor: DomTreeViewDescriptor) {
    let window = WindowMain::new(WindowMainOpts {
        title: format!("{} — Document Tree — Retina", descriptor.page_title),
        size: (800, 600),
        style: co::WS::CAPTION
            | co::WS::SYSMENU
            | co::WS::CLIPCHILDREN
            | co::WS::VISIBLE
            | co::WS::SIZEBOX
            | co::WS::MINIMIZEBOX
            | co::WS::MAXIMIZEBOX,
        ..Default::default()
    });

    let tree_view = TreeView::new(&window, TreeViewOpts {
        position: (0, 0),
        size: (800, 600),
        resize_behavior: (Horz::Resize, Vert::Resize),
        tree_view_ex_style: co::TVS_EX::DOUBLEBUFFER,

        ..Default::default()
    });

    let dom_tree_view = DomTreeView {
        dom_root: descriptor.root,
        window,
        tree_view,
    };

    dom_tree_view.setup();

    let window = dom_tree_view.window.clone();
    std::thread::spawn(move || {
        let result = window.run_main(None);
        println!("[dom tree view] Result: {result:#?}");
    });
}

fn tree_add_node(parent: &TreeViewItem, node: &Node) {
    if node.is_text_with_only_whitespace() {
        return;
    }

    let item = tree_format_node(node.as_ref(), |text| parent.add_child(text, None));

    // Open the <body> tag and all of its parents (typically <html> and #document)
    if node.as_dom_element().is_some_and(|e| e.qualified_name().local.eq_str_ignore_ascii_case("body")) {
        item.ensure_visible();
    }

    if let Some(as_parent) = node.as_parent_node() {
        if !node.is_element() || node.children_count() > 1 {
            tree_add_children(&item, as_parent.children().as_slice());
        }
    }
}

fn tree_add_children(parent: &TreeViewItem, children: &[Node]) {
    for child in children {
        tree_add_node(parent, child);
    }
}

fn tree_format_node<T>(node: &NodeKind, callback: impl FnOnce(&str) -> T) -> T {
    if let Some(text) = node.as_text() {
        return callback(text.data_as_str().trim());
    }

    if let Some(comment) = node.as_comment() {
        return callback(&format!("<!--{}-->", comment.data_as_str()));
    }

    if let Some(dom) = node.as_dom_element() {
        let tag_name = &node.as_dom_element().unwrap().qualified_name().local;
        let attrs = node.as_dom_element().unwrap().attributes();

        let child_count = dom.as_parent_node().children().len();

        if child_count == 1 {
            if let Some(text) = dom.as_parent_node().children().first().unwrap().as_text() {
                let trim = text.data_as_str().trim();
                if !trim.is_empty() {
                    let text_storage = format!(
                        "<{tag_name} {attrs}>{}</{tag_name}>",
                        trim,
                    );
                    return callback(&text_storage);
                }
            }
        }

        if child_count == 0 {
            return callback(&format!("<{tag_name} {attrs}></{tag_name}>"));
        }

        return callback(&format!("<{tag_name} {attrs}>"));
    }

    callback(&format!("{:?}", node.to_short_dumpable()))
}
