// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use super::{
    IResult,
    SvgPathEllipticArcArgumentSequence,
    SvgPathEllipticArcArgument,
    parse_coordinate::{
        parse_comma_wsp,
        parse_coordinate_pair, parse_coordinate,
    },
    parse_literal::parse_number,
};

use nom::{
    branch::alt,
    combinator::{opt, value},
    character::complete::char,
    multi::{many0, many1},
    sequence::{tuple, terminated},
};

pub fn parse_elliptic_arc_argument_sequence(input: &str) -> IResult<&str, SvgPathEllipticArcArgumentSequence> {
    let (input, vec) = many1(
        terminated(
            parse_elliptic_arc_argument,
            many0(parse_comma_wsp)
        )
    )(input)?;

    Ok((input, SvgPathEllipticArcArgumentSequence(vec)))
}

/// ```text
/// elliptical_arc_argument::=
///   number comma_wsp? number comma_wsp? number comma_wsp
///   flag comma_wsp? flag comma_wsp? coordinate_pair
/// ```
pub fn parse_elliptic_arc_argument(input: &str) -> IResult<&str, SvgPathEllipticArcArgument> {
    let (input, data) = tuple((
        // rx
        parse_number,
        opt(parse_comma_wsp),

        // ry
        parse_number,
        opt(parse_comma_wsp),

        // x-axis-rotation
        // parse_number,
        // The spec says it should be a `number`, but that cannot start
        // with a sign, even though the first example below contains a negative
        // x-coordinate :/
        //
        // number::= fractional-constant exponent?
        // fractional-constant::= (digit* "." digit+) | digit+
        // coordinate::= sign? number
        parse_coordinate,
        parse_comma_wsp,

        // large-arc-flag
        parse_flag,
        opt(parse_comma_wsp),

        // sweep-flag
        parse_flag,
        opt(parse_comma_wsp),

        // x y
        parse_coordinate_pair,
    ))(input)?;

    let (rx, _, ry, _, x_axis_rotation, _, large_arc_flag, _, sweep_flag, _, coords) = data;

    Ok((input, SvgPathEllipticArcArgument { rx, ry, x_axis_rotation, large_arc_flag, sweep_flag, coords, }))
}

pub fn parse_flag(input: &str) -> IResult<&str, bool> {
    alt((
        value(true, char('1')),
        value(false, char('0')),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use pretty_assertions::assert_eq;
    use crate::path::SvgPathCoordinatePair;

    #[rstest]
    #[case("25,25 -30 0,1 50,-25", Ok(("", SvgPathEllipticArcArgumentSequence(vec![
        SvgPathEllipticArcArgument{
            rx: 25.0,
            ry: 25.0,
            x_axis_rotation: -30.0,
            large_arc_flag: false,
            sweep_flag: true,
            coords: SvgPathCoordinatePair {
                x: 50.0,
                y: -25.0,
            }
        }
    ]))))]
    fn elliptic_arc_argument_sequence(#[case] input: &str, #[case] expected: IResult<&str, SvgPathEllipticArcArgumentSequence>) {
        assert_eq!(parse_elliptic_arc_argument_sequence(input), expected);
    }

    #[rstest]
    #[case("25,25 -30 0,1 50,-25", Ok(("", SvgPathEllipticArcArgument{
        rx: 25.0,
        ry: 25.0,
        x_axis_rotation: -30.0,
        large_arc_flag: false,
        sweep_flag: true,
        coords: SvgPathCoordinatePair {
            x: 50.0,
            y: -25.0,
        }
    })))]
    fn elliptic_arc_argument_one(#[case] input: &str, #[case] expected: IResult<&str, SvgPathEllipticArcArgument>) {
        assert_eq!(parse_elliptic_arc_argument(input), expected);
    }
}
