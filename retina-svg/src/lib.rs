// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.
//
//! retina-svg, the SVG renderer.
//!
//! ## References
//! * [Scalable Vector Graphics (SVG) 1.1 Specification](https://www.w3.org/TR/SVG11/)
//! * [Scalable Vector Graphics (SVG) 1.1 Specification (Single Page)](https://www.w3.org/TR/SVG11/single-page.html)

#[cfg(windows)]
pub mod direct2d;
mod painter;
mod path;
// mod tesselator;

use std::sync::Weak;

use crate::path::SvgPathCommand;

pub use self::painter::{
    CapStyle,
    Geometry,
    GeometrySink,
    GeometrySinkFillType,
    Material,
    Painter,
    StrokeStyle,
    StrokeStyleProperties,
};

use euclid::{default::{Box2D, Rect, Size2D}, Point2D};
use log::{error, warn, info};

use lyon::geom::point;
use retina_common::Color;
use retina_dom::{
    Element,
    Node,
};
use retina_style::{CssColor, CssLength};

pub fn render(node: &Node, painter: &mut dyn Painter) {
    render_inner(node, painter);
}

fn render_inner(node: &Node, painter: &mut dyn Painter) {
    let mut renderer = SvgRenderer {
        root_node: node.clone(),
        painter,
    };

    renderer.render_node(&node);
}

pub struct SvgRenderer<'painter> {
    #[allow(dead_code)]
    root_node: Node,
    painter: &'painter mut dyn Painter,
}

impl<'painter> SvgRenderer<'painter> {
    fn render_node(&mut self, node: &Node) {
        let Some(element) = node.as_dom_element() else { return };
        println!("[SvgRenderer] Rendering node: {}", element.qualified_name().local);

        match element.qualified_name().local.as_ref() {
            "circle" => self.render_circle(element),
            "ellipse" => self.render_ellipse(element),
            "path" => self.render_path(element),
            "rect" => self.render_rect(element),
            "svg" => self.render_svg(element),
            _ => (),
        }

        for child in element.as_parent_node().children().iter() {
            self.render_node(child);
        }
    }

    fn render_circle(&mut self, element: &Element) {
        let center = element.properties_circle_center();

        // The r attribute defines the radius of the circle. A negative value
        // is invalid and must be ignored. A computed value of zero disables
        // rendering of the element.
        let radius = element.property_r();
        if radius < 0.0 {
            return;
        }
        let radius = Point2D::new(radius, radius);

        let fill = element.property_fill();
        if !fill.is_transparent() {
            self.painter.draw_ellipse(center, radius, fill);
        }

        let stroke = element.property_stroke();
        let stroke_width = element.property_stroke_width();
        if !stroke.is_transparent() && stroke_width > 0.0 {
            let stroke_style = element.stroke_style(self.painter);
            self.painter.stroke_ellipse(center, radius, stroke, stroke_width, stroke_style.as_deref());
        }
    }

    fn render_ellipse(&mut self, element: &Element) {
        let center = element.properties_circle_center();

        // The rx and ry properties define the x- and y-axis radii of the
        // ellipse. A negative value for either property is invalid and must be
        // ignored. A computed value of zero for either dimension, or a
        // computed value of auto for both dimensions, disables rendering of the
        // element.
        let radius = element.properties_radii();
        if radius.x <= 0.0 || radius.y <= 0.0 {
            return;
        }

        let fill = element.property_fill();
        if !fill.is_transparent() {
            self.painter.draw_ellipse(center, radius, fill);
        }

        let stroke = element.property_stroke();
        let stroke_width = element.property_stroke_width();
        if !stroke.is_transparent() && stroke_width > 0.0 {
            let stroke_style = element.stroke_style(self.painter);
            self.painter.stroke_ellipse(center, radius, stroke, stroke_width, stroke_style.as_deref());
        }
    }

    fn render_path(&mut self, element: &Element) {
        let Some(path_data) = element.attributes().find_by_str("d") else {
            error!("<path> has no \"d\" path data attribute!");
            return;
        };

        info!("Raw path data: \"{path_data}\"");
        let fill = element.property_fill();
        let stroke = element.property_stroke();
        let stroke_width = element.property_stroke_width();
        if fill.is_transparent() && stroke.is_transparent() && stroke_width <= 0.0 {
            info!("Skipping transparent path!");
            return;
        }

        let path = match path::parse_path(path_data) {
            Ok((_, path)) => path,
            Err(e) => {
                error!("Failed to parse path: {e}");
                return;
            }
        };

        info!("Parsed path data: {path:#?}");

        let mut sink = self.painter.create_geometry(GeometrySinkFillType::Filled);

        for command in path.commands {
            match command {
                SvgPathCommand::MoveTo(ty, coords_sequence) => {
                    for coords in coords_sequence.0 {
                        sink.move_to(ty, coords);
                    }
                }
                SvgPathCommand::LineTo(ty, coords_sequence) => {
                    for coords in coords_sequence.0 {
                        sink.line_to(ty, coords);
                    }
                }
                SvgPathCommand::HorizontalLineTo(ty, lines) => sink.horizontal_lines_to(ty, lines),
                SvgPathCommand::VerticalLineTo(ty, lines) => sink.vertical_lines_to(ty, lines),
                SvgPathCommand::CurveTo(ty, sequence) => sink.curve_to(ty, sequence),
                SvgPathCommand::QuadraticBezierCurveTo(ty, sequence) => {
                    sink.quadratic_beziers_curve_to(ty, sequence)
                }
                SvgPathCommand::SmoothQuadraticBezierCurveTo(ty, sequence) => {
                    for pair in sequence.0 {
                        sink.smooth_quadratic_bezier_curve_to(ty, pair)
                    }
                }
                SvgPathCommand::EllipticArc(ty, sequence) => {
                    for argument in sequence.0 {
                        sink.elliptic_arc(ty, argument);
                    }
                }
                SvgPathCommand::ClosePath => sink.close_path(),
            }
        }

        let geometry = sink.finish();
        if !fill.is_transparent() {
            self.painter.draw_geometry(geometry.as_ref(), fill);
        }

        if !stroke.is_transparent() && stroke_width > 0.0 {
            let stroke_style = element.stroke_style(self.painter);
            self.painter.stroke_geometry(geometry.as_ref(), stroke, stroke_width, stroke_style.as_deref());
        }
    }

    fn render_rect(&mut self, element: &Element) {
        let min = point(element.property_x(), element.property_y());
        let max = point(element.property_width(), element.property_height());

        let rect = Box2D::new(min, max);
        let radius = element.properties_radii();

        let fill = element.property_fill();
        if !fill.is_transparent() {
            self.painter.draw_rect(rect, fill, radius);
        }

        let stroke = element.property_stroke();
        let stroke_width = element.property_stroke_width();
        if !stroke.is_transparent() && stroke_width > 0.0 {
            let stroke_style = element.stroke_style(self.painter);
            self.painter.stroke_rect(rect, stroke, radius, stroke_width, stroke_style.as_deref());
        }
    }

    fn render_svg(&mut self, element: &Element) {
        let width = element.property_width();
        let height = element.property_height();

        if let Some(view_box) = element.property_view_box() {
            println!("Width: {width}, height: {height}");
            if width > 0.0 && height > 0.0 {
                self.painter.set_size(Size2D::new(
                    width / view_box.width(),
                    height / view_box.height(),
                ));
            }

            info!("SVG ViewBox: {view_box:#?}");
            self.painter.push_view_box(view_box);
        } else if width > 0.0 && height > 0.0 {
            self.painter.set_size(Size2D::new(width, height));
        }
    }
}

trait SvgElementTraits {
    fn length_property_ext(&self, name: &str, default: f32) -> f32;
    fn paint_property_ext(&self, name: &str, default: Material) -> Material;

    #[inline]
    fn length_property(&self, name: &str) -> f32 {
        self.length_property_ext(name, 0.0)
    }

    #[inline]
    fn paint_property(&self, name: &str) -> Material {
        self.paint_property_ext(name, Material::Color(Color::BLACK))
    }

    /// <https://www.w3.org/TR/SVG11/single-page.html#painting-FillProperty>
    fn property_fill(&self) -> Material { self.paint_property("fill") }
    fn property_stroke(&self) -> Material { self.paint_property_ext("stroke", Material::Color(Color::TRANSPARENT)) }

    /// Radius of a circle
    fn property_r(&self) -> f32 { self.length_property("r") }
    fn property_x(&self) -> f32 { self.length_property("x") }
    fn property_y(&self) -> f32 { self.length_property("y") }
    fn property_width(&self) -> f32 { self.length_property("width") }
    fn property_height(&self) -> f32 { self.length_property("height") }
    fn property_stroke_width(&self) -> f32 { self.length_property_ext("stroke-width", 1.0) }

    fn property_view_box(&self) -> Option<Rect<f32>>;
    fn stroke_style(&self, painter: &dyn Painter) -> Option<Box<dyn StrokeStyle>>;

    fn properties_radii(&self) -> euclid::default::Point2D<f32> {
        Point2D::new(
            self.length_property("rx"),
            self.length_property("ry"),
        )
    }

    fn properties_circle_center(&self) -> euclid::default::Point2D<f32> {
        Point2D::new(
            self.length_property("cx"),
            self.length_property("cy"),
        )
    }
}

impl SvgElementTraits for Element {
    fn length_property_ext(&self, name: &str, default: f32) -> f32 {
        let Some(length) = self.attributes().find_by_str(name) else {
            println!("[Svg] Attribute \"{name}\" not found on element \"{}\"", self.qualified_name().local);
            return default;
        };

        if let Ok(float) = length.parse() {
            return float;
        }

        match retina_style_parser::parse_value_length(length) {
            Some(CssLength::Pixels(pixels)) => pixels as f32,

            unsupported => {
                warn!("Unsupported length type: {unsupported:?} for property \"{name}\"");
                default
            }
        }
    }

    fn paint_property_ext(&self, name: &str, default: Material) -> Material {
        let Some(color) = self.attributes().find_by_str(name) else {
            // Don't go above the <svg> element, because that is outside the SVG
            // figure.
            if self.qualified_name().local.eq_str_ignore_ascii_case("svg") {
                return default;
            }

            if let Some(parent) = self.as_node().parent().as_ref().and_then(Weak::upgrade) {
                if let Some(element) = parent.as_dom_element() {
                    return element.paint_property_ext(name, default);
                }
            }

            return default;
        };

        if color.eq_ignore_ascii_case("none") {
            return Material::Color(Color::TRANSPARENT);
        }

        match retina_style_parser::parse_value_color(color) {
            Some(CssColor::Color(color)) => Material::Color(color),
            _ => default,
        }
    }

    fn property_view_box(&self) -> Option<Rect<f32>> {
        let value = self.attributes().find_by_str("viewBox")?;

        info!("ViewBox: \"{value}\"");

        let values: Vec<f32> = value.split(|c| c == ' ' || c == ',')
            .filter_map(|x| {
                if x.is_empty() {
                    return None;
                }

                x.parse::<f32>().ok()
            })
            .collect();

        Some(match values[..] {
            // TODO
            [height] => Rect::new(Point2D::default(), Size2D::new(height, height)),
            [width, height] => Rect::new(Point2D::default(), Size2D::new(width, height)),
            [y, width, height] => Rect::new(Point2D::new(0.0, y), Size2D::new(width, height)),
            [x, y, width, height] => Rect::new(Point2D::new(x, y), Size2D::new(width, height)),

            _ => return None,
        })
    }

    fn stroke_style(&self, painter: &dyn Painter) -> Option<Box<dyn StrokeStyle>> {
        let value = self.attributes().find_by_str("stroke-linecap")?;
        let cap_style = match value {
            "butt" => CapStyle::Butt,
            "round" => CapStyle::Round,
            "square" => CapStyle::Square,
            _ => return None,
        };

        Some(painter.create_stroke_style(StrokeStyleProperties {
            cap_style,
        }))
    }
}
