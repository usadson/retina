// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod error;
mod parse_command;
mod parse_coordinate;
mod parse_elliptic;
mod parse_literal;

pub use parse_command::parse_path;
pub use error::{IResult, PathError};

/// The type of number in our representation.
pub type SvgNumber = f64;

#[derive(Debug)]
pub struct SvgPath {
    pub commands: Vec<SvgPathCommand>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SvgPathCommand {
    MoveTo(SvgPathType, SvgPathCoordinatePairSequence),
    ClosePath,
    LineTo(SvgPathType, SvgPathCoordinatePairSequence),
    HorizontalLineTo(SvgPathType, SvgPathCoordinateSequence),
    VerticalLineTo(SvgPathType, SvgPathCoordinateSequence),
    CurveTo(SvgPathType, SvgPathCoordinatePairTripletSequence),
    SmoothCurveTo(SvgPathType, SvgPathCoordinatePairDoubleSequence),
    QuadraticBezierCurveTo(SvgPathType, SvgPathCoordinatePairDoubleSequence),
    SmoothQuadraticBezierCurveTo(SvgPathType, SvgPathCoordinatePairSequence),
    EllipticArc(SvgPathType, SvgPathEllipticArcArgumentSequence),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SvgPathType {
    Relative,
    Absolute,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SvgPathCoordinateSequence(pub Vec<SvgNumber>);

#[derive(Clone, Debug, PartialEq)]
pub struct SvgPathCoordinatePairSequence(pub Vec<SvgPathCoordinatePair>);

#[derive(Clone, Debug, PartialEq)]
pub struct SvgPathCoordinatePairDoubleSequence(pub Vec<SvgPathCoordinatePairDouble>);

#[derive(Clone, Debug, PartialEq)]
pub struct SvgPathCoordinatePairTripletSequence(pub Vec<SvgPathCoordinatePairTriplet>);

#[derive(Clone, Debug, PartialEq)]
pub struct SvgPathEllipticArcArgumentSequence(pub Vec<SvgPathEllipticArcArgument>);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SvgPathCoordinatePair {
    pub x: SvgNumber,
    pub y: SvgNumber,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SvgPathCoordinatePairDouble {
    pub a: SvgPathCoordinatePair,
    pub b: SvgPathCoordinatePair,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SvgPathCoordinatePairTriplet {
    pub a: SvgPathCoordinatePair,
    pub b: SvgPathCoordinatePair,
    pub c: SvgPathCoordinatePair,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SvgPathEllipticArcArgument {
    pub rx: SvgNumber,
    pub ry: SvgNumber,
    pub x_axis_rotation: SvgNumber,
    pub large_arc_flag: bool,
    pub sweep_flag: bool,
    pub coords: SvgPathCoordinatePair,
}
