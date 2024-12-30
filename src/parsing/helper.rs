use nom::branch::alt;
use nom::bytes::complete::{is_a, is_not, tag, take_until};
use nom::character::complete::{digit1, hex_digit1, multispace0, multispace1, oct_digit1, one_of};
use nom::combinator::{eof, map_res, recognize, value};
use nom::{character, IResult};
use nom::error::{ParseError, VerboseError};
use nom::multi::{many0, many1};
use nom::sequence::{delimited, pair, preceded, terminated, tuple};

fn eat_comments(input: &str) -> IResult<&str, (), VerboseError<&str>> {
    let c = || tag("//");
    let lisp = || tag(";");
    let python = || tag("#");
    let ada = || tag("--");

    value(
        (),
        alt((
            pair(alt((c(), lisp(), python(), ada())), is_not("\n\r")), // case 1: [comment start]...
            pair(alt((c(), lisp(), python(), ada())), is_a("\n\r")), // case 2: [comment start]\r\n
            pair(alt((c(), lisp(), python(), ada())), eof), // case 3: [comment start]EOF
        )
    ))(input)
}

fn eat_multiline_comments<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, (), E> {
    value(
        (),
        tuple((tag("/*"), take_until("*/"), tag("*/")))
    )(input)
}

fn eat_whitespace(input: &str) -> IResult<&str, (), VerboseError<&str>> {
    value((), multispace1)(input)
}

pub(crate) fn skip(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    value("", many0(alt((eat_comments, eat_multiline_comments, eat_whitespace))))(input)
}

pub fn as_i16(input: &str) -> IResult<&str, i16, VerboseError<&str>> {
    map_res(ws(number), |s: &str| s.parse::<i16>())(input)
}

pub fn as_u16(input: &str) -> IResult<&str, u16, VerboseError<&str>> {
    map_res(ws(number), |s: &str| s.parse::<u16>())(input)
}

pub fn decimal_as_u8(input: &str) -> IResult<&str, u8, VerboseError<&str>> {
    map_res(ws(decimal), |s: &str| s.parse::<u8>())(input)
}

pub fn number(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    alt((hexadecimal, octal, binary, decimal))(input)
}

pub(crate) fn hexadecimal(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    preceded(
        alt((tag("0x"), tag("0X"))),
        recognize(
            many1(
                terminated(hex_digit1, many0(character::complete::char('_')))
            )
        )
    )(input)
}

pub fn decimal(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    recognize(
        many1(
            terminated(digit1, many0(character::complete::char('_')))
        )
    )(input)
}

pub(crate) fn binary(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    preceded(
        alt((tag("0b"), tag("0B"))),
        recognize(
            many1(
                terminated(one_of("01"), many0(character::complete::char('_')))
            )
        )
    )(input)
}

pub(crate) fn octal(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    preceded(
        alt((tag("0o"), tag("0O"))),
        recognize(
            many1(
                terminated(oct_digit1, many0(character::complete::char('_')))
            )
        )
    )(input)
}

pub(crate) fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) ->
impl FnMut(&'a str) -> IResult<&'a str, O, E>
where F: Fn(&'a str) -> IResult<&'a str, O, E>,{ delimited(multispace0, inner, multispace0) }