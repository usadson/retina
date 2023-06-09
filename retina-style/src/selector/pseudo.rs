// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use tendril::StrTendril;

use crate::SelectorList;

/// # References
/// * [CSS - Selectors Level 4 - 3.5](https://drafts.csswg.org/selectors/#functional-pseudo-class)
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum FunctionalPseudoClassSelectorKind {
    /// <https://drafts.csswg.org/selectors/#relational>
    Has(Vec<SelectorList>),

    /// <https://drafts.csswg.org/selectors/#matches>
    Is(Vec<SelectorList>),

    /// <https://drafts.csswg.org/selectors/#negation>
    Not(Vec<SelectorList>),

    /// <https://drafts.csswg.org/selectors/#zero-matches>
    Where(Vec<SelectorList>),
}

/// # References
/// * [CSS - Selectors Level 4 - 3.5](https://drafts.csswg.org/selectors/#pseudo-classes)
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PseudoClassSelectorKind {
    /// <https://drafts.csswg.org/selectors/#the-any-link-pseudo>
    AnyLink,

    /// <https://drafts.csswg.org/selectors/#the-active-pseudo>
    Active,

    /// <https://drafts.csswg.org/selectors/#autofill>
    Autofill,

    /// <https://drafts.csswg.org/selectors/#blank>
    Blank,

    /// <https://drafts.csswg.org/selectors/#media-loading-state>
    Buffering,

    /// <https://drafts.csswg.org/selectors/#checked>
    Checked,

    /// <https://drafts.csswg.org/selectors/#open-state>
    Closed,

    /// <https://drafts.csswg.org/selectors/#the-current-pseudo>
    Current,

    /// <https://drafts.csswg.org/selectors/#the-default-pseudo>
    Default,

    /// <https://drafts.csswg.org/selectors/#the-defined-pseudo>
    Defined,

    /// <https://drafts.csswg.org/selectors/#enableddisabled>
    Disabled,

    /// <https://drafts.csswg.org/selectors/#the-empty-pseudo>
    Empty,

    /// <https://drafts.csswg.org/selectors/#enableddisabled>
    Enabled,

    /// <https://drafts.csswg.org/selectors/#the-first-child-pseudo>
    FirstChild,

    /// <https://drafts.csswg.org/selectors/#the-first-of-type-pseudo>
    FirstOfType,

    /// <https://drafts.csswg.org/selectors/#the-focus-pseudo>
    Focus,

    /// <https://drafts.csswg.org/selectors/#the-focus-visible-pseudo>
    FocusVisible,

    /// <https://drafts.csswg.org/selectors/#the-focus-within-pseudo>
    FocusWithin,

    /// <https://drafts.csswg.org/selectors/#fullscreen-state>
    Fullscreen,

    /// <https://drafts.csswg.org/selectors/#the-future-pseudo>
    Future,

    /// <https://drafts.csswg.org/selectors/#the-hover-pseudo>
    Hover,

    /// <https://drafts.csswg.org/selectors/#range-pseudos>
    InRange,

    /// <https://drafts.csswg.org/selectors/#indeterminate>
    Intermediate,

    /// <https://drafts.csswg.org/selectors/#validity-pseudos>
    Invalid,

    /// <https://drafts.csswg.org/selectors/#the-last-child-pseudo>
    LastChild,

    /// <https://drafts.csswg.org/selectors/#the-last-of-type-pseudo>
    LastOfType,

    /// <https://drafts.csswg.org/selectors/#link>
    Link,

    /// <https://drafts.csswg.org/selectors/#the-local-link-pseudo>
    LocalLink,

    /// <https://drafts.csswg.org/selectors/#modal-state>
    Modal,

    /// <https://drafts.csswg.org/selectors/#sound-state>
    Muted,

    /// <https://drafts.csswg.org/selectors/#the-only-child-pseudo>
    OnlyChild,

    /// <https://drafts.csswg.org/selectors/#the-only-of-type-pseudo>
    OnlyOfType,

    /// <https://drafts.csswg.org/selectors/#open-state>
    Open,

    /// <https://drafts.csswg.org/selectors/#opt-pseudos>
    Optional,

    /// <https://drafts.csswg.org/selectors/#range-pseudos>
    OutOfRange,

    /// <https://drafts.csswg.org/selectors/#the-past-pseudo>
    Past,

    /// <https://drafts.csswg.org/selectors/#video-state>
    Paused,

    /// <https://drafts.csswg.org/selectors/#pip-state>
    PictureInPicture,

    /// <https://drafts.csswg.org/selectors/#placeholder>
    PlaceholderShown,

    /// <https://drafts.csswg.org/selectors/#video-state>
    Playing,

    /// <https://drafts.csswg.org/selectors/#rw-pseudos>
    ReadOnly,

    /// <https://drafts.csswg.org/selectors/#rw-pseudos>
    ReadWrite,

    /// <https://drafts.csswg.org/selectors/#the-root-pseudo>
    Root,

    /// <https://drafts.csswg.org/selectors/#opt-pseudos>
    Required,

    /// <https://drafts.csswg.org/selectors/#the-scope-pseudo>
    Scope,

    /// <https://drafts.csswg.org/selectors/#video-state>
    Seeking,

    /// <https://drafts.csswg.org/selectors/#media-loading-state>
    Stalled,

    /// <https://drafts.csswg.org/selectors/#the-target-pseudo>
    Target,

    /// <https://drafts.csswg.org/selectors/#the-target-within-pseudo>
    TargetWithin,

    /// <https://drafts.csswg.org/selectors/#user-pseudos>
    UserInvalid,

    /// <https://drafts.csswg.org/selectors/#user-pseudos>
    UserValid,

    /// <https://drafts.csswg.org/selectors/#validity-pseudos>
    Valid,

    /// <https://drafts.csswg.org/selectors/#link>
    Visited,

    /// <https://drafts.csswg.org/selectors/#sound-state>
    VolumeLocked,
}
