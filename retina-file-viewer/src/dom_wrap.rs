// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::sync::Arc;

use retina_common::StrTendril;
use retina_dom::{
    Document,
    HtmlElementKind,
    HtmlUnknownElement,
    Node,
    NodeKind,
    qual_name,
};

pub(crate) fn wrap_in_document_node(node: Node) -> Node {
    let document = Arc::new(NodeKind::Document(Document::new()));
    wrap(&document, [node]);
    document.into()
}

pub(crate) fn wrap_in_element<N>(element_name: &str, nodes: N) -> Node
        where N: IntoIterator<Item = Node> {
    let element = Arc::new(NodeKind::HtmlElement(
        HtmlElementKind::Unknown(
            HtmlUnknownElement::new(qual_name(element_name))
        )
    ));

    wrap(&element, nodes);
    element.into()
}

fn wrap<N>(node: &Arc<NodeKind>, nodes: N)
        where N: IntoIterator<Item = Node> {
    let parent = Arc::downgrade(&node);

    let mut children = node.as_parent_node().unwrap()
        .children_mut();

    for child in nodes.into_iter() {
        child.as_node().set_parent(Some(parent.clone()));
        children.push(child);
    }
}

#[allow(dead_code)]
pub(crate) fn wrap_in_element_with_style<S, N>(style: S, nodes: N) -> Node
        where S: Into<StrTendril>,
            N: IntoIterator<Item = Node> {
    let mut element = HtmlElementKind::Unknown(
        HtmlUnknownElement::new(qual_name("span"))
    );

    element.as_dom_element_mut().attributes_mut().set("style", style.into());

    let element = Arc::new(NodeKind::HtmlElement(element));

    wrap(&element, nodes);
    element.into()
}

pub(crate) fn wrap_in_document_html_body<N>(nodes: N) -> Node
        where N: IntoIterator<Item = Node> {
    wrap_in_document_node(
        wrap_in_element(
            "html",
            [
                wrap_in_element("head", []),
                wrap_in_element(
                    "body",
                    nodes,
                )
            ]
        )
    )
}
