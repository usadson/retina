// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

//! The [Interface `Tex`][spec] implementation.
//!
//! [spec]: https://dom.spec.whatwg.org/#interface-comment

use std::ops::{Deref, DerefMut};

use retina_common::{StrTendril, DynamicSizeOf};

use crate::{CharacterData, Node, NodeKind};

/// The [Interface `Tex`][spec] implementation.
///
/// [spec]: https://dom.spec.whatwg.org/#interface-comment
#[derive(Debug)]
pub struct Comment {
    superclass_character_data: CharacterData,
}

impl Comment {
    pub fn new(data: StrTendril) -> Self {
        Self {
            superclass_character_data: CharacterData::new(data),
        }
    }

    pub fn new_handle(data: StrTendril) -> Node {
        Node::new(
            NodeKind::Comment(
                Self::new(data)
            )
        )
    }
}

impl DynamicSizeOf for Comment {
    fn dynamic_size_of(&self) -> usize {
        self.superclass_character_data.dynamic_size_of()
    }
}

impl Deref for Comment {
    type Target = CharacterData;

    fn deref(&self) -> &Self::Target {
        &self.superclass_character_data
    }
}

impl DerefMut for Comment {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.superclass_character_data
    }
}
