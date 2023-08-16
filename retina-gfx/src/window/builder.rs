// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::time::Instant;

use euclid::default::Size2D;
use raw_window_handle::{HasRawWindowHandle, HasRawDisplayHandle};
use retina_common::Color;
use winit::{dpi::PhysicalSize, event_loop::EventLoop};

use crate::{Window, WindowEventProxy, GfxResult, WindowSurface};

use super::{painter::WindowPainter, state::WindowState};

pub trait WindowHandle: HasRawWindowHandle + HasRawWindowHandle + Sized {}
impl<T> WindowHandle for T where T: HasRawWindowHandle + HasRawWindowHandle + Sized {}

pub struct WindowBuilder<EventType: 'static> {
    event_loop: EventLoop<EventType>,
    initial_window_size: Size2D<u32>
}

impl<EventType> WindowBuilder<EventType>
        where EventType: 'static {
    pub fn new() -> Self {
        Self {
            event_loop: winit::event_loop::EventLoopBuilder::with_user_event().build(),
            initial_window_size: Size2D::new(800, 600),
        }
    }

    pub fn with_title<S: Into<String>>(self, title: S) -> WindowBuilder2<EventType> {
        let window = winit::window::WindowBuilder::new()
            .with_title(title)
            .with_inner_size(PhysicalSize::new(self.initial_window_size.width, self.initial_window_size.height))
            .build(&self.event_loop)
            .unwrap();
        retina_common::set_scale_factor(window.scale_factor());
        WindowBuilder2 {
            event_loop: self.event_loop,
            window,
            window_size: self.initial_window_size,
        }
    }
}

pub struct WindowBuilder2<EventType: 'static> {
    window_size: Size2D<u32>,
    event_loop: EventLoop<EventType>,
    window: winit::window::Window,
}

impl<EventType> WindowBuilder2<EventType>
        where EventType: 'static {
    pub fn build(self) -> GfxResult<Window<EventType>> {
        let painter = WindowPainter::new(
            WindowSurface {
                display: self.window.raw_display_handle(),
                window: self.window.raw_window_handle(),
            },
            self.window.inner_size().to_logical(1.0),
        )?;

        self.build_impl(painter)
    }

    pub fn build_with(
        self,
        f: impl FnOnce(&winit::window::Window) -> Option<WindowPainter>
    ) -> GfxResult<Window<EventType>> {
        if let Some(painter) = f(&self.window) {
            return Self::build_impl(self, painter);
        }

        self.build()
    }

    fn build_impl(self, painter: WindowPainter) -> GfxResult<Window<EventType>> {
        let event_proxy = WindowEventProxy { proxy: self.event_loop.create_proxy() };
        Ok(Window {
            event_loop: Some(self.event_loop),
            event_proxy,
            window: self.window,

            painter,

            state: WindowState::new(),
            window_size: self.window_size.cast_unit(),
            start_time: Instant::now(),

            background_color: Color::WHITE,
        })
    }
}
