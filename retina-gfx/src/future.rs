// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

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
}

impl std::future::Future for SubmissionFuture {
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let maintain = wgpu::Maintain::WaitForSubmissionIndex(self.submission_index.clone());
        _ = self.context.device().poll(maintain.clone());

        // Maintain::WaitForSubmissionIndex always blocks. See docs.
        std::task::Poll::Ready(())
    }
}
