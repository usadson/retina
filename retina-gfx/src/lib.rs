// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

pub mod canvas;
pub mod context;
pub mod glyph_brush;
pub mod paint;
pub mod vertex;
pub mod window;

pub(crate) use glyph_brush::GlyphBrush;

pub(crate) type GfxResult<T> = Result<T, Box<dyn std::error::Error>>;

pub use retina_common::Color;
pub use self::{
    context::Context,
    window::{
        event_proxy::WindowEventProxy,
        render_pass::WindowRenderPass,
        interface::{WindowApplication, WindowKeyPressEvent},
    },
};

pub use winit::event::VirtualKeyCode;

pub use euclid;
