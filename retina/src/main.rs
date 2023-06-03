// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use url::Url;

fn main() {
    env_logger::init();

    let url = Url::parse("about:not-found")
        .expect("failed to parse URL");

    let mut page_handle = retina_page::spawn(url);

    while let Ok(message) = page_handle.receive_message() {
        println!("[main] Received message from page: {message:#?}");
    }

    let window = retina_gfx::window::Window::new()
        .expect("failed to create window");

    window.run()
}
