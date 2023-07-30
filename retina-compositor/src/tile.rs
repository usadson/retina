// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use retina_gfx::{
    canvas::CanvasPaintingContext,
    Context,
    euclid::{Rect, Size2D}, Color, SubmissionFuture,
};
use retina_layout::LayoutBox;
use tracing::instrument;

use crate::painting::PaintInvoker;

pub struct TileSpace;

pub const TILE_SIZE: Size2D<u32, TileSpace> = Size2D::new(256, 256);

#[derive(Debug)]
pub struct Tile {
    pub(crate) canvas: CanvasPaintingContext,
    pub(crate) rect: Rect<u32, TileSpace>,
    pub(crate) submission_future: Option<SubmissionFuture>,
    pub(crate) dirty: bool,
}

impl Tile {
    pub fn new(context: Context, rect: Rect<u32, TileSpace>) -> Self {
        Self {
            canvas: CanvasPaintingContext::new(context.clone(), "Compositor Tile", rect.size.cast_unit()),
            rect,
            submission_future: None,
            dirty: true,
        }
    }

    #[instrument(skip_all)]
    pub fn paint(&mut self, layout_box: &LayoutBox) {
        if !self.dirty {
            return;
        }

        let color = match (self.rect.origin.x / TILE_SIZE.width, self.rect.origin.y / TILE_SIZE.height) {
            (0, 0) => Color::RED,
            (0, 1) => Color::GREEN,
            (1, 0) => Color::BLUE,
            (1, 1) => Color::MAGENTA,
            _ => Color::rgb(0.2, 0.5, 0.4),
        };
        let mut painter = self.canvas.begin(color, self.rect.origin.cast().cast_unit());

        let invoker = PaintInvoker::new();
        invoker.paint(layout_box, &mut painter);

        self.submission_future = Some(painter.submit_async());
        self.dirty = false;
    }
}
