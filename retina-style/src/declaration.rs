// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::{Property, Value};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Declaration {
    pub(crate) property: Property,
    pub(crate) value: Value,
}
