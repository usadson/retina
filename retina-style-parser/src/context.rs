// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::{
    error::display_parse_error,
    ParseError,
};

/// Show at most 20 error messages, to prevent the console from being spammed.
const MAXIMUM_ERRORS_TO_DISPLAY: usize = 20;

#[derive(Debug, Default)]
pub(crate) struct Context {
    pub(crate) error_count: usize,
}

impl Context {
    pub(crate) fn parse_error<'i, 't>(
        &mut self,
        parser: &cssparser::Parser<'i, 't>,
        constituent: &str,
        error: (ParseError<'i>, &str)
    ) {
        self.error_count += 1;

        if self.error_count <= MAXIMUM_ERRORS_TO_DISPLAY {
            display_parse_error(parser, constituent, error)
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        if self.error_count <= MAXIMUM_ERRORS_TO_DISPLAY {
            return;
        }

        let errors_skipped = self.error_count - MAXIMUM_ERRORS_TO_DISPLAY;
        let word = if errors_skipped == 1 { "error" } else { "skipped" };

        log::warn!("{errors_skipped} {word} skipped");
    }
}
