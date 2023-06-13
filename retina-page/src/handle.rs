// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::{
    sync::mpsc::{Receiver, RecvTimeoutError, Sender},
    time::Duration,
};

use crate::{PageCommand, PageMessage};

/// This class represents the bridge between a page and a browser.
pub struct PageHandle {
    pub(crate) send: PageHandleSendHalf,
    pub(crate) receive: PageHandleReceiveHalf,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PageHandleCommunicationError {
    Disconnected,
    Timeout,
}

pub struct PageHandleReceiveHalf {
    pub(crate) is_page_still_connected: bool,
    pub(crate) message_receiver: Receiver<PageMessage>,
    pub receive_timeout: Duration,
}

#[derive(Clone)]
pub struct PageHandleSendHalf {
    pub(crate) is_page_still_connected: bool,
    pub(crate) command_sender: Sender<PageCommand>,
}

impl PageHandleReceiveHalf {
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
}

impl PageHandleSendHalf {
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

impl PageHandle {
    pub fn receive_message(&mut self) -> Result<PageMessage, PageHandleCommunicationError> {
        self.receive.receive_message()
    }

    pub fn send_command(&mut self, command: PageCommand) -> Result<(), PageHandleCommunicationError> {
        self.send.send_command(command)
    }

    pub fn split(self) -> (PageHandleSendHalf, PageHandleReceiveHalf) {
        (self.send, self.receive)
    }
}

impl From<PageHandle> for (PageHandleSendHalf, PageHandleReceiveHalf) {
    fn from(value: PageHandle) -> Self {
        value.split()
    }
}
