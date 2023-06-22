// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use retina_gfx::{canvas::CanvasPainter, euclid::{Rect, Point2D, Size2D, UnknownUnit}};
use retina_layout::LayoutBox;
use retina_style::{CssColor, CssDecimal, CssLineStyle};
use retina_style_computation::BorderProperties;

pub struct Compositor {

}

impl Compositor {
    pub fn new() -> Self {
        Self {
        }
    }

    fn calculate_border_rect_bottom(
        &self,
        mut position: Point2D<CssDecimal, UnknownUnit>,
        layout_box: &LayoutBox,
    ) -> Rect<CssDecimal, UnknownUnit> {
        position.y += layout_box.dimensions().height().value() + layout_box.dimensions().border().top().value();
        let size = Size2D::new(
            layout_box.dimensions().border().left().value()
                + layout_box.dimensions().width().value()
                + layout_box.dimensions().border().right().value(),
            layout_box.dimensions().border().bottom().value(),
        );
        Rect::new(position, size)
    }

    fn calculate_border_rect_left(
        &self,
        position: Point2D<CssDecimal, UnknownUnit>,
        layout_box: &LayoutBox,
    ) -> Rect<CssDecimal, UnknownUnit> {
        let size = Size2D::new(
            layout_box.dimensions().border().left().value(),
            layout_box.dimensions().border().top().value()
                + layout_box.dimensions().height().value()
                + layout_box.dimensions().border().bottom().value(),
        );
        Rect::new(position, size)
    }

    fn calculate_border_rect_right(
        &self,
        mut position: Point2D<CssDecimal, UnknownUnit>,
        layout_box: &LayoutBox,
    ) -> Rect<CssDecimal, UnknownUnit> {
        position.x += layout_box.dimensions().width().value() + layout_box.dimensions().border().left().value();
        let size = Size2D::new(
            layout_box.dimensions().border().right().value(),
            layout_box.dimensions().border().top().value()
                + layout_box.dimensions().height().value()
                + layout_box.dimensions().border().bottom().value(),
        );
        Rect::new(position, size)
    }

    fn calculate_border_rect_top(
        &self,
        position: Point2D<CssDecimal, UnknownUnit>,
        layout_box: &LayoutBox,
    ) -> Rect<CssDecimal, UnknownUnit> {
        let size = Size2D::new(
            layout_box.dimensions().border().left().value()
                + layout_box.dimensions().width().value()
                + layout_box.dimensions().border().right().value(),
            layout_box.dimensions().border().top().value(),
        );
        Rect::new(position, size)
    }

    pub fn paint(&self, layout_box: &LayoutBox, painter: &mut CanvasPainter) {
        _ = layout_box;

        self.paint_background(layout_box, painter);
        self.paint_border(layout_box, painter);

        for child in layout_box.children() {
            self.paint(child, painter);
        }
    }

    fn paint_background(&self, layout_box: &LayoutBox, painter: &mut CanvasPainter) {
        let position = layout_box.dimensions().position_padding_box();

        let size = layout_box.dimensions().size_padding_box();

        if size.is_empty() {
            return;
        }

        match layout_box.computed_style().background_color() {
            CssColor::Color(background_color) => {
                if background_color.alpha() <= 0.0 {
                    return;
                }

                painter.paint_rect_colored(Rect::new(position, size), background_color);
            }
        }
    }

    fn paint_border(&self, layout_box: &LayoutBox, painter: &mut CanvasPainter) {
        let position = layout_box.dimensions().position_border_box();

        self.paint_border_part(
            layout_box.computed_style().border_bottom,
            self.calculate_border_rect_bottom(position, layout_box),
            painter,
        );

        self.paint_border_part(
            layout_box.computed_style().border_left,
            self.calculate_border_rect_left(position, layout_box),
            painter,
        );

        self.paint_border_part(
            layout_box.computed_style().border_right,
            self.calculate_border_rect_right(position, layout_box),
            painter,
        );

        self.paint_border_part(
            layout_box.computed_style().border_top,
            self.calculate_border_rect_top(position, layout_box),
            painter,
        );
    }

    fn paint_border_part(
        &self,
        border: BorderProperties,
        rect: Rect<CssDecimal, UnknownUnit>,
        painter: &mut CanvasPainter,
    ) {
        let CssLineStyle::Solid = border.style else {
            return;
        };

        match border.color {
            CssColor::Color(color) => painter.paint_rect_colored(rect, color),
        }
    }
}
