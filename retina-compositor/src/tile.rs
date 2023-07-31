// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use retina_gfx::{
    canvas::CanvasPaintingContext,
    Context,
    euclid::{Rect, Size2D, Point2D}, Color, SubmissionFuture,
};
use retina_layout::LayoutBox;
use tracing::instrument;

use crate::painting::PaintInvoker;

pub struct TileSpace;

pub const TILE_SIZE: Size2D<u32, TileSpace> = Size2D::new(256, 256);

#[inline]
pub fn tile_rect_by_coordinate<T>(x: T, y: T) -> Rect<u32, TileSpace>
        where T: Into<u32> {
    let x = x.into();
    let y = y.into();
    let point = Point2D::new(x * TILE_SIZE.width, y * TILE_SIZE.height);
    Rect::new(point, TILE_SIZE)
}

#[derive(Debug)]
pub struct Tile {
    pub(crate) canvas: CanvasPaintingContext,
    pub(crate) rect: Rect<u32, TileSpace>,
    pub(crate) submission_future: Option<SubmissionFuture>,

    /// The dirty flag signifies whether or not this tile should be repainted
    /// or not. It is initially `true`, since the initial pixels are fully
    /// transparent, but can be reset to `true` when a whole page
    /// repaint is requested.
    ///
    /// When accounting for new tiles (e.g. because the viewport has increased)
    /// this default value of `true` is useful for resizing a vector using
    /// `Vec::resize_with`, and you don't have to loop the tile table again.
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

        let mut painter = self.canvas.begin(Color::WHITE, self.rect.origin.cast().cast_unit());

        let invoker = PaintInvoker::new();
        invoker.paint(layout_box, &mut painter);

        self.submission_future = Some(painter.submit_async());
        self.dirty = false;
    }
}
