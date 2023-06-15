// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use strum::EnumString;

use crate::LinkKind;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[derive(EnumString, strum::Display, strum::AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum LinkType {
    Alternate,

    Author,

    Bookmark,

    Canonical,

    DnsPrefetch,

    External,

    Help,

    Icon,

    Manifest,

    #[strum(serialize = "modulepreload")]
    ModulePreload,

    License,

    Next,

    #[strum(serialize = "nofollow")]
    NoFollow,

    #[strum(serialize = "noopener")]
    NoOpener,

    #[strum(serialize = "noreferrer")]
    NoReferrer,

    Opener,

    Pingback,

    Preconnect,

    Prefetch,

    Preload,

    Prev,

    Search,

    Stylesheet,

    Tag,
}

impl LinkType {
    pub fn from_str(s: &str) -> Option<Self> {
        std::str::FromStr::from_str(s).ok()
    }

    pub fn from_str_link_element(s: &str) -> Option<Self> {
        Self::from_str(s).filter(|ty| ty.link_element_effect().is_some())
    }

    /// Gives the effect of this [`LinkType`] for that it has on a `<link>`
    /// HTML element.
    ///
    /// ## References
    /// * [HTML Living Standard § 4.6.7.][spec]
    ///
    /// [spec]: https://html.spec.whatwg.org/multipage/links.html#linkTypes
    pub fn link_element_effect(&self) -> Option<LinkKind> {
        match self {
            Self::Alternate => Some(LinkKind::Hyperlink),
            Self::Author => Some(LinkKind::Hyperlink),
            Self::Canonical => Some(LinkKind::Hyperlink),
            Self::Bookmark => None,
            Self::DnsPrefetch => Some(LinkKind::ExternalResource),
            Self::External => None,
            Self::Help => Some(LinkKind::Hyperlink),
            Self::Icon => Some(LinkKind::ExternalResource),
            Self::Manifest => Some(LinkKind::ExternalResource),
            Self::ModulePreload => Some(LinkKind::ExternalResource),
            Self::License => Some(LinkKind::Hyperlink),
            Self::Next => Some(LinkKind::Hyperlink),
            Self::NoFollow => None,
            Self::NoOpener => None,
            Self::NoReferrer => None,
            Self::Opener => None,
            Self::Pingback => Some(LinkKind::ExternalResource),
            Self::Preconnect => Some(LinkKind::ExternalResource),
            Self::Prefetch => Some(LinkKind::ExternalResource),
            Self::Preload => Some(LinkKind::ExternalResource),
            Self::Prev => Some(LinkKind::Hyperlink),
            Self::Search => Some(LinkKind::Hyperlink),
            Self::Stylesheet => Some(LinkKind::ExternalResource),
            Self::Tag => None,
        }
    }
}
