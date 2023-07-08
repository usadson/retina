// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

pub(crate) mod dirty_state;
pub(crate) mod command;
pub(crate) mod handle;
pub(crate) mod message;
pub(crate) mod page;
pub(crate) mod scroller;

pub use command::{PageCommand, PageCommandAction};
pub use handle::{PageHandle, PageHandleCommunicationError, PageHandleReceiveHalf, PageHandleSendHalf};
pub use message::{PageMessage, PageProgress};

use page::Page;
use dirty_state::DirtyState;
use retina_compositor::Compositor;
use retina_gfx::{canvas::CanvasPaintingContext, euclid::Size2D};
use retina_gfx_font::FontProvider;
use scroller::Scroller;

use std::{sync::{mpsc::{channel, sync_channel}, Arc}, time::Duration};
use url::Url;

pub fn spawn(
    url: Url,
    font_provider: FontProvider,
    graphics_context: retina_gfx::Context,
    canvas_size: Size2D<u32, u32>,
) -> PageHandle {
    let (command_sender, command_receiver) = channel();
    let (message_sender, message_receiver) = sync_channel(128);

    let handle = PageHandle {
        receive: PageHandleReceiveHalf {
            is_page_still_connected: true,
            receive_timeout: Duration::from_secs(10),
            message_receiver
        },
        send: PageHandleSendHalf {
            is_page_still_connected: true,
            command_sender,
        },
    };

    let canvas = CanvasPaintingContext::new(graphics_context, "Page Canvas", canvas_size);

    std::thread::spawn(move || {
        let runtime = Arc::new(
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap()
        );
        runtime.clone().block_on(async {
            let (page_task_message_sender, page_task_message_receiver) = tokio::sync::mpsc::channel(128);

            let page = Page {
                runtime,
                message_sender,

                url,
                title: String::new(),
                document: None,
                style_sheets: None,
                layout_root: None,

                scroller: Scroller::new(canvas_size.cast().cast_unit()),
                canvas,
                font_provider,
                compositor: Compositor::new(),
                fetch: retina_fetch::Fetch::new(),
                page_task_message_sender,
                browsing_context: None,
                event_queue: None,
                dirty_state: DirtyState::new(),
            };

            page.start(command_receiver, page_task_message_receiver).await.unwrap()
        })
    });

    handle
}
