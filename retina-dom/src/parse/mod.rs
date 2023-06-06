// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::borrow::Cow;
use std::default::Default;
use std::rc::Rc;

use html5ever::local_name;
use html5ever::parse_document;
use html5ever::tendril::*;
use html5ever::tree_builder::{ElementFlags, NodeOrText, QuirksMode, TreeSink};
use html5ever::{Attribute, ExpandedName, QualName};
use log::warn;
use retina_common::DumpableNode;

use crate::Document;
use crate::HtmlElementKind;
use crate::HtmlStyleElement;
use crate::HtmlUnknownElement;
use crate::NodeKind;
use crate::Text;

pub struct Parser {
    _private: (),
}

impl Parser {
    #[must_use]
    pub fn parse(input: &str) -> Rc<NodeKind> {
        let sink = Sink {
            document: Document::new_handle(),
        };

        let mut input = std::io::Cursor::new(input);

        let sink = parse_document(sink, Default::default())
            .from_utf8()
            .read_from(&mut input)
            .unwrap();

        sink.document.dump();
        sink.document
    }

}

struct Sink {
    document: Rc<NodeKind>,
}

impl TreeSink for Sink {
    type Handle = Rc<NodeKind>;
    type Output = Self;
    fn finish(self) -> Self {
        self
    }

    fn get_document(&mut self) -> Self::Handle {
        Rc::clone(&self.document)
    }

    fn get_template_contents(&mut self, _target: &Self::Handle) -> Self::Handle {
        todo!();
    }

    fn same_node(&self, x: &Self::Handle, y: &Self::Handle) -> bool {
        Rc::<NodeKind>::ptr_eq(x, y)
    }

    fn elem_name<'handle>(&self, target: &'handle Self::Handle) -> ExpandedName<'handle> {
        match target.as_ref() {
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

        Rc::new(node)
    }

    fn create_comment(&mut self, _text: StrTendril) -> Self::Handle {
        todo!()
    }

    #[allow(unused_variables)]
    fn create_pi(&mut self, target: StrTendril, value: StrTendril) -> Self::Handle {
        todo!()
    }

    fn append_before_sibling(&mut self, _sibling: &Self::Handle, _new_node: NodeOrText<Self::Handle>) {
        todo!()
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
        let child = match child {
            NodeOrText::AppendNode(node) => node,
            NodeOrText::AppendText(text) => Text::new_handle(text),
        };

        child.as_node().set_parent(Some(Rc::downgrade(parent)));

        parent.as_parent_node().unwrap().children().borrow_mut().push(child);
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
