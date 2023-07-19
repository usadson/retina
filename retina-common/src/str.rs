// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

pub trait StrExt {
    /// Get the index of the substring, which can be useful for extending a
    /// slice, or just getting the length of the whole thing.
    fn index_of_substring(&self, other: &str) -> Option<usize>;

    fn slice_from_substring(&self, other: &str) -> Option<&str>;

    fn slice_after_substring(&self, other: &str) -> Option<&str>;

    fn as_end_ptr(&self) -> *const u8;

    fn try_include_following_space(&self, other: &str) -> Option<&str>;
}

impl StrExt for str {
    fn index_of_substring(&self, other: &str) -> Option<usize> {
        let self_begin = self.as_ptr() as usize;
        let self_end = self_begin + self.len();

        let other_begin = other.as_ptr() as usize;
        let other_end = other_begin + other.len();

        if self_begin > other_begin {
            return None;
        }

        if other_end > self_end {
            return None;
        }

        Some(other_begin - self_begin)
    }

    fn as_end_ptr(&self) -> *const u8 {
        (self.as_ptr() as usize + self.len()) as _
    }

    fn slice_after_substring(&self, other: &str) -> Option<&str> {
        let other_index = self.index_of_substring(other)?;
        let after_index = other_index + other.len();

        if after_index < other.len() {
            Some(&self[after_index..])
        } else {
            None
        }
    }

    fn slice_from_substring(&self, other: &str) -> Option<&str> {
        let other_index = self.index_of_substring(other)?;

        if other_index + other.len() < self.len() {
            Some(&self[other_index..])
        } else {
            None
        }
    }

    fn try_include_following_space(&self, word: &str) -> Option<&str> {
        let word_and_after = self.slice_from_substring(word)?;

        debug_assert_eq!(word, &word_and_after[0..word.len()]);

        if word_and_after.len() == word.len() {
            return None;
        }

        let chars_in_word = word.chars().count();
        let after_word_char = word_and_after.chars().nth(chars_in_word)?;

        if after_word_char.is_ascii_whitespace() {
            Some(&word_and_after[0..word.len() + 1])
        } else {
            None
        }
    }
}
