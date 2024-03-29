// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

use windows::Win32::{
    Foundation::HWND,
    Graphics::Direct2D::{
        Common::D2D_SIZE_U,

        ID2D1Factory,
        ID2D1PathGeometry,
        ID2D1HwndRenderTarget,
        ID2D1StrokeStyle,

        D2D1CreateFactory,

        D2D1_RENDER_TARGET_PROPERTIES,
        D2D1_HWND_RENDER_TARGET_PROPERTIES,
        D2D1_FACTORY_OPTIONS,
        D2D1_STROKE_STYLE_PROPERTIES,

        D2D1_DEBUG_LEVEL_INFORMATION,
        D2D1_FACTORY_TYPE_SINGLE_THREADED,
        D2D1_CAP_STYLE_FLAT,
        D2D1_CAP_STYLE_ROUND,
        D2D1_CAP_STYLE_SQUARE,
    },
};

use crate::{
    CapStyle,
    StrokeStyle,
    StrokeStyleProperties,
};

pub struct DirectFactory {
    factory: ID2D1Factory,
}

impl DirectFactory {
    pub fn new() -> Self {
        let opts = D2D1_FACTORY_OPTIONS {
            #[cfg(debug_assertions)]
            debugLevel: D2D1_DEBUG_LEVEL_INFORMATION,

            ..Default::default()
        };
        Self {
            factory: unsafe {
                D2D1CreateFactory(D2D1_FACTORY_TYPE_SINGLE_THREADED, Some(&opts)).unwrap()
            },
        }
    }

    pub fn create_render_target(&self, window: &winit::window::Window) -> ID2D1HwndRenderTarget {
        let size = window.inner_size();
        let RawWindowHandle::Win32(window) = window.raw_window_handle() else {
            panic!("Invalid window handle: {:?}", window.raw_window_handle());
        };

        let render_target_properties = D2D1_RENDER_TARGET_PROPERTIES::default();
        let hwnd_render_target_properties = D2D1_HWND_RENDER_TARGET_PROPERTIES {
            hwnd: HWND(window.hwnd as _),
            pixelSize: D2D_SIZE_U {
                width: size.width,
                height: size.height,
            },
            ..Default::default()
        };

        unsafe {
            self.factory.CreateHwndRenderTarget(
                &render_target_properties,
                &hwnd_render_target_properties,
            ).unwrap()
        }
    }

    pub fn create_stroke_style(&self, properties: StrokeStyleProperties) -> Box<dyn StrokeStyle> {
        let cap_style = |style| match style {
            CapStyle::Butt => D2D1_CAP_STYLE_FLAT,
            CapStyle::Round => D2D1_CAP_STYLE_ROUND,
            CapStyle::Square => D2D1_CAP_STYLE_SQUARE,
        };

        unsafe {
            let style = self.factory.CreateStrokeStyle(&D2D1_STROKE_STYLE_PROPERTIES {
                dashCap: cap_style(properties.cap_style_dash),
                endCap: cap_style(properties.cap_style_end),
                startCap: cap_style(properties.cap_style_start),
                ..Default::default()
            }, None).unwrap();
            Box::new(DirectStrokeStyle { style })
        }
    }

    pub fn create_geometry(&self) -> ID2D1PathGeometry {
        unsafe {
            self.factory.CreatePathGeometry().unwrap()
        }
    }
}

pub struct DirectStrokeStyle {
    pub style: ID2D1StrokeStyle,
}

impl StrokeStyle for DirectStrokeStyle {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
