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
mod tesselator;

use crate::path::SvgPathCommand;

pub use self::painter::{
    Geometry,
    GeometrySink,
    GeometrySinkFillType,
    Material,
    Painter,
};

use euclid::default::Box2D;
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
            "rect" => self.render_rect(element),
            "path" => self.render_path(element),
            _ => (),
        }

        for child in element.as_parent_node().children().iter() {
            self.render_node(child);
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
                SvgPathCommand::ClosePath => sink.close_path(),
                _command => info!("Todo: {_command:#?}"),
            }
        }

        let geometry = sink.finish();
        if !fill.is_transparent() {
            self.painter.draw_geometry(geometry.as_ref(), fill);
        }

        if !stroke.is_transparent() && stroke_width > 0.0 {
            self.painter.stroke_geometry(geometry.as_ref(), stroke, stroke_width);
        }
    }

    fn render_rect(&mut self, element: &Element) {
        let min = point(element.property_x(), element.property_y());
        let max = point(element.property_width(), element.property_height());

        let rect = Box2D::new(min, max);

        let fill = element.property_fill();
        if !fill.is_transparent() {
            self.painter.draw_rect(rect, fill);
        }

        let stroke = element.property_stroke();
        let stroke_width = element.property_stroke_width();
        if !stroke.is_transparent() && stroke_width > 0.0 {
            self.painter.stroke_rect(rect, stroke, stroke_width);
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

    fn property_x(&self) -> f32 { self.length_property("x") }
    fn property_y(&self) -> f32 { self.length_property("y") }
    fn property_width(&self) -> f32 { self.length_property("width") }
    fn property_height(&self) -> f32 { self.length_property("height") }
    fn property_stroke_width(&self) -> f32 { self.length_property_ext("stroke-width", 1.0) }
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
}
