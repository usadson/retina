// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::http::download_resource;

use gleam::gl;
use glutin;
use std::env;
use std::path::PathBuf;
use webrender;
use winit;
use winit::platform::run_return::EventLoopExtRunReturn;
use webrender::{DebugFlags, ShaderPrecacheFlags};
use webrender::api::*;
use webrender::render_api::*;
use webrender::api::units::*;

pub mod http;

pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

struct Notifier {
    events_proxy: winit::event_loop::EventLoopProxy<()>,
}

impl Notifier {
    fn new(events_proxy: winit::event_loop::EventLoopProxy<()>) -> Notifier {
        Notifier { events_proxy }
    }
}

impl RenderNotifier for Notifier {
    fn clone(&self) -> Box<dyn RenderNotifier> {
        Box::new(Notifier {
            events_proxy: self.events_proxy.clone(),
        })
    }

    fn wake_up(&self, _composite_needed: bool) {
        #[cfg(not(target_os = "android"))]
        let _ = self.events_proxy.send_event(());
    }

    fn new_frame_ready(&self,
                       _: DocumentId,
                       _scrolled: bool,
                       composite_needed: bool) {
        self.wake_up(composite_needed);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let result = String::from_utf8(download_resource("http://theoldnet.com/").await?)?;

    println!("Resource: {}", result);

    let mut events_loop = winit::event_loop::EventLoop::new();
    let window_builder = winit::window::WindowBuilder::new()
        .with_title("Retina")
        .with_inner_size(winit::dpi::LogicalSize::new(1280.0, 720.0));
    let windowed_context = glutin::ContextBuilder::new()
        .with_gl(glutin::GlRequest::GlThenGles {
            opengl_version: (3, 2),
            opengles_version: (3, 0),
        })
        .build_windowed(window_builder, &events_loop)
        .unwrap();

    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    let gl = match windowed_context.get_api() {
        glutin::Api::OpenGl => unsafe {
            gl::GlFns::load_with(
                |symbol| windowed_context.get_proc_address(symbol) as *const _
            )
        },
        glutin::Api::OpenGlEs => unsafe {
            gl::GlesFns::load_with(
                |symbol| windowed_context.get_proc_address(symbol) as *const _
            )
        },
        glutin::Api::WebGl => unimplemented!(),
    };

    println!("OpenGL version {}", gl.get_string(gl::VERSION));

    let device_pixel_ratio = windowed_context.window().scale_factor() as f32;
    println!("Device pixel ratio: {}", device_pixel_ratio);

    println!("Loading shaders...");

    let mut debug_flags = DebugFlags::ECHO_DRIVER_MESSAGES | DebugFlags::TEXTURE_CACHE_DBG;
    let opts = webrender::WebRenderOptions {
        precache_flags: ShaderPrecacheFlags::ASYNC_COMPILE,
        clear_color: ColorF::new(0.3, 0.0, 0.0, 1.0),
        debug_flags,
        //allow_texture_swizzling: false,
        ..webrender::WebRenderOptions::default()
    };

    let device_size = {
        let size = windowed_context
            .window()
            .inner_size();
        DeviceIntSize::new(size.width as i32, size.height as i32)
    };
    let notifier = Box::new(Notifier::new(events_loop.create_proxy()));
    let (mut renderer, sender) = webrender::create_webrender_instance(
        gl.clone(),
        notifier,
        opts,
        None,
    ).unwrap();

    let mut api = sender.create_api();
    let document_id = api.add_document(device_size);

    events_loop.run_return(|global_event, _elwt, control_flow| {
        let mut txn = Transaction::new();
        let mut custom_event = true;

        let old_flags = debug_flags;
        let win_event = match global_event {
            winit::event::Event::WindowEvent { event, .. } => event,
            _ => return,
        };
        match win_event {
            winit::event::WindowEvent::CloseRequested => {
                *control_flow = winit::event_loop::ControlFlow::Exit;
                return;
            }

            _ => (),
        }
    });

    renderer.deinit();
    Ok(())
}
