// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::sync::atomic::Ordering;

use atomic_float::AtomicF64;

static SCALE_FACTOR: AtomicF64 = AtomicF64::new(1.0);

// Get the scale factor of the screen.
pub fn scale_factor() -> f64 {
    // We can use relaxed ordering since the value is very unlikely to change,
    // and weird scaling issues can easily be resolved by a repaint.
    SCALE_FACTOR.load(Ordering::Relaxed)
}

pub fn set_scale_factor(value: f64) {
    SCALE_FACTOR.store(value, Ordering::Release);
}
