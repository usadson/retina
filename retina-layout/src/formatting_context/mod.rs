// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

pub mod inline;
pub mod block;

pub use block::BlockFormattingContext;
pub use inline::InlineFormattingContext;

#[derive(Clone, Debug)]
pub struct FormattingContext {

}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum FormattingContextKind {
    Block,
    Inline,
}
