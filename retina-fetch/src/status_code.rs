// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

/// A convenient wrapper for the [HTTP Status Code][spec].
///
/// [spec]: https://httpwg.org/specs/rfc9110.html#status.codes
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct StatusCode(u16);

impl StatusCode {
    /// Get the numeric value of this status code.
    pub fn as_u16(&self) -> u16 {
        self.0
    }

    /// The class of the status code determine the result of the request and the
    /// semantics of the response.
    pub fn class(&self) -> StatusCodeClass {
        match self.0 {
            100..=199 => StatusCodeClass::Informational,
            200..=299 => StatusCodeClass::Successful,
            300..=339 => StatusCodeClass::Redirection,
            400..=499 => StatusCodeClass::ClientError,
            500..=599 => StatusCodeClass::ServerError,
            _ => panic!("invalid value: {}", self.0)
        }
    }

    /// Checks if the status code is of the _client error_ class, i.e. 4xx.
    pub fn is_client_error(&self) -> bool {
        self.class().is_client_error()
    }

    /// Checks if the status code is of the _informational_ class, i.e. 1xx.
    pub fn is_informational(&self) -> bool {
        self.class().is_informational()
    }

    /// Checks if the status code is of the _redirection_ class, i.e. 3xx.
    pub fn is_redirection(&self) -> bool {
        self.class().is_redirection()
    }

    /// Checks if the status code is of the _server error_ class, i.e. 5xx.
    pub fn is_server_error(&self) -> bool {
        self.class().is_server_error()
    }

    /// Checks if the status code is of the _successful_ class, i.e. 2xx.
    pub fn is_successful(&self) -> bool {
        self.class().is_successful()
    }
}

/// The class of the status code determine the result of the request and the
/// semantics of the response.
///
/// # References
/// * [RFC 9110: HTTP Semantics § 15][spec]
///
/// [spec]: https://www.rfc-editor.org/rfc/rfc9110.html#section-15
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum StatusCodeClass {
    /// The request was received, continuing process.
    Informational,

    /// The request was successfully received, understood, and accepted.
    Successful,

    /// Further action needs to be taken in order to complete the request.
    Redirection,

    /// The request contains bad syntax or cannot be fulfilled.
    ClientError,

    /// The server failed to fulfill an apparently valid request.
    ServerError,
}

impl StatusCodeClass {
    /// Checks if the status code is of the _client error_ class, i.e. 4xx.
    pub const fn is_client_error(&self) -> bool {
        matches!(self, StatusCodeClass::ClientError)
    }

    /// Checks if the status code is of the _informational_ class, i.e. 1xx.
    pub const fn is_informational(&self) -> bool {
        matches!(self, StatusCodeClass::Informational)
    }

    /// Checks if the status code is of the _redirection_ class, i.e. 3xx.
    pub const fn is_redirection(&self) -> bool {
        matches!(self, StatusCodeClass::Redirection)
    }

    /// Checks if the status code is of the _server error_ class, i.e. 5xx.
    pub const fn is_server_error(&self) -> bool {
        matches!(self, StatusCodeClass::ServerError)
    }

    /// Checks if the status code is of the _successful_ class, i.e. 2xx.
    pub const fn is_successful(&self) -> bool {
        matches!(self, StatusCodeClass::Successful)
    }
}

impl From<hyper::StatusCode> for StatusCode {
    fn from(value: hyper::StatusCode) -> Self {
        assert!(value.as_u16() >= 100, "invalid status code");
        assert!(value.as_u16() <= 599, "invalid status code");
        Self(value.as_u16())
    }
}
