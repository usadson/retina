// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

pub(crate) mod builder;
pub(crate) mod event_proxy;
pub(crate) mod keyboard;
pub(crate) mod interface;
pub(crate) mod painter;
pub(crate) mod state;
pub(crate) mod swap_chain;

use std::time::{Instant, Duration};

use euclid::Size2D;
use retina_common::Color;
use winit::window::Icon;

use self::{
    event_proxy::WindowEventProxy,
    painter::WindowPainter,
    state::WindowState,
};

use crate::{GfxResult, WindowApplication, Context, CursorIcon};

const MINIMUM_DURATION_BEFORE_RESIZES_ARE_ACCEPTED: Duration = Duration::from_millis(250);

pub struct Window<EventType = ()>
        where EventType: 'static {
    event_loop: Option<winit::event_loop::EventLoop<EventType>>,
    event_proxy: WindowEventProxy<EventType>,
    window: winit::window::Window,

    painter: WindowPainter,
    state: WindowState,

    window_size: Size2D<u32, u32>,

    start_time: Instant,
    background_color: Color,
}

//
// Public Window APIs
//
impl<EventType> Window<EventType>
        where EventType: 'static {
    /// Get the wgpu instance, which is useful for sending it to the page.
    pub fn context(&self) -> Context {
        self.painter.context.clone()
    }

    pub fn create_proxy(&self) -> WindowEventProxy<EventType> {
        self.event_proxy.clone()
    }

    pub fn builder() -> builder::WindowBuilder<EventType> {
        builder::WindowBuilder::new()
    }

    pub fn request_repaint(&self) {
        self.window.request_redraw();
    }

    pub fn run(mut self, mut app: Box<dyn WindowApplication<EventType>>) -> GfxResult<()> {
        // Render loop
        self.window.request_redraw();

        self.event_loop.take().unwrap().run(move |event, _, control_flow| {
            match event {
                winit::event::Event::UserEvent(event) => app.on_event(event, &mut self),

                winit::event::Event::WindowEvent {
                    event: winit::event::WindowEvent::CloseRequested,
                    ..
                } => *control_flow = winit::event_loop::ControlFlow::Exit,

                winit::event::Event::WindowEvent {
                    event: winit::event::WindowEvent::Resized(new_size),
                    ..
                } => {
                    if self.start_time.elapsed() < MINIMUM_DURATION_BEFORE_RESIZES_ARE_ACCEPTED {
                        return;
                    }

                    let logical_size = new_size.to_logical(1.0);
                    let euclid_size = Size2D::new(logical_size.width, logical_size.height);

                    if !euclid_size.is_empty() && self.window_size != euclid_size {

                        self.window_size = euclid_size;
                        self.painter.on_resize(logical_size);
                        app.on_resize(euclid_size);
                    }
                }

                winit::event::Event::DeviceEvent { event, .. } => {
                    self.state.on_device_event(event, app.as_mut());
                }

                winit::event::Event::WindowEvent {
                    event: winit::event::WindowEvent::CursorEntered { .. },
                    ..
                } => {
                    self.state.on_cursor_entered(app.as_mut());
                }

                winit::event::Event::WindowEvent {
                    event: winit::event::WindowEvent::CursorLeft { .. },
                    ..
                } => {
                    self.state.on_cursor_left(app.as_mut());
                }

                winit::event::Event::WindowEvent {
                    event: winit::event::WindowEvent::CursorMoved { position, .. },
                    ..
                } => {
                    self.state.on_cursor_moved(position, app.as_mut());
                }

                winit::event::Event::WindowEvent {
                    event: winit::event::WindowEvent::MouseInput { state, button, .. },
                    ..
                } => {
                    self.state.on_mouse_input(state, button, app.as_mut());
                }

                winit::event::Event::WindowEvent {
                    event: winit::event::WindowEvent::ModifiersChanged(event),
                    ..
                } => {
                    self.state.on_modifiers_event(event);
                }

                winit::event::Event::RedrawRequested { .. } => {
                    self.painter.paint(app.as_mut(), self.background_color);
                }

                _ => {
                    *control_flow = winit::event_loop::ControlFlow::Wait;
                }
            }
        })
    }

    pub fn set_background_color(&mut self, value: Color) {
        self.background_color = value;
    }

    pub fn set_cursor_icon(&self, cursor: CursorIcon) {
        match cursor {
            CursorIcon::Winit(cursor) => self.window.set_cursor_icon(cursor),
            // ... options for bitmap cursors (todo)
        }
    }

    pub fn set_icon(&self, rgba: Vec<u8>, width: u32, height: u32) {
        match Icon::from_rgba(rgba, width, height) {
            Ok(icon) => {
                self.window.set_window_icon(Some(icon));
            },

            Err(e) => {
                log::error!("Bad icon: {e}");
            }
        }
    }

    pub fn set_title(&self, title: &str) {
        self.window.set_title(title);
    }

    pub fn size(&self) -> Size2D<u32, u32> {
        let size = self.window.inner_size();
        Size2D::new(size.width, size.height)
    }
}
