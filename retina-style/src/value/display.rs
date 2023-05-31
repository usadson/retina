// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CssDisplay {
    /// `contents`
    Contents,

    /// `flex`, `block flex`
    BlockFlex,

    /// `block`, `block flow`
    BlockFlow,

    /// `flow-root`, `block flow-root`
    BlockFlowRoot,

    /// `list-item`, `block flow list item`
    BlockFlowListItem,

    /// `grid`, `block grid`
    BlockGrid,

    /// `block ruby`
    BlockRuby,

    /// `table`, `block table`,
    BlockTable,

    /// `inline-flex`, `inline flex`
    InlineFlex,

    /// `inline`, `inline flow`
    InlineFlow,

    /// `inline list-item` `inline flow list-item`
    InlineFlowListItem,

    /// `inline-block`, `inline flow-root`
    InlineFlowRoot,

    /// `inline-grid`, `inline grid`
    InlineGrid,

    /// `ruby`, `inline ruby`
    InlineRuby,

    /// `inline-table`, `inline table`
    InlineTable,

    /// `none`
    None,

    /// `run-in` `run-in flow`
    RunInFlow,
}
