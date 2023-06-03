// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

pub(crate) mod keyboard;
pub(crate) mod interface;
pub(crate) mod painter;
pub(crate) mod render_pass;
pub(crate) mod state;
pub(crate) mod swap_chain;

use self::{painter::WindowPainter, state::WindowState};

use crate::{GfxResult, WindowApplication, Context};

pub struct Window {
    event_loop: winit::event_loop::EventLoop<()>,
    window: winit::window::Window,

    painter: WindowPainter,
    state: WindowState,
}

//
// Public Window APIs
//
impl Window {
    /// Create a new [`Window`] instance.
    pub fn new() -> GfxResult<Self> {
        // Open window and create a surface
        let event_loop = winit::event_loop::EventLoop::new();

        let window = winit::window::WindowBuilder::new()
            .with_resizable(false)
            .build(&event_loop)
            .unwrap();

        let painter = WindowPainter::new(&window)?;

        Ok(Self {
            event_loop,
            window,

            painter,

            state: WindowState::new(),
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
                    self.painter.on_resize(new_size);
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
                    self.painter.paint(app.as_mut());
                }

                _ => {
                    *control_flow = winit::event_loop::ControlFlow::Wait;
                }
            }
        })
    }

    /// Get the wgpu instance, which is useful for sending it to the page.
    pub fn context(&self) -> Context {
        self.painter.context.clone()
    }

}
