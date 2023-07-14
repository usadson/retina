// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

pub struct SvgUnit;

pub type SvgNumber = f64;

pub type Point2D = euclid::Point2D<SvgNumber, SvgUnit>;
pub type Rect = euclid::Rect<SvgNumber, SvgUnit>;
pub type Size2D = euclid::Size2D<SvgNumber, SvgUnit>;
