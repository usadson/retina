// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::fmt::Display;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CssDisplay {
    Normal {
        inside: CssDisplayInside,
        outside: CssDisplayOutside,
        is_list_item: bool,
    },

    Internal(CssDisplayInternal),

    Box(CssDisplayBox),
}

impl Display for CssDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal { inside, outside, .. } => {
                f.write_fmt(format_args!("{inside} {outside}"))
            }
            Self::Internal(internal) => f.write_fmt(format_args!("{internal}")),
            Self::Box(display_box) => f.write_fmt(format_args!("{display_box}")),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[derive(strum::Display)]
#[strum(serialize_all = "kebab-case")]
pub enum CssDisplayBox {
    Contents,
    None,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[derive(strum::Display)]
#[strum(serialize_all = "kebab-case")]
pub enum CssDisplayInside {
    Flow,
    FlowRoot,
    Table,
    Flex,
    Grid,
    Ruby,
}

/// <https://drafts.csswg.org/css-display-4/#typedef-display-internal>
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[derive(strum::Display)]
#[strum(serialize_all = "kebab-case")]
pub enum CssDisplayInternal {
    TableRowGroup,
    TableHeaderGroup,
    TableFooterGroup,
    TableRow,
    TableCell,
    TableColumnGroup,
    TableColumn,
    TableCaption,
    RubyBase,
    RubyText,
    RubyBaseContainer,
    RubyTextContainer,
}

/// <https://drafts.csswg.org/css-display-4/#typedef-display-outside>
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[derive(strum::Display)]
#[strum(serialize_all = "kebab-case")]
pub enum CssDisplayOutside {
    Block,
    Inline,
    RunIn,
}
