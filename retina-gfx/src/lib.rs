// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

pub mod canvas;
pub mod context;
mod future;
pub(crate) mod math;
mod material;
mod painter;
pub mod vertex;
pub mod window;

pub(crate) type GfxResult<T> = Result<T, Box<dyn std::error::Error>>;

pub use retina_common::Color;
pub use self::{
    context::Context,
    future::SubmissionFuture,
    painter::Painter,
    window::{
        event_proxy::WindowEventProxy,
        interface::{
            WindowApplication,
            WindowKeyPressEvent,
        },
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

pub use winit::event::VirtualKeyCode;

pub use euclid;
