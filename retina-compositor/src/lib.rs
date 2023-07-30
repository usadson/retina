// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod painting;
mod tile;

use std::{
    sync::Mutex,
    time::Instant,
};

use retina_gfx::{
    Context,
    euclid::{
        Point2D,
        Rect,
    },
    Painter,
};
use retina_layout::LayoutBox;
use tracing::instrument;

use self::tile::{
    Tile,
    TILE_SIZE,
};

#[derive(Debug)]
pub struct Compositor {
    context: Context,
    tiles: Vec<Vec<Mutex<Tile>>>,
}

impl Compositor {
    pub fn new(context: Context) -> Self {
        Self {
            context,
            tiles: Vec::new(),
        }
    }

    #[instrument(skip_all)]
    pub async fn paint(&mut self, layout_box: &LayoutBox, painter: &mut Painter<'_>) {
        let viewport = painter.viewport_rect().cast();
        let vertical_tiles = divide_and_round_up(viewport.max_y() as _, TILE_SIZE.height);
        let horizontal_tiles = divide_and_round_up(viewport.max_x(), TILE_SIZE.width);

        self.tiles.resize_with(vertical_tiles as _, || Vec::with_capacity(horizontal_tiles as _));

        let begin = Instant::now();
        log::info!("Preparing tiles: {vertical_tiles} x {horizontal_tiles}");
        for y in 0..vertical_tiles {
            let row = &mut self.tiles[y as usize];

            for x in 0..horizontal_tiles {
                if row.len() == x as usize {
                    let point = Point2D::new(x * TILE_SIZE.width, y * TILE_SIZE.height);
                    let rect = Rect::new(point, TILE_SIZE);
                    row.push(Mutex::new(Tile::new(self.context.clone(), rect)));
                }
            }
        }

        log::info!("Tiles prepared in {} ms", begin.elapsed().as_millis());
        log::info!("Initiating paint...");

        let viewport_tile_vertical_range = (viewport.min_y() / TILE_SIZE.height)..divide_and_round_up(viewport.max_y(), TILE_SIZE.height);
        let viewport_tile_horizontal_range = (viewport.min_x() / TILE_SIZE.width)..divide_and_round_up(viewport.max_x(), TILE_SIZE.width);

        let begin = Instant::now();
        crossbeam::thread::scope(|s| {
            for y in viewport_tile_vertical_range.clone() {
                for x in viewport_tile_horizontal_range.clone() {
                    let tile = &self.tiles[y as usize][x as usize];
                    s.spawn(move |_| {
                        let mut tile = tile.lock().unwrap();
                        tile.paint(layout_box);
                        log::info!("        Tile {y} x {x} ready in {} ms", begin.elapsed().as_millis());
                        if let Some(submission_future) = tile.submission_future.take() {
                            submission_future.wait();
                            log::info!("        Tile {y} x {x} finished in {} ms", begin.elapsed().as_millis());
                        }
                    });
                }
                log::info!("    Row {y} done in {} ms", begin.elapsed().as_millis());
            }
        }).unwrap();

        log::info!("Painted in {} ms", begin.elapsed().as_millis());
        log::info!("Compositing...");

        let begin = Instant::now();
        for y in viewport_tile_vertical_range {
            log::info!("    Row {y}...");
            for x in viewport_tile_horizontal_range.clone() {
                log::info!("        Tile {x}...");
                let tile = &self.tiles[y as usize][x as usize];
                let tile = tile.lock().unwrap();

                painter.paint_rect_textured(tile.rect.to_f64(), &tile.canvas.create_view());
                log::info!("        Tile {x} composited");
            }
            log::info!("    Row {y} done!");
        }

        log::info!("Compositor done ^_^ in {} ms", begin.elapsed().as_millis());
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
