// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::borrow::Cow;

use euclid::default::Point2D;

#[derive(Debug)]
pub struct ContextMenu {
    position: Point2D<f32>,
    items: Vec<ContextMenuItem>,
}

impl ContextMenu {
    pub const fn new(position: Point2D<f32>) -> Self {
        Self {
            position,
            items: Vec::new(),
        }
    }

    pub fn position(&self) -> Point2D<f32> {
        self.position
    }

    pub fn items(&self) -> &[ContextMenuItem] {
        &self.items
    }

    pub(crate) fn into_items(self) -> Vec<ContextMenuItem> {
        self.items
    }

    pub fn add_item(&mut self, item: ContextMenuItem) {
        self.items.push(item);
    }

    pub fn with_item(mut self, item: ContextMenuItem) -> Self {
        self.add_item(item);
        self
    }
}

#[derive(Debug)]
pub struct ContextMenuItem {
    kind: ContextMenuItemKind,
}

impl ContextMenuItem {
    pub fn new<S>(title: S, action: Box<dyn Fn() + Send + Sync + 'static>) -> Self
            where S: Into<Cow<'static, str>> {
        Self {
            kind: ContextMenuItemKind::Text(ContextMenuItemBase {
                title: title.into(),
                action,
            }),
        }
    }

    pub const fn new_separator() -> Self {
        Self {
            kind: ContextMenuItemKind::Separator,
        }
    }

    pub(crate) fn kind(&self) -> &ContextMenuItemKind {
        &self.kind
    }
}

#[derive(Debug)]
pub(crate) enum ContextMenuItemKind {
    Separator,
    Text(ContextMenuItemBase),
}

pub(crate) struct ContextMenuItemBase {
    title: Cow<'static, str>,
    action: Box<dyn Fn() + Send + Sync + 'static>,
}

impl core::fmt::Debug for ContextMenuItemBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ContextMenuItemBase")
            .field("title", &self.title)
            .finish_non_exhaustive()
    }
}

impl ContextMenuItemBase {
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn invoke_action(&self) {
        (self.action)();
    }
}
