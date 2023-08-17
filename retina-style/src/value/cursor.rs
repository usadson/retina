// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use strum::{AsRefStr, EnumIter};

/// The values for the [`cursor`][spec] property.
///
/// ## TODO
/// Support URL cursors.
///
/// ## References
/// * [CSS Basic User Interface Module Level 4][spec]
///
/// [spec]: https://drafts.csswg.org/css-ui/#cursor
#[derive(Copy)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[derive(Default, AsRefStr, EnumIter)]
#[strum(serialize_all = "kebab-case")]
pub enum CssCursor {
    #[default]
    Auto,
    Default,
    None,
    ContextMenu,
    Help,
    Pointer,
    Progress,
    Wait,
    Cell,
    Crosshair,
    Text,
    VerticalText,
    Alias,
    Copy,
    Move,
    NoDrop,
    NotAllowed,
    Grab,
    Grabbing,
    EResize,
    NResize,
    NeResize,
    NwResize,
    SResize,
    SeResize,
    SwResize,
    WResize,
    EwResize,
    NsResize,
    NeswResize,
    NwseResize,
    ColResize,
    RowResize,
    AllScroll,
    ZoomIn,
    ZoomOut,
}
