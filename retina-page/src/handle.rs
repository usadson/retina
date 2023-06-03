// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::{sync::mpsc::{Receiver, Sender, RecvTimeoutError}, time::Duration};

use crate::{PageCommand, PageMessage};

/// This class represents the bridge between a page and a browser.
pub struct PageHandle {
    pub(crate) is_page_still_connected: bool,
    pub(crate) command_sender: Sender<PageCommand>,
    pub(crate) message_receiver: Receiver<PageMessage>,
    pub receive_timeout: Duration,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PageHandleCommunicationError {
    Disconnected,
    Timeout,
}

impl PageHandle {
    pub fn receive_message(&mut self) -> Result<PageMessage, PageHandleCommunicationError> {
        if !self.is_page_still_connected {
            return Err(PageHandleCommunicationError::Disconnected);
        }

        match self.message_receiver.recv_timeout(self.receive_timeout) {
            Ok(message) => Ok(message),

            Err(RecvTimeoutError::Disconnected) => {
                self.is_page_still_connected = false;
                Err(PageHandleCommunicationError::Disconnected)
            }

            Err(RecvTimeoutError::Timeout) => Err(PageHandleCommunicationError::Timeout),
        }
    }

    pub fn send_command(&mut self, command: PageCommand) -> Result<(), PageHandleCommunicationError> {
        if self.is_page_still_connected {
            self.is_page_still_connected = self.command_sender.send(command).is_ok();
        }

        if self.is_page_still_connected {
            Ok(())
        } else {
            Err(PageHandleCommunicationError::Disconnected)
        }
    }
}
