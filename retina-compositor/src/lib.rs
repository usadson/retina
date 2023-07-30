// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod painting;
mod tile;

use std::sync::Mutex;

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

        log::info!("Tiles prepared!");
        log::info!("Initiating paint...");

        crossbeam::thread::scope(|s| {
            for y in 0..(vertical_tiles as usize) {
                for x in 0..(horizontal_tiles as usize) {
                    let tile = &self.tiles[y][x];
                    s.spawn(move |_| {
                        let mut tile = tile.lock().unwrap();
                        tile.paint(layout_box);
                        log::info!("        Tile {y} x {x} ready!");
                    });
                }
                log::info!("    Row {y} done...");
            }
        }).unwrap();

        log::info!("Paint initiated!");
        log::info!("Compositing...");

        for y in 0..vertical_tiles as usize {
            log::info!("    Row {y}...");
            for x in 0..horizontal_tiles as usize {
                log::info!("        Tile {x}...");
                let tile = &self.tiles[y][x];
                let mut tile = tile.lock().unwrap();
                if let Some(submission_future) = tile.submission_future.take() {
                    submission_future.await;
                    log::info!("        Tile {x} finished");
                }

                painter.paint_rect_textured(tile.rect.to_f64(), &tile.canvas.create_view());
                log::info!("        Tile {x} composited");
            }
            log::info!("    Row {y} done!");
        }

        log::info!("Compositor done ^_^");
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
