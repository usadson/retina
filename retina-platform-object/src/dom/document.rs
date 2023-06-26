// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use boa_engine::prelude::*;
use crate::prelude::*;

use boa_engine::class::{Class, ClassBuilder};
use boa_engine::property::Attribute;
use boa_gc::{Finalize, Trace, empty_trace};
use retina_common::StrTendril;
use retina_dom::Node;


#[derive(Finalize)]
pub struct Document {
    document_node: Node,
}

impl Document {
    pub fn create(document_node: Node, context: &mut Context) -> Self {
        _ = context;
        Self {
            document_node
        }
    }

    pub fn get_title(this: &JsValue, _args: &[JsValue], _context: &mut Context) -> JsResult<JsValue> {
        let this = this.as_object()
            .and_then(|obj| obj.downcast_ref::<Self>())
            .ok_or_else(|| JsError::from_opaque("Value is not `Window`".into()))?;
        let document = this.document_node.as_document().unwrap();
        let document_data = document.data();
        let str = &document_data.title()[..];
        Ok(JsValue::String(str.into()))
    }

    pub fn set_title(this: &JsValue, args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let this = this.as_object()
            .and_then(|obj| obj.downcast_ref::<Self>())
            .ok_or_else(|| JsError::from_opaque("Value is not `Window`".into()))?;

        let title = args[0].to_string(context)?;

        let document = this.document_node.as_document().unwrap();
        document.data_mut().set_title(StrTendril::from(title.to_std_string_escaped()));

        Ok(JsValue::String(title))
    }
}

impl Class for Document {
    const NAME: &'static str = "Document";
    const LENGTH: usize = 0;
    const ATTRIBUTES: Attribute = Attribute::empty();

    fn constructor(_: &JsValue, _: &[JsValue], context: &mut Context<'_>) -> JsResult<Self> {
        let document_node = context.with_window(|w| w.document().clone());
        Ok(Document {
            document_node,
        })
    }

    fn init(class: &mut ClassBuilder<'_, '_>) -> JsResult<()> {
        class.accessor_with_function_ptr(
            "title",
            Some(Self::get_title),
            Some(Self::set_title),
            Attribute::all()
        );
        Ok(())
    }
}

unsafe impl Trace for Document { empty_trace!(); }
