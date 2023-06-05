// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use core::fmt::Debug;
use std::{borrow::{Borrow, BorrowMut}, ops::{Deref, DerefMut}};

/// The transparent wrapper wraps a generic object transparently in a
/// single-field tuple. This can be useful for having generics of either
/// `Option<T>` or `TransparentWrapper<T>`.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct TransparentWrapper<Inner>(Inner);

impl<Inner> Debug for TransparentWrapper<Inner>
        where Inner: Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<Inner> TransparentWrapper<Inner> {
    pub const fn new(value: Inner) -> Self {
        Self(value)
    }
}

impl<Inner> AsRef<Inner> for TransparentWrapper<Inner> {
    fn as_ref(&self) -> &Inner {
        &self.0
    }
}

impl<Inner> AsMut<Inner> for TransparentWrapper<Inner> {
    fn as_mut(&mut self) -> &mut Inner {
        &mut self.0
    }
}

impl<Inner> Borrow<Inner> for TransparentWrapper<Inner> {
    fn borrow(&self) -> &Inner {
        &self.0
    }
}

impl<Inner> BorrowMut<Inner> for TransparentWrapper<Inner> {
    fn borrow_mut(&mut self) -> &mut Inner {
        &mut self.0
    }
}

impl<Inner> Deref for TransparentWrapper<Inner> {
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Inner> DerefMut for TransparentWrapper<Inner> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<Inner> From<Inner> for TransparentWrapper<Inner> {
    fn from(value: Inner) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::TransparentWrapper;

    #[test]
    fn debug() {
        let value = 5;
        let _obj = TransparentWrapper::new(value);
        assert_eq!(format!("{_obj:#?}"), format!("{value:#?}"));
    }

    #[test]
    fn as_ref() {
        let value = "hello";
        let obj = TransparentWrapper::new(value);
        assert!(obj.as_ref().eq_ignore_ascii_case("HelLO"));
    }

    #[test]
    fn deref() {
        let value = "hello";
        let obj = TransparentWrapper::new(value);
        assert!(obj.eq_ignore_ascii_case("HELLO"));
    }
}
