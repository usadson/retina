// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::borrow::Cow;
use std::default::Default;

use html5ever::local_name;
use html5ever::parse_document;
use html5ever::tendril::*;
use html5ever::tree_builder::{ElementFlags, NodeOrText, QuirksMode, TreeSink};
use html5ever::{Attribute, ExpandedName, QualName};
use log::warn;

use crate::Comment;
use crate::Document;
use crate::HtmlElementKind;
use crate::HtmlLinkElement;
use crate::HtmlStyleElement;
use crate::HtmlUnknownElement;
use crate::Node;
use crate::NodeKind;
use crate::Text;

pub struct Parser {
    _private: (),
}

impl Parser {
    #[must_use]
    pub fn parse(input: &str) -> Node {
        let mut input = std::io::Cursor::new(input);
        Self::parse_with_reader(&mut input)
    }

    #[must_use]
    pub fn parse_with_reader<R: std::io::Read>(reader: &mut R) -> Node {
        let sink = Sink {
            document: Document::new_handle(),
        };

        let sink = parse_document(sink, Default::default())
            .from_utf8()
            .read_from(reader)
            .unwrap();

        // sink.document.dump();
        sink.document
    }
}

struct Sink {
    document: Node,
}

impl TreeSink for Sink {
    type Handle = Node;
    type Output = Self;
    fn finish(self) -> Self {
        self
    }

    fn get_document(&mut self) -> Self::Handle {
        Node::clone(&self.document)
    }

    fn get_template_contents(&mut self, _target: &Self::Handle) -> Self::Handle {
        todo!();
    }

    fn same_node(&self, x: &Self::Handle, y: &Self::Handle) -> bool {
        Node::ptr_eq(x, y)
    }

    fn elem_name<'handle>(&self, target: &'handle Self::Handle) -> ExpandedName<'handle> {
        match target.as_ref() {
            NodeKind::Comment(..) => unimplemented!(),
            NodeKind::Document(..) => unimplemented!(),
            NodeKind::Text(..) => unimplemented!(),

            NodeKind::HtmlElement(html_element) => {
                html_element.as_dom_element().qualified_name().expanded()
            }
        }
    }

    /// [Creates an element for a token][algo].
    ///
    /// # References
    /// * [HTML Living Standard - 13.2.6.1 Creating and inserting nodes][algo]
    ///
    /// [algo]: https://html.spec.whatwg.org/multipage/parsing.html#create-an-element-for-the-token
    fn create_element(&mut self, qualified_name: QualName, attributes: Vec<Attribute>, _: ElementFlags) -> Self::Handle {
        // 9. Let element be the result of creating an element given document,
        //    localName, given namespace, null, and is. If will execute script
        //    is true, set the synchronous custom elements flag; otherwise,
        //    leave it unset.
        let mut node = create_element_for_qualified_name(qualified_name);
        let element = node.as_html_element_kind_mut().unwrap();

        // 10. Append each attribute in the given token to element.
        for attribute in attributes {
            element.as_dom_element_mut()
                .attributes_mut()
                .append_attribute(attribute);
        }

        Node::new(node)
    }

    fn create_comment(&mut self, _text: StrTendril) -> Self::Handle {
        Comment::new_handle(retina_common::StrTendril::from(_text.as_ref()))
    }

    #[allow(unused_variables)]
    fn create_pi(&mut self, target: StrTendril, value: StrTendril) -> Self::Handle {
        todo!()
    }

    fn append_before_sibling(&mut self, _sibling: &Self::Handle, _new_node: NodeOrText<Self::Handle>) {
        // todo!()
    }

    fn append_based_on_parent_node(
        &mut self,
        _element: &Self::Handle,
        _prev_element: &Self::Handle,
        _new_node: NodeOrText<Self::Handle>,
    ) {
        todo!()
    }

    fn parse_error(&mut self, msg: Cow<'static, str>) {
        warn!("[Parser] Parse Error: {msg}");
    }

    fn set_quirks_mode(&mut self, mode: QuirksMode) {
        warn!("[Parser] Quirks mode: QuirksMode::{mode:?}");
    }

    fn append(&mut self, parent: &Self::Handle, child: NodeOrText<Self::Handle>) {
        let mut replace_previous = false;

        let Some(parent_node) = parent.as_parent_node() else {
            panic!("append() called with a non-parent parent: {parent:#?}");
        };

        let child = match child {
            NodeOrText::AppendNode(node) => node,
            NodeOrText::AppendText(text) => {
                let text = retina_common::StrTendril::from(text.as_ref());
                if let Some(mut previous_text) = parent_node
                    .children()
                    .last()
                    .and_then(|child| child.as_text())
                    .map(|s| retina_common::StrTendril::clone(s.data())) {
                    previous_text.push_tendril(&text);
                    replace_previous = true;
                    Text::new_handle(previous_text)
                } else {
                    Text::new_handle(text)
                }
            }
        };

        child.as_node().set_parent(Some(Node::downgrade(parent)));

        let mut children = parent.as_parent_node().unwrap().children_mut();
        if replace_previous {
            let idx = children.len() - 1;
            children[idx] = child;
            // children.insert(idx, child);
        } else {
            children.push(child);
        }
    }

    fn append_doctype_to_document(&mut self, _: StrTendril, _: StrTendril, _: StrTendril) {
        // ignored
    }

    fn add_attrs_if_missing(&mut self, _target: &Self::Handle, _attrs: Vec<Attribute>) {
        todo!()
    }

    fn remove_from_parent(&mut self, _target: &Self::Handle) {
        todo!()
    }

    fn reparent_children(&mut self, _node: &Self::Handle, _new_parent: &Self::Handle) {
        todo!()
    }

    fn mark_script_already_started(&mut self, _node: &Self::Handle) {
        todo!()
    }
}

/// [Creates the element][concept] for a given qualified name.
///
/// # References
/// * [DOM Standard - **create an element**][concept]
///
/// [concept]: https://dom.spec.whatwg.org/#concept-create-element
fn create_element_for_qualified_name(
    qualified_name: QualName
) -> NodeKind {
    // In the future SVG, MathML, and custom elements can be constructed here.
    NodeKind::HtmlElement(create_html_element_with_name(qualified_name))
}

/// This function creates the appropriate [`HtmlElementKind`] by using the HTML
/// [Element Interfaces Index][element interfaces].
///
/// # References
/// * [HTML Living Standard - Element interfaces][element interfaces]
///
/// [element interfaces]: https://html.spec.whatwg.org/multipage/indices.html#element-interfaces
fn create_html_element_with_name(
    qualified_name: QualName,
) -> HtmlElementKind {
    match &qualified_name.local {
        &local_name!("link") => HtmlElementKind::Link(HtmlLinkElement::new(qualified_name)),
        &local_name!("style") => HtmlElementKind::Style(HtmlStyleElement::new(qualified_name)),

        _ => HtmlElementKind::Unknown(HtmlUnknownElement::new(qualified_name)),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Parser,
    };

    #[test]
    fn parse_simple() {
        const TEXT: &str = include_str!("../../../test/html/empty/index.html");
        _ = Parser::parse(TEXT);
    }

}
