// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

//! # References
//! * [CSS - Selectors Level 4 - 3.1](https://www.w3.org/TR/selectors-4/#simple)

use tendril::StrTendril;

/// # References
/// * [CSS - Selectors Level 4 - 3.1](https://www.w3.org/TR/selectors-4/#simple)
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Selector {
    Simple(SimpleSelector),
}

/// # References
/// * [CSS - Selectors Level 4 - 3.1](https://www.w3.org/TR/selectors-4/#simple)
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SelectorList {
    pub items: Vec<Selector>,
}

/// # References
/// * [CSS - Selectors Level 4 - 3.1](https://www.w3.org/TR/selectors-4/#simple)
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SimpleSelector {
    /// The type selector selects an element by it's tag name.
    ///
    /// > A ___type selector___ is the name of a document language element type,
    /// > and represents an instance of that element type in the document tree.
    ///
    /// # References
    /// * [CSS - Selectors Level 4 - 5.1](https://www.w3.org/TR/selectors-4/#type-selector)
    TypeSelector(StrTendril),

    /// The `*` selector.
    ///
    /// # References
    /// * [CSS - Selectors Level 4 - 5.2](https://www.w3.org/TR/selectors-4/#universal-selector)
    Universal,
}
