// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::rc::Rc;

use retina_style::{Stylesheet, CssDisplay, CssReferencePixels};
use retina_style_computation::{PropertyMap, StyleCollector, Cascade};

use crate::{
    DomNode,
    LayoutBox,
    LayoutBoxDimensions,
    LayoutBoxKind,
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

        let html_element = Rc::clone(
            &root.as_parent_node()
                .expect("root DOM node not the Document")
                .children()
                .borrow()
                .first()
                .expect("DOM Document should have 1 child, the HTMLHtmlElement")
        );

        let mut initial_containing_block = instance.generate_initial_containing_block(root);

        let html_box = instance.generate_for(html_element, &initial_containing_block)
            .expect("root node has no layout box generated");

        initial_containing_block.children.push(html_box);
        initial_containing_block.dump();
        initial_containing_block
    }

    fn calculate_dimensions_for_block_flow(
        &self,
        computed_style: &PropertyMap,
        parent: &LayoutBox
    ) -> LayoutBoxDimensions {
        // TODO
        _ = computed_style;
        LayoutBoxDimensions {
            width: parent.dimensions().width,
            height: parent.dimensions().height,
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
    ) -> LayoutBoxDimensions {
        // TODO
        self.calculate_dimensions_for_block_flow(computed_style, parent)
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

        if node.is_text() {
            // TODO
            let dimensions = Default::default();

            let parent_display = parent.computed_style().display();
            return match parent_display {
                CssDisplay::BlockFlow => Some(
                    LayoutBox::new(LayoutBoxKind::AnonymousBlock, node, computed_style, dimensions)
                ),

                CssDisplay::InlineFlow => Some(
                    LayoutBox::new(LayoutBoxKind::AnonymousInline, node, computed_style, dimensions)
                ),

                _ => {
                    println!("[layout] Warning: text node was omitted because of an unknown parent box `display` value: {parent_display:?}");
                    None
                }
            }
        }

        let mut layout_box = match computed_style.display() {
            // `display: inline`
            CssDisplay::InlineFlow => {
                let dimensions = self.calculate_dimensions_for_inline_flow(&computed_style, parent);
                LayoutBox::new(LayoutBoxKind::Inline{ line_boxes: Vec::new() }, node, computed_style, dimensions)
            }

            CssDisplay::BlockFlow => {
                let dimensions = self.calculate_dimensions_for_block_flow(&computed_style, parent);
                LayoutBox::new(LayoutBoxKind::Block, node, computed_style, dimensions)
            }

            _ => {
                println!(
                    "[layout] Warning: element was omitted because of an unknown `display` value: {:?}",
                    computed_style.display()
                );
                return None;
            }
        };

        if let Some(node) = layout_box.node.as_parent_node() {
            for child in node.children().borrow().iter() {
                if let Some(child) = self.generate_for(Rc::clone(child), &layout_box) {
                    layout_box.children.push(child);
                }
            }
        }

        Some(layout_box)
    }

    fn generate_initial_containing_block(&self, root: DomNode) -> LayoutBox {
        let computed_style = PropertyMap {
            display: Some(CssDisplay::BlockFlow),
            ..Default::default()
        };

        LayoutBox::new(
            LayoutBoxKind::Block,
            root,
            computed_style,
            self.calculate_dimensions_for_initial_containing_block(),
        )
    }

}
