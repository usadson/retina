// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

//! # References
//! * [CSS - Selectors Level 4 - 3.1](https://www.w3.org/TR/selectors-4/#simple)

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
    /// The `*` selector.
    ///
    /// # References
    /// * [CSS - Selectors Level 4 - 5.2](https://www.w3.org/TR/selectors-4/#universal-selector)
    Universal,
}
