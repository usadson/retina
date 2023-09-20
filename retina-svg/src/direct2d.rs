// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod factory;

use euclid::default::Box2D;
use windows::{
    Foundation::Numerics::Matrix3x2,
    Win32::Graphics::Direct2D::{
        Common::{
            D2D1_COLOR_F,
            D2D_RECT_F,
            D2D_POINT_2F,
            D2D1_FIGURE_BEGIN,
            D2D1_FIGURE_BEGIN_FILLED,
            D2D1_FIGURE_BEGIN_HOLLOW, D2D1_FIGURE_END_CLOSED,
        },
        ID2D1Brush,
        ID2D1GeometrySink,
        ID2D1HwndRenderTarget,
        ID2D1PathGeometry,
    },
};

use windows::core::ComInterface;

use crate::{
    Geometry,
    GeometrySink,
    GeometrySinkFillType,
    Material,
    Painter,
    path::{
        SvgPathCoordinatePair,
        SvgPathType,
    },
};
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
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            }));
        }
    }

    pub fn end(&self) {
        println!("Ending draw");
        unsafe {
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

    fn rect(&self, rect: Box2D<f32>) -> D2D_RECT_F {
        let rect = rect.to_rect();
        D2D_RECT_F {
            left: rect.min_x(),
            top: rect.max_y(),
            right: rect.max_x(),
            bottom: rect.min_y(),
        }
    }
}

impl Painter for DirectContext {
    fn create_geometry(&self, fill_type: GeometrySinkFillType) -> Box<dyn GeometrySink> {
        let geometry = self.factory.create_geometry();
        let sink = unsafe { geometry.Open().unwrap() };

        Box::new(DirectGeometrySink {
            geometry: Some(DirectGeometry { geometry }),
            sink,
            begin_type: match fill_type {
                GeometrySinkFillType::Filled => D2D1_FIGURE_BEGIN_FILLED,
                GeometrySinkFillType::Hollow => D2D1_FIGURE_BEGIN_HOLLOW,
            },
            state: DirectGeometrySinkState::Initial,
            current: Default::default(),
        })
    }

    fn draw_geometry(&mut self, geometry: &dyn Geometry, material: Material) {
        let geo = geometry.as_any()
            .downcast_ref::<DirectGeometry>()
            .unwrap();

        let material = self.create_material(material);
        unsafe {
            self.render_target.FillGeometry(&geo.geometry, &material, None)
        }
    }

    fn draw_rect(&mut self, rect: euclid::default::Box2D<f32>, material: Material) {
        let material = self.create_material(material);

        unsafe {
            self.render_target.FillRectangle(&self.rect(rect), &material);
        }
    }

    fn stroke_geometry(&mut self, geometry: &dyn Geometry, material: Material, width: f32) {
        let geo = geometry.as_any()
            .downcast_ref::<DirectGeometry>()
            .unwrap();

        unsafe {
            self.render_target.DrawGeometry(
                &geo.geometry,
                &self.create_material(material),
                width,
                None,
            )
        }
    }

    fn stroke_rect(&mut self, rect: Box2D<f32>, material: Material, width: f32) {
        unsafe {
            self.render_target.DrawRectangle(
                &self.rect(rect),
                & self.create_material(material),
                width,
                None,
            );
        }
    }
}

struct DirectGeometry {
    geometry: ID2D1PathGeometry,
}

impl Geometry for DirectGeometry {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

struct DirectGeometrySink {
    geometry: Option<DirectGeometry>,
    sink: ID2D1GeometrySink,
    begin_type: D2D1_FIGURE_BEGIN,

    state: DirectGeometrySinkState,
    current: D2D_POINT_2F,
}

impl DirectGeometrySink {
    fn point(&self, ty: SvgPathType, coords: SvgPathCoordinatePair) -> D2D_POINT_2F {
        let coords = point(coords);

        match ty {
            SvgPathType::Absolute => coords,
            SvgPathType::Relative => D2D_POINT_2F {
                x: self.current.x + coords.x,
                y: self.current.y + coords.y,
            }
        }
    }
}

impl GeometrySink for DirectGeometrySink {
    fn close_path(&mut self) {
        log::info!("Closing path...");
        if self.state != DirectGeometrySinkState::Opened {
            log::info!("Not closing because it isn't opened: {:?}", self.state);
            return;
        }

        unsafe {
            // TODO should we use this or D2D1_FIGURE_END_OPEN instead?
            self.sink.EndFigure(D2D1_FIGURE_END_CLOSED)
        }

        self.state = DirectGeometrySinkState::Closed;
    }

    fn line_to(&mut self, ty: SvgPathType, coords: SvgPathCoordinatePair) {
        unsafe {
            log::info!("Line {ty:?} to {coords:?}");
            self.sink.AddLine(self.point(ty, coords));
        }
    }

    fn move_to(&mut self, ty: SvgPathType, coords: SvgPathCoordinatePair) {
        let state = std::mem::replace(&mut self.state, DirectGeometrySinkState::Opened);
        log::info!("Move {ty:?} to {coords:?} while {state:?}");

        if state == DirectGeometrySinkState::Initial {
            log::info!("Absolute initial");
            unsafe {
                self.sink.BeginFigure(point(coords), self.begin_type);
            }

            self.current = point(coords);
            return;
        }

        self.close_path();
        let coords = self.point(ty, coords);
        self.current = coords;

        unsafe {
            log::info!("Beginning figure");
            self.sink.BeginFigure(coords, self.begin_type);
        }
    }

    fn finish(&mut self) -> Box<dyn Geometry> {
        log::info!("Finishing...");
        self.close_path();

        unsafe {
            self.sink.Close()
        }.unwrap();

        Box::new(self.geometry.take().unwrap())
    }
}

#[inline]
const fn point(value: SvgPathCoordinatePair) -> D2D_POINT_2F {
    D2D_POINT_2F {
        x: value.x as _,
        y: value.y as _,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DirectGeometrySinkState {
    Initial,
    Closed,
    Opened,
}
