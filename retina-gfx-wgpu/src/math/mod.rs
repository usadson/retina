// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use euclid::{
    default::{
        Transform3D,
        Vector3D,
    },
    Rect,
    Size2D,
};

pub fn project<VU, RU>(
    viewport: Size2D<f32, VU>,
    rect: Rect<f64, RU>,
) -> [[f32; 4]; 4] {
    let model = Transform3D::identity()
        .then_scale(rect.size.width as f32, rect.size.height as f32, 1.0)
        .then_translate(Vector3D::new(rect.origin.x as f32, viewport.height - rect.size.height as f32 - rect.origin.y as f32, 0.0));

    let projection = Transform3D::ortho(
        0.0,
        viewport.width as f32,
        0.0,
        viewport.height as f32,
        -1.0,
        1.0
    );

    model.then(&projection).to_arrays()
}
