// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::{sync::Arc, path::Path};

use tokio::{runtime::Runtime, sync::mpsc::channel};
use url::Url;

use crate::{
    Error,
    FetchPromise,
    FetchResponse,
    InternalError,
    NetworkError,
    Request,
    Response,
};

type HyperConnector = hyper_tls::HttpsConnector<hyper::client::HttpConnector>;
type HyperClient = hyper::client::Client<HyperConnector>;

#[derive(Clone, Debug)]
pub struct Fetch {
    client: HyperClient,
    runtime: Arc<tokio::runtime::Runtime>,
}

impl Fetch {
    /// Create a new FetchPromise that resolves instantaneously.
    pub fn create_instantaneous_response(&self, request: Arc<Request>, response: FetchResponse) -> FetchPromise {
        let (sender, receiver) = channel(1);
        self.runtime.spawn(async move {
            sender.send(response).await.unwrap();
        });
        FetchPromise {
            request,
            receiver
        }
    }

    pub fn new() -> Self {
        let connector = HyperConnector::new();
        let client = hyper::client::Client::builder().build::<_, hyper::Body>(connector);

        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        let runtime = Arc::new(runtime);

        spawn_runtime_stopper(Arc::clone(&runtime));

        Self {
            client,
            runtime,
        }
    }

    pub fn fetch(&self, request: Request) -> FetchPromise {
        let request = Arc::new(request);

        if request.url.scheme() == "file" {
            return self.fetch_file(request);
        }

        self.fetch_http(request)
    }

    pub fn fetch_document(&self, url: Url) -> FetchPromise {
        if url.scheme() == "about" {
            return self.fetch_document_about(url);
        }

        if url.scheme() == "file" {
            return self.fetch_document_file(url);
        }

        self.fetch(Request::get_document(url))
    }

    fn fetch_document_about(&self, url: Url) -> FetchPromise {
        use retina_user_agent::url_scheme::about;

        let body = match url.path() {
            // https://fetch.spec.whatwg.org/#scheme-fetch
            "blank" => "",
            "not-found" => about::NOT_FOUND,
            _ => about::NOT_FOUND,
        };

        let request = Arc::new(Request::get_document(url));

        self.create_instantaneous_response(
            Arc::clone(&request),
            Ok(Response::new_about(request, body)),
        )
    }

    fn fetch_file(&self, request: Arc<Request>) -> FetchPromise {
        let (sender, receiver) = channel(1);

        let task_request = Arc::clone(&request);
        self.runtime.spawn(async move {
            let request = task_request;
            let mut path = request.url.path();

            if cfg!(windows) && path.starts_with('/') {
                path = &path[1..];
            }

            let path = Path::new(path);
            if !path.exists() {
                sender.send(Err(Error::NetworkError(NetworkError::LocalFileNotFound))).await.unwrap();
                return;
            }

            let file = tokio::fs::File::open(path).await.unwrap();
            let decoder = tokio_util::codec::BytesCodec::new();

            let file = tokio_util::codec::FramedRead::new(file, decoder);
            sender.send(Ok(Response::new_file(request, file))).await.unwrap();
        });

        FetchPromise {
            request,
            receiver,
        }
    }

    /// Fetch using the HTTP protocol, this also includes the TLS-wrapped HTTPS.
    fn fetch_http(&self, request: Arc<Request>) -> FetchPromise {
        let task_client = self.client.clone();
        let task_request = Arc::clone(&request);

        let (sender, receiver) = channel(1);

        self.runtime.spawn(async move {
            let client = task_client;
            let request = task_request;

            let hyper_request = hyper::Request::builder()
                .uri(request.url.as_str())
                .method(&request.method)
                .body(hyper::Body::empty());

            let hyper_request = match hyper_request {
                Ok(request) => request,
                Err(e) => {
                    log::warn!("Failed to build request: {e}");
                    sender.send(Err(Error::InternalError(InternalError::HyperError))).await.unwrap();
                    return;
                }
            };

            let response = match client.request(hyper_request).await {
                Ok(response) => Ok((request, response).into()),
                Err(e) => Err(e.into()),
            };

            sender.send(response).await.unwrap();
        });

        FetchPromise {
            request,
            receiver,
        }
    }

    fn fetch_document_file(&self, url: Url) -> FetchPromise {
        self.fetch_file(Arc::new(Request::get_document(url)))
    }
}

fn spawn_runtime_stopper(runtime: Arc<Runtime>) {
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(std::time::Duration::from_secs(30));

            if Arc::weak_count(&runtime) != 0 {
                continue;
            }

            if Arc::strong_count(&runtime) == 1 {
                return;
            }
        }
    });
}
