// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

fn main() {
    env_logger::init();

    let window = retina_gfx::window::Window::new()
        .expect("failed to create window");

    window.run()
}
