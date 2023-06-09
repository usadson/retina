// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

pub mod inline;

pub use inline::InlineFormattingContext;

#[derive(Clone, Debug)]
pub struct FormattingContext {

}

#[derive(Clone, Debug)]
pub enum FormattingContextKind {
    Inline(InlineFormattingContext),
}
