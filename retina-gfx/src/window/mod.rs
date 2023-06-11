// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

pub(crate) mod keyboard;
pub(crate) mod interface;
pub(crate) mod painter;
pub(crate) mod render_pass;
pub(crate) mod state;
pub(crate) mod swap_chain;

use euclid::Size2D;
use log::info;

use self::{painter::WindowPainter, state::WindowState};

use crate::{GfxResult, WindowApplication, Context};

pub struct Window {
    event_loop: winit::event_loop::EventLoop<()>,
    window: winit::window::Window,

    painter: WindowPainter,
    state: WindowState,

    window_size: Size2D<u32, u32>,
}

//
// Public Window APIs
//
impl Window {
    /// Get the wgpu instance, which is useful for sending it to the page.
    pub fn context(&self) -> Context {
        self.painter.context.clone()
    }

    /// Create a new [`Window`] instance.
    pub fn new() -> GfxResult<Self> {
        // Open window and create a surface
        let event_loop = winit::event_loop::EventLoop::new();

        let window = winit::window::WindowBuilder::new()
            .with_resizable(false)
            .with_title("Retina")
            .build(&event_loop)
            .unwrap();

        let window_size = window.inner_size();
        let window_size = Size2D::new(window_size.width, window_size.height);

        let painter = WindowPainter::new(&window)?;

        Ok(Self {
            event_loop,
            window,

            painter,

            state: WindowState::new(),
            window_size,
        })
    }

    pub fn run(mut self, mut app: Box<dyn WindowApplication>) {
        // Render loop
        self.window.request_redraw();

        self.event_loop.run(move |event, _, control_flow| {
            match event {
                winit::event::Event::WindowEvent {
                    event: winit::event::WindowEvent::CloseRequested,
                    ..
                } => *control_flow = winit::event_loop::ControlFlow::Exit,

                winit::event::Event::WindowEvent {
                    event: winit::event::WindowEvent::Resized(new_size),
                    ..
                } => {
                    let logical_size = new_size.to_logical(1.0);
                    let euclid_size = Size2D::new(logical_size.width, logical_size.height);

                    if !euclid_size.is_empty() && self.window_size != euclid_size {
                        self.window_size = euclid_size;
                        self.painter.on_resize(logical_size);
                        app.on_resize(euclid_size);
                    }
                }

                winit::event::Event::DeviceEvent { event, .. } => {
                    self.state.on_device_event(event);
                }

                winit::event::Event::WindowEvent {
                    event: winit::event::WindowEvent::ModifiersChanged(event),
                    ..
                } => {
                    self.state.on_modifiers_event(event);
                }

                winit::event::Event::RedrawRequested { .. } => {
                    info!("Redraw requested!");
                    self.painter.paint(app.as_mut());
                }

                _ => {
                    *control_flow = winit::event_loop::ControlFlow::Wait;
                }
            }
        })
    }

    pub fn set_title(&self, title: &str) {
        self.window.set_title(title);
    }

    pub fn size(&self) -> Size2D<u32, u32> {
        let size = self.window.inner_size();
        Size2D::new(size.width, size.height)
    }
}
