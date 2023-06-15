// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::cmp::Ordering;

use retina_style::{Selector, SimpleSelector};

/// [CSS Selectors Level 4][spec].
///
/// [spec]: https://drafts.csswg.org/selectors/#specificity-rules
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct SelectorSpecificity {
    pub id_selectors: usize,
    pub class_attribute_pseudo_class_selectors: usize,
    pub type_and_pseudo_element_selectors: usize,
}

impl SelectorSpecificity {
    pub const fn new(a: usize, b: usize, c: usize) -> Self {
        Self {
            id_selectors: a,
            class_attribute_pseudo_class_selectors: b,
            type_and_pseudo_element_selectors: c,
        }
    }
}

impl PartialOrd for SelectorSpecificity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SelectorSpecificity {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let ord = self.id_selectors.cmp(&other.id_selectors);
        if ord != Ordering::Equal {
            return ord;
        }

        let ord = self.class_attribute_pseudo_class_selectors.cmp(&other.class_attribute_pseudo_class_selectors);
        if ord != Ordering::Equal {
            return ord;
        }

        self.type_and_pseudo_element_selectors.cmp(&other.type_and_pseudo_element_selectors)
    }
}

pub trait CalculateSpecificity {
    fn calculate_specificity(&self) -> SelectorSpecificity;
}

impl CalculateSpecificity for Selector {
    fn calculate_specificity(&self) -> SelectorSpecificity {
        match self {
            Selector::Simple(simple) => simple.calculate_specificity(),
        }
    }
}

impl CalculateSpecificity for SimpleSelector {
    fn calculate_specificity(&self) -> SelectorSpecificity {
        match self {
            SimpleSelector::Attribute(..)
                | SimpleSelector::Class(..)
            => SelectorSpecificity {
                class_attribute_pseudo_class_selectors: 1,
                ..Default::default()
            },

            SimpleSelector::Id(..) => SelectorSpecificity {
                id_selectors: 1,
                ..Default::default()
            },

            SimpleSelector::TypeSelector(..) => SelectorSpecificity {
                type_and_pseudo_element_selectors: 1,
                ..Default::default()
            },

            SimpleSelector::Universal => Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use retina_style::{AttributeSelector, AttributeSelectorCaseSensitivity, AttributeSelectorKind};
    use rstest::rstest;
    use super::*;
    use pretty_assertions::assert_eq;

    #[rstest]
    #[case(selector(id("my-blog")), SelectorSpecificity::new(1, 0, 0))]
    #[case(selector(class("my-post")), SelectorSpecificity::new(0, 1, 0))]
    #[case(selector(attr_name("my-data")), SelectorSpecificity::new(0, 1, 0))]
    #[case(selector(ty("article")), SelectorSpecificity::new(0, 0, 1))]
    fn specificity(#[case] selector: Selector, #[case] spec: SelectorSpecificity) {
        assert_eq!(selector.calculate_specificity(), spec);
    }

    #[rstest]
    #[case(selector(id("my-id")), Ordering::Greater, selector(class("my-class")))]
    #[case(selector(id("my-id")), Ordering::Equal, selector(id("other-id")))]
    #[case(selector(class("red")), Ordering::Equal, selector(class("blue")))]
    #[case(selector(class("red")), Ordering::Less, selector(id("blue")))]
    #[case(selector(attr_name("open")), Ordering::Less, selector(id("navbar")))]
    #[case(selector(attr_name("open")), Ordering::Equal, selector(class("bg-red")))]
    #[case(selector(attr_name("open")), Ordering::Equal, selector(attr_name("closed")))]
    #[case(selector(ty("h1")), Ordering::Less, selector(class("bg-red")))]
    #[case(selector(ty("h1")), Ordering::Less, selector(id("container")))]
    #[case(selector(id("container")), Ordering::Greater, selector(attr_name("container")))]
    fn compare_specificity(#[case] this: Selector, #[case] ord: Ordering, #[case] other: Selector) {
        let this_spec = this.calculate_specificity();
        let other_spec = other.calculate_specificity();
        assert_eq!(this_spec.cmp(&other_spec), ord, "{this:?} ({this_spec:?}) {ord:?} {other:?} ({other_spec:?})");
    }

    #[rstest]
    #[case(&[selector(attr_name("open")), selector(class("bg-red"))])]
    #[case(&[selector(class("bg-red")), selector(attr_name("closed"))])]
    #[case(&[selector(attr_name("method")), selector(id("my-id"))])]
    #[case(&[selector(class("p-3")), selector(id("my-id"))])]
    #[case(&[selector(ty("h1")), selector(class("p-3")), selector(id("my-id"))])]
    fn already_sorted_selectors(#[case] selectors: &[Selector]) {
        let specificities: Vec<_> = selectors.iter().map(CalculateSpecificity::calculate_specificity).collect();
        let mut sorted = specificities.clone();
        sorted.sort();
        assert_eq!(specificities, sorted);
    }

    fn id(name: &str) -> SimpleSelector {
        SimpleSelector::Id(name.into())
    }

    fn class(name: &str) -> SimpleSelector {
        SimpleSelector::Class(name.into())
    }

    fn attr_name(name: &str) -> SimpleSelector {
        SimpleSelector::Attribute(AttributeSelector::new(name.into(), AttributeSelectorCaseSensitivity::AsciiCaseInsensitive, AttributeSelectorKind::RegardlessOfValue))
    }

    fn selector(simple: SimpleSelector) -> Selector {
        Selector::Simple(simple)
    }

    fn ty(name: &str) -> SimpleSelector {
        SimpleSelector::TypeSelector(name.into())
    }

}
