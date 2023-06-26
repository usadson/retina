// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use boa_engine::prelude::*;

use boa_engine::class::{Class, ClassBuilder, ClassConstructor};
use boa_engine::property::Attribute;
use boa_gc::{Finalize, Trace};
use retina_dom::Node;

use crate::dom::Document;
use crate::prelude::illegal_constructor;

#[derive(Finalize)]
pub struct Window {
    document_node: Node,
}

unsafe impl Trace for Window {
    boa_gc::empty_trace!();
}

impl Window {
    pub fn new(document_node: Node) -> Self {
        Self {
            document_node,
        }
    }

    pub fn document(&self) -> &Node {
        &self.document_node
    }

    /// When [`Window`] is the global object, use this method to initialize the properties.
    pub fn initialize_global(context: &mut Context, document: Node) -> JsResult<()> {
        context.register_global_property("window", context.global_object(), Attribute::empty())?;

        // let window_object = context.global_object();
        // let window = window_object
        //     .downcast_ref::<Window>()
        //     .ok_or(JsError::from_opaque("`Window` (Self) is not the global object!".into()))?;
        _ = document;

        let document = Document::raw_constructor(&JsValue::Object(JsObject::default()), &[], context)?;
        context.register_global_property("document", document, Attribute::all())?;



        Ok(())
    }
}

impl Class for Window {
    const NAME: &'static str = "Window";
    const LENGTH: usize = 0;
    const ATTRIBUTES: Attribute = Attribute::empty();

    fn constructor(_: &JsValue, _: &[JsValue], _: &mut Context<'_>) -> JsResult<Self> {
        illegal_constructor()
    }

    fn init(_: &mut ClassBuilder<'_, '_>) -> JsResult<()> {
        Ok(())
    }
}
