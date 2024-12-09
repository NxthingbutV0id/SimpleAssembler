use std::str::FromStr;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while, is_not, take_until, take_while_m_n},
    character::complete::{alpha1, digit1, multispace0, space1, char, one_of},
    combinator::{map, opt, map_res, recognize, value},
    multi::{separated_list0, many0, many1},
    sequence::{pair, preceded, separated_pair, terminated, tuple},
    IResult,
};

pub fn line_comments(input: &str) -> IResult<&str, ()> {
    value(
        (), // Output is thrown away.
        pair(tag("//"), is_not("\n\r"))
    )(input)
}

pub fn multi_line_comment(input: &str) -> IResult<&str, ()> {
    value(
        (), // Output is thrown away.
        (
            tag("/*"),
            take_until("*/"),
            tag("*/")
        )
    )(input)
}

fn hexadecimal(input: &str) -> IResult<&str, &str> {
    preceded(
        alt((tag("0x"), tag("0X"))),
        recognize(
            many1(
                terminated(one_of("0123456789abcdefABCDEF"), many0(char('_')))
            )
        )
    )(input)
}

fn binary(input: &str) -> IResult<&str, &str> {
    preceded(
        alt((tag("0b"), tag("0B"))),
        recognize(
            many1(
                terminated(one_of("01"), many0(char('_')))
            )
        )
    )(input)
}

pub fn decimal(input: &str) -> IResult<&str, &str> {
    recognize(
        many1(
            terminated(one_of("0123456789"), many0(char('_')))
        )
    )(input)
}

pub fn parse_immediate(input: &str) -> IResult<&str, u8> {
    alt((
        map_res(hexadecimal, |&out| u8::from_str_radix(&out, 16)),
        map_res(binary, |&out| u8::from_str_radix(&out, 2)),
        map_res(decimal, |&out| u8::from_str(&out))
    ))(input)
}

pub fn parse_register(input: &str) -> IResult<&str, u8> {
    map_res(
        preceded(tag("r"), decimal),
        |&out| u8::from_str(&out)
    )(input)
}

pub fn parse_instruction(input: &str) -> IResult<&str, (&str, Vec<u8>)> {
    let (input, opcode) = terminated(alpha1, space1)(input)?;
    let (input, operands) = separated_list0(tag(", "), parse_register)(input)?;
    Ok((input, (opcode, operands)))
}