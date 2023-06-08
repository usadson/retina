// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod app;

use app::Application;

fn main() {
    if cfg!(debug_assertions) {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn,retina,retina_common,retina_compositor,retina_dom,retina_gfx,retina_layout,retina_page,retina_style,retina_style_computation,retina_user_agent"))
            .target(env_logger::Target::Stdout)
            .init();
    } else {
        env_logger::init();
    }

    let window = retina_gfx::window::Window::new()
        .expect("failed to create window");

    let app = Box::new(Application::new(&window));

    window.run(app)
}
