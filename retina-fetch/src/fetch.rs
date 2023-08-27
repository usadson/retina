// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::{sync::Arc, path::Path};

use log::{warn, trace};
use tokio::{runtime::Runtime, sync::mpsc::channel};
use url::Url;

use crate::{
    Error,
    FetchPromise,
    FetchResponse,
    InternalError,
    NetworkError,
    Request,
    RequestReferrer,
    Response,
};

type HyperConnector = hyper_tls::HttpsConnector<hyper::client::HttpConnector>;
type HyperClient = hyper::client::Client<HyperConnector>;

/// This struct contains the [Fetch] object, which can be used to fetch
/// resources using the internet. This object can be safely shared across
/// components and threads.
#[derive(Clone, Debug)]
pub struct Fetch {
    client: HyperClient,
    runtime: Arc<tokio::runtime::Runtime>,
    user_agent_product: Arc<str>,
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

    /// Create a new [Fetch] object.
    pub fn new() -> Self {
        Self::with_user_agent("Mozilla/5.0 Retina-Fetch")
    }

    /// Create a new [Fetch] object.
    pub fn with_user_agent<S>(user_agent: S) -> Self
            where S: Into<Arc<str>> {
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
            user_agent_product: user_agent.into(),
        }
    }

    /// Load the resource associated with the [`request`][Request]
    /// asynchronously.
    pub fn fetch(&self, request: Request) -> FetchPromise {
        let request = Arc::new(request);

        match request.url.scheme() {
            "file" => self.fetch_file(request),
            "http" | "https" => self.fetch_http(request),
            _ => self.fetch_unknown_scheme(request),
        }
    }

    /// Fetch a document, given the [`url`][Url] and optionally a
    /// [referrer][RequestReferrer].
    pub fn fetch_document(&self, url: Url, referrer: RequestReferrer) -> FetchPromise {
        if url.scheme() == "about" {
            return self.fetch_document_about(url);
        }

        if url.scheme() == "file" {
            return self.fetch_document_file(url);
        }

        self.fetch(Request::get_document(url, referrer))
    }

    /// Fetch a `about` document.
    fn fetch_document_about(&self, url: Url) -> FetchPromise {
        let body = match url.path() {
            // https://fetch.spec.whatwg.org/#scheme-fetch
            "blank" => "",
            _ => "", // TODO
        };

        let request = Arc::new(Request::get_document(url, RequestReferrer::default()));

        self.create_instantaneous_response(
            Arc::clone(&request),
            Ok(Response::new_about(request, body)),
        )
    }

    /// Fetch a file from the filesystem.
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

        let user_agent = Arc::clone(&self.user_agent_product);
        self.runtime.spawn(async move {
            let client = task_client;
            let request = task_request;

            let mut hyper_request = hyper::Request::builder()
                .uri(request.url.as_str())
                .method(&request.method)
                .header(http::header::ACCEPT, request.accept_header_value())
                .header(http::header::CONNECTION, "keep-alive")
                .header(http::header::USER_AGENT, user_agent.as_ref())
                .header("Sec-Fetch-Dest", request.destination.as_str())
                .header("Sec-Fetch-Mode", request.mode.as_str())
            ;

            // TODO follow <https://w3c.github.io/webappsec-referrer-policy/#determine-requests-referrer>
            if let RequestReferrer::Url(url) = &request.referrer {
                hyper_request = hyper_request.header(http::header::REFERER, url.to_string());
            }

            let hyper_request = hyper_request
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
                Ok(response) => {
                    if response.status().is_redirection() {
                        trace!("Redirection from {}", request.url.as_str());
                    }

                    if response.status().is_client_error() || response.status().is_server_error() {
                        warn!("Failed to fetch \"{}\": {}", request.url.as_ref(), response.status());
                        warn!("Response Headers: {:#?}", response.headers());
                    }

                    Ok((request, response).into())
                }
                Err(e) => Err(e.into()),
            };

            sender.send(response).await.unwrap();
        });

        FetchPromise {
            request,
            receiver,
        }
    }

    /// Handle a fetch to an unknown scheme.
    fn fetch_unknown_scheme(&self, request: Arc<Request>) -> FetchPromise {
        warn!("Unknown scheme: \"{}\" for URL: {}", request.url().scheme(), request.url().as_str());
        warn!("{:#?}", request.url());
        if request.destination == crate::RequestDestination::Document {
            self.fetch_unknown_scheme_document(request)
        } else {
            self.fetch_unknown_scheme_asset(request)
        }
    }

    /// Asset in the sense of non-documents
    fn fetch_unknown_scheme_asset(&self, request: Arc<Request>) -> FetchPromise {
        self.create_instantaneous_response(
            Arc::clone(&request),
            Ok(Response::new_about(request, "")),
        )
    }

    fn fetch_unknown_scheme_document(&self, request: Arc<Request>) -> FetchPromise {
        self.create_instantaneous_response(
            Arc::clone(&request),
            Ok(Response::new_about(request, "Unknown URL scheme.")),
        )
    }

    fn fetch_document_file(&self, url: Url) -> FetchPromise {
        self.fetch_file(Arc::new(Request::get_document(url, RequestReferrer::default())))
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
