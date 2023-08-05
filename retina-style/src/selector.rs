// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

//! # References
//! * [CSS - Selectors Level 4 - 3.1](https://www.w3.org/TR/selectors-4/#simple)

use retina_common::StrTendril;

mod pseudo;

pub use self::pseudo::{
    FunctionalPseudoClassSelectorKind,
    PseudoClassSelectorKind,
};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AttributeSelector {
    pub(crate) attribute: StrTendril,
    pub(crate) case_sensitivity: AttributeSelectorCaseSensitivity,
    pub(crate) kind: AttributeSelectorKind,
}

impl AttributeSelector {
    pub fn new(
        attribute: StrTendril,
        case_sensitivity: AttributeSelectorCaseSensitivity,
        kind: AttributeSelectorKind,
    ) -> Self {
        Self { attribute, case_sensitivity, kind }
    }

    pub fn attribute_name(&self) -> &StrTendril {
        &self.attribute
    }

    pub fn case_sensitivity(&self) -> AttributeSelectorCaseSensitivity {
        self.case_sensitivity
    }

    pub fn kind(&self) -> &AttributeSelectorKind {
        &self.kind
    }
}

/// <https://www.w3.org/TR/selectors-4/#attribute-case>
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum AttributeSelectorCaseSensitivity {
    /// Depends on the document language.
    Default,

    /// `i`
    AsciiCaseInsensitive,

    /// `s`
    Identical,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum AttributeSelectorKind {
    /// `[attr]`
    RegardlessOfValue,

    /// `[attr=val]`
    Exact(StrTendril),

    /// `[attr~=val]`
    OneOfWhitespaceSeparatedList(StrTendril),

    /// `[attr|=val]`
    ExactOrStartsWithAndHyphen(StrTendril),

    /// `[attr^=val]`
    BeginsWith(StrTendril),

    /// `[attr$=val]`
    EndsWith(StrTendril),

    /// `[attr*=val]`
    Contains(StrTendril),
}

/// # References
/// * [CSS - Selectors Level 4 - 3.1](https://drafts.csswg.org/selectors/#complex)
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ComplexSelector {
    pub topmost: CompoundSelector,
    pub combinators: Vec<(SelectorCombinator, CompoundSelector)>,
}

/// # References
/// * [CSS - Selectors Level 4 - 3.1](https://www.w3.org/TR/selectors-4/#compound)
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CompoundSelector(pub Vec<SimpleSelector>);

/// # References
/// * [CSS - Selectors Level 4 - 3.1](https://www.w3.org/TR/selectors-4/#simple)
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Selector {
    Complex(ComplexSelector),
    Compound(CompoundSelector),
    Simple(SimpleSelector),
}

/// # References
/// * [CSS - Selectors Level 4 - 3.1](https://drafts.csswg.org/selectors/#selector-combinator)
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SelectorCombinator {
    /// ` ` (white space)
    Descendant,

    /// `>`
    Child,

    /// `+`
    NextSibling,

    /// `~`
    SubsequentSibling,
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
    Attribute(AttributeSelector),

    /// The class selector selects an element if that class is one of the
    /// elements classes.
    ///
    /// # References
    /// * [CSS - Selectors Level 4 - 6.6](https://www.w3.org/TR/selectors-4/#class-html)
    Class(StrTendril),

    /// The class selector selects an element if that id is the element's id.
    ///
    /// # References
    /// * [CSS - Selectors Level 4 - 6.6](https://www.w3.org/TR/selectors-4/#class-html)
    Id(StrTendril),

    PseudoClass(PseudoClassSelectorKind),

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
