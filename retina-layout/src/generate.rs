// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use retina_style::{Stylesheet, CssDisplay};
use retina_style_computation::{PropertyMap, StyleCollector, Cascade};

use crate::{
    DomNode,
    LayoutBox, LayoutBoxKind,
};

pub struct LayoutGenerator<'stylesheets> {
    stylesheets: &'stylesheets [Stylesheet],
}

impl<'stylesheets> LayoutGenerator<'stylesheets> {

    pub fn generate(
        root: DomNode,
        stylesheets: &'stylesheets [Stylesheet],
    ) -> LayoutBox {
        let instance = Self {
            stylesheets,
        };

        instance.generate_for(root, None)
            .expect("root node has no layout box generated")
    }

    fn resolve_style(&self, node: &DomNode) -> PropertyMap {
        StyleCollector::new(self.stylesheets)
            .collect(node.as_ref())
            .cascade()
    }

    fn generate_for(
            &self,
            node: DomNode,
            parent: Option<&LayoutBox>,
    ) -> Option<LayoutBox> {
        let computed_style = self.resolve_style(&node);

        if node.is_text() {
            let parent_display = parent.expect("text node cannot be the root node").computed_style().display();
            return match parent_display {
                CssDisplay::BlockFlow => Some(
                    LayoutBox::new(LayoutBoxKind::AnonymousBlock, node, computed_style)
                ),

                CssDisplay::InlineFlow => Some(
                    LayoutBox::new(LayoutBoxKind::AnonymousInline, node, computed_style)
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
                LayoutBox::new(LayoutBoxKind::AnonymousInline, node, computed_style)
            }

            CssDisplay::BlockFlow => {
                LayoutBox::new(LayoutBoxKind::Inline, node, computed_style)
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
