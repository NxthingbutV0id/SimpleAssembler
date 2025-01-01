use nom::branch::alt;
use nom::error::{context, convert_error, VerboseError};
use nom::{Finish, IResult};
use nom::combinator::cut;
use nom::multi::many0;
use nom::sequence::preceded;
use crate::parsing::complex::{parse_labels, parse_definitions, parse_instruction};
use crate::parsing::helper::skip;
use crate::symbols::instruction::Instruction;

pub fn parse_program(input: &str) -> IResult<&str, Vec<Instruction>, VerboseError<&str>> {
    let e: Result<(&str, Vec<Instruction>), VerboseError<&str>> = context(
        "Program",
        many0(
            preceded(
                skip,
                alt((
                    parse_labels, 
                    parse_definitions, 
                    cut(parse_instruction)
                ))
            )
        )
    )(input).finish();
    match e {
        Ok((rest, program)) => {
            debug!("Program parsed successfully");
            Ok((rest, program))
        },
        Err(e) => {
            error!("Failed to parse program: \n{}", convert_error(input, e.clone()));
            Err(nom::Err::Error(e))
        }
    }
}