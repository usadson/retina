// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use boa_engine::{
    class::ClassBuilder,
    native_function::NativeFunctionPointer,
    object::FunctionObjectBuilder,
    property::{Attribute, PropertyKey},
    prelude::*,
};

use crate::nav_history::Window;

#[inline]
pub fn illegal_constructor<T>() -> JsResult<T> {
    let native_error = JsNativeError::typ().with_message("Invalid constructor.");
    JsResult::Err(JsError::from_native(native_error))
}

pub trait ClassBuilderExt {
    fn accessor_with_function_ptr<K>(
        &mut self,
        title: K,
        set: Option<NativeFunctionPointer>,
        get: Option<NativeFunctionPointer>,
        attribute: Attribute,
    ) -> &mut Self
            where K: Into<PropertyKey>;
}

impl<'ctx, 'host> ClassBuilderExt for ClassBuilder<'ctx, 'host> {
    fn accessor_with_function_ptr<K>(
            &mut self,
            key: K,
            get: Option<NativeFunctionPointer>,
            set: Option<NativeFunctionPointer>,
            attribute: Attribute,
        ) -> &mut Self
                where K: Into<PropertyKey> {
        let set = set.map(|set| {
            FunctionObjectBuilder::new(
                self.context(),
                NativeFunction::from_fn_ptr(set)
            ).build()
        });

        let get = get.map(|get| {
            FunctionObjectBuilder::new(
                self.context(),
                NativeFunction::from_fn_ptr(get)
            ).build()
        });

        self.accessor(
            key,
            get,
            set,
            attribute,
        )
    }
}

pub trait ContextExt {
    fn with_window<T>(&self, f: impl FnOnce(&Window) -> T) -> T;
}

impl<'host> ContextExt for Context<'host> {
    fn with_window<T>(&self, f: impl FnOnce(&Window) -> T) -> T {
        f(&self.global_object().downcast_ref::<Window>().unwrap())
    }
}
