// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use tracing::instrument;

use crate::Context;

#[derive(Debug)]
pub struct SubmissionFuture {
    context: Context,
    submission_index: wgpu::SubmissionIndex,
}

impl SubmissionFuture {
    pub const fn new(context: Context, submission_index: wgpu::SubmissionIndex) -> Self {
        Self {
            context,
            submission_index,
        }
    }

    #[inline]
    #[instrument]
    pub fn wait(&self) {
        let maintain = wgpu::Maintain::WaitForSubmissionIndex(self.submission_index.clone());
        _ = self.context.device().poll(maintain.clone());
    }
}

impl std::future::Future for SubmissionFuture {
    type Output = ();

    #[inline]
    #[instrument]
    fn poll(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        self.wait();

        // Maintain::WaitForSubmissionIndex always blocks. See docs.
        std::task::Poll::Ready(())
    }
}
