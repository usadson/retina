// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::sync::mpsc::Sender;

use retina_common::StrTendril;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PlatformMessage {
    TitleUpdated(StrTendril),
}

#[derive(Clone, Debug)]
pub struct PlatformMessenger {
    sender: Sender<PlatformMessage>,
}

impl PlatformMessenger {
    #[inline]
    pub fn send(&self, message: PlatformMessage) {
        _ = self.sender.send(message);
    }

    #[inline]
    pub fn send_title(&self, title: impl Into<StrTendril>) {
        self.send(PlatformMessage::TitleUpdated(title.into()));
    }
}
