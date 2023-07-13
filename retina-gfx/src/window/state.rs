// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use euclid::default::Point2D;
use winit::{
    dpi::PhysicalPosition,
    event::{
        DeviceEvent,
        ModifiersState,
    },
};

use crate::WindowApplication;

use super::{keyboard::WindowKeyboardState, interface::MouseMoveEvent};

/// Handles the state of a [`Window`].
pub(crate) struct WindowState {
    pub(crate) keyboard_state: WindowKeyboardState,
    pub(crate) cursor_position: Option<Point2D<f64>>,
}

//
// Window Events
//
impl WindowState {
    pub fn new() -> Self {
        Self {
            keyboard_state: WindowKeyboardState::new(),
            cursor_position: None,
        }
    }

    pub(crate) fn on_cursor_entered<EventType>(&mut self, app: &mut dyn WindowApplication<EventType>) {
        _ = app;
    }

    pub(crate) fn on_cursor_left<EventType>(&mut self, app: &mut dyn WindowApplication<EventType>) {
        _ = app;
        self.cursor_position = None;
    }

    pub(crate) fn on_cursor_moved<EventType>(
        &mut self,
        position: PhysicalPosition<f64>,
        app: &mut dyn WindowApplication<EventType>,
    )
            where EventType: 'static{
        let to = Point2D::new(position.x, position.y);
        let from = self.cursor_position.replace(to);
        let Some(from) = from else {
            return;
        };

        app.on_mouse_move(MouseMoveEvent {
            from,
            to,
            delta_x: to.x - from.y,
            delta_y: to.y - from.y,
        })

    }

    pub(crate) fn on_device_event<EventType>(&mut self, event: DeviceEvent, app: &mut dyn WindowApplication<EventType>)
            where EventType: 'static {
        match event {
            DeviceEvent::Key(event) => self.keyboard_state.on_input(event, app),
            DeviceEvent::MouseWheel { delta } => {
                // Only pass this event if the cursor is inside the window
                if self.cursor_position.is_some() {
                    app.on_mouse_wheel(delta);
                }
            }
            _ => (),
        }
    }

    pub(crate) fn on_modifiers_event(&mut self, event: ModifiersState) {
        self.keyboard_state.on_modifiers_event(event);
    }
}
