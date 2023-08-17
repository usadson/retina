// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

pub mod canvas;
pub mod context;
mod future;
pub(crate) mod math;
pub mod material;
mod painter;
mod texture;
pub mod vertex;
pub mod window;

pub(crate) type GfxResult<T> = Result<T, Box<dyn std::error::Error>>;

use raw_window_handle::{RawWindowHandle, RawDisplayHandle, HasRawWindowHandle, HasRawDisplayHandle};
pub use retina_common::Color;
pub use winit::{
    event::{MouseButton, ElementState},
    window::CursorIcon as WinitCursorIcon,
};
pub use self::{
    context::Context,
    future::SubmissionFuture,
    painter::Painter,
    texture::Texture,
    window::{
        event_proxy::WindowEventProxy,
        interface::{
            MouseMoveEvent,
            WindowApplication,
            WindowKeyPressEvent,
        },
        painter::WindowPainter,
        Window,
    },
};

pub(crate) use self::{
    painter::Artwork,
    material::{
        ColorMaterialRenderer,
        TextureMaterialRenderer,
    },
};

pub use winit::event::{
    MouseScrollDelta,
    VirtualKeyCode,
};

pub use euclid;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CursorIcon {
    Winit(WinitCursorIcon),
    // ... support for bitmap cursors (todo)
}

pub struct WindowSurface {
    pub window: RawWindowHandle,
    pub display: RawDisplayHandle,
}

unsafe impl HasRawDisplayHandle for WindowSurface {
    fn raw_display_handle(&self) -> RawDisplayHandle {
        self.display.clone()
    }
}

unsafe impl HasRawWindowHandle for WindowSurface {
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.window.clone()
    }
}
