// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

pub struct WindowEventProxy<EventType>
        where EventType: 'static {
    pub(crate) proxy: winit::event_loop::EventLoopProxy<EventType>,
}

impl<T: 'static> Clone for WindowEventProxy<T> {
    fn clone(&self) -> Self {
        Self {
            proxy: self.proxy.clone(),
        }
    }
}

impl<EventType> WindowEventProxy<EventType>
        where EventType: 'static {
    pub fn send(&self, event: EventType) -> Result<(), EventType> {
        self.proxy.send_event(event).map_err(|err| err.0)
    }
}
