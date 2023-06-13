// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use euclid::Size2D;

use super::{render_pass::WindowRenderPass, Window};

pub trait WindowApplication<EventType>
        where EventType: 'static {
    fn on_event(&mut self, event: EventType, window: &mut Window<EventType>) {
        _ = event;
        _ = window;
    }
    fn on_paint(&mut self, render_pass: &mut WindowRenderPass) {
        _ = render_pass;
    }

    fn on_resize(&mut self, size: Size2D<u32, u32>) {
        _ = size;
    }
}

// pub struct WindowInterface<'window> {
//     window: &'window
// }
