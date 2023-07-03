// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use euclid::Size2D;
use winit::event::{ModifiersState, VirtualKeyCode};

use crate::{Painter, Window};

pub trait WindowApplication<EventType>
        where EventType: 'static {
    fn on_event(&mut self, event: EventType, window: &mut Window<EventType>) {
        _ = event;
        _ = window;
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
