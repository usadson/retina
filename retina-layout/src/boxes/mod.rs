// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

//! # References
//! * [CSS Box Model Module Level 3](https://www.w3.org/TR/css-box-3/)
//! * [CSS Level 2 Revision 2 (CSS 2.2) - Box Model](https://www.w3.org/TR/CSS22/box.html)

mod dimensions;
mod edge;
mod line;

use std::borrow::Cow;

pub use dimensions::LayoutBoxDimensions;
pub use edge::LayoutEdge;
use euclid::default::Size2D;
use log::warn;
use retina_common::{
    Color,
    DumpableNode,
    DynamicSizeOf,
    StrExt,
    StrTendril,
};
use retina_dom::{ImageData, HtmlElementKind};
use retina_gfx_font::{FontHandle, TextHintingOptions};
use retina_style::{CssDecimal, CssReferencePixels, CssLength, CssWhiteSpace};

use crate::{
    ActualValueMap,
    formatting_context::{
        BlockFormattingContext,
        FormattingContext,
        FormattingContextKind,
        FormattingContextWhitespaceState,
        InlineFormattingContext,
        inline::InlineFormattingContextState,
    },
    text::is_emoji,
};

pub use self::line::{
    LineBox,
    LineBoxFragment,
};

use super::DomNode;
use retina_style_computation::PropertyMap;

#[derive(Clone, Debug, PartialEq)]
pub struct LayoutBox {
    pub(crate) kind: LayoutBoxKind,
    pub(crate) formatting_context: FormattingContextKind,
    pub node: DomNode,
    pub(crate) computed_style: PropertyMap,
    pub(crate) actual_value_map: ActualValueMap,
    pub(crate) dimensions: LayoutBoxDimensions,
    pub(crate) children: Vec<LayoutBox>,
    pub(crate) font: FontHandle,
    pub(crate) font_emoji: Option<FontHandle>,
    pub(crate) font_size: CssReferencePixels,
    pub(crate) background_image: Option<ImageData>,
    pub(crate) line_box_fragments: Vec<LineBoxFragment>,
}

unsafe impl Sync for LayoutBox {}

impl LayoutBox {
    pub fn new(
        kind: LayoutBoxKind,
        formatting_context: FormattingContextKind,
        node: DomNode,
        computed_style: PropertyMap,
        actual_value_map: ActualValueMap,
        dimensions: LayoutBoxDimensions,
        font: FontHandle,
        font_emoji: Option<FontHandle>,
        font_size: CssReferencePixels,
    ) -> Self {
        Self {
            kind,
            formatting_context,
            node,
            computed_style,
            actual_value_map,
            dimensions,
            children: Vec::new(),
            font,
            font_emoji,
            font_size,
            background_image: None,
            line_box_fragments: Vec::new(),
        }
    }

    pub const fn actual_values(&self) -> &ActualValueMap {
        &self.actual_value_map
    }

    /// Get the background-color, applicable for root boxes.
    pub fn background_color_as_root(&self) -> Color {
        debug_assert_eq!(self.kind, LayoutBoxKind::Root);

        let color = self.actual_value_map.background_color;

        if color == Color::TRANSPARENT {
            return Color::WHITE;
        }

        color.with_alpha(1.0)
    }

    pub fn background_image(&self) -> Option<&ImageData> {
        self.background_image.as_ref()
    }

    pub fn children(&self) -> &[LayoutBox] {
        &self.children
    }

    pub fn computed_style(&self) -> &PropertyMap {
        &self.computed_style
    }

    pub fn dimensions(&self) -> LayoutBoxDimensions {
        self.dimensions
    }

    pub fn dimensions_mut(&mut self) -> &mut LayoutBoxDimensions {
        &mut self.dimensions
    }

    pub const fn font(&self) -> &FontHandle {
        &self.font
    }

    pub const fn font_size(&self) -> CssReferencePixels {
        self.font_size
    }

    #[inline]
    pub fn line_box_fragments(&self) -> &[LineBoxFragment] {
        &self.line_box_fragments
    }

    pub fn kind(&self) -> &LayoutBoxKind {
        &self.kind
    }

    pub fn kind_mut(&mut self) -> &mut LayoutBoxKind {
        &mut self.kind
    }

    #[inline]
    pub fn dump(&self) {
        DumpableNode::dump(self);
    }

    fn run_anonymous_layout(&mut self, parent: &mut FormattingContext) {
        let Some(text) = self.node.as_text() else {
            warn!("Anonymous layout with a non-Text DOM node: {:#?}", self.node);
            return;
        };

        let mut text = Cow::Borrowed(text.data_as_str());
        if self.computed_style.white_space().collapses() {
            text = crate::text::collapse_white_space(text, parent.whitespace_state);
        }

        if self.computed_style.white_space() == CssWhiteSpace::Pre {
            text = Cow::Owned(text.replace('\t', "    "));
        }

        let parent_node;
        let mut element = self.node.as_html_element_kind();
        if element.is_none() {
            parent_node = self.node.as_node().parent().and_then(|x| x.upgrade());
            element = parent_node.as_ref().and_then(|x| x.as_html_element_kind());
        }


        let language = element.and_then(|e| e.as_html_element().language());

        text = crate::text::transform(text, self.computed_style.text_transform.unwrap_or_default(), language);

        if text.ends_with(' ') {
            parent.whitespace_state = FormattingContextWhitespaceState::EndedWithWhitespace;
        } else {
            parent.whitespace_state = FormattingContextWhitespaceState::NoWhitespace;
        }

        let hinting_options = self.actual_value_map.text_hinting_options;
        let text = StrTendril::from(text.as_ref());

        self.run_anonymous_layout_algorithm(parent, text, hinting_options);

        self.run_anonymous_layout_calculate_size();
    }

    fn run_anonymous_layout_algorithm(&mut self, parent: &mut FormattingContext, text: StrTendril, hinting_options: TextHintingOptions) {
        self.line_box_fragments.clear();
        let honor_forced_line_breaks = self.computed_style.white_space() == CssWhiteSpace::Pre;

        let max_width = parent.max_width;

        let font_size = self.font_size().value() as f32;
        let mut fragment_begin_index: u32 = 0;
        let mut initial_begin_index: u32 = 0;
        let mut line_break_reason;

        use unicode_segmentation::UnicodeSegmentation;
        let mut was_last_word_emoji = false;
        for word in text.split_word_bounds() {
            if self.computed_style.white_space().collapses() && word.chars().all(char::is_whitespace) {
                initial_begin_index += word.len() as u32;
                continue;
            }
            let is_word_emoji = is_emoji(word);
            let was_last_word_emoji = std::mem::replace(&mut was_last_word_emoji, is_word_emoji);

            let font = if is_word_emoji {
                self.font_emoji.as_ref().unwrap_or(&self.font).clone()
            } else {
                self.font.clone()
            };

            let original_word = word;
            let mut word = word;
            if self.computed_style.white_space().collapses() {
                word = text.try_include_following_space(word).unwrap_or(word);
            }

            let is_forced_line_break = honor_forced_line_breaks && (word.contains('\n') || word.contains('\r'));

            if is_forced_line_break {
                let without_spaces = word.trim_matches(|c| c == '\n' || c == '\r');
                word = without_spaces;
            }

            let Some(fragment) = self.line_box_fragments.last_mut() else {
                let word_size = font.calculate_size(self.font_size().value() as _, word, hinting_options);
                if !honor_forced_line_breaks {
                    debug_assert!(fragment_begin_index == 0);
                }

                self.line_box_fragments.push(LineBoxFragment {
                    position: self.dimensions.content_position,
                    text: text.subtendril(initial_begin_index, word.len() as u32),
                    size: word_size.cast(),
                    font,
                });
                continue;
            };

            let mut new_fragment_text_length = word.as_end_ptr() as u32 - text.as_ptr() as u32 - fragment_begin_index;
            if let Some(after_word) = text.slice_after_substring(word) {
                if after_word.chars().nth(0).as_ref().is_some_and(char::is_ascii_whitespace) {
                    new_fragment_text_length += 1;
                }
            }

            let fragment_text = text.subtendril(
                fragment_begin_index,
                new_fragment_text_length,
            );

            let fragment_size = font.calculate_size(font_size, &fragment_text, hinting_options).cast();

            // Does this word already fit on the last fragment?
            let is_wrap_line_break = self.computed_style.white_space() != CssWhiteSpace::Pre
                && (is_word_emoji
                    || was_last_word_emoji
                    || max_width.is_some_and(|max_width| fragment_size.width < max_width.value())
                );

            if !is_forced_line_break && !is_wrap_line_break {
                fragment.size = fragment_size;
                fragment.text = fragment_text;
                continue;
            }

            if is_word_emoji || was_last_word_emoji {
                line_break_reason = LineBreakReason::Emoji;
            } else {
                line_break_reason = LineBreakReason::Normal;
            }

            // No, create a new line box
            let position = {
                let last_fragment = self.line_box_fragments.last().unwrap();
                let mut position = last_fragment.position;

                match line_break_reason {
                    LineBreakReason::Normal => {
                        position.y += last_fragment.size.height;
                    }
                    LineBreakReason::Emoji => {
                        position.x += last_fragment.size.width;
                    }
                }

                position
            };

            fragment_begin_index += self.line_box_fragments.last().unwrap().text.len32();
            if is_forced_line_break {
                // The current word is a forced line break, so we must skip
                // those characters to avoid rendering tofu.
                fragment_begin_index += original_word.len() as u32;
            }

            let text = match text.try_subtendril(fragment_begin_index, word.len() as u32)  {
                Ok(text) => text,
                Err(_) => match text.try_subtendril(fragment_begin_index, original_word.len() as u32) {
                    Ok(text) => {
                        warn!("Falling back to original_word for subtendrilling");
                        text
                    }
                    Err(e) => {
                        warn!("Failed to subtendril: {e:?}");
                        warn!("  text.len={}", text.len());
                        warn!("  begin_index={fragment_begin_index}");
                        warn!("  length={}", word.len());
                        warn!("  end={}", fragment_begin_index + word.len() as u32);
                        warn!("  with word=\"{word}\"");
                        warn!("  originally=\"{original_word}\"");
                        warn!("  full text=\"{text}\"");
                        warn!("  fragment text=\"{fragment_text}\"");
                        break;
                    }
                }
            };

            self.line_box_fragments.push(LineBoxFragment {
                position,
                text,
                size: font.calculate_size(font_size, word, hinting_options).cast(),
                font,
            });
        }
    }

    fn run_anonymous_layout_calculate_size(&mut self) {
        let min_x = self.line_box_fragments.iter()
            .map(|fragment| fragment.position.x)
            .reduce(CssDecimal::min)
            .unwrap_or_default();

        let max_x = self.line_box_fragments.iter()
            .map(|fragment| fragment.position.x + fragment.size.width)
            .reduce(CssDecimal::max)
            .unwrap_or_default();

        let min_y = self.line_box_fragments.first()
            .map(|fragment| fragment.position.y)
            .unwrap_or_default();

        let max_y = self.line_box_fragments.last()
            .map(|fragment| fragment.position.y + fragment.size.height)
            .unwrap_or_default();

        self.dimensions.width = CssReferencePixels::new(max_x - min_x);
        self.dimensions.height = CssReferencePixels::new(max_y - min_y);
    }

    pub fn run_layout(
        &mut self,
        parent: Option<&mut FormattingContext>,
        ifc_state: Option<&mut InlineFormattingContextState>,
    ) {
        if let LayoutBoxKind::Anonymous = self.kind {
            if let Some(parent) = parent {
                // TODO participate in the IFC state
                _ = ifc_state;

                self.run_anonymous_layout(parent);
            } else {
                warn!("Anonymous layout without a parent node.");
                if cfg!(debug_assertions) {
                    panic!("Anonyous layout without a parent node")
                }
            }

            return;
        }

        if parent.is_none() {
            self.dimensions = self.actual_value_map.dimensions;
        }

        if self.run_replaced_layout() {
            return;
        }

        let parent = parent.map(|parent| &*parent);

        match self.formatting_context {
            FormattingContextKind::Block => BlockFormattingContext::perform(self, parent),
            FormattingContextKind::Inline => {
                // TODO
                _ = parent;
                InlineFormattingContext::perform(self, parent)
            }
        }
    }

    fn run_replaced_layout(&mut self) -> bool {
        let Some(element) = self.node.as_html_element_kind() else {
            return false;
        };

        match element {
            HtmlElementKind::Img(img) => {
                let mut image_size = Size2D::default();
                if let Ok(image) = img.data().read().unwrap().image().read() {
                    image_size = Size2D::new(image.width(), image.height());
                }

                self.run_replaced_layout_for_image(image_size);
                true
            }

            _ => false,
        }
    }

    fn run_replaced_layout_for_image(&mut self, image_size: Size2D<u32>) {
        if let CssLength::Auto = self.computed_style.width() {
            self.dimensions.width = CssReferencePixels::new(image_size.width as _);
        }

        if let CssLength::Auto = self.computed_style.height() {
            self.dimensions.height = CssReferencePixels::new(image_size.height as _);
        }
    }
}

impl DumpableNode for LayoutBox {
    fn dump_to(&self, depth: usize, writer: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
        writeln!(
            writer,
            "{pad:pad_width$}LayoutBox({kind:?}, {display}), {dom:?}, {width}x{height} @ ({x}, {y})",
            pad = "",
            pad_width = depth * 4,
            kind = self.kind,
            width = self.dimensions.width().value(),
            height = self.dimensions.height().value(),
            dom = self.node.to_short_dumpable(),
            display = self.computed_style.display(),
            x = self.dimensions.position_padding_box().x,
            y = self.dimensions.position_padding_box().y,
        )?;

        for child in &self.children {
            child.dump_to(depth + 1, writer)?;
        }

        Ok(())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LayoutBoxKind {
    Root,
    Normal,
    Anonymous,
}

impl DynamicSizeOf for LayoutBox {
    fn dynamic_size_of(&self) -> usize {
        let mut size = std::mem::size_of_val(self);

        size += self.children.dynamic_size_of();
        size += self.line_box_fragments.dynamic_size_of();

        if let Some(image) = &self.background_image {
            size += image.dynamic_size_of();
        }

        size
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LineBreakReason {
    Normal,
    Emoji,
}
