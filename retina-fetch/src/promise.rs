// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::{
    future::Future,
    sync::Arc,
    task::{Poll, Context},
};

use tokio::sync::mpsc::Receiver;

use crate::{Request, FetchResponse, InternalError};

/// An async-library agnostic awaitable.
#[derive(Debug)]
pub struct FetchPromise {
    pub(crate) request: Arc<Request>,
    pub(crate) receiver: Receiver<FetchResponse>,
}

impl FetchPromise {
    /// Get the [`Request`] associated with this `fetch`.
    pub fn request(&self) -> &Request {
        &self.request
    }
}

impl Future for FetchPromise {
    type Output = FetchResponse;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.get_mut().receiver.poll_recv(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(None) => Poll::Ready(InternalError::SynchronizationFault.into()),
            Poll::Ready(Some(response)) => Poll::Ready(response),
        }
    }
}
