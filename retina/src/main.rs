// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod app;

use app::Application;

fn main() {
    if cfg!(debug_assertions) {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn"))
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
