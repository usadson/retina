// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use retina_dom::event::queue::EventQueue;
use crate::BrowsingContext;

/// Create a simple [`BrowsingContext`] from the `test/html/empty/index.html`
/// document in the repository root.
pub fn create_simple_context_and_document() -> BrowsingContext {
    let document = retina_dom::Parser::parse(include_str!("../../test/html/empty/index.html"));

    let mut context = BrowsingContext::new(document, EventQueue::new());
    context.attach_assertion_module();

    context
}
