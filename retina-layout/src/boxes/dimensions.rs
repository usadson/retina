// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use euclid::default::{Point2D, Size2D, Rect};
use retina_style::{CssReferencePixels, CssDecimal};

use super::LayoutEdge;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct LayoutBoxDimensions {
    pub(crate) content_position: Point2D<CssDecimal>,

    pub(crate) width: CssReferencePixels,
    pub(crate) height: CssReferencePixels,

    pub(crate) padding: LayoutEdge,
    pub(crate) border: LayoutEdge,
    pub(crate) margin: LayoutEdge,
}

impl LayoutBoxDimensions {
    pub fn combined_bottom_edges(&self) -> CssReferencePixels {
        self.margin().bottom()
            + self.border().bottom()
            + self.padding().bottom()
    }

    pub fn combined_horizontal_edges(&self) -> CssReferencePixels {
        self.combined_left_edges() + self.combined_right_edges()
    }

    pub fn combined_left_edges(&self) -> CssReferencePixels {
        self.margin().left()
            + self.border().left()
            + self.padding().left()
    }

    pub fn combined_right_edges(&self) -> CssReferencePixels {
        self.margin().right()
            + self.border().right()
            + self.padding().right()
    }

    pub fn combined_top_edges(&self) -> CssReferencePixels {
        self.margin().top()
            + self.border().top()
            + self.padding().top()
    }

    pub fn combined_vertical_edges(&self) -> CssReferencePixels {
        self.combined_top_edges() + self.combined_bottom_edges()
    }

    pub fn position_border_box(&self) -> Point2D<CssDecimal> {
        Point2D::new(
            self.content_position.x - self.padding.left.value() - self.border.left.value(),
            self.content_position.y - self.padding.top.value() - self.border.top.value(),
        )
    }

    pub fn position_content_box(&self) -> Point2D<CssDecimal> {
        self.content_position
    }

    pub fn position_margin_box(&self) -> Point2D<CssDecimal> {
        Point2D::new(
            self.content_position.x - self.padding.left.value() - self.border.left.value() - self.margin.left.value(),
            self.content_position.y - self.padding.top.value() - self.border.top.value() - self.margin.top.value(),
        )
    }

    pub fn position_padding_box(&self) -> Point2D<CssDecimal> {
        Point2D::new(
            self.content_position.x - self.padding.left.value(),
            self.content_position.y - self.padding.top.value(),
        )
    }

    pub fn set_content_position(&mut self, position: Point2D<CssDecimal>) {
        self.content_position = position;
    }

    pub fn set_margin_position(&mut self, mut position: Point2D<CssDecimal>) {
        position.x += self.margin.left.value() + self.border.left.value() + self.padding.left.value();
        position.y += self.margin.top.value() + self.border.top.value() + self.padding.top.value();

        self.set_content_position(position);
    }

    pub fn set_margin_size(&mut self, mut width: CssReferencePixels, mut height: CssReferencePixels) {
        width -= self.margin.left + self.border.left + self.padding.left;
        width -= self.margin.right + self.border.right + self.padding.right;

        height -= self.margin.top + self.border.top + self.padding.top;
        height -= self.margin.bottom + self.border.bottom + self.padding.bottom;

        self.width = width;
        self.height = height;
    }

    pub fn size_content_box(&self) -> Size2D<CssDecimal> {
        Size2D::new(
            self.width.value(),
            self.height.value(),
        )
    }

    pub fn size_padding_box(&self) -> Size2D<CssDecimal> {
        let padding = Size2D::new(
            self.padding.left.value() + self.padding.right.value(),
            self.padding.top.value() + self.padding.bottom.value()
        );
        self.size_content_box() + padding
    }

    pub fn size_border_box(&self) -> Size2D<CssDecimal> {
        let border = Size2D::new(
            self.border.left.value() + self.border.right.value(),
            self.border.top.value() + self.border.bottom.value()
        );
        self.size_padding_box() + border
    }

    pub fn size_margin_box(&self) -> Size2D<CssDecimal> {
        let margins = Size2D::new(
            self.margin.left.value() + self.margin.right().value(),
            self.margin.top.value() + self.margin.bottom.value(),
        );
        self.size_border_box() + margins
    }

    pub fn rect_border_box(&self) -> Rect<CssDecimal> {
        Rect::new(self.position_border_box(), self.size_border_box())
    }

    pub fn width(&self) -> CssReferencePixels {
        self.width
    }

    pub fn height(&self) -> CssReferencePixels {
        self.height
    }

    pub fn padding(&self) -> LayoutEdge {
        self.padding
    }

    pub fn border(&self) -> LayoutEdge {
        self.border
    }

    pub fn margin(&self) -> LayoutEdge {
        self.margin
    }
}
