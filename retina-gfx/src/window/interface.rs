// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use euclid::{Size2D, default::Point2D};
use winit::event::{
    ModifiersState,
    MouseScrollDelta,
    VirtualKeyCode,
};

use crate::{Painter, Window};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MouseMoveEvent {
    pub from: Point2D<f64>,
    pub to: Point2D<f64>,

    pub delta_x: f64,
    pub delta_y: f64,
}

pub trait WindowApplication<EventType>
        where EventType: 'static {
    fn on_event(&mut self, event: EventType, window: &mut Window<EventType>) {
        _ = event;
        _ = window;
    }

    fn on_mouse_wheel(&mut self, delta: MouseScrollDelta) {
        _ = delta;
    }

    fn on_mouse_move(&mut self, event: MouseMoveEvent) {
        _ = event;
    }

    fn on_key_press(&mut self, event: WindowKeyPressEvent) {
        _ = event;
    }

    fn on_paint(&mut self, render_pass: &mut Painter) {
        _ = render_pass;
    }

    fn on_resize(&mut self, size: Size2D<u32, u32>) {
        _ = size;
    }
}

pub struct WindowKeyPressEvent {
    pub(crate) modifiers: ModifiersState,
    pub(crate) key: VirtualKeyCode,
}

impl WindowKeyPressEvent {
    pub fn key(&self) -> VirtualKeyCode {
        self.key
    }

    pub fn with_alt(&self) -> bool {
        self.modifiers.alt()
    }

    pub fn with_control(&self) -> bool {
        self.modifiers.ctrl()
    }

    pub fn with_meta(&self) -> bool {
        self.modifiers.logo()
    }

    pub fn with_shift(&self) -> bool {
        self.modifiers.shift()
    }
}

// pub struct WindowInterface<'window> {
//     window: &'window
// }
