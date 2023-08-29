// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::sync::{
    Arc,
    Mutex,
};

#[derive(Clone)]
pub struct EventQueue {
    inner: Arc<Mutex<EventQueueInner>>,
}

impl EventQueue {
    pub fn new() -> Self {
        let s = Self {
            inner: Arc::new(Mutex::new(
                EventQueueInner {

                }
            ))
        };

        _ = s.inner.as_ref();

        s
    }
}

struct EventQueueInner {

}
