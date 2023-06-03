// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod app;

use app::Application;

fn main() {
    env_logger::init();

    let window = retina_gfx::window::Window::new()
        .expect("failed to create window");

    let app = Box::new(Application::new(&window));

    window.run(app)
}
