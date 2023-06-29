// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod app;
mod event;

use app::Application;
pub(crate) use event::RetinaEvent;

fn main() {
    if cfg!(debug_assertions) {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn,retina,retina_common,retina_compositor,retina_dom,retina_gfx,retina_gfx_font,retina_layout,retina_page,retina_platform_object,retina_scrittura,retina_style,retina_style_computation,retina_style_parser,retina_user_agent"))
            .target(env_logger::Target::Stdout)
            .init();
    } else {
        env_logger::init();
    }

    let mut window = retina_gfx::window::Window::<RetinaEvent>::new()
        .expect("failed to create window");

    let app = Box::new(Application::new(&mut window));

    window.run(app)
}
