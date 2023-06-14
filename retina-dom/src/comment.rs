// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

//! The [Interface `Tex`][spec] implementation.
//!
//! [spec]: https://dom.spec.whatwg.org/#interface-comment

use std::{ops::{Deref, DerefMut}, rc::Rc};

use tendril::StrTendril;

use crate::{CharacterData, NodeKind};

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

    pub fn new_handle(data: StrTendril) -> Rc<NodeKind> {
        Rc::new(
            NodeKind::Comment(
                Self::new(data)
            )
        )
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
