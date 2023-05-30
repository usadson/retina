// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use winit::event::{
    DeviceEvent,
    ModifiersState,
};

use super::keyboard::WindowKeyboardState;

/// Handles the state of a [`Window`].
pub(crate) struct WindowState {
    pub(crate) keyboard_state: WindowKeyboardState,
}

//
// Window Events
//
impl WindowState {
    pub fn new() -> Self {
        Self {
            keyboard_state: WindowKeyboardState::new(),
        }
    }

    pub(crate) fn on_device_event(&mut self, event: DeviceEvent) {
        match event {
            DeviceEvent::Key(event) => self.keyboard_state.on_input(event),
            _ => (),
        }
    }

    pub(crate) fn on_modifiers_event(&mut self, event: ModifiersState) {
        self.keyboard_state.on_modifiers_event(event);
    }
}
