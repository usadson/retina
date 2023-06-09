// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::sync::Arc;

/// The opaque shared context of the graphics engine.
///
/// ## Implementation
/// This is a wrapper of [`ContextData`] in an [`Arc`] to allow syncing across
/// threads.
///
/// ## Note
/// AFAIK, all WGPU instances use an [`Arc`] under the hood already, so it's
/// really unfortunate that we have to double wrap it. The problem is that
/// almost no WGPU object derives from [`Clone`] or something alike, which
/// allows sharing owned values without lifetime hell. This is especially
/// important in the way the `retina-page` works, a multi-thread environment.
#[derive(Clone, Debug)]
pub struct Context {
    data: Arc<ContextData>,
}

impl Context {
    pub(crate) fn new(
        instance: wgpu::Instance,
        device: wgpu::Device,
        queue: wgpu::Queue,
    ) -> Self {
        Self {
            data: Arc::new(ContextData {
                instance,
                device,
                queue,
            })
        }
    }

    pub(crate) fn device(&self) -> &wgpu::Device {
        &self.data.as_ref().device
    }

    #[allow(dead_code)]
    pub(crate) fn instance(&self) -> &wgpu::Instance {
        &self.data.as_ref().instance
    }

    pub(crate) fn queue(&self) -> &wgpu::Queue {
        &self.data.as_ref().queue
    }
}

/// The internal contents of the ContextData. This is wrapped in [`Context`]
/// in an [`Arc`] to allow syncing across threads.
#[derive(Debug)]
pub(crate) struct ContextData {
    #[allow(dead_code)]
    instance: wgpu::Instance,
    device: wgpu::Device,
    queue: wgpu::Queue,
}
