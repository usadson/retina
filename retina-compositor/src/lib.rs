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
    Painter, euclid::Rect,
};
use retina_layout::LayoutBox;
use tile::TileSpace;
use tracing::instrument;

use self::tile::{
    Tile,
    tile_rect_by_coordinate,
    TILE_SIZE,
};

#[derive(Debug)]
pub struct Compositor {
    context: Context,
    tiles: Vec<Vec<Mutex<Tile>>>,
    tile_textures: Vec<Vec<wgpu::TextureView>>,
}

impl Compositor {
    pub fn new(context: Context) -> Self {
        Self {
            context,
            tiles: Vec::new(),
            tile_textures: Vec::new(),
        }
    }

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

        self.tiles.resize_with(vertical_tiles as _, || Vec::with_capacity(horizontal_tiles as _));
        self.tile_textures.resize_with(vertical_tiles as _, || Vec::with_capacity(horizontal_tiles as _));

        let begin = Instant::now();
        log::info!("Preparing tiles: {vertical_tiles} x {horizontal_tiles}");
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

        log::info!("Tiles prepared in {} ms", begin.elapsed().as_millis());
        log::info!("Initiating paint...");

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
                painter.submit_async_concurrently().wait();
                upload_image_callback(painter);

                let mut has_new_images = false;

                loop {
                    match receiver.recv_timeout(Duration::from_millis(50)) {
                        Ok((view, rect, y, x)) => {
                            let rect: Rect<u32, TileSpace> = rect;
                            painter.paint_rect_textured(rect.cast(), &view);
                            tile_textures_ref[y as usize][x as usize] = view;
                            has_new_images = true;
                        }

                        Err(RecvTimeoutError::Timeout) => {
                            if has_new_images {
                                painter.submit_async_concurrently().wait();
                                upload_image_callback(painter);
                                has_new_images = false;
                            } else {
                                log::info!("Still waiting...");
                            }
                        }

                        Err(RecvTimeoutError::Disconnected) => {
                            log::info!("Compositor has no more tiles left!");
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
                            log::info!("        Tile {y} x {x} cached {} ms (waited {wait} ms)", begin.elapsed().as_millis());
                            // TODO: we don't really have to send the tile, no?
                            _ = sender.send((tile.canvas.create_view(), tile.rect, y, x)).ok();
                            return;
                        }

                        tile.paint(layout_box);
                        log::info!("        Tile {y} x {x} ready in {} ms (waited {wait} ms)", begin.elapsed().as_millis());

                        if let Some(submission_future) = tile.submission_future.take() {
                            submission_future.wait();
                            log::info!("        Tile {y} x {x} finished in {} ms (waited {wait} ms)", begin.elapsed().as_millis());
                        }

                        _ = sender.send((tile.canvas.create_view(), tile.rect, y, x)).ok();
                    });
                }
                log::info!("    Row {y} done in {} ms", begin.elapsed().as_millis());
            }
            drop(sender);
        }).unwrap();

        self.tile_textures = tile_textures;

        log::info!("Painted in {} ms", begin.elapsed().as_millis());
    }

    pub fn mark_tile_cache_dirty(&mut self) {
        for row in &mut self.tiles {
            for tile in row {
                tile.lock().unwrap().dirty = true;
            }
        }
    }
}

fn divide_and_round_up(lhs: u32, rhs: u32) -> u32 {
    (lhs + rhs - 1) / rhs
}
