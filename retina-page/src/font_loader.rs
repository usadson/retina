// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::collections::HashMap;

use log::trace;
use retina_fetch::Fetch;
use retina_gfx_font::{FontProvider, FontDescriptor, FontWeight};
use retina_layout::LayoutBox;
use retina_style::Stylesheet;
use tokio::sync::mpsc::Sender;

use crate::message::PageTaskMessage;

pub(crate) struct FontLoader {
    fetch: Fetch,
    page_task_message_sender: Sender<PageTaskMessage>,
    font_provider: FontProvider,
    fonts: HashMap<FontDescriptor, FontState>
}

impl FontLoader {
    pub fn new(
        fetch: Fetch,
        page_task_message_sender: Sender<PageTaskMessage>,
        font_provider: FontProvider,
    ) -> Self {
        Self {
            fetch,
            page_task_message_sender,
            font_provider,
            fonts: HashMap::new(),
        }
    }

    pub fn process_enqueued(&mut self, stylesheets: &[Stylesheet]) {
        for (descriptor, state) in &mut self.fonts {
            let FontState::Initial = state else { continue };

            *state = FontState::Loading;

            let descriptor = descriptor.clone();
            let page_task_message_sender = self.page_task_message_sender.clone();
            let font_provider = self.font_provider.clone();

            // TODO: support @font-face
            _ = stylesheets;
            _ = self.fetch;

            tokio::task::spawn(async move {
                let state = if font_provider.load_from_system(descriptor.clone()) {
                    FontState::Loaded
                } else {
                    FontState::InvalidReference
                };

                _ = page_task_message_sender.send(PageTaskMessage::FontLoadResult {
                    descriptor,
                    state,
                }).await;
            });
        }
    }

    pub fn process_load_state(&mut self, descriptor: FontDescriptor, state: FontState) -> FontLoadResult {
        let result = match state {
            FontState::Initial => unreachable!(),
            FontState::Loading => unreachable!(),

            FontState::InvalidReference => FontLoadResult {
                rerun_algorithm: true,
                rerun_layout: false,
            },

            FontState::Loaded => FontLoadResult {
                rerun_layout: true,
                rerun_algorithm: false,
            }
        };

        trace!("Font load state: {state:?} for descriptor: {descriptor:#?}");

        self.fonts.insert(descriptor, state);

        result
    }

    pub fn register(&mut self, layout_box: &LayoutBox) {
        let Some(families) = &layout_box.computed_style().font_family_list else {
            return;
        };

        for family in families {
            let name = retina_layout::convert_font_family(family);

            let descriptor = FontDescriptor {
                name,

                weight: FontWeight::new(layout_box.computed_style().font_weight() as _),
                style: retina_layout::convert_font_style(layout_box.computed_style().font_style.unwrap_or_default()),
            };

            if layout_box.font().descriptor() == &descriptor {
                // The desired font is already loaded!
                return;
            }

            match self.fonts.get(&descriptor) {
                // The font is already enqueued. Let's stop for now and see if
                // the font will load. If it doesn't, this algorithm will run
                // again.
                Some(FontState::Initial) => break,

                // The font is already being loaded. Let's see if another font
                // is already present, and use that one instead!
                Some(FontState::Loading) => continue,

                // The font cannot be resolved or failed to load. Ignore this
                // one, and let's see if the next one can be loaded instead!
                Some(FontState::InvalidReference) => continue,

                // The font is already loaded: we can ignore the rest of the
                // families of this layout box.
                Some(FontState::Loaded) => break,

                // Not enqueued yet, let's enqueue it!
                None => (),
            }

            // Enqueue the font.
            trace!("Enqueuing {descriptor:?}...");
            self.fonts.insert(descriptor, FontState::Initial);


            // The first non-loaded font was found, and is now enqueued.
            // We can stop for now, but if this failed to load, this algorithm
            // will be ran again to check for others.
            break;
        }
    }
}

#[derive(Debug)]
pub(crate) enum FontState {
    /// Nothing has been done with this font thus far.
    Initial,

    /// The font is being loaded.
    Loading,

    /// Tried to load this font, but it is neither in an `@font-face` rule
    /// nor in the system font list.
    InvalidReference,

    /// The font is loaded.
    Loaded
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct FontLoadResult {
    pub rerun_layout: bool,
    pub rerun_algorithm: bool,
}
