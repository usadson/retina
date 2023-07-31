// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::sync::OnceLock;

use tracing_chrome::ChromeLayerBuilder;
use tracing_subscriber::prelude::*;

pub(crate) struct CompositorTracingGuard {
    _chrome_guard: tracing_chrome::FlushGuard,
    _tracing_subscriber_guard: tracing::subscriber::DefaultGuard,
}

static ENABLE_TRACING: OnceLock<bool> = OnceLock::new();
impl CompositorTracingGuard {

    pub fn new() -> Option<Self> {
        if !Self::is_enabled() {
            return None;
        }

        let (chrome_layer, _chrome_guard) = ChromeLayerBuilder::new().build();

        let _tracing_subscriber_guard = tracing_subscriber::registry()
            .with(chrome_layer)
            .set_default();

        Some(Self {
            _chrome_guard,
            _tracing_subscriber_guard,
        })
    }

    #[inline]
    pub fn is_enabled() -> bool {
        *ENABLE_TRACING.get_or_init(|| {
            std::env::var("RETINA_TRACE")
                .is_ok_and(|val| val.trim().eq_ignore_ascii_case("1"))
        })
    }
}
