// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

//! The [Interface `Tex`](https://dom.spec.whatwg.org/#interface-text)
//! implementation.

use std::ops::{Deref, DerefMut};

use retina_common::StrTendril;

use crate::{CharacterData, Node, NodeKind};

/// The [Interface `Text`](https://dom.spec.whatwg.org/#interface-text)
/// implementation.
#[derive(Debug)]
pub struct Text {
    superclass_character_data: CharacterData,
}

impl Text {
    pub fn new(data: StrTendril) -> Self {
        Self {
            superclass_character_data: CharacterData::new(data),
        }
    }

    pub fn new_handle(data: StrTendril) -> Node {
        Node::new(
            NodeKind::Text(
                Self::new(data)
            )
        )
    }
}

impl Deref for Text {
    type Target = CharacterData;

    fn deref(&self) -> &Self::Target {
        &self.superclass_character_data
    }
}

impl DerefMut for Text {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.superclass_character_data
    }
}
