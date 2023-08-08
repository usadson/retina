// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::collections::HashMap;

use log::trace;
use retina_fetch::Fetch;
use retina_gfx_font::{FontProvider, FontDescriptor, FontWeight};
use retina_layout::LayoutBox;
use retina_style::{CssFontFamilyName, Stylesheet};
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
        for (descriptor, state) in &self.fonts {
            let FontState::Initial = state else { continue };

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
            let CssFontFamilyName::Name(desired_font) = &family else {
                // The font was generic at this stage, so we can skip any font
                // loading!
                return;
            };

            let descriptor = FontDescriptor {
                name: desired_font.clone().into(),

                weight: FontWeight::new(layout_box.computed_style().font_weight() as _),
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
