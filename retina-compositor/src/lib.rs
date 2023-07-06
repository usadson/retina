// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::sync::OnceLock;

use retina_dom::HtmlElementKind;
use retina_gfx::{Painter, euclid::{Rect, Point2D, Size2D, UnknownUnit}, Texture};
use retina_layout::LayoutBox;
use retina_style::{CssColor, CssDecimal, CssLineStyle};
use retina_style_computation::BorderProperties;
use tracing::instrument;

#[derive(Debug)]
pub struct Compositor {

}

impl Compositor {
    pub fn new() -> Self {
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
        let _guard = CompositorTracingGuard::new();
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

        match layout_box.computed_style().background_color() {
            CssColor::Color(background_color) => {
                if background_color.alpha() <= 0.0 {
                    return;
                }

                painter.paint_rect_colored(Rect::new(position, size), background_color);
            }
        }
    }

    #[instrument(skip_all)]
    fn paint_border(&self, layout_box: &LayoutBox, painter: &mut Painter) {
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

    #[instrument(skip_all)]
    fn paint_border_part(
        &self,
        border: BorderProperties,
        rect: Rect<CssDecimal, UnknownUnit>,
        painter: &mut Painter,
    ) {
        let CssLineStyle::Solid = border.style else {
            return;
        };

        match border.color {
            CssColor::Color(color) => painter.paint_rect_colored(rect, color),
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
        let Some(text) = layout_box.node.as_text() else { return };

        let text = text.data().trim();
        if text.is_empty() { return };

        let position = layout_box.dimensions().position_content_box().cast();
        let size = layout_box.font_size().as_abs().value() as f32;
        if size <= 0.0 {
            return;
        }

        match layout_box.computed_style().color() {
            CssColor::Color(color) => {
                if color.alpha() <= 0.0 {
                    return;
                }

                let mut brush = layout_box.font().brush();

                painter.paint_text(
                    &mut brush,
                    text,
                    color,
                    position,
                    size,
                );
            }
        }
    }
}

struct CompositorTracingGuard {
    _chrome_guard: tracing_chrome::FlushGuard,
    _tracing_subscriber_guard: tracing::subscriber::DefaultGuard,
}

impl CompositorTracingGuard {
    const ENABLED: OnceLock<bool> = OnceLock::new();

    pub fn new() -> Option<Self> {
        if !Self::is_enabled() {
            return None;
        }

        use tracing_chrome::ChromeLayerBuilder;
        use tracing_subscriber::prelude::*;

        let (chrome_layer, _chrome_guard) = ChromeLayerBuilder::new().build();

        let _tracing_subscriber_guard = tracing_subscriber::registry()
            .with(chrome_layer)
            .set_default();

        Some(Self {
            _chrome_guard,
            _tracing_subscriber_guard,
        })
    }

    #[inline]
    pub fn is_enabled() -> bool {
        *Self::ENABLED.get_or_init(|| {
            std::env::var("RETINA_TRACE")
                .is_ok_and(|val| val.trim().eq_ignore_ascii_case("1"))
        })
    }
}
