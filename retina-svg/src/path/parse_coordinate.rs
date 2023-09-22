// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use nom::{
    branch::alt,
    combinator::{opt, value},
    character::complete::{char, one_of},
    multi::{many0, many1},
    sequence::{tuple, pair, terminated},
};

use super::{
    IResult,
    SvgNumber,
    SvgPathCoordinatePair,
    SvgPathCoordinatePairDouble,
    SvgPathCoordinatePairDoubleSequence,
    SvgPathCoordinatePairSequence,
    SvgPathCoordinatePairTriplet,
    SvgPathCoordinatePairTripletSequence,
    SvgPathCoordinateSequence,
    parse_literal::parse_number,
};

/// Parse a coordinate (a number with an optional sign).
pub fn parse_coordinate(input: &str) -> IResult<&str, SvgNumber> {
    let (input, (sign, number)) = pair(
        opt(one_of("+-")),
        parse_number
    )(input)?;

    if sign == Some('-') {
        Ok((input, -number))
    } else {
        Ok((input, number))
    }
}

/// Parse a [coordinate pair](SvgPathCoordinatePair), i.e. an x and y
/// coordinate.
pub fn parse_coordinate_pair(input: &str) -> IResult<&str, SvgPathCoordinatePair> {
    let (input, (x, _, y)) = tuple((
        parse_coordinate,
        opt(parse_comma_wsp),
        parse_coordinate
    ))(input)?;

    Ok((input, SvgPathCoordinatePair { x, y, }))
}

pub fn parse_coordinate_sequence(input: &str) -> IResult<&str, SvgPathCoordinateSequence> {
    let (input, vec) = many1(
        terminated(
            parse_coordinate,
            many0(parse_comma_wsp)
        )
    )(input)?;

    Ok((input, SvgPathCoordinateSequence(vec)))
}

pub fn parse_coordinate_pair_sequence(input: &str) -> IResult<&str, SvgPathCoordinatePairSequence> {
    let (input, vec) = many1(
        terminated(
            parse_coordinate_pair,
            many0(parse_comma_wsp)
        )
    )(input)?;

    Ok((input, SvgPathCoordinatePairSequence(vec)))
}

pub fn parse_coordinate_pair_triplet(input: &str) -> IResult<&str, SvgPathCoordinatePairTriplet> {
    let (input, (a, _, b, _, c)) = tuple((
        parse_coordinate_pair,
        opt(parse_comma_wsp),
        parse_coordinate_pair,
        opt(parse_comma_wsp),
        parse_coordinate_pair,
    ))(input)?;

    Ok((input, SvgPathCoordinatePairTriplet { a, b, c  }))
}

pub fn parse_coordinate_pair_triplet_sequence(input: &str) -> IResult<&str, SvgPathCoordinatePairTripletSequence> {
    let (input, vec) = many1(
        terminated(
            parse_coordinate_pair_triplet,
            many0(parse_comma_wsp)
        )
    )(input)?;

    Ok((input, SvgPathCoordinatePairTripletSequence(vec)))
}

pub fn parse_coordinate_pair_double(input: &str) -> IResult<&str, SvgPathCoordinatePairDouble> {
    let (input, (a, _, b)) = tuple((
        parse_coordinate_pair,
        opt(parse_comma_wsp),
        parse_coordinate_pair,
    ))(input)?;

    Ok((input, SvgPathCoordinatePairDouble { a, b  }))
}

pub fn parse_coordinate_pair_double_sequence(input: &str) -> IResult<&str, SvgPathCoordinatePairDoubleSequence> {
    let (input, vec) = many1(
        terminated(
            parse_coordinate_pair_double,
            many0(parse_comma_wsp)
        )
    )(input)?;

    Ok((input, SvgPathCoordinatePairDoubleSequence(vec)))
}

/// Parse a comma or whitespace separator. There must be:
/// - Exactly one comma __`,`__ character with optional whitespace,
/// - Or at least one whitespace character
pub fn parse_comma_wsp(input: &str) -> IResult<&str, ()> {
    alt((
        value(
            (),
            tuple((
                many1(parse_wsp),
                opt(char(',')),
                many0(parse_wsp),
            ))
        ),
        value(
            (),
            tuple((
                char(','),
                many0(parse_wsp),
            ))
        )
    ))(input)
}

/// Parse exactly one of the whitespace characters.
///
/// ```text
/// wsp::= (#x9 | #x20 | #xA | #xC | #xD)
/// U+0009 CHARACTER TABULATION
/// U+0020 SPACE
/// U+000A LINE FEED
/// U+000C FORM FEED
/// U+000D CARRIAGE RETURN
/// ```
pub fn parse_wsp(input: &str) -> IResult<&str, char> {
    one_of("\t \n\u{000C}\r")(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use pretty_assertions::assert_eq;

    #[rstest]
    #[case("480-120", Ok(("", SvgPathCoordinatePair { x: 480.0, y: -120.0 })))]
    fn coordinate_pair(#[case] input: &str, #[case] expected: IResult<&str, SvgPathCoordinatePair>) {
        assert_eq!(parse_coordinate_pair(input), expected);
    }

    #[rstest]
    #[case("480-120-58-52", Ok(("", SvgPathCoordinatePairSequence(vec![
        SvgPathCoordinatePair { x: 480.0, y: -120.0 },
        SvgPathCoordinatePair { x: -58.0, y: -52.0 }
    ]))))]
    fn coordinate_pair_sequence(#[case] input: &str, #[case] expected: IResult<&str, SvgPathCoordinatePairSequence>) {
        assert_eq!(parse_coordinate_pair_sequence(input), expected);
    }
}
