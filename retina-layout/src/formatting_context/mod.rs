// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

pub mod inline;
pub mod block;

pub use block::BlockFormattingContext;
pub use inline::InlineFormattingContext;
use retina_style::CssReferencePixels;

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

    pub(crate) max_width: Option<CssReferencePixels>,
    pub(crate) max_height: Option<CssReferencePixels>,
}

impl<'bx> FormattingContext<'bx> {
    pub(crate) fn new(
        parent: Option<&FormattingContext>,
        layout_box: &'bx mut LayoutBox,
    ) -> Self {
        let max_width = parent.map(|parent| parent.max_width)
            .flatten()
            .map(|value| value - layout_box.dimensions().combined_horizontal_edges());

        let max_height = parent.map(|parent| parent.max_height)
            .flatten()
            .map(|value| value - layout_box.dimensions().combined_vertical_edges());

        Self {
            layout_box,
            whitespace_state: FormattingContextWhitespaceState::Initial,
            max_width,
            max_height,
        }
    }
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
