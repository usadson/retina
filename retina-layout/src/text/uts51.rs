// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use unicode_properties::{EmojiStatus, UnicodeEmoji};

use super::CharExt;

const EMOJI_PRESENTATION_SELECTOR: char = '\u{FE0F}';
const ZERO_WIDTH_JOINER: char = '\u{200D}';

/// <https://www.unicode.org/reports/tr51/proposed.html#def_emoji_character>
/// emoji_character := \p{Emoji}
fn is_emoji_character(c: char) -> bool {
    !matches!(c.emoji_status(), EmojiStatus::NonEmoji | EmojiStatus::NonEmojiButEmojiComponent)
}

/// <https://www.unicode.org/reports/tr51/proposed.html#def_emoji_character>
/// emoji_character := \p{Emoji}
fn is_emoji_character_str(value: &str) -> bool {
    let mut chars = value.chars();

    if !chars.next().is_some_and(is_emoji_character) {
        return false;
    }

    chars.next().is_none()
}

/// <https://www.unicode.org/reports/tr51/proposed.html#def_emoji_presentation_selector>
/// emoji_presentation_selector := \x{FE0F}
///
/// <https://www.unicode.org/reports/tr51/proposed.html#def_emoji_presentation_sequence>
/// emoji_presentation_sequence := emoji_character emoji_presentation_selector
fn is_emoji_presentation_sequence(value: &str) -> bool {
    let mut chars = value.chars();

    if !chars.next().is_some_and(is_emoji_character) {
        return false;
    }

    if chars.next() != Some(EMOJI_PRESENTATION_SELECTOR) {
        return false;
    }

    chars.next().is_none()
}

fn is_emoji_keycap_sequence(value: &str) -> bool {
    _ = value; // TODO
    false
}

/// <https://www.unicode.org/reports/tr51/proposed.html#def_emoji_modifier_base>
fn is_emoji_modifier_base(c: char) -> bool {
    match c.emoji_status() {
        EmojiStatus::EmojiModifierBase => true,
        EmojiStatus::EmojiPresentationAndModifierBase => true,
        EmojiStatus::EmojiPresentationAndModifierAndEmojiComponent => true,
        _ => false,
    }
}

/// <https://www.unicode.org/reports/tr51/proposed.html#def_emoji_modifier>
fn is_emoji_modifier(c: char) -> bool {
    // TODO is this correct?
    c.emoji_status() == EmojiStatus::EmojiPresentationAndModifierAndEmojiComponent
}

/// <https://www.unicode.org/reports/tr51/proposed.html#def_emoji_modifier_sequence>
/// emoji_modifier_sequence :=
///   emoji_modifier_base emoji_modifier
fn is_emoji_modifier_sequence(value: &str) -> bool {
    let mut chars = value.chars();

    if !chars.next().is_some_and(is_emoji_modifier_base) {
        return false;
    }

    if !chars.next().is_some_and(is_emoji_modifier) {
        return false;
    }

    chars.next().is_none()
}

/// <https://www.unicode.org/reports/tr51/proposed.html#def_emoji_core_sequence>
/// emoji_core_sequence :=
///    emoji_character
///  | emoji_presentation_sequence
///  | emoji_keycap_sequence
///  | emoji_modifier_sequence
///  | emoji_flag_sequence
fn is_emoji_core_sequence(value: &str) -> bool {
    is_emoji_character_str(value)
        || is_emoji_presentation_sequence(value)
        || is_emoji_keycap_sequence(value)
        || is_emoji_modifier_sequence(value)
        || is_emoji_flag_sequence(value)
}

/// <https://www.unicode.org/reports/tr51/proposed.html#def_emoji_flag_sequence>
fn is_emoji_flag_sequence(value: &str) -> bool {
    let mut chars = value.chars();

    if !chars.next().as_ref().is_some_and(CharExt::is_regional_indicator) {
        return false;
    }

    if !chars.next().as_ref().is_some_and(CharExt::is_regional_indicator) {
        return false;
    }

    chars.next().is_none()
}

fn is_emoji_zwj_element(value: &str) -> bool {
    is_emoji_core_sequence(value)
        // || is_emoji_tag_sequence(value) // TODO
}

pub fn is_emoji_zwj_sequence(mut value: &str) -> bool {
    while !value.is_empty() {
        let next_zwj = value.find(ZERO_WIDTH_JOINER);

        let element = match next_zwj {
            Some(zwj) => {
                &value[0..zwj]
            }
            None => {
                value
            }
        };

        if !is_emoji_zwj_element(element) {
            return false;
        }

        let Some(zwj_idx) = next_zwj else {
            break;
        };

        value = &value[zwj_idx + 3..];
    }

    true
}

/// <https://www.unicode.org/reports/tr51/proposed.html#def_emoji_sequence>
/// emoji_sequence :=
///   emoji_core_sequence
/// | emoji_zwj_sequence
/// | emoji_tag_sequence
pub fn is_emoji_sequence(value: &str) -> bool {
    is_emoji_core_sequence(value)
        || is_emoji_zwj_sequence(value)
}
