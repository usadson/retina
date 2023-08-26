// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::{
    any::Any,
    fmt::Debug,
    sync::{Arc, RwLock},
};

use image::{
    AnimationDecoder,
    DynamicImage,
};

use log::{warn, info};
use retina_common::DynamicSizeOf;
use retina_fetch::{
    Fetch,
    Request,
    RequestDestination,
    RequestInitiator,
    RequestMode,
    Url,
};

#[derive(Debug, Clone)]
pub struct ImageData {
    state: Arc<RwLock<ImageDataState>>,
    internal: Arc<RwLock<ImageDataKind>>,
    graphics: Arc<RwLock<Arc<dyn Any + Send + Sync>>>,
}

impl PartialEq for ImageData {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.state, &other.state)
    }
}

impl ImageData {
    pub fn new() -> Self {
        Self {
            state: Arc::new(ImageDataState::Initial.into()),
            internal: Arc::new(RwLock::new(ImageDataKind::None)),
            graphics: Arc::new(RwLock::new(Arc::new(()))),
        }
    }

    pub fn image(&self) -> &Arc<RwLock<ImageDataKind>> {
        &self.internal
    }

    pub fn graphics(&self) -> &Arc<RwLock<Arc<dyn Any + Send + Sync>>> {
        &self.graphics
    }

    pub fn state(&self) -> ImageDataState {
        *self.state.read().unwrap()
    }

    /// When the user agent is to [update the image data][spec] of an `img`
    /// element, it must run the following steps.
    ///
    /// ## TODO
    /// Enqueue a task when running in parallel.
    ///
    /// [spec]: https://html.spec.whatwg.org/multipage/images.html#updating-the-image-data
    pub async fn update(
        &self,
        base_url: Url,
        fetch: Fetch,
        src: &str,
    ) {
        *self.state.write().unwrap() = ImageDataState::Running;

        // 12. Parse selected source, relative to the element's node document,
        //     and let urlString be the resulting URL string.
        let Ok(url) = Url::options().base_url(Some(&base_url)).parse(src.trim()) else {
            warn!("Invalid image URL: {src}");
            *self.state.write().unwrap() = ImageDataState::InvalidUrl;
            return;
        };

        let src = url.to_string();

        // 18. Let request be the result of creating a potential-CORS request
        //     given urlString, "image", and the current state of the element's
        //     crossorigin content attribute.
        // https://html.spec.whatwg.org/multipage/urls-and-fetching.html#create-a-potential-cors-request
        let request = Request::new(url, RequestInitiator::None, RequestDestination::Image, RequestMode::NoCors);
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

        match image_format {
            image::ImageFormat::Gif => {
                let image = match image::codecs::gif::GifDecoder::new(&mut reader) {
                    Ok(image) => image,
                    Err(e) => {
                        warn!("Image(GIF): {src} failed to decode: {e}");
                        *self.state.write().unwrap() = ImageDataState::DecodeFailed;
                        return;
                    }
                };

                let frames = match image.into_frames().collect_frames() {
                    Ok(frames) => frames,
                    Err(e) => {
                        warn!("Image(GIF): {src} failed to decode frame: {e}");
                        *self.state.write().unwrap() = ImageDataState::DecodeFailed;
                        return;
                    }
                };

                *self.internal.write().unwrap() = ImageDataKind::Animated(AnimatedImage {
                    frames,
                    frames_graphics: Vec::new(),
                });
            }

            // TODO APNG and WebP support animated images too

            _ => {
                let image = match image::io::Reader::with_format(&mut reader, image_format).decode() {
                    Ok(image) => image,
                    Err(e) => {
                        warn!("Image: {src} failed to decode: {e}");
                        *self.state.write().unwrap() = ImageDataState::DecodeFailed;
                        return;
                    }
                };

                *self.internal.write().unwrap() = ImageDataKind::Bitmap(image);
            }
        }

        info!("Image: {src} successfully loaded & decoded!");
        *self.state.write().unwrap() = ImageDataState::Ready;
    }

    #[inline]
    pub fn update_was_already_started(&self) -> bool {
        match self.state.read() {
            Ok(state) => match *state {
                ImageDataState::Initial => false,
                ImageDataState::Ready => true,
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

impl DynamicSizeOf for ImageData {
    fn dynamic_size_of(&self) -> usize {
        let mut size = std::mem::size_of_val(self);

        size += self.internal.read().unwrap().dynamic_size_of();

        size
    }
}

#[derive(Debug)]
pub enum ImageDataKind {
    None,
    Uploaded { width: u32, height: u32 },
    Bitmap(DynamicImage),
    Animated(AnimatedImage),
}

impl ImageDataKind {
    #[inline]
    pub fn width(&self) -> u32 {
        match self {
            Self::None => 0,
            Self::Uploaded { width, .. } => *width,
            Self::Bitmap(bitmap) => bitmap.width(),
            Self::Animated(animated) => animated.width(),
        }
    }

    #[inline]
    pub fn height(&self) -> u32 {
        match self {
            Self::None => 0,
            Self::Uploaded { height, .. } => *height,
            Self::Bitmap(bitmap) => bitmap.height(),
            Self::Animated(animated) => animated.height(),
        }
    }
}

impl DynamicSizeOf for ImageDataKind {
    fn dynamic_size_of(&self) -> usize {
        let mut size = std::mem::size_of_val(self);

        size += match self {
            Self::Animated(animated) => animated.dynamic_size_of(),
            Self::Bitmap(bitmap) => bitmap.as_bytes().len(),
            Self::None => 0,
            Self::Uploaded { .. } => 0,
        };

        size
    }
}

pub struct AnimatedImage {
    frames: Vec<image::Frame>,
    pub frames_graphics: Vec<Arc<dyn Any + Send + Sync>>,
}

impl AnimatedImage {
    #[inline]
    pub fn width(&self) -> u32 {
        self.frames.get(0)
            .map(image::Frame::buffer)
            .map(image::RgbaImage::width)
            .unwrap_or(0)
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.frames.get(0)
            .map(image::Frame::buffer)
            .map(image::RgbaImage::height)
            .unwrap_or(0)
    }

    #[inline]
    pub fn frames(&self) -> &[image::Frame] {
        &self.frames
    }
}

impl Debug for AnimatedImage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnimatedImage")
            .field("frames", &format!("len({})", self.frames.len()))
            .finish()
    }
}

impl DynamicSizeOf for AnimatedImage {
    fn dynamic_size_of(&self) -> usize {
        let mut size = std::mem::size_of_val(self);

        size += self.frames.iter()
            .map(|frame| std::mem::size_of_val(frame) + frame.buffer().len())
            .sum::<usize>();

            size += self.frames_graphics.iter()
            .map(|frame| std::mem::size_of_val(frame))
            .sum::<usize>();

        size
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ImageDataState {
    Initial,
    Ready,
    Running,
    InvalidUrl,
    LoadFailed,
    UnknownType,
    DecodeFailed,
}
