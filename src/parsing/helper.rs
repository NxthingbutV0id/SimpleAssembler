use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, tag_no_case, take_until};
use nom::character::complete::{multispace0, multispace1, one_of};
use nom::combinator::{recognize, value};
use nom::{character, Compare, IResult, InputLength, InputTake};
use nom::error::{Error, ErrorKind, ParseError};
use nom::multi::{many0, many1};
use nom::sequence::{delimited, pair, preceded, terminated, tuple};

fn eat_comments(input: &str) -> IResult<&str, ()> {
    value((), pair(alt((tag("//"), tag(";"), tag("#"))), is_not("\n\r")))(input)
}

fn eat_multiline_comments(input: &str) -> IResult<&str, ()> {
    value(
        (),
        tuple((tag("/*"), take_until("*/"), tag("*/")))
    )(input)
}

fn eat_whitespace(input: &str) -> IResult<&str, ()> {
    value((), multispace1)(input)
}

pub(crate) fn skip(input: &str) -> IResult<&str, ()> {
    value((), many0(alt((eat_comments, eat_multiline_comments, eat_whitespace))))(input)
}

pub(crate) fn hexadecimal(input: &str) -> IResult<&str, &str> {
    preceded(
        alt((tag("0x"), tag("0X"))),
        recognize(
            many1(
                terminated(one_of("0123456789abcdefABCDEF"), many0(character::complete::char('_')))
            )
        )
    )(input)
}

pub fn decimal(input: &str) -> IResult<&str, &str> {
    recognize(
        many1(
            terminated(one_of("0123456789"), many0(character::complete::char('_')))
        )
    )(input)
}

pub(crate) fn binary(input: &str) -> IResult<&str, &str> {
    preceded(
        alt((tag("0b"), tag("0B"))),
        recognize(
            many1(
                terminated(one_of("01"), many0(character::complete::char('_')))
            )
        )
    )(input)
}

pub(crate) fn alternative<T>(input: T, alts: &[&'static str]) -> IResult<T, T>
where T: Copy + InputLength + InputTake + Compare<&'static str> {
    let mut last_err = None;

    for &alt in alts {
        match tag(alt)(input) {
            Ok(result) => return Ok(result),
            Err(error) => {
                last_err = Some(error);
            }
        }
    }
    Err(last_err.unwrap_or(nom::Err::Error(Error { input, code: ErrorKind::NonEmpty })))
}

pub(crate) fn alternative_no_case<T>(input: T, alts: &[&'static str]) -> IResult<T, T>
where T: Copy + InputLength + InputTake + Compare<&'static str> {
    let mut last_err = None;

    for &alt in alts {
        match tag_no_case(alt)(input) {
            Ok(result) => return Ok(result),
            Err(error) => {
                last_err = Some(error);
            }
        }
    }
    Err(last_err.unwrap_or(nom::Err::Error(Error { input, code: ErrorKind::NonEmpty })))
}

pub(crate) fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) ->
impl FnMut(&'a str) -> IResult<&'a str, O, E>
where F: Fn(&'a str) -> IResult<&'a str, O, E>,{ delimited(multispace0, inner, multispace0) }