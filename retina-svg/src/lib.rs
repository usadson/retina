// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.
//
//! retina-svg, the SVG renderer.
//!
//! ## References
//! * [Scalable Vector Graphics (SVG) 1.1 Specification](https://www.w3.org/TR/SVG11/)
//! * [Scalable Vector Graphics (SVG) 1.1 Specification (Single Page)](https://www.w3.org/TR/SVG11/single-page.html)

mod painter;

use log::warn;
use lyon::lyon_tessellation::geometry_builder::simple_builder;
use lyon::math::{Point, point, Box2D};
use lyon::path::{Path, Winding};
use lyon::tessellation::*;

use retina_common::Color;
use retina_dom::{
    Element,
    Node,
};
use retina_gfx::Painter;
use retina_gfx::{
    Context,
    canvas::CanvasPaintingContext,
    euclid::Size2D,
};
use retina_style::{CssColor, CssLength};

pub fn render(node: &Node, context: Context) {
    let size = Size2D::new(300, 200);

    let mut rendering_context = CanvasPaintingContext::new(context, "SvgRenderer", size);
    let mut painter = rendering_context.begin(Color::TRANSPARENT, Default::default());

    render_inner(node, &mut painter);

    painter.submit_sync();
    // TODO return the texture
}

fn render_inner(node: &Node, painter: &mut Painter<'_>) {
    let mut renderer = SvgRenderer {
        root_node: node.clone(),
        painter,
    };

    renderer.render_node(&node);
}

pub struct SvgRenderer<'painter, 'art> {
    #[allow(dead_code)]
    root_node: Node,
    painter: &'painter mut Painter<'art>,
}

impl<'painter, 'art> SvgRenderer<'painter, 'art> {
    fn render_node(&mut self, node: &Node) {
        let Some(element) = node.as_dom_element() else { return };
        match element.qualified_name().local.as_ref() {
            "rect" => self.render_rect(element),
            _ => (),
        }

        for child in element.as_parent_node().children().iter() {
            self.render_node(child);
        }
    }

    fn render_rect(&mut self, element: &Element) {
        let mut geometry: VertexBuffers<Point, u16> = VertexBuffers::new();
        let mut geometry_builder = simple_builder(&mut geometry);
        let options = FillOptions::tolerance(0.1);
        let mut tessellator = FillTessellator::new();

        let mut builder = tessellator.builder(
            &options,
            &mut geometry_builder,
        );

        let min = point(element.property_x(), element.property_y());
        let max = point(element.property_width(), element.property_height());

        builder.add_rectangle(
            &Box2D::new(min, max),
            Winding::Positive
        );

        builder.build().expect("failed to build path");
        // self.painter.paint_rect_textured(rect, texture_view)
    }
}

trait SvgElementTraits {
    fn length_property(&self, name: &str) -> f32;

    /// <https://www.w3.org/TR/SVG11/single-page.html#painting-FillProperty>
    fn property_fill(&self) -> Color;

    fn property_x(&self) -> f32 { self.length_property("x") }
    fn property_y(&self) -> f32 { self.length_property("y") }
    fn property_width(&self) -> f32 { self.length_property("width") }
    fn property_height(&self) -> f32 { self.length_property("height") }
}

impl SvgElementTraits for Element {
    fn length_property(&self, name: &str) -> f32 {
        const DEFAULT: f32 = 0.0;

        let Some(length) = self.attributes().find_by_str(name) else {
            return DEFAULT;
        };

        match retina_style_parser::parse_value_length(length) {
            Some(CssLength::Pixels(pixels)) => pixels as f32,

            unsupported => {
                warn!("Unsupported length type: {unsupported:?} for property \"{name}\"");
                DEFAULT
            }
        }
    }

    fn property_fill(&self) -> Color {
        const DEFAULT: Color = Color::BLACK;

        let Some(color) = self.attributes().find_by_str("fill") else {
            return DEFAULT;
        };

        match retina_style_parser::parse_value_color(color) {
            Some(CssColor::Color(color)) => color,
            _ => DEFAULT,
        }
    }
}
