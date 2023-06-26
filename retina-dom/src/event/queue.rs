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
        Self {
            inner: Arc::new(Mutex::new(
                EventQueueInner {

                }
            ))
        }
    }
}

struct EventQueueInner {

}
