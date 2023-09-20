// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

#[cfg(windows)]
use retina_svg::direct2d::DirectContext;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

fn main() {
    env_logger::builder().parse_env(
        env_logger::Env::default()
            .default_filter_or("debug,retina-svg")
    ).init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Retina SVG Test")
        .build(&event_loop)
        .unwrap();

    #[cfg(windows)]
    let mut context = DirectContext::new(&window);

    let data = std::fs::read_to_string("test/html/svg/material-icons/index.html")
        .unwrap();
    let document = retina_dom::Parser::parse(&data);

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();
        control_flow.set_wait();

        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                control_flow.set_exit();
            },

            #[cfg(windows)]
            Event::RedrawRequested(_) => {
                context.begin();

                document.for_each_child_node_recursive_handle(&mut |node| {
                    if node.tag_name() != Some("svg") {
                        return;
                    }
                    retina_svg::render(node, &mut context);
                });

                context.end();
            },

            _ => ()
        }
    });
}
