use std::str::FromStr;
use log::error;
use nom::*;
use nom::{
    error::{Error, ErrorKind, context, convert_error, ParseError, VerboseError},
    branch::alt,
    multi::{ many0 },
    bytes::complete::{ tag, take_while1, tag_no_case, take_while_m_n },
    character::complete::one_of,
    combinator::{ opt, fail, map_res },
    sequence::{ preceded, delimited, terminated, pair },
};

use crate::parsing::{
    KEYWORDS,
    helper::*
};
use crate::symbols::{
    opcodes::*,
    opcodes::Opcode::*,
    operands::{Operand, Offset, Condition, Register},
    instruction::*,
    definitions::*
};

pub fn parse_program(input: &str) -> IResult<&str, Vec<Instruction>, VerboseError<&str>> {
    let e: Result<(&str, Vec<Instruction>), VerboseError<&str>> = context(
        "parsing program",
        many0(
            preceded(
                skip,
                alt((parse_labels, parse_definitions, parse_instruction))
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
            Err(Err::Error(e))
        }
    }
}

pub fn parse_labels(input: &str) -> IResult<&str, Instruction, VerboseError<&str>> {
    trace!("parse_labels current input: <{:?}>", input.chars().take(20).collect::<String>());
    let (rest, label) = opt(label)(input)?;
    if label.is_some() {
        Ok((rest, {
            trace!("Found label: {}", label.unwrap());
            let label = Operand::Label(label.unwrap().to_string());
            let mut temp = Instruction::new(_Label);
            temp.add_operand(label);
            temp
        }))
    } else {
        Err(Err::Error(VerboseError::from_error_kind(input, ErrorKind::Tag)))
    }
}

pub fn parse_definitions(input: &str) -> IResult<&str, Instruction, VerboseError<&str>> {
    trace!("parse_definitions current input: <{:?}>", input.chars().take(20).collect::<String>());
    let (rest, define) = opt(definition)(input)?;
    if define.is_some() {
        let define = define.unwrap();
        Ok((rest, {
            trace!("Found definition: {}", define);
            let mut temp = Instruction::new(_Definition);
            temp.add_operand(Operand::Definition(define));
            temp
        }))
    } else {
        Err(Err::Error(VerboseError::from_error_kind(input, ErrorKind::Tag)))
    }
}

pub fn parse_instruction(input: &str) -> IResult<&str, Instruction, VerboseError<&str>> {
    trace!("parse_instruction current input: <{:?}...>", input.chars().take(20).collect::<String>());
    let (rest, opcode) = opcode(input)?;
    match opcode {
        NOP | HLT | RET => {
            Ok((rest, Instruction::new(opcode)))
        }
        ADD | SUB | NOR | AND | XOR => {
            let (rest, a) = register(rest)?;
            let (rest, b) = register(rest)?;
            let (rest, c) = register(rest)?;
            Ok((rest, {
                let mut temp = Instruction::new(opcode);
                temp.add_operand(Operand::Register(a));
                temp.add_operand(Operand::Register(b));
                temp.add_operand(Operand::Register(c));
                temp
            }))
        },
        RSH | CMP | MOV | LSH | NOT | NEG => {
            let (rest, a) = register(rest)?;
            let (rest, c) = register(rest)?;
            Ok((rest, {
                let mut temp = Instruction::new(opcode);
                temp.add_operand(Operand::Register(a));
                temp.add_operand(Operand::Register(c));
                temp
            }))
        },
        INC | DEC => {
            let (rest, a) = register(rest)?;
            Ok((rest, {
                let mut temp = Instruction::new(opcode);
                temp.add_operand(Operand::Register(a));
                temp
            }))
        }
        LDI | ADI => {
            let (rest, a) = register(rest)?;
            let (rest, imm) = opt(immediate)(rest)?;
            if let Some(imm) = imm {
                return Ok((rest, {
                    let mut temp = Instruction::new(opcode);
                    temp.add_operand(Operand::Register(a));
                    temp.add_operand(Operand::Immediate(imm));
                    temp
                }));
            }
            let (rest, port) = opt(port)(rest)?;
            if let Some(port) = port {
                return Ok((rest, {
                    let mut temp = Instruction::new(opcode);
                    temp.add_operand(Operand::Register(a));
                    temp.add_operand(Operand::Port(port));
                    temp
                }));
            }
            let (rest, def) = opt(definition_usage)(rest)?;
            if let Some(def) = def {
                return Ok((rest, {
                    let mut temp = Instruction::new(opcode);
                    temp.add_operand(Operand::Register(a));
                    temp.add_operand(Operand::Definition(def));
                    temp
                }));
            }
            let (rest, character) = opt(character)(rest)?;
            if let Some(character) = character {
                return Ok((rest, {
                    let mut temp = Instruction::new(opcode);
                    temp.add_operand(Operand::Register(a));
                    temp.add_operand(Operand::Character(character));
                    temp
                }));
            }
            error!("Error: Missing operand on {opcode} instructions");
            Err(Err::Error(VerboseError::from_error_kind(rest, ErrorKind::Tag)))
        },
        JMP | CAL => {
            let (rest, address) = opt(address)(rest)?;
            let (rest, offset) = opt(offset)(rest)?;
            if let Some(address) = address {
                Ok((rest, {
                    let mut temp = Instruction::new(opcode);
                    temp.add_operand(Operand::Address(address));
                    temp
                }))
            } else if let Some(offset) = offset {
                Ok((rest, {
                    let mut temp = Instruction::new(opcode);
                    temp.add_operand(Operand::Offset(offset));
                    temp
                }))
            } else {
                error!("Error: Missing address or label on {opcode} instruction");
                Err(Err::Error(VerboseError::from_error_kind(rest, ErrorKind::Tag)))
            }
        },
        BRH => {
            let (rest, cond) = condition(rest)?;
            let (rest, address) = opt(address)(rest)?;
            let (rest, offset) = offset(rest)?;
            if let Some(address) = address {
                Ok((rest, {
                    let mut temp = Instruction::new(opcode);
                    temp.add_operand(Operand::Condition(cond));
                    temp.add_operand(Operand::Address(address));
                    temp
                }))
            } else {
                Ok((rest, {
                    let mut temp = Instruction::new(opcode);
                    temp.add_operand(Operand::Condition(cond));
                    temp.add_operand(Operand::Offset(offset));
                    temp
                }))
            }
        },
        LOD | STR => {
            let (rest, a) = register(rest)?;
            let (rest, b) = register(rest)?;
            let (rest, imm) = opt(immediate)(rest)?;
            if let Some(imm) = imm {
                return Ok((rest, {
                    let mut temp = Instruction::new(opcode);
                    temp.add_operand(Operand::Register(a));
                    temp.add_operand(Operand::Register(b));
                    temp.add_operand(Operand::Immediate(imm));
                    temp
                }));
            }
            let (rest, offset) = opt(offset)(rest)?;
            if let Some(offset) = offset {
                return Ok((rest, {
                    let mut temp = Instruction::new(opcode);
                    temp.add_operand(Operand::Register(a));
                    temp.add_operand(Operand::Register(b));
                    temp.add_operand(Operand::Offset(offset));
                    temp
                }));
            }
            Ok((rest, {
                let mut temp = Instruction::new(opcode);
                temp.add_operand(Operand::Register(a));
                temp.add_operand(Operand::Register(b));
                temp.add_operand(Operand::Immediate(0));
                temp
            }))
        },
        _ => { panic!("Error: How in the fuck????"); }
    }
}

pub fn character(input: &str) -> IResult<&str, char, VerboseError<&str>> {
    let allowed_chars =
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890-=!@#$%^&*()_+[]\\{}|;':\",./<>?`~ ";
    context(
        "character",
        alt((
            delimited(
                tag("\""),
                one_of(allowed_chars),
                tag("\"")
            ),
            delimited(
                tag("'"),
                one_of(allowed_chars),
                tag("'")
            )
        ))
    )(input)
}

pub fn port(input: &str) -> IResult<&str, Port, VerboseError<&str>>{
    let result: Result<(&str, &str), VerboseError<&str>> = context(
        "port",
        ws(take_while1(|x: char| x.is_alphanumeric() || x == '_'))
    )(input).finish();
    match result {
        Ok((rest, port)) => {
            let p = Port::from_str(port);
            if p.is_ok() {
                Ok((rest, p.unwrap()))
            } else {
                context("Invalid Port", fail)(input)
            }
        }
        Err(e) => {
            Err(Err::Error(e))
        }
    }
}

pub fn definition(input: &str) -> IResult<&str, Definition, VerboseError<&str>> {
    context(
        "Definition declaration",
        map_res(
            preceded(
                ws(tag_no_case("define")),
                pair(
                    ws(take_while1(|c: char| c.is_alphanumeric() || c == '_')),
                    ws(as_i16)
                )
            ), |(s, value)| {
                if KEYWORDS.contains(&s.to_lowercase().as_str()) {
                    error!("Invalid definition name: {}", s);
                    return Err(Error::from_error_kind(s, ErrorKind::Tag));
                }
                let mut def = Definition::new(s);
                def.value = Some(value);
                Ok(def)
            }
        )
    )(input)
}

pub fn definition_usage(input: &str) -> IResult<&str, Definition, VerboseError<&str>> {
    let result: Result<(&str, &str), VerboseError<&str>> = context(
        "definition usage",
        ws(take_while1(|c: char| c.is_alphanumeric() || c == '_'))
    )(input).finish();
    match result {
        Ok((rest, name)) => {
            Ok((rest, Definition::new(name)))
        },
        Err(e) => {
            Err(Err::Error(e))
        }
    }
}

pub fn condition(input: &str) -> IResult<&str, Condition, VerboseError<&str>> {
    context(
        "condition parsing",
        map_res(
            terminated(
                ws(take_while1(|x: char| x.is_alphanumeric())),
                opt(ws(tag(",")))
            ),
            |s: &str| {
                trace!("Parsing condition: {}", s);
                let cond = Condition::from_str(s.to_lowercase().as_str());
                if cond.is_ok() {
                    Ok(cond.unwrap())
                } else {
                    Err(Err::Error(()))
                }
            })
    )(input)
}

pub fn offset(input: &str) -> IResult<&str, Offset, VerboseError<&str>> {
    context(
        "parsing offset",
        map_res(
            preceded(
                ws(tag(".")),
                take_while1(|c: char| c.is_alphanumeric() || c == '_')
            ),
            |s: &str| {
                trace!("Parsing offset: {}", s);
                Ok::<Offset, VerboseError<&str>>(Offset::new(s.to_string()))
            }
        )
    )(input)
}

pub fn address(input: &str) -> IResult<&str, u16, VerboseError<&str>> {
    context(
        "address",
        ws(as_u16)
    )(input)
}

pub fn immediate(input: &str) -> IResult<&str, i16, VerboseError<&str>> {
    let (rest, signed) = opt(alt((tag("-"), tag("+"))))(input)?;
    let negative: bool = signed.unwrap_or("+") == "-";
    let x = context(
        "parsing immediate",
        ws(as_i16)
    )(rest).finish();
    match x {
        Ok((rest, imm)) => {
            Ok((rest, if negative {-imm} else {imm} ))
        },
        Err(e) => {
            Err(Err::Error(e))
        }
    }
}

pub fn register(input: &str) -> IResult<&str, Register, VerboseError<&str>> {
    context(
        "register parsing",
        map_res(
            delimited(
                ws(tag_no_case("r")),
                decimal_as_u8,
                alt((ws(tag(",")), skip))
            ),
            |s| {
                let reg = Register::try_from(s);
                if reg.is_ok() {
                    Ok(reg.unwrap())
                } else {
                    Err(Err::Error(()))
                }
            }
        )
    )(input)
}

pub fn opcode(input: &str) -> IResult<&str, Opcode, VerboseError<&str>> {
    context(
        "Opcode (Failed to parse opcode)",
        map_res(
            ws(take_while_m_n(3, 3, |c: char| c.is_alphabetic())),
            |s| Opcode::from_str(s.to_lowercase().as_str())
        ))(input)
}

pub fn label(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    context(
        "label",
        preceded(
            ws(tag(".")),
            take_while1(|c: char| c.is_alphanumeric() || c == '_')
    ))(input)
}