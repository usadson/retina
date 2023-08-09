// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

//! The [Interface `Tex`](https://dom.spec.whatwg.org/#interface-text)
//! implementation.

use retina_common::{StrTendril, DynamicSizeOf};

use crate::NodeInterface;

/// The [Interface `CharacterData`][spec] implementation. This
/// **`CharacterData`** interface is the overarching interface for both the
/// [`Text`][crate::Text] and [`Comment`][crate::Comment] nodes.
///
/// [spec]: https://dom.spec.whatwg.org/#interface-characterdata
#[derive(Debug)]
pub struct CharacterData {
    superclass_node: NodeInterface,
    data: StrTendril,
}

impl CharacterData {
    pub fn new(data: StrTendril) -> Self {
        Self {
            superclass_node: NodeInterface::new(),
            data,
        }
    }

    pub fn as_node(&self) -> &NodeInterface {
        &self.superclass_node
    }

    pub fn as_node_mut(&mut self) -> &mut NodeInterface {
        &mut self.superclass_node
    }

    pub fn data(&self) -> &StrTendril {
        &self.data
    }

    pub fn data_as_str(&self) -> &str {
        &self.data
    }
}

impl DynamicSizeOf for CharacterData {
    fn dynamic_size_of(&self) -> usize {
        let mut size = std::mem::size_of_val(self);

        size += self.data.len();

        size
    }
}
