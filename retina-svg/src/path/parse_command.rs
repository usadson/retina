// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use nom::{
    IResult,
    branch::alt,
    character::complete::char,
    combinator::{eof, value},
    multi::{many0, many1},
    sequence::tuple,
};

use super::{
    SvgPath,
    SvgPathCommand,
    SvgPathType,
    parse_coordinate::{
        parse_wsp,
        parse_coordinate_pair_double_sequence,
        parse_coordinate_pair_sequence,
        parse_coordinate_pair_triplet_sequence,
        parse_coordinate_sequence,
    },
};

pub fn parse_path(input: &str) -> IResult<&str, SvgPath> {
    let (input, (_, commands, _)) = tuple((
        many0(parse_wsp),
        many1(parse_draw_to_command),
        eof,
    ))(input)?;

    Ok((input, SvgPath {
        commands,
    }))
}

fn parse_draw_to_command(input: &str) -> IResult<&str, SvgPathCommand> {
    alt((
        parse_move_to,
        parse_close_path,
        parse_line_to,
        parse_horizontal_line_to,
        parse_vertical_line_to,
        parse_curve_to,
        parse_quadratic_bezier_curve_to,
        parse_smooth_quadratic_bezier_curve_to,
    ))(input)
}

fn parse_move_to(input: &str) -> IResult<&str, SvgPathCommand> {
    let (input, (ty, _, sequence)) = tuple((
        parse_path_type('M', 'm'),
        many0(parse_wsp),
        parse_coordinate_pair_sequence,
    ))(input)?;

    Ok((input, SvgPathCommand::MoveTo(ty, sequence)))
}

fn parse_close_path(input: &str) -> IResult<&str, SvgPathCommand> {
    value(
        SvgPathCommand::ClosePath,
        parse_path_type('Z', 'z')
    )(input)
}

fn parse_line_to(input: &str) -> IResult<&str, SvgPathCommand> {
    let (input, (ty, _, sequence)) = tuple((
        parse_path_type('L', 'l'),
        many0(parse_wsp),
        parse_coordinate_pair_sequence,
    ))(input)?;

    Ok((input, SvgPathCommand::LineTo(ty, sequence)))
}

fn parse_horizontal_line_to(input: &str) -> IResult<&str, SvgPathCommand> {
    let (input, (ty, _, sequence)) = tuple((
        parse_path_type('H', 'h'),
        many0(parse_wsp),
        parse_coordinate_sequence,
    ))(input)?;

    Ok((input, SvgPathCommand::HorizontalLineTo(ty, sequence)))
}

fn parse_vertical_line_to(input: &str) -> IResult<&str, SvgPathCommand> {
    let (input, (ty, _, sequence)) = tuple((
        parse_path_type('V', 'v'),
        many0(parse_wsp),
        parse_coordinate_sequence,
    ))(input)?;

    Ok((input, SvgPathCommand::VerticalLineTo(ty, sequence)))
}

fn parse_curve_to(input: &str) -> IResult<&str, SvgPathCommand> {
    let (input, (ty, _, sequence)) = tuple((
        parse_path_type('C', 'c'),
        many0(parse_wsp),
        parse_coordinate_pair_triplet_sequence,
    ))(input)?;

    Ok((input, SvgPathCommand::CurveTo(ty, sequence)))
}

fn parse_quadratic_bezier_curve_to(input: &str) -> IResult<&str, SvgPathCommand> {
    let (input, (ty, _, sequence)) = tuple((
        parse_path_type('Q', 'q'),
        many0(parse_wsp),
        parse_coordinate_pair_double_sequence,
    ))(input)?;

    Ok((input, SvgPathCommand::QuadraticBezierCurveTo(ty, sequence)))
}

fn parse_smooth_quadratic_bezier_curve_to(input: &str) -> IResult<&str, SvgPathCommand> {
    let (input, (ty, _, sequence)) = tuple((
        parse_path_type('T', 't'),
        many0(parse_wsp),
        parse_coordinate_pair_sequence,
    ))(input)?;

    Ok((input, SvgPathCommand::SmoothQuadraticBezierCurveTo(ty, sequence)))
}

fn parse_path_type(
    absolute: char,
    relative: char
) -> impl Fn(&str) -> IResult<&str, SvgPathType> {
    move |input: &str| alt((
        value(SvgPathType::Absolute, char(absolute)),
        value(SvgPathType::Relative, char(relative)),
    ))(input)
}

#[cfg(test)]
mod tests {
    use crate::path::{
        SvgPathCoordinatePair,
        SvgPathCoordinatePairDouble,
        SvgPathCoordinatePairDoubleSequence,
    };

    use super::*;
    use rstest::rstest;
    use pretty_assertions::assert_eq;

    #[rstest]
    #[case(
        "Q111-500 95.5-544",
        Ok((
            "",
            SvgPathCommand::QuadraticBezierCurveTo(
                SvgPathType::Absolute,
                SvgPathCoordinatePairDoubleSequence(vec![
                    SvgPathCoordinatePairDouble {
                        a: SvgPathCoordinatePair { x: 111.0, y: -500.0 },
                        b: SvgPathCoordinatePair { x: 95.5, y: -544.0 },
                    },
                ])
            )
        ))
    )]
    fn quadratic_bezier_curve_to(#[case] input: &str, #[case] expected: IResult<&str, SvgPathCommand>) {
        assert_eq!(parse_quadratic_bezier_curve_to(input), expected);
    }

}
