// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use euclid::default::Point2D;
use log::warn;
use retina_dom::Node;
use retina_style::{Stylesheet, CssDisplay, CssReferencePixels, CssDisplayInside, CssDisplayOutside, CssLength, CssDisplayBox};
use retina_style_computation::{PropertyMap, StyleCollector, Cascade};

use crate::{
    DomNode,
    LayoutBox,
    LayoutBoxDimensions,
    LayoutBoxKind, LayoutEdge,
};

pub struct LayoutGenerator<'stylesheets> {
    stylesheets: &'stylesheets [Stylesheet],
    viewport_width: CssReferencePixels,
    viewport_height: CssReferencePixels,
}

impl<'stylesheets> LayoutGenerator<'stylesheets> {

    pub fn generate(
        root: DomNode,
        stylesheets: &'stylesheets [Stylesheet],
        viewport_width: CssReferencePixels,
        viewport_height: CssReferencePixels,
    ) -> LayoutBox {
        let instance = Self {
            stylesheets,
            viewport_width,
            viewport_height,
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

        let border = LayoutEdge {
            bottom: self.resolve_length(font_size, CssReferencePixels::new(0 as _), computed_style.border_bottom.width, computed_style),
            left: self.resolve_length(font_size, CssReferencePixels::new(0 as _), computed_style.border_left.width, computed_style),
            right: self.resolve_length(font_size, CssReferencePixels::new(0 as _), computed_style.border_right.width, computed_style),
            top: self.resolve_length(font_size, CssReferencePixels::new(0 as _), computed_style.border_top.width, computed_style),
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

        let position = Point2D::new(
            parent.dimensions.position.x + margin.left.value(),
            parent.dimensions.position.y + margin.top.value()
        );

        // TODO
        _ = computed_style;
        LayoutBoxDimensions {
            position,
            width,
            height,
            border,
            margin,
            ..Default::default()
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
            .cascade(parent.map(|parent| parent.computed_style()))
    }

    fn generate_for(
        &self,
        node: DomNode,
        parent: &LayoutBox,
    ) -> Option<LayoutBox> {
        let computed_style = self.resolve_style(&node, Some(parent));
        let font_size = self.resolve_length(parent.font_size, parent.font_size, computed_style.font_size(), &computed_style);

        if node.is_text() {
            // TODO
            let dimensions = Default::default();

            let parent_display = parent.computed_style().display();
            return match parent_display {
                CssDisplay::Normal { inside: CssDisplayInside::Flow, outside: CssDisplayOutside::Block, .. } => Some(
                    LayoutBox::new(LayoutBoxKind::AnonymousBlock, node, computed_style, dimensions, font_size)
                ),

                CssDisplay::Normal { inside: CssDisplayInside::Flow, outside: CssDisplayOutside::Inline, .. } => Some(
                    LayoutBox::new(LayoutBoxKind::AnonymousInline, node, computed_style, dimensions, font_size)
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
                LayoutBox::new(LayoutBoxKind::Inline, node, computed_style, dimensions, font_size)
            }

            CssDisplay::Normal { inside: CssDisplayInside::Flow, outside: CssDisplayOutside::Block, .. } => {
                let dimensions = self.calculate_dimensions_for_block_flow(&computed_style, parent, font_size);
                LayoutBox::new(LayoutBoxKind::Block, node, computed_style, dimensions, font_size)
            }

            _ => {
                warn!(
                    "Element was omitted because of an unknown `display` value: {:?}",
                    computed_style.display()
                );
                return None;
            }
        };

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

        let font_size = self.resolve_length(default_reference_pixels, default_reference_pixels, computed_style.font_size(), &computed_style);

        LayoutBox::new(
            LayoutBoxKind::Block,
            root,
            computed_style,
            self.calculate_dimensions_for_initial_containing_block(),
            font_size
        )
    }

}
