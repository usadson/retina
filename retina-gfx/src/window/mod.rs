// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod glyph_brush;
mod keyboard;
mod painter;
mod render_pass;
mod state;
mod swap_chain;

use self::{painter::WindowPainter, state::WindowState};

use std::error::Error;

pub(crate) type GfxResult<T> = Result<T, Box<dyn Error>>;

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

    pub fn run(mut self) {
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
                    self.painter.paint();
                }

                _ => {
                    *control_flow = winit::event_loop::ControlFlow::Wait;
                }
            }
        })
    }

}
