// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::collections::HashMap;

use log::{error, trace, warn};
use retina_fetch::{
    Fetch,
    Request,
    RequestDestination,
    RequestInitiator,
    RequestReferrer,
    RequestMode,
};
use retina_gfx_font::{
    FamilyName,
    FontDescriptor,
    FontProvider,
    FontWeight,
};
use retina_layout::LayoutBox;
use retina_style::{
    CssFontFaceAtRule,
    CssFontFaceDeclaration,
    CssFontFaceFormat,
    CssFontFaceSrc,
    Rule,
    Stylesheet,
};
use tokio::sync::mpsc::Sender;
use url::Url;

use crate::message::PageTaskMessage;

pub(crate) struct FontLoader {
    fetch: Fetch,
    page_task_message_sender: Sender<PageTaskMessage>,
    font_provider: FontProvider,
    fonts: HashMap<FontDescriptor, FontState>,
    document_url: Url,
}

impl FontLoader {
    pub fn new(
        fetch: Fetch,
        page_task_message_sender: Sender<PageTaskMessage>,
        font_provider: FontProvider,
        document_url: Url,
    ) -> Self {
        Self {
            fetch,
            page_task_message_sender,
            font_provider,
            fonts: HashMap::new(),
            document_url,
        }
    }

    pub fn process_enqueued(&mut self, stylesheets: &[Stylesheet]) {
        for (descriptor, state) in &mut self.fonts {
            match state {
                FontState::Initial => (),
                FontState::LoadingLocal | FontState::LoadingRemote | FontState::TryLoadRemote => {
                    trace!("Re-evaluating font: {descriptor:#?}");
                }

                _ => continue,
            }

            let descriptor = descriptor.clone();
            let page_task_message_sender = self.page_task_message_sender.clone();
            let font_provider = self.font_provider.clone();

            match find_font_face_rule(&descriptor, stylesheets) {
                Some(font_face) => {
                    *state = FontState::LoadingRemote;
                    load_remote_font(font_provider, descriptor, font_face, page_task_message_sender, self.fetch.clone(), self.document_url.clone());
                }
                None => {
                    if *state == FontState::LoadingRemote || *state == FontState::TryLoadRemote {
                        trace!("Font is already being loaded remotely: {descriptor:#?}");
                        continue;
                    }

                    *state = FontState::LoadingLocal;
                    load_local_font(font_provider, descriptor, page_task_message_sender);
                }
            }
        }
    }

    pub fn process_load_state(&mut self, descriptor: FontDescriptor, state: FontState) -> FontLoadResult {
        let result = match state {
            FontState::Initial => unreachable!(),
            FontState::TryLoadRemote => unreachable!(),
            FontState::LoadingLocal => unreachable!(),
            FontState::LoadingRemote => unreachable!(),

            FontState::InvalidLocalReference | FontState::InvalidRemoteReference => FontLoadResult {
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
                Some(FontState::Initial) | Some(FontState::TryLoadRemote) => break,

                // The font is already being loaded. Let's see if another font
                // is already present, and use that one instead!
                Some(FontState::LoadingRemote) => continue,

                // The font cannot be resolved or failed to load. Ignore this
                // one, and let's see if the next one can be loaded instead!
                Some(FontState::InvalidRemoteReference) => continue,

                Some(FontState::LoadingLocal) | Some(FontState::InvalidLocalReference) => {
                    trace!("Enqueuing for remote {descriptor:?}...");
                    self.fonts.insert(descriptor, FontState::TryLoadRemote);
                }

                // The font is already loaded: we can ignore the rest of the
                // families of this layout box.
                Some(FontState::Loaded) => break,

                // Not enqueued yet, let's enqueue it!
                None => {
                    trace!("Enqueuing {descriptor:?}...");
                    self.fonts.insert(descriptor, FontState::Initial);

                    // The first non-loaded font was found, and is now enqueued.
                    // We can stop for now, but if this failed to load, this algorithm
                    // will be ran again to check for others.
                    break;
                }
            }
        }
    }

    pub fn set_document_url(&mut self, url: Url) {
        self.document_url = url;
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum FontState {
    /// Nothing has been done with this font thus far.
    Initial,

    TryLoadRemote,

    /// The font is being loaded from the system.
    LoadingLocal,

    /// The font is being loaded from an `@font-face` rule.
    LoadingRemote,

    /// Tried to load this font, but it is not in the system font list.
    InvalidLocalReference,

    /// Tried to load this font, but it is not in an `@font-face` rule.
    InvalidRemoteReference,

    /// The font is loaded.
    Loaded
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct FontLoadResult {
    pub rerun_layout: bool,
    pub rerun_algorithm: bool,
}


fn find_font_face_rule<'stylesheets>(
    descriptor: &FontDescriptor,
    stylesheets: &'stylesheets [Stylesheet],
) -> Option<&'stylesheets CssFontFaceAtRule> {
    for stylesheet in stylesheets {
        for rule in stylesheet.rules() {
            if let Rule::AtFontFace(font_face) = rule {
                if matches_font_face_rule(font_face, descriptor) {
                    return Some(font_face);
                }
            }
        }
    }

    None
}

fn matches_font_face_rule(font_face: &CssFontFaceAtRule, descriptor: &FontDescriptor) -> bool {
    let FamilyName::Title(requested_name) = &descriptor.name else {
        // Must be a non-generic name for it to be able to match.
        return false
    };

    let alpha = 'a' as u32;

    let mut name_matches = false;
    let mut style_matches = false;
    let mut weight_matches = false;
    let mut unicode_matches = true;

    for declaration in &font_face.declarations {
        match declaration {
            CssFontFaceDeclaration::FontFamily(family) => {
                name_matches = requested_name == &family.into();
            }

            CssFontFaceDeclaration::FontStyle(style) => {
                style_matches = retina_layout::convert_font_style(*style) == descriptor.style;
            }

            CssFontFaceDeclaration::FontWeight(weight) => {
                weight_matches = *weight as f32 == descriptor.weight.value();
            }

            CssFontFaceDeclaration::UnicodeRanges(ranges) => {
                // TODO
                unicode_matches = ranges.iter()
                    .any(|range| range.contains(&alpha));
            }

            _ => (),
        }
    }

    if !name_matches {
        return false;
    }

    let criteria = [
        name_matches, style_matches, weight_matches, unicode_matches
    ];

    if criteria.iter().all(|v| *v) {
        return true;
    }

    if criteria.iter().any(|v| *v) {
        warn!("@font-face found for missing font, but not all criteria matches: {criteria:#?}");
    }

    false
}

fn load_local_font(font_provider: FontProvider, descriptor: FontDescriptor, page_task_message_sender: Sender<PageTaskMessage>) {
    tokio::task::spawn(async move {
        let state = load_local_font_inner(font_provider, descriptor.clone(), None).await;

        _ = page_task_message_sender.send(PageTaskMessage::FontLoadResult {
            descriptor,
            state,
        }).await;
    });
}

async fn load_local_font_inner(
    font_provider: FontProvider,
    descriptor: FontDescriptor,
    search_descriptor: Option<FontDescriptor>
) -> FontState {
    let search_descriptor = search_descriptor.unwrap_or_else(|| descriptor.clone());

    if font_provider.load_from_system(search_descriptor) {
        FontState::Loaded
    } else {
        FontState::InvalidLocalReference
    }
}

fn load_remote_font(
    font_provider: FontProvider,
    descriptor: FontDescriptor,
    font_face: &CssFontFaceAtRule,
    page_task_message_sender: Sender<PageTaskMessage>,
    fetch: Fetch,
    document_url: Url,
) {
    let referrer = RequestReferrer::Url(document_url);
    let src = font_face.declarations.iter().find_map(|declaration| {
        match declaration {
            CssFontFaceDeclaration::Src { sources } => Some(sources),
            _ => None,
        }
    });

    let Some(sources) = src.cloned() else {
        error!("@font-face has no `src` property, descriptor: {descriptor:#?}");
        return;
    };

    tokio::task::spawn(async move {
        // TODO!
        let font_index = 0u32;

        for src in sources {
            match src {
                CssFontFaceSrc::WebFont { url, format } => {
                    match format {
                        CssFontFaceFormat::Collection
                            | CssFontFaceFormat::EmbeddedOpentype
                            | CssFontFaceFormat::Svg
                            | CssFontFaceFormat::Unknown => {
                            warn!("Format of @font-face source \"{url}\" not supported: {format}");
                            continue;
                        }

                        _ => (),
                    }

                    let url_string = url;
                    let url = match Url::parse(&url_string) {
                        Ok(url) => url,
                        Err(err) => {
                            error!("Failed to parse @font-face URL \"{url_string}\": {err}");
                            continue;
                        }
                    };

                    // <https://www.w3.org/TR/css-fonts-4/#font-face-src-parsing>
                    let request = Request::new(url, RequestInitiator::None, RequestDestination::Font, RequestMode::Cors, referrer.clone());
                    let mut response = match fetch.fetch(request).await {
                        Ok(response) => response,
                        Err(e) => {
                            error!("Failed to load @font-face source \"{url_string}\": {e}");
                            continue;
                        }
                    };

                    if !response.status().is_successful() {
                        error!("Failed to load @font-face source \"{url_string}\", status was: {}", response.status().as_u16());
                        continue;
                    }

                    match format {
                        CssFontFaceFormat::Collection => unreachable!(),

                        CssFontFaceFormat::Truetype | CssFontFaceFormat::Opentype => {
                            let mut data = Vec::new();
                            response.body().await.read_to_end(&mut data).unwrap();
                            if font_provider.load(descriptor.clone(), data, font_index) {
                                _ = page_task_message_sender.send(PageTaskMessage::FontLoadResult {
                                    descriptor,
                                    state: FontState::Loaded,
                                }).await;

                                return;
                            }

                            error!("Failed to load OpenType/TrueType @font-face from source: \"{url_string}\"");
                        }

                        CssFontFaceFormat::EmbeddedOpentype => unreachable!(),
                        CssFontFaceFormat::Unknown => unreachable!(),
                        CssFontFaceFormat::Svg => unreachable!(),

                        CssFontFaceFormat::Woff => {
                            let mut output = Vec::new();
                            let mut input = Vec::new();
                            response.body().await.read_to_end(&mut input).unwrap();

                            rs_woff::woff2otf(&mut std::io::Cursor::new(input), &mut output).unwrap();

                            if font_provider.load(descriptor.clone(), output, font_index) {
                                _ = page_task_message_sender.send(PageTaskMessage::FontLoadResult {
                                    descriptor,
                                    state: FontState::Loaded,
                                }).await;

                                return;
                            }

                            error!("Failed to load WOFF @font-face from source \"{url_string}\"!");
                        }

                        CssFontFaceFormat::Woff2 => {
                            let font_data = match woff2::convert_woff2_to_ttf(&mut response.body_bytes().await) {
                                Ok(font_data) => font_data,
                                Err(e) => {
                                    error!("Failed to load WOFF @font-face from source \"{url_string}\": {e}");
                                    continue;
                                }
                            };

                            if font_provider.load(descriptor.clone(), font_data, font_index) {
                                _ = page_task_message_sender.send(PageTaskMessage::FontLoadResult {
                                    descriptor,
                                    state: FontState::Loaded,
                                }).await;

                                return;
                            }

                            error!("Failed to load WOFF2 @font-face from source \"{url_string}\"!");
                        }
                    }
                }

                CssFontFaceSrc::Local(local) => {
                    let search_descriptor = FontDescriptor {
                        name: FamilyName::Title(local.into()),
                        style: descriptor.style,
                        weight: descriptor.weight
                    };

                    let result = load_local_font_inner(font_provider.clone(), descriptor.clone(), Some(search_descriptor)).await;

                    if result == FontState::Loaded {
                        _ = page_task_message_sender.send(PageTaskMessage::FontLoadResult {
                            descriptor,
                            state: FontState::Loaded,
                        }).await;

                        return;
                    }
                }
            }
        }

        _ = page_task_message_sender.send(PageTaskMessage::FontLoadResult {
            descriptor,
            state: FontState::InvalidRemoteReference,
        }).await;
    });
}
