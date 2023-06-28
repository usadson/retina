// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

pub mod inline;
pub mod block;

pub use block::BlockFormattingContext;
pub use inline::InlineFormattingContext;

use crate::LayoutBox;

#[derive(Debug)]
pub struct FormattingContext<'bx> {
    pub(crate) layout_box: &'bx mut LayoutBox,
    pub(crate) ended_with_whitespace: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum FormattingContextKind {
    Block,
    Inline,
}
