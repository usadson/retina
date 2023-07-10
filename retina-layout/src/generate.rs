// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use euclid::default::Point2D;
use log::warn;
use retina_common::Color;
use retina_dom::{Node, ImageData};
use retina_fetch::{Fetch, Url};
use retina_gfx_font::{FontProvider, FontDescriptor, FontWeight, FontHandle, FamilyName};
use retina_style::{Stylesheet, CssDisplay, CssReferencePixels, CssDisplayInside, CssDisplayOutside, CssLength, CssDisplayBox, CssFontFamilyName, CssGenericFontFamilyName, CssFontWeight, CssLineStyle, CssImage, CssColor};
use retina_style_computation::{PropertyMap, StyleCollector, Cascade, BorderProperties};

use crate::{
    DomNode,
    formatting_context::FormattingContextKind,
    LayoutBox,
    LayoutBoxDimensions,
    LayoutBoxKind,
    LayoutEdge, ActualValueMap,
};

pub struct LayoutGenerator<'stylesheets> {
    stylesheets: &'stylesheets [Stylesheet],
    viewport_width: CssReferencePixels,
    viewport_height: CssReferencePixels,
    font_provider: FontProvider,
    document_url: &'stylesheets Url,
    fetch: Fetch,
}

impl<'stylesheets> LayoutGenerator<'stylesheets> {

    pub fn generate(
        root: DomNode,
        stylesheets: &'stylesheets [Stylesheet],
        viewport_width: CssReferencePixels,
        viewport_height: CssReferencePixels,
        font_provider: FontProvider,
        document_url: &'stylesheets Url,
        fetch: Fetch,
    ) -> LayoutBox {
        let instance = Self {
            stylesheets,
            viewport_width,
            viewport_height,
            font_provider,
            document_url,
            fetch,
        };

        let html_element = Node::clone(
            &root.as_parent_node()
                .expect("root DOM node not the Document")
                .children()
                .first()
                .expect("DOM Document should have 1 child, the HTMLHtmlElement")
        );

        let mut initial_containing_block = instance.generate_initial_containing_block(root);

        let html_box = instance.generate_for(html_element, &initial_containing_block)
            .expect("root node has no layout box generated");

        initial_containing_block.children.push(html_box);

        initial_containing_block.dimensions_mut().set_margin_size(
            viewport_width,
            viewport_height,
        );
        initial_containing_block.run_layout(None);

        // initial_containing_block.dump();
        initial_containing_block
    }

    fn calculate_dimensions_for_block_flow(
        &self,
        computed_style: &PropertyMap,
        parent: &LayoutBox,
        font_size: CssReferencePixels,
    ) -> LayoutBoxDimensions {
        let parent_width = parent.dimensions().width;
        let parent_height = parent.dimensions().height;

        let margin = LayoutEdge {
            bottom: self.resolve_length(font_size, CssReferencePixels::new(0 as _), computed_style.margin_bottom(), computed_style),
            left: self.resolve_length(font_size, CssReferencePixels::new(0 as _), computed_style.margin_left(), computed_style),
            right: self.resolve_length(font_size, CssReferencePixels::new(0 as _), computed_style.margin_right(), computed_style),
            top: self.resolve_length(font_size, CssReferencePixels::new(0 as _), computed_style.margin_top(), computed_style),
        };

        let border = self.resolve_border_edge(computed_style, font_size);

        let padding = LayoutEdge {
            bottom: self.resolve_length(font_size, CssReferencePixels::new(0 as _), computed_style.padding_bottom(), computed_style),
            left: self.resolve_length(font_size, CssReferencePixels::new(0 as _), computed_style.padding_left(), computed_style),
            right: self.resolve_length(font_size, CssReferencePixels::new(0 as _), computed_style.padding_right(), computed_style),
            top: self.resolve_length(font_size, CssReferencePixels::new(0 as _), computed_style.padding_top(), computed_style),
        };

        let mut width = self.resolve_length(font_size, parent_width, computed_style.width(), computed_style);
        let mut height = self.resolve_length(font_size, parent_height, computed_style.height(), computed_style);

        if let CssLength::Auto = computed_style.width() {
            width -= margin.left + border.left + border.right + margin.right;
        }

        if let CssLength::Auto = computed_style.height() {
            height -= margin.top + border.top + border.bottom + margin.bottom;
        }

        width.ensure_abs();
        height.ensure_abs();

        let content_position = Point2D::new(
            parent.dimensions.content_position.x + margin.left.value() + border.left.value() + padding.left.value(),
            parent.dimensions.content_position.y + margin.top.value() + border.top.value() + padding.top.value()
        );

        // TODO
        _ = computed_style;
        LayoutBoxDimensions {
            content_position,

            width,
            height,

            margin,
            border,
            padding,
        }
    }

    /// The [initial containing block][icb-lvl4-display] is the root node of the
    /// layout tree, and [has the dimensions of the viewport][icb-css22-dimensions].
    ///
    /// [icb-css22-dimensions]: https://www.w3.org/TR/CSS22/visudet.html#x1
    /// [icb-lvl4-display]: https://drafts.csswg.org/css-display-4/#initial-containing-block
    fn calculate_dimensions_for_initial_containing_block(&self) -> LayoutBoxDimensions {
        LayoutBoxDimensions {
            width: self.viewport_width,
            height: self.viewport_height,
            ..Default::default()
        }
    }

    fn calculate_dimensions_for_inline_flow(
        &self,
        computed_style: &PropertyMap,
        parent: &LayoutBox,
        font_size: CssReferencePixels,
    ) -> LayoutBoxDimensions {
        // TODO
        self.calculate_dimensions_for_block_flow(computed_style, parent, font_size)
    }

    fn compute_actual_values(
        &self,
        parent: &LayoutBox,
        computed_style: &PropertyMap
    ) -> ActualValueMap {
        let text_color = match computed_style.color() {
            CssColor::Color(color) => color,
            CssColor::CurrentColor => parent.actual_value_map.text_color,
        };

        ActualValueMap {
            background_color: match computed_style.background_color() {
                CssColor::Color(color) => color,
                CssColor::CurrentColor => text_color,
            },
            text_color,
        }
    }

    /// <https://drafts.csswg.org/css-fonts-4/#relative-weights>
    fn compute_font_weight(
        &self,
        parent: &LayoutBox,
        value: CssFontWeight,
    ) -> FontWeight {
        let parent_weight = parent.font.descriptor().weight;

        match value {
            CssFontWeight::Absolute(value) => FontWeight::new(value as _),
            CssFontWeight::Bolder => {
                if parent_weight.value() < 350.0 {
                    FontWeight::new(400.0)
                } else if parent_weight.value() <= 550.0 {
                    FontWeight::new(700.0)
                } else {
                    FontWeight::new(900.0)
                }
            }
            CssFontWeight::Lighter => {
                if parent_weight.value() < 100.0 {
                    parent_weight
                } else if parent_weight.value() < 550.0 {
                    FontWeight::new(100.0)
                } else if parent_weight.value() <= 750.0 {
                    FontWeight::new(400.0)
                } else {
                    FontWeight::new(700.0)
                }
            }
        }
    }

    fn resolve_border_edge(
        &self,
        computed_style: &PropertyMap,
        font_size: CssReferencePixels,
    ) -> LayoutEdge {
        LayoutEdge {
            bottom: self.resolve_border_edge_part(&computed_style.border_bottom, font_size, computed_style),
            left: self.resolve_border_edge_part(&computed_style.border_left, font_size, computed_style),
            right: self.resolve_border_edge_part(&computed_style.border_right, font_size, computed_style),
            top: self.resolve_border_edge_part(&computed_style.border_top, font_size, computed_style),
        }
    }

    fn resolve_border_edge_part(
        &self,
        border: &BorderProperties,
        font_size: CssReferencePixels,
        computed_style: &PropertyMap
    ) -> CssReferencePixels {
        if border.style == CssLineStyle::None {
            return Default::default()
        }

        self.resolve_length(
            font_size,
            CssReferencePixels::new(0 as _),
            border.width,
            computed_style
        )
    }

    fn resolve_font(
        &self,
        node: &DomNode,
        parent: &LayoutBox,
        computed_style: &PropertyMap,
    ) -> FontHandle {
        _ = node;

        if computed_style.has_same_font_properties(&parent.computed_style) {
            return parent.font.clone();
        }

        if let Some(font_families) = &computed_style.font_family_list {
            for font_family in font_families {
                let name = match font_family {
                    CssFontFamilyName::Name(name) => FamilyName::Title(name.clone()),
                    CssFontFamilyName::Generic(generic) => match generic {
                        CssGenericFontFamilyName::Cursive => FamilyName::Cursive,
                        CssGenericFontFamilyName::Fantasy => FamilyName::Fantasy,
                        CssGenericFontFamilyName::Monospace => FamilyName::Monospace,
                        CssGenericFontFamilyName::SansSerif => FamilyName::SansSerif,
                        CssGenericFontFamilyName::Serif => FamilyName::Serif,
                        _ => continue,
                    }
                };

                let weight = self.compute_font_weight(parent, computed_style.font_weight());

                let descriptor = FontDescriptor {
                    name,
                    weight,
                };

                if let Some(font) = self.font_provider.get(descriptor) {
                    return font;
                }
            }
        }

        parent.font.clone()
    }

    fn resolve_length(
        &self,
        font_size: CssReferencePixels,
        parent_value: CssReferencePixels,
        length_value: CssLength,
        computed_style: &PropertyMap,
    ) -> CssReferencePixels {
        _ = computed_style;
        match length_value {
            CssLength::Auto => parent_value,
            CssLength::FontSize(percentage) => font_size * percentage,

            // TODO this should use the size of the root element
            CssLength::FontSizeOfRootElement(percentage) => font_size * percentage,

            CssLength::Percentage(percentage) => parent_value * percentage,
            CssLength::Pixels(pixels) => CssReferencePixels::new(pixels),
            CssLength::UaDefaultViewportHeightPercentage(percentage) => self.viewport_height * percentage,
            CssLength::UaDefaultViewportWidthPercentage(percentage) => self.viewport_width * percentage,
        }
    }

    fn resolve_style(
        &self,
        node: &DomNode,
        parent: Option<&LayoutBox>,
    ) -> PropertyMap {
        StyleCollector::new(self.stylesheets)
            .collect(node.as_ref())
            .cascade(Some(node.as_ref()), parent.map(|parent| parent.computed_style()))
    }

    fn generate_for(
        &self,
        node: DomNode,
        parent: &LayoutBox,
    ) -> Option<LayoutBox> {
        let computed_style = self.resolve_style(&node, Some(parent));
        let font = self.resolve_font(&node, parent, &computed_style);
        let font_size = self.resolve_length(parent.font_size, parent.font_size, computed_style.font_size(), &computed_style);
        let actual_value_map = self.compute_actual_values(parent, &computed_style);

        if node.is_text() {
            let dimensions = self.calculate_dimensions_for_inline_flow(&computed_style, parent, font_size);

            let parent_display = parent.computed_style().display();

            return match parent_display {
                CssDisplay::Normal { inside: CssDisplayInside::Flow, outside: CssDisplayOutside::Block, .. } => Some(
                    LayoutBox::new(LayoutBoxKind::Anonymous, FormattingContextKind::Inline, node, computed_style, actual_value_map, dimensions, font, font_size)
                ),

                CssDisplay::Normal { inside: CssDisplayInside::Flow, outside: CssDisplayOutside::Inline, .. } => Some(
                    LayoutBox::new(LayoutBoxKind::Anonymous, FormattingContextKind::Inline, node, computed_style, actual_value_map, dimensions, font, font_size)
                ),

                _ => {
                    warn!("[layout] Warning: text node was omitted because of an unknown parent box `display` value: {parent_display:?}");
                    None
                }
            }
        }

        let mut layout_box = match computed_style.display() {
            CssDisplay::Box(CssDisplayBox::None) => return None,

            // `display: inline`
            CssDisplay::Normal { inside: CssDisplayInside::Flow, outside: CssDisplayOutside::Inline, .. } => {
                let dimensions = self.calculate_dimensions_for_inline_flow(&computed_style, parent, font_size);
                LayoutBox::new(LayoutBoxKind::Normal, FormattingContextKind::Inline, node, computed_style, actual_value_map, dimensions, font, font_size)
            }

            CssDisplay::Normal { inside: CssDisplayInside::Flow, outside: CssDisplayOutside::Block, .. } => {
                let dimensions = self.calculate_dimensions_for_block_flow(&computed_style, parent, font_size);
                LayoutBox::new(LayoutBoxKind::Normal, FormattingContextKind::Block, node, computed_style, actual_value_map, dimensions, font, font_size)
            }

            _ => {
                warn!(
                    "Element was omitted because of an unknown `display` value: {:?}",
                    computed_style.display()
                );
                return None;
            }
        };

        if let Some(CssImage::Url(background_image_url)) = layout_box.computed_style.background_image.clone() {
            let data = ImageData::new();
            {
                let data = data.clone();
                let fetch = self.fetch.clone();
                let base_url = self.document_url.clone();
                tokio::task::spawn(async move {
                    data.update(base_url, fetch, &background_image_url).await;
                });
            }
            layout_box.background_image = Some(data);
        }

        if let Some(node) = layout_box.node.as_parent_node() {
            for child in node.children().iter() {
                if let Some(child) = self.generate_for(Node::clone(child), &layout_box) {
                    layout_box.children.push(child);
                }
            }
        }

        Some(layout_box)
    }

    fn generate_initial_containing_block(&self, root: DomNode) -> LayoutBox {
        let computed_style = PropertyMap {
            display: Some(CssDisplay::Normal {
                inside: CssDisplayInside::Flow,
                outside: CssDisplayOutside::Block,
                is_list_item: false,
            }),
            ..Default::default()
        };

        let default_reference_pixels = CssReferencePixels::new(16.0);

        let font = self.font_provider.get(FontDescriptor {
            name: retina_gfx_font::FamilyName::SansSerif,
            weight: FontWeight::REGULAR,
        }).expect("failed to load sans-serif font");

        let font_size = self.resolve_length(default_reference_pixels, default_reference_pixels, computed_style.font_size(), &computed_style);

        let actual_value_map = ActualValueMap {
            text_color: Color::BLACK,
            background_color: Color::WHITE,
        };

        LayoutBox::new(
            LayoutBoxKind::Root,
            FormattingContextKind::Block,
            root,
            computed_style,
            actual_value_map,
            self.calculate_dimensions_for_initial_containing_block(),
            font,
            font_size
        )
    }

}
