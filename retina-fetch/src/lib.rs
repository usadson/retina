// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

//! This API is modeled after the [Fetch API][spec]
//!
//! # References
//! * [Fetch API - WHATWG Specification][spec]
//! * [Fetch API - MDN][mdn]
//!
//! [mdn]: https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API
//! [spec]: https://fetch.spec.whatwg.org

pub(crate) mod destination;
pub(crate) mod error;
pub(crate) mod initiator;
pub(crate) mod fetch;
pub(crate) mod promise;
pub(crate) mod request;
pub(crate) mod response;
pub(crate) mod status_code;

pub use destination::RequestDestination;
pub use error::{Error, InternalError};
pub use fetch::Fetch;
pub use initiator::RequestInitiator;
pub use promise::FetchPromise;
pub use request::Request;
pub use response::Response;
pub use status_code::{StatusCode, StatusCodeClass};

pub type FetchResponse = Result<Response, Error>;
