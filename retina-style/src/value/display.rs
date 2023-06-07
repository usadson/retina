// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

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

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CssDisplayBox {
    Contents,
    None,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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
pub enum CssDisplayOutside {
    Block,
    Inline,
    RunIn,
}
