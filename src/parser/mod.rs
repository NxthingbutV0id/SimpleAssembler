use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use nom::branch::alt;
use nom::combinator::eof;
use nom::error::{context, convert_error};
use nom::Finish;
use nom::multi::many0;
use nom::sequence::{preceded, terminated};
use thiserror::Error;
use crate::architecture::batpu2::instruction::Instruction;
use crate::architecture::batpu2::opcode::Opcode;
use crate::parser::helpers::{skip, Res};
use crate::parser::wrappers::{parse_definitions, parse_instruction, parse_labels};

pub mod helpers;
pub mod tokens;
pub mod wrappers;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Invalid extension: {0}")]
    InvalidExtension(String),
    #[error("No instructions found in file: {0}")]
    NoInstructions(String),
    #[error("Failed to parse file: {file}.\n{reason}")]
    FailedToParse {
        file: String,
        reason: String
    },
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
}

pub fn parse(path: PathBuf) -> Result<Vec<Instruction>, ParseError> {
    info!("Parsing file: {}", path.display());
    if path.extension().unwrap() != "asm" ||
        path.extension().unwrap() != "as" ||
        path.extension().unwrap() != "s" {
        return Err(ParseError::InvalidExtension(path.display().to_string()));
    }

    match File::open(path.clone()) {
        Ok(mut file) => {
            let mut contents = String::new();
            match file.read_to_string(&mut contents) {
                Ok(b) => {
                    trace!("Read {} bytes from file", b);
                    match parse_program(&contents).finish() {
                        Ok((_, program)) => {
                            if program.is_empty() {
                                warn!("No instructions found in file");
                                return Ok(program);
                            }

                            for instruction in program.iter() {
                                if instruction.opcode !=  Opcode::_Label && instruction.opcode != Opcode::_Definition {
                                    return Ok(program);
                                }
                            }
                            Err(ParseError::NoInstructions(path.display().to_string()))
                        },
                        Err(e) => {
                            Err(ParseError::FailedToParse {
                                file: path.display().to_string(),
                                reason: convert_error(contents.as_str(), e)
                            })
                        }
                    }
                },
                Err(e) => {
                    Err(ParseError::from(e))
                }
            }
        },
        Err(e) => {
            Err(ParseError::from(e))
        }
    }
}

fn parse_program(input: &str) -> Res<&str, Vec<Instruction>> {
    context(
        "Program",
        terminated(
            many0(
                preceded(
                    skip,
                    alt((
                        parse_labels,
                        parse_definitions,
                        parse_instruction
                    ))
                )
            ),
            preceded(skip, eof)
        )
    )(input)
}