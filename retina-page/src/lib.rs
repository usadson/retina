// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

pub(crate) mod dirty_state;
pub(crate) mod command;
pub(crate) mod cursor_state;
pub(crate) mod font_loader;
pub(crate) mod handle;
pub(crate) mod image_provider;
pub(crate) mod message;
pub(crate) mod page;
pub(crate) mod scroller;

pub use command::{PageCommand, PageCommandAction};
use cursor_state::CursorState;
pub use handle::{PageHandle, PageHandleCommunicationError, PageHandleReceiveHalf, PageHandleSendHalf};
use image_provider::ImageProvider;
pub use message::{PageMessage, PageProgress};

use self::{
    font_loader::FontLoader,
    page::Page,
    dirty_state::DirtyState,
    scroller::Scroller,
};

use retina_compositor::Compositor;
use retina_gfx::{canvas::CanvasPaintingContext, euclid::Size2D};
use retina_gfx_font::FontProvider;

use std::{
    panic::PanicInfo,
    sync::{
        Arc,
        mpsc::{
            channel,
            sync_channel,
            SyncSender,
        },
    },
    time::Duration,
};

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
        let panic_message_sender = message_sender.clone();
        let old_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            handle_panic(&panic_message_sender, info);
            old_hook(info);
        }));

        let runtime = Arc::new(
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap()
        );
        runtime.clone().block_on(async {
            let (page_task_message_sender, page_task_message_receiver) = tokio::sync::mpsc::channel(128);

            let fetch = retina_fetch::Fetch::new();
            let font_loader = FontLoader::new(
                fetch.clone(),
                page_task_message_sender.clone(),
                font_provider.clone(),
            );

            let compositor = Compositor::new(canvas.context().clone());

            let image_provider = ImageProvider::new(fetch.clone());
            let cursor_state = CursorState::new(message_sender.clone());

            let page = Page {
                runtime,
                message_sender,

                url,
                queued_redirect_url: None,
                title: String::new(),
                document: None,
                style_sheets: None,
                layout_root: None,

                cursor_state,
                scroller: Scroller::new(canvas_size.cast().cast_unit()),
                canvas,
                font_provider,
                compositor,
                fetch,
                page_task_message_sender,
                browsing_context: None,
                event_queue: None,
                dirty_state: DirtyState::new(),
                font_loader,
                image_provider,
                earliest_scroll_request: None,
            };

            page.start(command_receiver, page_task_message_receiver).await.unwrap()
        })
    });

    handle
}

fn handle_panic(
    sender: &SyncSender<PageMessage>,
    info: &PanicInfo<'_>,
) {
    let message = format_panic_message(info)
        .unwrap_or_else(|| info.to_string());

    _ = sender.try_send(PageMessage::Crash {
        message,
    }).ok();
}

#[cfg(not(debug_assertions))]
fn format_panic_message(info: &PanicInfo<'_>) -> Option<String> {
    None
}

#[cfg(debug_assertions)]
fn format_panic_message(info: &PanicInfo<'_>) -> Option<String> {
    use std::{
        fs::File,
        io::{
            BufReader,
            BufRead,
        },
        path::Path,
    };

    const PREFIX_LINES: usize = 5;
    const SUFFIX_LINES: usize = PREFIX_LINES;

    use log::warn;

    let Some(location) = info.location() else {
        warn!("[panic] No panic location provided!");
        return None;
    };

    let this_crate_manifest = Path::new(env!("CARGO_MANIFEST_DIR"));

    let Some(workspace_path) = this_crate_manifest.parent() else {
        warn!("[panic] Crate manifest directory invalid: {}", this_crate_manifest.display());
        return None;
    };

    if !workspace_path.exists() {
        warn!("[panic] Workspace path does not exist: {}", workspace_path.display());
        return None;
    }

    let file_location = {
        let mut file_location = workspace_path.to_path_buf();
        file_location.push(location.file());
        file_location
    };

    if !file_location.exists() {
        warn!("[panic] Panicking file does not exist: {}", file_location.display());
        return None;
    }

    let file = match File::open(&file_location) {
        Ok(file) => file,
        Err(err) => {
            warn!("[panic] Panicking file {} failed to open: {err}", file_location.display());
            return None;
        }
    };

    let mut message = info.to_string();

    let first_shown_line = (location.line() as usize).saturating_sub(PREFIX_LINES);

    let mut lines = BufReader::new(file)
        .lines()
        .skip(first_shown_line);

    let mut append_line = |message: &mut String, n| {
        let Some(line) = lines.next().map(|result| result.ok()).flatten() else {
            return;
        };

        let line_number = first_shown_line + n + 1;
        *message = format!("{}\n{line_number:>4}: {line}", *message);
    };

    for n in 0..PREFIX_LINES.min(location.line() as usize) {
        append_line(&mut message, n);
    }

    let line_number_length = 4;
    let space_count = location.column() + line_number_length;

    let error_message = info.payload()
        .downcast_ref::<&str>()
        .map(|message| *message)
        .unwrap_or("error occurred here");

    let spaces = " ".repeat(space_count as usize);
    message = format!("{message}\n{spaces}^ {error_message}");

    for n in 0..SUFFIX_LINES {
        append_line(&mut message, n);
    }

    Some(message)
}
