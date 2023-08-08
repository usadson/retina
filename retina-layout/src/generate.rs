// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use euclid::default::Point2D;
use log::warn;
use retina_common::Color;
use retina_dom::{Node, ImageData};
use retina_fetch::{Fetch, Url};

use retina_gfx_font::{
    CapitalLetterMode,
    EastAsianGlyphForm,
    EastAsianGlyphWidth,
    FamilyName,
    FontDescriptor,
    FontHandle,
    FontProvider,
    FontWeight,
    LigatureMode,
    TextHintingOptions,
    TypographicPositionMode,
};

use retina_style::{
    Stylesheet,
    CssColor,
    CssDisplay,
    CssDisplayBox,
    CssDisplayInside,
    CssDisplayOutside,
    CssFontFamilyName,
    CssFontKerning,
    CssFontVariantEastAsian,
    CssFontVariantEastAsianValues,
    CssFontVariantEastAsianWidth,
    CssFontVariantCaps,
    CssFontVariantLigatures,
    CssFontVariantPosition,
    CssGenericFontFamilyName,
    CssImage,
    CssLength,
    CssLineStyle,
    CssReferencePixels,
};

use retina_style_computation::{
    Cascade,
    BorderProperties,
    PropertyMap,
    StyleCollector,
};

use crate::{
    ActualValueMap,
    DomNode,
    formatting_context::FormattingContextKind,
    LayoutBox,
    LayoutBoxDimensions,
    LayoutBoxKind,
    LayoutEdge,
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
        initial_containing_block.run_layout(None, None);

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
            text_hinting_options: self.convert_text_hinting_options(computed_style),
            text_color,
        }
    }

    fn convert_text_hinting_options(&self, computed_style: &PropertyMap) -> TextHintingOptions {
        let capitals = match computed_style.font_variant_caps.unwrap_or_default() {
            CssFontVariantCaps::Normal => CapitalLetterMode::Normal,
            CssFontVariantCaps::SmallCaps => CapitalLetterMode::SmallCaps,
            CssFontVariantCaps::AllSmallCaps => CapitalLetterMode::AllSmallCaps,
            CssFontVariantCaps::PetiteCaps => CapitalLetterMode::PetiteCaps,
            CssFontVariantCaps::AllPetiteCaps => CapitalLetterMode::AllPetiteCaps,
            CssFontVariantCaps::Unicase => CapitalLetterMode::Unicase,
            CssFontVariantCaps::TitlingCaps => CapitalLetterMode::TitlingCaps,
        };

        let east_asian_width;
        let east_asian_form;
        let ruby = match computed_style.font_variant_east_asian.unwrap_or_default() {
            CssFontVariantEastAsian::Normal => {
                east_asian_form = Default::default();
                east_asian_width = Default::default();
                false
            }
            CssFontVariantEastAsian::Specific { values, width, ruby } => {
                east_asian_form = match values {
                    CssFontVariantEastAsianValues::Normal => EastAsianGlyphForm::Normal,
                    CssFontVariantEastAsianValues::Jis78 => EastAsianGlyphForm::Jis78,
                    CssFontVariantEastAsianValues::Jis83 => EastAsianGlyphForm::Jis83,
                    CssFontVariantEastAsianValues::Jis90 => EastAsianGlyphForm::Jis90,
                    CssFontVariantEastAsianValues::Jis04 => EastAsianGlyphForm::Jis04,
                    CssFontVariantEastAsianValues::Simplified => EastAsianGlyphForm::Simplified,
                    CssFontVariantEastAsianValues::Traditional => EastAsianGlyphForm::Traditional,
                };

                east_asian_width = match width {
                    CssFontVariantEastAsianWidth::Normal => EastAsianGlyphWidth::Normal,
                    CssFontVariantEastAsianWidth::FullWidth => EastAsianGlyphWidth::FullWidth,
                    CssFontVariantEastAsianWidth::ProportionalWidth => EastAsianGlyphWidth::ProportionalWidth,
                };

                ruby
            }
        };

        let kerning = match computed_style.font_kerning.unwrap_or_default() {
            CssFontKerning::Auto => true,
            CssFontKerning::None => false,
            CssFontKerning::Normal => true,
        };

        let ligatures = match computed_style.font_variant_ligatures.unwrap_or_default() {
             CssFontVariantLigatures::None => LigatureMode::None,
             CssFontVariantLigatures::Normal => LigatureMode::Normal,
             CssFontVariantLigatures::Specific {
                common,
                discretionary,
                historical,
                contextual,
            } => LigatureMode::Specific {
                common,
                discretionary,
                historical,
                contextual,
            },
        };

        let typographic_position = match computed_style.font_variant_position.unwrap_or_default() {
            CssFontVariantPosition::Normal => TypographicPositionMode::Normal,
            CssFontVariantPosition::Sub => TypographicPositionMode::Subscript,
            CssFontVariantPosition::Super => TypographicPositionMode::Superscript,
        };

        TextHintingOptions {
            capitals,
            east_asian_form,
            east_asian_width,
            kerning,
            ligatures,
            ruby,
            typographic_position,
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
                    CssFontFamilyName::Name(name) => name.clone().into(),
                    CssFontFamilyName::Generic(generic) => match generic {
                        CssGenericFontFamilyName::Cursive => FamilyName::Cursive,
                        CssGenericFontFamilyName::Fantasy => FamilyName::Fantasy,
                        CssGenericFontFamilyName::Monospace => FamilyName::Monospace,
                        CssGenericFontFamilyName::SansSerif => FamilyName::SansSerif,
                        CssGenericFontFamilyName::Serif => FamilyName::Serif,
                        _ => continue,
                    }
                };

                let descriptor = FontDescriptor {
                    name,
                    weight: FontWeight::new(computed_style.font_weight() as _),
                };

                if let Some(font) = self.font_provider.get(descriptor) {
                    return font;
                }

                warn!("[font-family] Font not found: {font_family:#?}");
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
        let font_emoji = parent.font_emoji.clone();

        let mut layout_box = LayoutBox::new(
            LayoutBoxKind::Normal,
            FormattingContextKind::Inline,
            node.clone(),
            computed_style,
            actual_value_map,
            Default::default(),
            font,
            font_emoji,
            font_size,
        );

        if node.is_text() {
            layout_box.kind = LayoutBoxKind::Anonymous;
            layout_box.dimensions = self.calculate_dimensions_for_inline_flow(layout_box.computed_style(), parent, font_size);
            return Some(layout_box);
        }

        let mut layout_box = match layout_box.computed_style().display() {
            CssDisplay::Box(CssDisplayBox::None) => return None,

            // `display: inline`
            CssDisplay::Normal { inside: CssDisplayInside::Flow, outside: CssDisplayOutside::Inline, .. } |
            CssDisplay::Normal { inside: CssDisplayInside::FlowRoot, outside: CssDisplayOutside::Inline, .. } => {
                layout_box.dimensions = self.calculate_dimensions_for_inline_flow(layout_box.computed_style(), parent, font_size);
                layout_box.formatting_context = FormattingContextKind::Inline;
                layout_box
            }

            CssDisplay::Normal { inside: CssDisplayInside::Flow, outside: CssDisplayOutside::Block, .. } |
            CssDisplay::Normal { inside: CssDisplayInside::FlowRoot, outside: CssDisplayOutside::Block, .. } => {
                layout_box.dimensions = self.calculate_dimensions_for_block_flow(layout_box.computed_style(), parent, font_size);
                layout_box.formatting_context = FormattingContextKind::Block;
                layout_box
            }

            _ => {
                warn!(
                    "Element was omitted because of an unknown `display` value: {:?}",
                    layout_box.computed_style().display()
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
            name: retina_gfx_font::FamilyName::Serif,
            weight: FontWeight::REGULAR,
        }).expect("failed to load serif font");

        let font_emoji = self.font_provider.get(FontDescriptor {
            name: retina_gfx_font::FamilyName::Emoji,
            weight: FontWeight::REGULAR,
        });

        let font_size = self.resolve_length(default_reference_pixels, default_reference_pixels, computed_style.font_size(), &computed_style);

        let actual_value_map = ActualValueMap {
            text_color: Color::BLACK,
            background_color: Color::WHITE,
            text_hinting_options: TextHintingOptions::default(),
        };

        LayoutBox::new(
            LayoutBoxKind::Root,
            FormattingContextKind::Block,
            root,
            computed_style,
            actual_value_map,
            self.calculate_dimensions_for_initial_containing_block(),
            font,
            font_emoji,
            font_size
        )
    }

}
