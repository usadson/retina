// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod factory;

use euclid::default::{Box2D, Rect, Size2D};
use windows::{
    Foundation::Numerics::Matrix3x2,
    Win32::Graphics::Direct2D::{
        Common::{
            D2D_RECT_F,
            D2D_POINT_2F,
            D2D1_COLOR_F,
            D2D1_BEZIER_SEGMENT,
            D2D1_FIGURE_BEGIN,
            D2D1_FIGURE_BEGIN_FILLED,
            D2D1_FIGURE_BEGIN_HOLLOW,
            D2D1_FIGURE_END_OPEN,
            D2D_SIZE_U,
            D2D_SIZE_F,
        },
        D2D1_ARC_SEGMENT,
        D2D1_ARC_SIZE_LARGE,
        D2D1_ARC_SIZE_SMALL,
        D2D1_QUADRATIC_BEZIER_SEGMENT,
        D2D1_SWEEP_DIRECTION_CLOCKWISE,
        D2D1_SWEEP_DIRECTION_COUNTER_CLOCKWISE,

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
        SvgPathCoordinatePairDoubleSequence,
        SvgPathCoordinatePairTripletSequence,
        SvgPathCoordinateSequence,
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

    pub fn resize(&self, width: u32, height: u32) {
        unsafe {
            println!("Resizing to {width}x{height}");
            self.render_target.Resize(&D2D_SIZE_U {
               width, height,
            }).unwrap();
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
            previous_quadratic_control_point: None,
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

    fn push_view_box(&self, view_box: Rect<f32>) {
        unsafe {
            let mut transform = Matrix3x2::default();
            self.render_target.GetTransform(&mut transform);

            transform = transform * Matrix3x2::translation(view_box.origin.x, -view_box.origin.y);
            self.render_target.SetTransform(&transform);
        }
    }

    fn set_size(&self, size: Size2D<f32>) {
        unsafe {
            let mut transform = Matrix3x2::default();
            self.render_target.GetTransform(&mut transform);

            transform = transform * Matrix3x2::scale(size.width, size.height);
            self.render_target.SetTransform(&transform);
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
    previous_quadratic_control_point: Option<D2D_POINT_2F>,
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
            log::info!("  Not closing because it isn't opened: {:?}", self.state);
            return;
        }

        unsafe {
            // TODO should we use this or D2D1_FIGURE_END_CLOSED instead?
            self.sink.EndFigure(D2D1_FIGURE_END_OPEN)
        }

        self.previous_quadratic_control_point = None;
        self.state = DirectGeometrySinkState::Closed;
    }

    fn line_to(&mut self, ty: SvgPathType, coords: SvgPathCoordinatePair) {
        debug_assert_eq!(self.state, DirectGeometrySinkState::Opened);
        let point = self.point(ty, coords);
        unsafe {
            log::info!("Line {ty:?} to {coords:?}");
            self.sink.AddLine(point);
        }
        self.current = point;
        self.previous_quadratic_control_point = None;
    }

    fn horizontal_lines_to(&mut self, ty: SvgPathType, lines: SvgPathCoordinateSequence) {
        for x in lines.0 {
            let point = self.point(ty, SvgPathCoordinatePair { x, y: 0.0 });
            self.line_to(SvgPathType::Absolute, SvgPathCoordinatePair { x: point.x as _, y: self.current.y as _ });
        }
    }

    fn vertical_lines_to(&mut self, ty: SvgPathType, lines: SvgPathCoordinateSequence) {
        for y in lines.0 {
            let point = self.point(ty, SvgPathCoordinatePair { x: 0.0, y });
            self.line_to(SvgPathType::Absolute, SvgPathCoordinatePair { x: self.current.x as _, y: point.y as _ });
        }
    }

    fn move_to(&mut self, ty: SvgPathType, coords: SvgPathCoordinatePair) {
        let state = self.state;
        log::info!("Move {ty:?} to {coords:?} while {state:?}");

        if state == DirectGeometrySinkState::Initial {
            log::info!("  Absolute initial");
            unsafe {
                self.sink.BeginFigure(point(coords), self.begin_type);
            }

            self.current = point(coords);
            self.state = DirectGeometrySinkState::Opened;
            return;
        }

        self.close_path();

        let coords = self.point(ty, coords);
        self.current = coords;
        self.previous_quadratic_control_point = None;
        self.state = DirectGeometrySinkState::Opened;

        unsafe {
            log::info!("  Beginning figure");
            self.sink.BeginFigure(coords, self.begin_type);
        }
    }

    fn curve_to(&mut self, ty: SvgPathType, sequence: SvgPathCoordinatePairTripletSequence) {
        debug_assert_eq!(self.state, DirectGeometrySinkState::Opened);

        let mut beziers = Vec::with_capacity(sequence.0.len());
        for triplet in sequence.0 {
            let bezier = D2D1_BEZIER_SEGMENT {
                // First Control
                point1: self.point(ty, triplet.a),
                // Second Control
                point2: self.point(ty, triplet.b),
                // Point to
                point3: self.point(ty, triplet.c),
            };

            beziers.push(bezier);
            self.current = bezier.point3
        }

        unsafe {
            self.sink.AddBeziers(&beziers);
        }
    }

    fn quadratic_beziers_curve_to(&mut self, ty: SvgPathType, sequence: SvgPathCoordinatePairDoubleSequence) {
        debug_assert_eq!(self.state, DirectGeometrySinkState::Opened);
        let mut beziers = Vec::with_capacity(sequence.0.len());
        for double in sequence.0 {
            let bezier = D2D1_QUADRATIC_BEZIER_SEGMENT {
                // Control
                point1: self.point(ty, double.a),
                // Point to
                point2: self.point(ty, double.b),
            };
            beziers.push(bezier);

            self.current = bezier.point2;
            self.previous_quadratic_control_point = Some(bezier.point1);
        }

        unsafe {
            self.sink.AddQuadraticBeziers(&beziers);
        }
    }

    fn smooth_quadratic_bezier_curve_to(&mut self, ty: SvgPathType, coords: SvgPathCoordinatePair) {
        debug_assert_eq!(self.state, DirectGeometrySinkState::Opened);
        let next_point = self.point(ty, coords);

        let control_point = match self.previous_quadratic_control_point {
            // The control point is assumed to be the reflection of the control
            // point on the previous command relative to the current point.
            Some(previous_control_point) => reflect_point(self.current, previous_control_point),

            // (If there is no previous command or if the previous command was
            // not a Q, q, T or t, assume the control point is coincident with
            // the current point.)
            None => self.current,
        };

        unsafe {
            self.sink.AddQuadraticBezier(&D2D1_QUADRATIC_BEZIER_SEGMENT {
                point1: control_point,
                point2: next_point,
            });
        }

        self.previous_quadratic_control_point = Some(control_point);
        self.current = next_point;
    }

    fn elliptic_arc(&mut self, ty: SvgPathType, argument: crate::path::SvgPathEllipticArcArgument) {
        let end_point = self.point(ty, argument.coords);
        let arc = &D2D1_ARC_SEGMENT {
            point: end_point,
            rotationAngle: argument.x_axis_rotation as _,

            size: D2D_SIZE_F {
                width: argument.rx as _,
                height: argument.ry as _,
            },

            arcSize: if argument.large_arc_flag {
                D2D1_ARC_SIZE_LARGE
            } else {
                D2D1_ARC_SIZE_SMALL
            },

            sweepDirection: if argument.sweep_flag {
                D2D1_SWEEP_DIRECTION_CLOCKWISE
            } else {
                D2D1_SWEEP_DIRECTION_COUNTER_CLOCKWISE
            },

        };
        unsafe {
            self.sink.AddArc(arc);
        }

        self.current = end_point;
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

#[inline]
fn reflect_point(point: D2D_POINT_2F, relative_to: D2D_POINT_2F) -> D2D_POINT_2F {
    D2D_POINT_2F {
        x: 2.0 * point.x - relative_to.x,
        y: 2.0 * point.y - relative_to.y,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DirectGeometrySinkState {
    Initial,
    Closed,
    Opened,
}

trait MatrixExtensions<T> {
    fn scale(width: T, height: T) -> Self;
}

impl MatrixExtensions<f32> for Matrix3x2 {
    fn scale(width: f32, height: f32) -> Self {
        Self {
            M11: width, M12: 0.0,
            M21: 0.0,   M22: height,
            M31: 0.0,   M32: 0.0,
        }
    }
}
