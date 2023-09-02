// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::borrow::Cow;

use retina_i18n::IetfLanguageSubtag;
use retina_style::CssTextTransform;

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

/// Test whether or not the given `value` is (contains) an emoji. Emoji in
/// Unicode are complex, so a simple check isn't possible unfortunately.
///
/// ## Resources
/// * [UTS #51: Unicode Emoji](https://www.unicode.org/reports/tr51/#def_rgi_set)
/// * [UTF-8 to Code Points Converter](https://onlineutf8tools.com/convert-utf8-to-code-points)
///   might be useful for dissecting text to find out why something does or
///   doesn't work.
pub fn is_emoji(value: &str) -> bool {
    use unicode_properties::UnicodeEmoji;
    value.chars().all(|c| c.is_emoji_char_or_emoji_component())
}

pub fn transform<'str>(
    text: Cow<'str, str>,
    text_transform: CssTextTransform,
    language: Option<IetfLanguageSubtag>,
) -> Cow<'str, str> {
    match text_transform {
        CssTextTransform::None => text,
        CssTextTransform::Lowercase => Cow::Owned(text.to_lowercase()),
        CssTextTransform::Uppercase => Cow::Owned(text.to_uppercase()),
        CssTextTransform::Capitalize => transform_capitalize(text, language),
    }
}

/// Puts the first typographic letter unit of each word, if lowercase, in
/// titlecase; other characters are unaffected. ([spec][spec])
///
/// [spec]: https://drafts.csswg.org/css-text-4/#valdef-text-transform-capitalize
fn transform_capitalize<'str>(
    text: Cow<'str, str>,
    language: Option<IetfLanguageSubtag>,
) -> Cow<'str, str> {
    let mut iter = text.char_indices();
    let Some((_, first_char)) = iter.next() else {
        return text;
    };

    if (first_char == 'i' || first_char == 'I') && language == Some(IetfLanguageSubtag::Dutch) {
        let mut iter = iter.clone();
        if let Some((_, next_char)) = iter.next() {
            if next_char == 'j' || next_char == 'J' {
                let rest = iter.next()
                    .map(|(idx, _)| &text[idx..])
                    .unwrap_or_default();
                return Cow::Owned(format!("{}{}{rest}", first_char.to_uppercase(), next_char.to_uppercase()));
            }
        }
    }

    let rest = iter.next()
        .map(|(idx, _)| &text[idx..])
        .unwrap_or_default();

    Cow::Owned(format!("{}{rest}", first_char.to_uppercase()))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case("❤️", true)] // U+2764 U+FF0F
    #[case("👩🏼‍💻", true)] // U+1F469 U+1F3FC U+200D U+1F4BB
    #[case("Hello, world!", false)]
    #[case("اَلْعَرَبِيَّةُ", false)]
    #[case("4.0", false)]
    #[case("Hello, world!", false)]
    fn test_is_emoji(#[case] input: &str, #[case] should_be: bool) {
        if super::is_emoji(input) == should_be {
            return;
        }

        println!("Test failed for: {input}, which result should've been: {should_be}");

        use unicode_properties::{UnicodeEmoji, UnicodeGeneralCategory};

        let breakdown: Vec<_> = input.chars().map(|c| {
            (c, c as u32, c.emoji_status(), c.general_category(), c.general_category())
        }).collect();

        println!("Breakdown of code points: {breakdown:#?}");
        panic!("Test failed for {input}, should_be={should_be}")
    }
}
