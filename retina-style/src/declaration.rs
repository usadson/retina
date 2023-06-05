// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::{Property, Value};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Declaration {
    pub(crate) property: Property,
    pub(crate) value: Value,
}

impl Declaration {
    pub fn new(property: Property, value: Value) -> Self {
        Self { property, value }
    }

    pub fn property(&self) -> Property {
        self.property
    }

    pub fn value(&self) -> &Value {
        &self.value
    }
}
