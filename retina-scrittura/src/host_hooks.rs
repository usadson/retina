// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use boa_engine::context::intrinsics::Intrinsics;
use boa_engine::object::ObjectData;
use boa_engine::prelude::*;
use boa_engine::context::HostHooks;
use retina_dom::Node;
use retina_platform_object::nav_history::Window;

pub struct ScritturaHostHooks {
    document: Node,
}

impl ScritturaHostHooks {
    pub fn new(document: Node) -> Self {
        Self {
            document,
        }
    }
}

impl ScritturaHostHooks {
    fn create_window_object(&self, intrinsics: &Intrinsics) -> JsObject {
        JsObject::from_proto_and_data(
            // TODO: `WindowPrototype`
            intrinsics.constructors().object().prototype(),

            ObjectData::native_object(Window::new(self.document.clone()))
        )
    }
}

impl HostHooks for ScritturaHostHooks {
    fn create_global_object(&self, intrinsics: &Intrinsics) -> boa_engine::JsObject {
        self.create_window_object(intrinsics)
    }
}
