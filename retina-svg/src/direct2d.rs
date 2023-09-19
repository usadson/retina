// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod factory;

use windows::{Win32::Graphics::Direct2D::{
    Common::{
        D2D_RECT_F,
        D2D1_COLOR_F,
    },
    ID2D1Brush,
    ID2D1HwndRenderTarget,
}, Foundation::Numerics::Matrix3x2};

use windows::core::Interface;

use crate::{Material, Painter};
use self::factory::DirectFactory;

pub struct DirectContext {
    #[allow(dead_code)]
    factory: DirectFactory,
    render_target: ID2D1HwndRenderTarget,
}

impl DirectContext {
    pub fn new(window: &winit::window::Window) -> Self {
        let factory = DirectFactory::new();
        let render_target = factory.create_render_target(window);
        println!("Window size: {:?}", window.inner_size());

        Self {
            factory,
            render_target,
        }
    }

    pub fn begin(&self) {
        println!("Beginning draw");
        unsafe {
            self.render_target.BeginDraw();
            self.render_target.SetTransform(&Matrix3x2::identity());
            self.render_target.Clear(Some(&D2D1_COLOR_F {
                r: 0.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            }));
        }
    }

    pub fn end(&self) {
        println!("Ending draw");
        unsafe {
            // TODO?
            self.render_target.EndDraw(None, None).unwrap()
        }
    }

    pub fn create_material(&self, material: Material) -> ID2D1Brush {
        match material {
            Material::Color(color) => unsafe {
                println!("Creating brush for color: {color:?}");
                let brush = self.render_target.CreateSolidColorBrush(
                    &D2D1_COLOR_F {
                        r: color.red() as _,
                        g: color.green() as _,
                        b: color.blue() as _,
                        a: color.alpha() as _,
                    },
                    None
                ).unwrap();
                brush.cast().unwrap()
            }
        }
    }
}

impl Painter for DirectContext {
    fn draw_rect(&mut self, rect: euclid::default::Box2D<f32>, material: Material) {
        let rect = rect.to_rect();
        let rect = D2D_RECT_F {
            left: rect.min_x(),
            top: rect.max_y(),
            right: rect.max_x(),
            bottom: rect.min_y(),
        };

        println!("Rendering rect: {rect:#?}");

        let material = self.create_material(material);

        unsafe {
            self.render_target.FillRectangle(&rect, &material);
        }
    }
}
