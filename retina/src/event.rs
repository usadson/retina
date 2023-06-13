// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use retina_page::PageMessage;

#[derive(Debug)]
pub enum RetinaEvent {
    PageEvent {
        message: PageMessage,
    },
}
