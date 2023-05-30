// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use winit::event::{KeyboardInput, ElementState, ModifiersState, VirtualKeyCode};

/// The keyboard state of a window.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct WindowKeyboardState {
    modifiers_state: ModifiersState,
}

impl WindowKeyboardState {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn on_input(&mut self, event: KeyboardInput) {
        if event.state != ElementState::Pressed {
            return;
        }

        if self.modifiers_state.ctrl() && event.virtual_keycode == Some(VirtualKeyCode::W) {
            std::process::exit(0);
        }
    }

    pub(crate) fn on_modifiers_event(&mut self, modifiers_state: ModifiersState) {
        self.modifiers_state = modifiers_state;
    }
}
