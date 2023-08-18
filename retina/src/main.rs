// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod app;
mod event;

use app::Application;
use retina_gfx::{Window, WindowPainter, WindowSurface};

fn main() {
    if cfg!(debug_assertions) {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn,retina,retina_common,retina_compositor=info,retina_dom,retina_gfx,retina_gfx_font,retina_layout,retina_page,retina_platform_object,retina_scrittura,retina_style,retina_style_computation,retina_style_parser,retina_user_agent"))
            .target(env_logger::Target::Stdout)
            .init();
    } else {
        env_logger::init();
    }

    let mut gui_manager = None;

    let mut window = Window::builder()
        .with_title("Retina")
        .build_with(|window| -> Option<WindowPainter> {
            match retina_gfx_gui::attach(window) {
                Ok(manager) => {
                    let painter = WindowPainter::new(
                        WindowSurface {
                            display: manager.raw_display_handle(),
                            window: manager.raw_window_handle(),
                        },
                        window.inner_size().to_logical(1.0),
                    ).expect("failed to create WindowPainter");

                    gui_manager = Some(manager);
                    Some(painter)
                }
                Err(e) => {
                    log::warn!("Cannot attach GUI: {e}");
                    None
                }
            }
        }).expect("failed to create window");

    let app = Box::new(Application::new(&mut window, gui_manager));

    window.run(app).unwrap()
}
