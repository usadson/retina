// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use retina_gfx::euclid::default::{Point2D, Size2D};

#[derive(Copy, Clone, Debug, Default)]
pub struct Scroller {
    position: Point2D<f64>,
    viewport_size: Size2D<f64>,
    content_size: Size2D<f64>,
}

impl Scroller {
    pub fn new(viewport_size: Size2D<f64>) -> Self {
        Self {
            viewport_size,
            ..Default::default()
        }
    }

    fn adjust_position_if_needed(&mut self) {
        self.position.x = self.position.x.max(0.0);
        self.position.y = self.position.y.max(0.0);

        if self.viewport_size.height >= self.content_size.height {
            return;
        }

        if self.position.y + self.viewport_size.height > self.content_size.height {
            self.position.y = self.content_size.height - self.viewport_size.height;
        }
    }

    pub fn did_content_resize(&mut self, size: Size2D<f64>) {
        self.content_size = size;
        self.adjust_position_if_needed();
    }

    pub fn did_viewport_resize(&mut self, size: Size2D<f64>) {
        self.viewport_size = size;
        self.adjust_position_if_needed();
    }

    pub fn page_down(&mut self) -> ScrollResult {
        self.with_result(|scroller| {
            scroller.position.y += scroller.viewport_size.height;
        })
    }

    pub fn page_up(&mut self) -> ScrollResult {
        self.with_result(|scroller| {
            scroller.position.y -= scroller.viewport_size.height;
        })
    }

    pub fn scroll_lines(&mut self, x: f32, y: f32, font_size: f64) -> ScrollResult {
        self.scroll_pixels(
            x as f64 * font_size,
            y as f64 * font_size,
        )
    }

    pub fn scroll_pixels(&mut self, x: f64, y: f64) -> ScrollResult {
        self.with_result(|scroller| {
            scroller.position.x -= x;
            scroller.position.y -= y;
        })
    }

    pub fn scroll_to_bottom(&mut self) -> ScrollResult {
        if self.content_size.height <= self.viewport_size.height {
            return ScrollResult::Unchanged;
        }

        self.with_result(|scroller| {
            scroller.position.y = scroller.content_size.height - scroller.viewport_size.height;
        })
    }

    pub fn scroll_to_top(&mut self) -> ScrollResult {
        self.with_result(|scroller| scroller.position.y = 0.0)
    }

    pub const fn viewport_position(&self) -> Point2D<f64> {
        self.position
    }

    fn with_result<F>(&mut self, f: F) -> ScrollResult
            where F: FnOnce(&mut Self) {
        let original_position = self.position;

        f(self);

        self.adjust_position_if_needed();

        if original_position == self.position {
            ScrollResult::Unchanged
        } else {
            ScrollResult::Changed
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ScrollResult {
    Unchanged,
    Changed,
}

impl ScrollResult {
    #[inline]
    pub const fn was_changed(&self) -> bool {
        matches!(self, Self::Changed)
    }
}
