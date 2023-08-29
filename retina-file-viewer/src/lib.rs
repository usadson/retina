// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

pub(crate) mod dom_wrap;

use dom_wrap::wrap_in_document_html_body;
use retina_dom::{Node, NodeKind, Text, HtmlLinkElement, qual_name, HtmlElementKind};
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

    let mut link = HtmlLinkElement::new(qual_name("link"));
    let style_sheet_src = format!("file:///{}/../resources/style/file-viewer/json.css", env!("CARGO_MANIFEST_DIR"));
    dbg!(&style_sheet_src);
    link.attributes_mut().set("rel", "stylesheet".into());
    link.attributes_mut().set("href", style_sheet_src.into());

    Some(wrap_in_document_html_body([
        Node::new(NodeKind::HtmlElement(HtmlElementKind::Link(link))),
        Node::new(NodeKind::Text(
            Text::new(pretty_string.into())
        ))
    ]))
}
