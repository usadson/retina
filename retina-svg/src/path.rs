// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod parse_command;
mod parse_coordinate;
mod parse_literal;

pub use parse_command::parse_path;

/// The type of number in our representation.
pub type SvgNumber = f64;

#[derive(Debug)]
pub struct SvgPath {
    pub commands: Vec<SvgPathCommand>,
}

#[derive(Clone, Debug)]
pub enum SvgPathCommand {
    MoveTo(SvgPathType, SvgPathCoordinatePairSequence),
    ClosePath,
    LineTo(SvgPathType, SvgPathCoordinatePairSequence),
    HorizontalLineTo(SvgPathType, SvgPathCoordinateSequence),
    VerticalLineTo(SvgPathType, SvgPathCoordinateSequence),
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SvgPathCoordinatePair {
    pub x: SvgNumber,
    pub y: SvgNumber,
}
