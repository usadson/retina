// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

pub(crate) mod command;
pub(crate) mod handle;
pub(crate) mod message;
pub(crate) mod page;

pub use command::PageCommand;
pub use handle::PageHandle;
pub use message::{PageMessage, PageProgress};

use page::Page;
use retina_compositor::Compositor;
use retina_gfx::{canvas::CanvasPaintingContext, euclid::Size2D};

use std::{sync::mpsc::channel, time::Duration};
use url::Url;

pub fn spawn(
    url: Url,
    graphics_context: retina_gfx::Context,
    canvas_size: Size2D<u32, u32>,
) -> PageHandle {
    let (command_sender, command_receiver) = channel();
    let (message_sender, message_receiver) = channel();

    let handle = PageHandle {
        is_page_still_connected: true,
        receive_timeout: Duration::from_secs(10),

        command_sender,
        message_receiver
    };

    let canvas = CanvasPaintingContext::new(graphics_context, "Page Canvas", canvas_size);

    std::thread::spawn(move || {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let mut page = Page {
                    command_receiver,
                    message_sender,

                    url,
                    document: None,
                    style_sheets: None,
                    layout_root: None,

                    canvas,
                    compositor: Compositor::new(),
                };

                page.start().await.unwrap()
            })
    });

    handle
}
