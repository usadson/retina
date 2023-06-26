// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use boa_engine::class::Class;

pub mod dom;
pub mod event_target;
pub mod nav_history;
pub mod prelude;

pub trait PlatformObject: Class {

}

pub fn register_all(context: &mut boa_engine::Context) -> Result<(), boa_engine::JsError> {
    use nav_history::*;
    use dom::*;

    context.register_global_class::<Document>()?;
    context.register_global_class::<Window>()?;

    Ok(())
}
