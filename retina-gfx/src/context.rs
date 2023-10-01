// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::sync::Arc;

pub struct Context {
    pub context: Arc<dyn ContextKind>,
}

pub trait ContextKind {

}
