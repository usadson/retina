// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use winit::event::{KeyboardInput, ElementState, ModifiersState, VirtualKeyCode};

use crate::WindowApplication;

use super::interface::WindowKeyPressEvent;

/// The keyboard state of a window.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct WindowKeyboardState {
    modifiers_state: ModifiersState,
}

impl WindowKeyboardState {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn on_input<EventType>(&mut self, event: KeyboardInput, app: &mut dyn WindowApplication<EventType>)
            where EventType: 'static {
        if event.state != ElementState::Pressed {
            return;
        }

        if let Some(key) = event.virtual_keycode {
           app.on_key_press(WindowKeyPressEvent {
                key,
                modifiers: self.modifiers_state,
            });
        }

        if self.modifiers_state.ctrl() && event.virtual_keycode == Some(VirtualKeyCode::W) {
            std::process::exit(0);
        }
    }

    pub(crate) fn on_modifiers_event(&mut self, modifiers_state: ModifiersState) {
        self.modifiers_state = modifiers_state;
    }
}
