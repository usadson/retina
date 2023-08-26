// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use url::Url;

#[derive(Clone, Debug, Default, PartialEq)]
pub enum RequestReferrer {
    NoReferrer,
    #[default]
    Client,
    Url(Url),
}
