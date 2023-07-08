// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

pub trait StrExt {
    /// Get the index of the substring, which can be useful for extending a
    /// slice, or just getting the length of the whole thing.
    ///
    /// ```
    /// let string = "Hello, world!";
    /// let world = &string[7..];
    /// assert_eq!(string.index_of_substring(world), Some(7));
    /// ```
    fn index_of_substring(&self, other: &str) -> Option<usize>;
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
}
