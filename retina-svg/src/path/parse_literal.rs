// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use nom::{
    branch::alt,
    character::complete::{digit0, char, one_of},
    combinator::{map, map_res, opt},
    sequence::{tuple, pair},
};

use super::{
    IResult,
    SvgNumber,
};

/// Parse a number without a sign, possible fractional and/or exponential.
pub fn parse_number(input: &str) -> IResult<&str, SvgNumber> {
    alt((
        map(
            pair(parse_fractional_constant, parse_exponent),
            |(fractional, exponent)| {
                fractional * (10.0f64.powf(exponent as SvgNumber))
            }
        ),
        parse_fractional_constant,
    ))(input)
}

/// Parse an integer exponent with an optional sign.
pub fn parse_exponent(input: &str) -> IResult<&str, isize> {
    let (input, (_, sign, exponent)) = tuple((
        one_of("eE"),
        opt(one_of("+-")),
        parse_usize1,
    ))(input)?;

    let exponent = exponent as isize;

    if sign == Some('-') {
        Ok((input, -exponent))
    } else {
        Ok((input, exponent))
    }
}

/// Parse a fractional number (without sign) with an optional decimal point.
pub fn parse_fractional_constant(input: &str) -> IResult<&str, SvgNumber> {
    alt((
        parse_fractional_constant_with_decimal,
        map(parse_usize1, |x| x as SvgNumber)
    ))(input)
}

/// Parse a fractional number (without sign) with a required decimal point.
pub fn parse_fractional_constant_with_decimal(input: &str) -> IResult<&str, SvgNumber> {
    let (input, (integer, _, fractional)) = tuple((
        parse_usize0,
        char('.'),
        digit0,
    ))(input)?;

    let (_, fractional_num) = parse_usize1(fractional)?;

    let fraction_places = fractional.len() as SvgNumber;
    let fraction_places = 10.0f64.powf(fraction_places);
    let fraction = (fractional_num as SvgNumber) / fraction_places;
    let number = integer as SvgNumber + fraction;

    Ok((input, number))
}

/// Parse a number without a sign or fractional component with at zero or more
/// digits. The empty string, or a string beginning with a non-digit character,
/// results in the value `0`.
pub fn parse_usize0(input: &str) -> IResult<&str, usize> {
    map_res(digit0, |s: &str| {
        if s.is_empty() {
            return Ok(0);
        }
        s.parse::<usize>()
    })(input)
}

/// Parse a number without a sign or fractional component with at least one
/// digit.
pub fn parse_usize1(input: &str) -> IResult<&str, usize> {
    map_res(
        digit0,
        |s: &str| s.parse::<usize>()
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use pretty_assertions::assert_eq;

    #[rstest]
    #[case["", Ok(("", 0))]]
    #[case["0", Ok(("", 0))]]
    #[case["0000000", Ok(("", 0))]]
    #[case["1", Ok(("", 1))]]
    #[case["1234141", Ok(("", 1234141))]]
    #[case["-1238", Ok(("-1238", 0))]]
    #[case["ABCdef-866", Ok(("ABCdef-866", 0))]]
    fn usize0(#[case] input: &str, #[case] expected: IResult<&str, usize>) {
        assert_eq!(parse_usize0(input), expected);
    }

    #[rstest]
    #[case(".0", Ok(("", 0.0)))]
    #[case("1.0", Ok(("", 1.0)))]
    #[case("2", Ok(("", 2.0)))]
    #[case("2.5", Ok(("", 2.5)))]
    #[case("123.456", Ok(("", 123.456)))]
    fn fractional_constant(#[case] input: &str, #[case] expected: IResult<&str, SvgNumber>) {
        assert_eq!(parse_fractional_constant(input), expected);
    }

    #[rstest]
    #[case(".0", Ok(("", 0.0)))]
    #[case("0.0e+0", Ok(("", 0.0)))]
    #[case("0.0e-0", Ok(("", 0.0)))]
    #[case("0.0e0", Ok(("", 0.0)))]
    #[case("5e0", Ok(("", 5.0)))]
    #[case("5e1", Ok(("", 50.0)))]
    fn number(#[case] input: &str, #[case] expected: IResult<&str, SvgNumber>) {
        assert_eq!(parse_number(input), expected);
    }
}
