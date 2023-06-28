// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::borrow::Cow;

use crate::formatting_context::FormattingContextWhitespaceState;

pub fn collapse_white_space<'str>(
    input: Cow<'str, str>,
    whitespace_state: FormattingContextWhitespaceState,
) -> Cow<'str, str> {
    let should_start_with_space = input.starts_with(|c: char| c.is_ascii_whitespace())
        && whitespace_state == FormattingContextWhitespaceState::NoWhitespace;

    let should_end_with_space = input.ends_with(|c: char| c.is_ascii_whitespace());

    let mut string = String::with_capacity(input.len());
    if should_start_with_space {
        string.push(' ');
    }

    for word in input.split_ascii_whitespace() {
        string.push_str(word);
        string.push(' ');
    }

    if !should_end_with_space {
        _ = string.pop();
    }

    Cow::Owned(string)
}
