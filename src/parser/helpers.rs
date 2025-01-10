use nom::{
    IResult,
    character::{
        is_alphabetic,
        complete::{
            hex_digit1,
            digit1,
            oct_digit1,
            one_of,
            char,
            multispace0,
            alpha1,
            alphanumeric1,
            multispace1,
            space0,
            space1
        }
    },
    branch::{
        alt
    },
    error::{
        VerboseError,
        ParseError
    },
    multi::{
        many0,
        many1,
        many_m_n,
        many0_count
    },
    sequence::{
        pair,
        preceded,
        terminated,
        delimited,
    },
    combinator::{
        map_res,
        value,
        peek,
        recognize,
        eof
    },
    bytes::complete::{
        is_not,
        take_until,
        tag,
    },
};
use num::Integer;

pub type Res<A, B> = IResult<A, B, VerboseError<A>>;

pub fn comment_start(i: &str) -> Res<&str, &str> {
    alt((
        tag("//"), // C
        tag(";"),  // Lisp
        tag("#"),  // Python
        tag("--"), // Ada
        tag("%")   // Matlab
    ))(i)
}

pub fn comment(i: &str) -> Res<&str, &str> {
    value(
        "",
        pair(
            comment_start,
            alt((
                is_not("\n\r"),
            ))
        )
    )(i)
}

pub fn weird_comment(i: &str) -> Res<&str, &str> {
    value(
        "",
        pair(
            comment_start,
            tag("*/")
        )
    )(i)
}

fn multiline_comment(input: &str) -> Res<&str, &str> {
    delimited(tag("/*"), take_until("*/"), tag("*/"))(input)
}

fn whitespace(input: &str) -> Res<&str, &str> {
    multispace1(input)
}

pub fn skip(input: &str) -> Res<&str, &str> {
    value("", many0(alt((comment, multiline_comment, whitespace, comment_start))))(input)
}

/// Grabs any variable name (Anything until whitespace). Does not allow numbers at the start
pub fn identifier(input: &str) -> Res<&str, &str> {
    recognize(
        pair(
            alt((alpha1, tag("_"), tag("~"))),
            many0_count(alt((alphanumeric1, tag("_"), tag("|"), tag("&"), tag("^"))))
        )
    )(input)
}

pub fn identifier_m_n(input: &str, n: usize, m: usize) -> Res<&str, &str> {
    recognize(
        pair(
            alt((alpha1, tag("_"))),
            many_m_n(n, m, alt((alphanumeric1, tag("_"))))
        )
    )(input)
}

pub fn number<T: Integer>(input: &str) -> Res<&str, T> {
    alt((binary, octal, hexadecimal, decimal))(input)
}

pub fn hexadecimal<T: Integer>(input: &str) -> Res<&str, T> {
    map_res(
        preceded(
            alt((tag("0x"), tag("0X"))),
            recognize(
                many1(
                    terminated(hex_digit1, many0(char('_')))
                )
            )
        ),
        |out: &str| T::from_str_radix(&str::replace(&out, "_", ""), 16)
    )(input)
}

pub fn decimal<T: Integer>(input: &str) -> Res<&str, T> {
    map_res(
        recognize(
            many1(
                terminated(digit1, many0(char('_')))
            )
        ),
        |out: &str| T::from_str_radix(&str::replace(&out, "_", ""), 10)
    )(input)
}

pub fn binary<T: Integer>(input: &str) -> Res<&str, T> {
    map_res(
        preceded(
            alt((tag("0b"), tag("0B"))),
            recognize(
                many1(
                    terminated(bin_digit1, many0(char('_')))
                )
            )
        ),
        |out: &str| T::from_str_radix(&str::replace(&out, "_", ""), 2)
    )(input)
}

pub fn octal<T: Integer>(input: &str) -> Res<&str, T> {
    map_res(
        preceded(
            alt((tag("0o"), tag("0O"))),
            recognize(
                many1(
                    terminated(oct_digit1, many0(char('_')))
                )
            )
        ),
        |out: &str| T::from_str_radix(&str::replace(&out, "_", ""), 8)
    )(input)
}

pub fn spaces0_nonl<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, (), E> {
    value(
        (),
        many0(one_of(" \t\x0c"))
    )(input)
}

pub fn spaces1_nonl<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, (), E> {
    value(
        (),
        many1(one_of(" \t\x0c"))
    )(input)
}

pub fn bin_digit1<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    recognize(many1(one_of("01")))(input)
}

pub fn is_alphabetic_char(i: char) -> bool {
    is_alphabetic(i as u8)
}

pub(crate) fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) ->
impl FnMut(&'a str) -> IResult<&'a str, O, E>
where F: Fn(&'a str) -> IResult<&'a str, O, E> { delimited(multispace0, inner, multispace0) }

pub(crate) fn trailing_ws<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) ->
impl FnMut(&'a str) -> IResult<&'a str, O, E>
where F: Fn(&'a str) -> IResult<&'a str, O, E> { terminated(inner, multispace0) }

pub(crate) fn leading_ws<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) ->
impl FnMut(&'a str) -> IResult<&'a str, O, E>
where F: Fn(&'a str) -> IResult<&'a str, O, E> { preceded(multispace0, inner) }

pub(crate) fn ws_nonl<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) ->
impl FnMut(&'a str) -> IResult<&'a str, O, E>
where F: Fn(&'a str) -> IResult<&'a str, O, E> { delimited(spaces0_nonl, inner, spaces0_nonl) }

pub(crate) fn trailing_ws_nonl<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) ->
impl FnMut(&'a str) -> IResult<&'a str, O, E>
where F: Fn(&'a str) -> IResult<&'a str, O, E> { terminated(inner, spaces0_nonl) }

pub(crate) fn leading_ws_nonl<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) ->
impl FnMut(&'a str) -> IResult<&'a str, O, E>
where F: Fn(&'a str) -> IResult<&'a str, O, E> { preceded(spaces0_nonl, inner) }

pub fn is_condition(i: char) -> bool {
    let symbols = ['=', '!', '>', '<'];
    is_alphabetic(i as u8) || symbols.contains(&i)
}

/// Move on to the next token if possible
pub fn next_token(input: &str) -> Res<&str, ()> {
    value(
        (),
        alt((
            preceded(tag(","), space0), // is there a comma? skip it and any spaces
            space1,                     // else skip at least one space
            comment,
            comment_start,
            eof
        ))
    )(input)
}

pub fn next_instruction(input: &str) -> Res<&str, ()> {
    value(
        (),
        alt((skip, eof))
    )(input)
}

/// Check if a string can be parsed into a number
pub fn is_number(input: &str) -> bool {
    let res = peek(number::<i128>)(input);
    if let Ok((_, _)) = res {
        true
    } else {
        false
    }
}