// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

pub(crate) mod built_in;
pub(crate) mod command;
pub(crate) mod handle;
pub(crate) mod message;
pub(crate) mod page;

pub use command::PageCommand;
pub use handle::PageHandle;
pub use message::{PageMessage, PageProgress};

use page::Page;

use std::{sync::mpsc::channel, time::Duration};
use url::Url;

pub fn spawn(url: Url) -> PageHandle {
    let (command_sender, command_receiver) = channel();
    let (message_sender, message_receiver) = channel();

    let handle = PageHandle {
        is_page_still_connected: true,
        receive_timeout: Duration::from_secs(10),

        command_sender,
        message_receiver
    };

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
                };

                page.start().await.unwrap()
            })
    });

    handle
}
