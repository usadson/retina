// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use euclid::Size2D;

use super::render_pass::WindowRenderPass;

pub trait WindowApplication {
    fn on_paint(&mut self, render_pass: &mut WindowRenderPass) {
        _ = render_pass;
    }

    fn on_resize(&mut self, size: Size2D<u32, u32>) {
        _ = size;
    }
}
