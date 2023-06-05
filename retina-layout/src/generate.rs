// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use retina_style::{Stylesheet, CssDisplay, CssReferencePixels};
use retina_style_computation::{PropertyMap, StyleCollector, Cascade};

use crate::{
    DomNode,
    LayoutBox, LayoutBoxKind, boxes::LayoutBoxDimensions,
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

        instance.generate_for(root, None)
            .expect("root node has no layout box generated")
    }



    fn calculate_dimensions_for_block_flow(
        &self,
        computed_style: &PropertyMap,
        parent: Option<&LayoutBox>
    ) -> LayoutBoxDimensions {
        // Initial containing block
        if parent.is_none() {
            return self.calculate_dimensions_for_initial_containing_block();
        }

        _ = computed_style;

        // Fixme
        return Default::default();
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
        parent: Option<&LayoutBox>,
    ) -> LayoutBoxDimensions {
        // TODO
        _ = computed_style;
        _ = parent;
        Default::default()
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
        parent: Option<&LayoutBox>,
    ) -> Option<LayoutBox> {
        let computed_style = self.resolve_style(&node, parent);

        if node.is_text() {
            // TODO
            let dimensions = Default::default();

            let parent_display = parent.expect("text node cannot be the root node").computed_style().display();
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

        Some(match computed_style.display() {
            // `display: inline`
            CssDisplay::InlineFlow => {
                let dimensions = self.calculate_dimensions_for_inline_flow(&computed_style, parent);
                LayoutBox::new(LayoutBoxKind::AnonymousInline, node, computed_style, dimensions)
            }

            CssDisplay::BlockFlow => {
                let dimensions = self.calculate_dimensions_for_block_flow(&computed_style, parent);
                LayoutBox::new(LayoutBoxKind::Inline, node, computed_style, dimensions)
            }

            _ => {
                println!(
                    "[layout] Warning: element was omitted because of an unknown `display` value: {:?}",
                    computed_style.display()
                );
                return None;
            }
        })
    }

}
