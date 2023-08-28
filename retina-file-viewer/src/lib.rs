// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

pub(crate) mod dom_wrap;

use dom_wrap::{wrap_in_document_html_body, wrap_in_element_with_style};
use retina_dom::{Node, NodeKind, Text};
use retina_fetch::{Response, mime};

/// Transform the given HTTP response to a DOM representation, if applicable
/// and/or needed.
pub async fn transform(response: &mut Response) -> Option<Node> {
    let ty = response.content_type();
    match (ty.type_(), ty.subtype()) {
        (mime::APPLICATION, mime::JSON) | (mime::TEXT, mime::JSON) => {
            return transform_json(response).await;
        }

        // Images, etc.

        _ => (),
    }

    None
}

async fn transform_json(response: &mut Response) -> Option<Node> {
    let reader = response.body().await;

    let json_value: serde_json::Value = serde_json::from_reader(reader).unwrap();
    let pretty_string = serde_json::to_string_pretty(&json_value).unwrap();

    let mut nodes = Vec::new();

    // for line in pretty_string.lines() {
        let text = Text::new(pretty_string.into());
        let node = Node::new(NodeKind::Text(text));
        nodes.push(wrap_in_element_with_style("white-space: pre; display: block; color: blue", [node]));
    // }

    Some(wrap_in_document_html_body(nodes))
}
