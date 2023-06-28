// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

pub mod inline;
pub mod block;

pub use block::BlockFormattingContext;
pub use inline::InlineFormattingContext;

use crate::LayoutBox;

#[derive(Debug)]
pub struct FormattingContext<'bx> {
    /// The layout box that is the container for a given Formatting Context.
    pub(crate) layout_box: &'bx mut LayoutBox,

    /// This state determines the state of whitespace in the formatting context,
    /// which is necessary for following the
    /// [White Space Processing Rules][spec].
    ///
    /// [spec]: https://drafts.csswg.org/css-text/#white-space-rules
    pub(crate) whitespace_state: FormattingContextWhitespaceState,
}

/// This state determines the state of whitespace in the formatting context,
/// which is necessary for following the [White Space Processing Rules][spec].
///
/// [spec]: https://drafts.csswg.org/css-text/#white-space-rules
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum FormattingContextWhitespaceState {
    /// No child was encountered that affected the whitespace state.
    Initial,

    /// The last child didn't end with whitespace.
    NoWhitespace,

    /// The last child ended with whitespace.
    EndedWithWhitespace,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum FormattingContextKind {
    Block,
    Inline,
}
