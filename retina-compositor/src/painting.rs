// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use retina_dom::{
    HtmlElementKind,
    ImageData,
};
use retina_gfx::{
    Color,
    euclid::{
        Point2D,
        Rect,
        Size2D,
        UnknownUnit,
    },
    Painter,
    Texture,
};
use retina_layout::LayoutBox;
use retina_style::{
    CssColor,
    CssDecimal,
    CssLineStyle,
    CssTextDecorationLine,
};
use retina_style_computation::BorderProperties;
use tracing::instrument;

#[derive(Debug)]
pub struct PaintInvoker {

}

impl PaintInvoker {
    pub const fn new() -> Self {
        Self {
        }
    }

    #[instrument(skip_all)]
    fn calculate_border_rect_bottom(
        &self,
        mut position: Point2D<CssDecimal, UnknownUnit>,
        layout_box: &LayoutBox,
    ) -> Rect<CssDecimal, UnknownUnit> {
        position.y += layout_box.dimensions().border().top().value();
        position.y += layout_box.dimensions().padding().top().value();
        position.y += layout_box.dimensions().height().value();
        position.y += layout_box.dimensions().padding().bottom().value();
        let size = Size2D::new(
            layout_box.dimensions().border().left().value()
                + layout_box.dimensions().padding().left().value()
                + layout_box.dimensions().width().value()
                + layout_box.dimensions().padding().right().value()
                + layout_box.dimensions().border().right().value(),
            layout_box.dimensions().border().bottom().value(),
        );
        Rect::new(position, size)
    }

    #[instrument(skip_all)]
    fn calculate_border_rect_left(
        &self,
        position: Point2D<CssDecimal, UnknownUnit>,
        layout_box: &LayoutBox,
    ) -> Rect<CssDecimal, UnknownUnit> {
        let size = Size2D::new(
            layout_box.dimensions().border().left().value(),
            layout_box.dimensions().border().top().value()
                + layout_box.dimensions().padding().top().value()
                + layout_box.dimensions().height().value()
                + layout_box.dimensions().padding().bottom().value()
                + layout_box.dimensions().border().bottom().value(),
        );
        Rect::new(position, size)
    }

    #[instrument(skip_all)]
    fn calculate_border_rect_right(
        &self,
        mut position: Point2D<CssDecimal, UnknownUnit>,
        layout_box: &LayoutBox,
    ) -> Rect<CssDecimal, UnknownUnit> {
        position.x += layout_box.dimensions().border().left().value();
        position.x += layout_box.dimensions().padding().left().value();
        position.x += layout_box.dimensions().width().value();
        position.x += layout_box.dimensions().padding().right().value();
        let size = Size2D::new(
            layout_box.dimensions().border().right().value(),
            layout_box.dimensions().border().top().value()
                + layout_box.dimensions().padding().top().value()
                + layout_box.dimensions().height().value()
                + layout_box.dimensions().padding().bottom().value()
                + layout_box.dimensions().border().bottom().value(),
        );
        Rect::new(position, size)
    }

    #[instrument(skip_all)]
    fn calculate_border_rect_top(
        &self,
        position: Point2D<CssDecimal, UnknownUnit>,
        layout_box: &LayoutBox,
    ) -> Rect<CssDecimal, UnknownUnit> {
        let size = Size2D::new(
            layout_box.dimensions().border().left().value()
                + layout_box.dimensions().padding().left().value()
                + layout_box.dimensions().width().value()
                + layout_box.dimensions().padding().right().value()
                + layout_box.dimensions().border().right().value(),
            layout_box.dimensions().border().top().value(),
        );
        Rect::new(position, size)
    }

    #[instrument(skip_all)]
    pub fn paint(&self, layout_box: &LayoutBox, painter: &mut Painter) {
        self.paint_box(layout_box, painter);
    }

    #[instrument(skip(painter))]
    fn paint_box(&self, layout_box: &LayoutBox, painter: &mut Painter) {
        if painter.is_rect_inside_viewport(layout_box.dimensions().rect_border_box().cast()) {
            self.paint_background(layout_box, painter);
            self.paint_border(layout_box, painter);
            self.paint_text(layout_box, painter);
        }

        self.paint_replaced_content(layout_box, painter);

        for child in layout_box.children() {
            self.paint_box(child, painter);
        }
    }

    #[instrument(skip_all)]
    fn paint_background(&self, layout_box: &LayoutBox, painter: &mut Painter) {
        let position = layout_box.dimensions().position_padding_box();

        let size = layout_box.dimensions().size_padding_box();

        if size.is_empty() {
            return;
        }

        let rect = Rect::new(position, size);
        self.paint_background_color(painter, layout_box, rect);

        if let Some(background_image) = layout_box.background_image() {
            self.paint_background_image(layout_box, painter, background_image, rect);
        }
    }

    #[instrument(skip_all)]
    #[inline]
    fn paint_background_color(
        &self,
        painter: &mut Painter,
        layout_box: &LayoutBox,
        rect: Rect<CssDecimal, UnknownUnit>
    ) {
        let background_color = layout_box.actual_values().background_color;

        if background_color.alpha() <= 0.0 {
            return;
        }

        painter.paint_rect_colored(rect, background_color);
    }

    #[instrument(skip_all)]
    fn paint_background_image(
        &self,
        layout_box: &LayoutBox,
        painter: &mut Painter,
        image_data: &ImageData,
        rect: Rect<CssDecimal, UnknownUnit>
    ) {
        let Ok(graphics) = image_data.graphics().read() else { return };

        let Some(texture) = graphics.downcast_ref::<Texture>() else {
            log::warn!("background-image Graphics wasn't an instance of `Texture`, state: {:?}", image_data.state());
            return;
        };

        // TODO background-repeat, background-size, etc.
        _ = layout_box;

        painter.paint_rect_textured(rect, texture.view());
    }

    #[instrument(skip_all)]
    fn paint_border(&self, layout_box: &LayoutBox, painter: &mut Painter) {
        let position = layout_box.dimensions().position_border_box();
        let text_color = layout_box.actual_values().text_color;

        self.paint_border_part(
            layout_box.computed_style().border_bottom,
            self.calculate_border_rect_bottom(position, layout_box),
            text_color,
            painter,
        );

        self.paint_border_part(
            layout_box.computed_style().border_left,
            self.calculate_border_rect_left(position, layout_box),
            text_color,
            painter,
        );

        self.paint_border_part(
            layout_box.computed_style().border_right,
            self.calculate_border_rect_right(position, layout_box),
            text_color,
            painter,
        );

        self.paint_border_part(
            layout_box.computed_style().border_top,
            self.calculate_border_rect_top(position, layout_box),
            text_color,
            painter,
        );
    }

    #[instrument(skip_all)]
    fn paint_border_part(
        &self,
        border: BorderProperties,
        rect: Rect<CssDecimal, UnknownUnit>,
        text_color: Color,
        painter: &mut Painter,
    ) {
        let CssLineStyle::Solid = border.style else {
            return;
        };

        match border.color {
            CssColor::Color(color) => painter.paint_rect_colored(rect, color),
            CssColor::CurrentColor => painter.paint_rect_colored(rect, text_color),
        }
    }

    #[instrument(skip_all)]
    fn paint_replaced_content(
        &self,
        layout_box: &LayoutBox,
        painter: &mut Painter,
    ) {
        let Some(HtmlElementKind::Img(img)) = layout_box.node.as_html_element_kind() else {
            return;
        };

        let Ok(graphics) = img.data_ref().graphics().read() else {
            log::warn!("No graphics was available for image");
            return;
        };

        let Some(texture) = graphics.downcast_ref::<Texture>() else {
            log::warn!("Graphics wasn't an instance of `Texture`");
            return;
        };

        let size = Size2D::new(texture.width() as _, texture.height() as _);
        let rect = Rect::new(layout_box.dimensions().position_content_box(), size);

        painter.paint_rect_textured(rect, texture.view());
    }

    #[instrument(skip_all)]
    fn paint_text(
        &self,
        layout_box: &LayoutBox,
        painter: &mut Painter,
    ) {
        let size = layout_box.font_size().as_abs().value() as f32;
        if size <= 0.0 {
            return;
        }

        let color = layout_box.actual_values().text_color;
        if color.alpha() <= 0.0 {
            return;
        }

        let text_decoration_color = match layout_box.computed_style().text_decoration_color.unwrap_or(CssColor::CurrentColor) {
            CssColor::Color(color) => color,
            CssColor::CurrentColor => layout_box.actual_values().text_color,
        };

        let line = layout_box.computed_style().text_decoration_line.unwrap_or_default();
        let font_baseline = layout_box.font().baseline_offset(size) as CssDecimal;
        let font_underline_offset  = layout_box.font().underline_position(size) as CssDecimal;

        let text_decoration_offset = match line {
            CssTextDecorationLine::Underline => font_baseline + font_underline_offset,
            CssTextDecorationLine::LineThrough => font_baseline / 2.0,
            CssTextDecorationLine::Overline => 0.0,
            _ => 0.0,
        };
        let text_decoration_thickness = layout_box.font().underline_thickness(size) as CssDecimal;

        for line_box_fragment in layout_box.line_box_fragments() {
            let position = line_box_fragment.position();

            if !painter.viewport_rect().cast_unit().intersects(&Rect::new(line_box_fragment.position(), line_box_fragment.size())) {
                return;
            }

            layout_box.font().paint(
                line_box_fragment.text(),
                color,
                position.cast(),
                size,
                layout_box.actual_values().text_hinting_options,
                painter,
            );

            match line {
                CssTextDecorationLine::None => (),
                _ => {
                    let rect: Rect<f64, UnknownUnit> = Rect::new(
                        Point2D::new(
                            position.x,
                            position.y + text_decoration_offset
                        ),
                        Size2D::new(line_box_fragment.size().width, text_decoration_thickness)
                    );

                    painter.paint_rect_colored(
                        rect, text_decoration_color
                    );
                }
            }
        }
    }
}
