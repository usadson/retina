// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::sync::mpsc::SyncSender;

use log::warn;
use retina_dom::Node;
use retina_gfx::{
    CursorIcon,
    MouseMoveEvent,
    euclid::Point2D,
    WinitCursorIcon,
};
use retina_layout::{LayoutBox, LayoutBoxKind};
use retina_style::CssCursor;
use tokio::sync::mpsc::Sender;
use url::Url;

use crate::{PageMessage, message::PageTaskMessage, PageCommand};

#[derive(Debug)]
pub(crate) struct CursorState {
    cursor: CursorIcon,
    page_message_sender: SyncSender<PageMessage>,
    task_sender: Sender<PageTaskMessage>,
    node: Option<Node>,
}

impl CursorState {
    pub(crate) fn new(
        page_message_sender: SyncSender<PageMessage>,
        task_sender: Sender<PageTaskMessage>,
    ) -> Self {
        Self {
            cursor: CursorIcon::Winit(WinitCursorIcon::Default),
            page_message_sender,
            task_sender,
            node: None,
        }
    }

    pub async fn click(&mut self, current_url: &Url) {
        let Some(mut node) = self.node.clone() else { return };

        loop {
            if let Some(element) = node.as_dom_element() {
                if element.qualified_name().local.as_ref().eq_ignore_ascii_case("a") {
                    if let Some(href) = element.attributes().find_by_str("href") {
                        match Url::options().base_url(Some(current_url)).parse(href) {
                            Ok(url) => {
                                _ = self.task_sender.send(PageTaskMessage::Command {
                                    command: PageCommand::OpenUrl(url.to_string()),
                                }).await.ok();
                            }
                            Err(e) => {
                                warn!("Invalid anchor hyper reference \"{href}\": {e}");
                            }
                        }
                    }

                    return;
                }
            }

            let Some(parent) = node.as_node().parent() else { break };
            let Some(parent) = parent.upgrade() else { break };
            node = Node::from(parent);
        }
    }

    pub async fn evaluate_move(
        &mut self,
        mouse_move_event: MouseMoveEvent,
        layout_root: Option<&LayoutBox>
    ) {
        let hit_stack = hit_test(mouse_move_event.to, layout_root);

        match hit_stack.last() {
            Some(layout_box) => {
                let cursor = layout_box.computed_style().cursor.unwrap_or_default();
                let cursor = convert_cursor_type(cursor, &layout_box);
                self.node = Some(layout_box.node.clone());
                self.set_cursor(cursor).await;
            }
            None => self.set_cursor(CursorIcon::Winit(WinitCursorIcon::Help)).await,
        }
    }

    async fn set_cursor(&mut self, cursor: CursorIcon) {
        if self.cursor == cursor {
            return;
        }

        self.cursor = cursor;
        _ = self.page_message_sender.send(PageMessage::CursorIcon(cursor)).ok();
    }
}

fn convert_cursor_type(cursor: CssCursor, layout_box: &LayoutBox) -> CursorIcon {
    let winit_cursor = match cursor {
        CssCursor::Auto => match layout_box.kind() {
            LayoutBoxKind::Anonymous => WinitCursorIcon::Text,
            _ => WinitCursorIcon::Default,
        },
        CssCursor::Default => WinitCursorIcon::Default,
        CssCursor::None => WinitCursorIcon::Default, // TODO
        CssCursor::ContextMenu => WinitCursorIcon::ContextMenu,
        CssCursor::Help => WinitCursorIcon::Help,
        CssCursor::Pointer => WinitCursorIcon::Hand, // is this correct?
        CssCursor::Progress => WinitCursorIcon::Progress,
        CssCursor::Wait => WinitCursorIcon::Wait,
        CssCursor::Cell => WinitCursorIcon::Cell,
        CssCursor::Crosshair => WinitCursorIcon::Crosshair,
        CssCursor::Text => WinitCursorIcon::Text,
        CssCursor::VerticalText => WinitCursorIcon::VerticalText,
        CssCursor::Alias => WinitCursorIcon::Alias,
        CssCursor::Copy => WinitCursorIcon::Copy,
        CssCursor::Move => WinitCursorIcon::Move,
        CssCursor::NoDrop => WinitCursorIcon::NoDrop,
        CssCursor::NotAllowed => WinitCursorIcon::NotAllowed,
        CssCursor::Grab => WinitCursorIcon::Grab,
        CssCursor::Grabbing => WinitCursorIcon::Grabbing,
        CssCursor::EResize => WinitCursorIcon::EResize,
        CssCursor::NResize => WinitCursorIcon::NResize,
        CssCursor::NeResize => WinitCursorIcon::NeResize,
        CssCursor::NwResize => WinitCursorIcon::NwResize,
        CssCursor::SResize => WinitCursorIcon::SResize,
        CssCursor::SeResize => WinitCursorIcon::SeResize,
        CssCursor::SwResize => WinitCursorIcon::SwResize,
        CssCursor::WResize => WinitCursorIcon::WResize,
        CssCursor::EwResize => WinitCursorIcon::EwResize,
        CssCursor::NsResize => WinitCursorIcon::NsResize,
        CssCursor::NeswResize => WinitCursorIcon::NeswResize,
        CssCursor::NwseResize => WinitCursorIcon::NwseResize,
        CssCursor::ColResize => WinitCursorIcon::ColResize,
        CssCursor::RowResize => WinitCursorIcon::RowResize,
        CssCursor::AllScroll => WinitCursorIcon::AllScroll,
        CssCursor::ZoomIn => WinitCursorIcon::ZoomIn,
        CssCursor::ZoomOut => WinitCursorIcon::ZoomOut,
    };

    CursorIcon::Winit(winit_cursor)
}

fn hit_test<U>(position: Point2D<f64, U>, layout_root: Option<&LayoutBox>) -> Vec<&LayoutBox> {
    _ = position;
    _ = layout_root;
    let mut hit_stack = Vec::new();

    match layout_root {
        Some(layout_box) => hit_test_impl(position, layout_box, &mut hit_stack),
        None => (),
    }

    hit_stack
}

fn hit_test_impl<'boxes, U>(
    position: Point2D<f64, U>,
    layout_box: &'boxes LayoutBox,
    hit_stack: &mut Vec<&'boxes LayoutBox>,
) {
    hit_stack.push(layout_box);

    for child in layout_box.children() {
        // <https://drafts.csswg.org/css-ui/#cursor>
        // This property specifies the type of cursor to be displayed for the
        // pointing device when the cursor’s hotspot is within the element’s
        // border edge.

        let border_edge = child.dimensions()
            .rect_border_box()
            .to_box2d();

        if border_edge.contains(position.cast_unit()) {
            hit_test_impl(position, child, hit_stack);
            return;
        }
    }
}
