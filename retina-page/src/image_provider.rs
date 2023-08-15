// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::{collections::HashMap, future::Future, sync::RwLock};

use retina_dom::ImageData;
use retina_fetch::Fetch;
use url::Url;

#[derive(Debug)]
pub struct ImageProvider {
    images: RwLock<HashMap<Url, ImageData>>,
    fetch: Fetch,
}

impl ImageProvider {
    pub fn new(fetch: Fetch) -> Self {
        Self {
            fetch,
            images: RwLock::new(HashMap::new()),
        }
    }

    pub fn get_from_url<CallbackOnLoad, CallbackOnLoadFuture>(
        &self,
        url: Url,
        on_load: CallbackOnLoad,
    ) -> ImageData
            where CallbackOnLoad: FnOnce(ImageData) -> CallbackOnLoadFuture + Send + 'static,
                CallbackOnLoadFuture: Future<Output = ()> + Send {
        if let Some(image) = self.images.read().unwrap().get(&url) {
            return image.clone();
        }

        let image = ImageData::new();
        let fetch = self.fetch.clone();

        {
            let image = image.clone();
            let url = url.clone();
            tokio::task::spawn(async move {
                image.update(url, fetch, "").await;
                on_load(image).await;
            });
        }

        self.images.write().unwrap().insert(url, image.clone());
        image
    }
}
