// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod host_hooks;
mod job_queue;

pub mod prelude;
pub mod test_helper;

use std::rc::Rc;

use boa_engine::{prelude::*, context::{MaybeShared, HostHooks}, job::JobQueue};
use retina_dom::{
    event::queue::EventQueue,
    Node,
};
use retina_platform_object::nav_history::Window;

use self::{
    host_hooks::ScritturaHostHooks,
    job_queue::ScritturaJobQueue,
};

pub struct BrowsingContext {
    context: Context<'static>,
}

impl BrowsingContext {
    pub fn new(
        document: Node,
        event_queue: EventQueue,
    ) -> Self {
        let hooks: Rc<dyn HostHooks> = Rc::new(ScritturaHostHooks::new(document.clone()));
        let hooks = MaybeShared::Shared(hooks);

        let job_queue: Rc<dyn JobQueue> = Rc::new(ScritturaJobQueue::new(event_queue));
        let job_queue = MaybeShared::Shared(job_queue);

        let mut context = Context::builder()
            .host_hooks(hooks)
            .job_queue(job_queue)
            .build()
            .unwrap();

        retina_platform_object::register_all(&mut context)
            .expect("failed to register platform objects");

        Window::initialize_global(&mut context, document)
            .expect("failed to register global properties and methods of `Window`");

        Self {
            context
        }
    }

    pub(crate) fn attach_assertion_module(&mut self) {
        self.context.register_global_builtin_callable("assert", 1, NativeFunction::from_copy_closure(
            |_this, args, _context | {
                if args.len() == 0 {
                    return Err(JsError::from_opaque("no argument given to `assert`".into()));
                }

                if !args[0].to_boolean() {
                    Err(JsError::from_opaque(args.get(1).cloned().unwrap_or("assertion failed".into())))
                } else {
                    Ok(JsValue::Undefined)
                }
            }
        )).expect("failed to register `assert`");
    }

    pub fn run_script_from_string_source(&mut self, source: &str) -> Result<JsValue, JsError> {
        self.context.eval(Source::from_bytes(source))
    }
}
