// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use boa_engine::job::JobQueue;
use retina_dom::event::queue::EventQueue;

pub struct ScritturaJobQueue {
    event_queue: EventQueue,
}

impl ScritturaJobQueue {
    pub fn new(event_queue: EventQueue) -> Self {
        Self {
            event_queue,
        }
    }
}

impl JobQueue for ScritturaJobQueue {
    fn enqueue_future_job(&self, future: boa_engine::job::FutureJob, context: &mut boa_engine::Context<'_>) {
        _ = future;
        _ = context;
        todo!();
    }

    fn enqueue_promise_job(&self, job: boa_engine::job::NativeJob, context: &mut boa_engine::Context<'_>) {
        _ = job;
        _ = context;
        todo!();
    }

    fn run_jobs(&self, context: &mut boa_engine::Context<'_>) {
        _ = context;
        _ = self.event_queue;
    }
}
