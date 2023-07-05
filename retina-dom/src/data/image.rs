// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::sync::{Arc, RwLock};

use image::DynamicImage;
use log::warn;
use retina_fetch::{Fetch, Request, Url, RequestInitiator, RequestDestination};

#[derive(Debug)]
pub struct ImageData {
    state: Arc<RwLock<ImageDataState>>,
    internal: Arc<RwLock<ImageDataInternal>>,
}

impl ImageData {
    pub(crate) fn new() -> Self {
        Self {
            state: Arc::new(ImageDataState::Initial.into()),
            internal: Arc::new(ImageDataInternal::default().into()),
        }
    }

    pub async fn update(
        &self,
        fetch: Fetch,
        src: &str,
    ) {
        *self.state.write().unwrap() = ImageDataState::Running;

        // 12. Parse selected source, relative to the element's node document,
        //     and let urlString be the resulting URL string.
        let Ok(url) = Url::parse(src.trim()) else {
            *self.state.write().unwrap() = ImageDataState::InvalidUrl;
            return;
        };

        // 18. Let request be the result of creating a potential-CORS request
        //     given urlString, "image", and the current state of the element's
        //     crossorigin content attribute.
        // https://html.spec.whatwg.org/multipage/urls-and-fetching.html#create-a-potential-cors-request
        let request = Request::new(url, RequestInitiator::None, RequestDestination::Image);
        let Ok(mut response) = fetch.fetch(request).await else {
            warn!("Failed to load image: {src}");
            *self.state.write().unwrap() = ImageDataState::LoadFailed;
            return;
        };

        // 26. As soon as possible, jump to the first applicable entry from the
        //     following list:
        let body = response.body_bytes().await;
        let mut reader = std::io::Cursor::new(body.as_ref());
        let content_type = response.content_type();

        // If the resource type and data corresponds to a supported
        // image format, as described below
        let Some(image_kind) = retina_media_type::sniff_in_an_image_context(&mut reader, &content_type) else {
            warn!("Image: {src} has an unknown magic value and the Content-Type: {}", content_type.as_ref());
            *self.state.write().unwrap() = ImageDataState::UnknownType;
            return;
        };

        let Some(image_format) = image_kind.to_bitmap_image_format() else {
            warn!("Image: {src} has a non-bitmap Content-Type: {}, ImageKind::{image_kind:?}", content_type.as_ref());
            *self.state.write().unwrap() = ImageDataState::UnknownType;
            return;
        };

        let image = match image::io::Reader::with_format(&mut reader, image_format).decode() {
            Ok(image) => image,
            Err(e) => {
                warn!("Image: {src} failed to decode: {e}");
                *self.state.write().unwrap() = ImageDataState::DecodeFailed;
                return;
            }
        };

        self.internal.write().unwrap().image = Some(image);
    }

    #[inline]
    pub fn update_was_already_started(&self) -> bool {
        match self.state.read() {
            Ok(state) => match *state {
                ImageDataState::Initial => false,
                ImageDataState::Running => true,
                ImageDataState::InvalidUrl => true,
                ImageDataState::LoadFailed => true,
                ImageDataState::UnknownType => true,
                ImageDataState::DecodeFailed => true,
            }
            Err(..) => true,
        }
    }
}

#[derive(Debug, Default)]
struct ImageDataInternal {
    image: Option<DynamicImage>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum ImageDataState {
    Initial,
    Running,
    InvalidUrl,
    LoadFailed,
    UnknownType,
    DecodeFailed,
}
