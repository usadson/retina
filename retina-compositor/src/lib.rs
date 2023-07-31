// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod painting;
mod tile;

use std::{
    sync::{
        mpsc::{
            channel,
            RecvTimeoutError,
        },
        Mutex,
    },
    time::{
        Duration,
        Instant,
    },
};

use retina_gfx::{
    Context,
    euclid::Rect,
    Painter,
};
use retina_layout::LayoutBox;
use tracing::instrument;

use self::tile::{
    Tile,
    TileSpace,
    tile_rect_by_coordinate,
    TILE_SIZE,
};

/// The compositor is responsible for painting the page and putting it all into
/// one texture. The term 'compositing' comes from the idea that the page is
/// split into square sections (_tiles_) that need to be put together to form
/// a single picture.
#[derive(Debug)]
pub struct Compositor {
    context: Context,

    /// The tiles are stored in a vector of rows and then columns, meaning the
    /// indexing order is __y__ first, and the __x__. They are also wrapped in
    /// a [`Mutex`] to allow for parallelization.
    tiles: Vec<Vec<Mutex<Tile>>>,

    /// This field has the same layout as [`tiles`](Compositor::tiles), and
    /// contains the cached versions of each tile.
    tile_textures: Vec<Vec<wgpu::TextureView>>,
}

impl Compositor {
    /// Creates a new [`Compositor`]. There really should only be one compositor
    /// per page, since it's job is also to be efficient and aggressively cache
    /// things.
    pub fn new(context: Context) -> Self {
        Self {
            context,
            tiles: Vec::new(),
            tile_textures: Vec::new(),
        }
    }

    /// See the [documentation for this structure][Compositor].
    #[instrument(skip_all)]
    pub async fn composite<Callback>(
        &mut self,
        layout_box: &LayoutBox,
        painter: &mut Painter<'_>,
        upload_image_callback: Callback,
    )
            where Callback: Fn(&mut Painter<'_>) + Send + Sync {
        let viewport = painter.viewport_rect().cast();
        let vertical_tiles = divide_and_round_up(viewport.max_y() as _, TILE_SIZE.height);
        let horizontal_tiles = divide_and_round_up(viewport.max_x(), TILE_SIZE.width);

        if self.tiles.len() < vertical_tiles as _ {
            self.tiles.resize_with(vertical_tiles as _, || Vec::with_capacity(horizontal_tiles as _));
            self.tile_textures.resize_with(vertical_tiles as _, || Vec::with_capacity(horizontal_tiles as _));
        }

        let begin = Instant::now();
        log::trace!("Preparing tiles: {vertical_tiles} x {horizontal_tiles}");
        for y in 0..vertical_tiles {
            let row = &mut self.tiles[y as usize];

            for x in 0..horizontal_tiles {
                if row.len() == x as usize {
                    let rect = tile_rect_by_coordinate(x, y);
                    let tile = Tile::new(self.context.clone(), rect);
                    self.tile_textures[y as usize].push(tile.canvas.create_view());
                    row.push(Mutex::new(tile));
                }
            }
        }

        log::trace!("Tiles prepared in {} ms", begin.elapsed().as_millis());
        log::trace!("Initiating paint...");

        let viewport_tile_vertical_range = (viewport.min_y() / TILE_SIZE.height)..divide_and_round_up(viewport.max_y(), TILE_SIZE.height);
        let viewport_tile_horizontal_range = (viewport.min_x() / TILE_SIZE.width)..divide_and_round_up(viewport.max_x(), TILE_SIZE.width);

        let (sender, receiver) = channel();

        let mut tile_textures = std::mem::take(&mut self.tile_textures);
        let tile_textures_ref = &mut tile_textures;

        let begin = Instant::now();
        crossbeam::thread::scope(|s| {
            let viewport_tile_vertical_range2 = viewport_tile_vertical_range.clone();
            let viewport_tile_horizontal_range2 = viewport_tile_horizontal_range.clone();
            s.spawn(move |_| {
                for y in viewport_tile_vertical_range2.clone() {
                    for x in viewport_tile_horizontal_range2.clone() {
                        let rect = tile_rect_by_coordinate(x, y).cast();
                        painter.paint_rect_textured(rect, &tile_textures_ref[y as usize][x as usize]);
                    }
                }
                let mut submission = Some(painter.submit_async_concurrently());

                let mut has_new_images = false;

                let loop_time = Instant::now();
                loop {
                    match receiver.recv_timeout(Duration::from_millis(50)) {
                        Ok((view, rect, y, x)) => {
                            let rect: Rect<u32, TileSpace> = rect;
                            painter.paint_rect_textured(rect.cast(), &view);
                            tile_textures_ref[y as usize][x as usize] = view;
                            has_new_images = true;
                        }

                        Err(RecvTimeoutError::Timeout) => {
                            if let Some(submission) = submission.take() {
                                submission.wait();
                                upload_image_callback(painter);
                            }

                            if has_new_images {
                                submission = Some(painter.submit_async_concurrently());
                                has_new_images = false;
                            } else {
                                log::trace!("Still waiting...");
                            }
                        }

                        Err(RecvTimeoutError::Disconnected) => {
                            if let Some(submission) = submission.take() {
                                submission.wait();
                                upload_image_callback(painter);
                            }

                            log::info!("Compositor done in {} ms, looped for {} ms",
                                begin.elapsed().as_millis(),
                                loop_time.elapsed().as_millis());
                            return;
                        }
                    }
                }
            });

            for y in viewport_tile_vertical_range.clone() {
                for x in viewport_tile_horizontal_range.clone() {
                    let tile = &self.tiles[y as usize][x as usize];
                    let sender = sender.clone();
                    s.spawn(move |_| {
                        let wait = begin.elapsed().as_millis();

                        let mut tile = tile.lock().unwrap();
                        if !tile.dirty {
                            log::trace!("        Tile {y} x {x} cached {} ms (waited {wait} ms)", begin.elapsed().as_millis());

                            // It isn't necessary to send the tile surface to
                            // the compositor thread, since the compositor can
                            // use the cached version in
                            // `Compositor::tile_texture_views`.
                            return;
                        }

                        tile.paint(layout_box);
                        log::trace!("        Tile {y} x {x} ready in {} ms (waited {wait} ms)", begin.elapsed().as_millis());

                        if let Some(submission_future) = tile.submission_future.take() {
                            submission_future.wait();
                            log::trace!("        Tile {y} x {x} finished in {} ms (waited {wait} ms)", begin.elapsed().as_millis());
                        }

                        _ = sender.send((tile.canvas.create_view(), tile.rect, y, x)).ok();
                    });
                }
                log::trace!("    Row {y} done in {} ms", begin.elapsed().as_millis());
            }
            drop(sender);
        }).unwrap();

        self.tile_textures = tile_textures;
    }

    /// Marking the tile cache as dirty ensures the compositor needs repaint and
    /// re-composite all of its tiles.
    pub fn mark_tile_cache_dirty(&mut self) {
        for row in &mut self.tiles {
            for tile in row {
                tile.lock().unwrap().dirty = true;
            }
        }
    }
}

/// The default integer division will inherently round down, so this function
/// can be used to round up, and has the advantage of avoiding floating point
/// arithmetic, and converting to and from float/integers.
#[inline]
const fn divide_and_round_up(lhs: u32, rhs: u32) -> u32 {
    (lhs + rhs - 1) / rhs
}
