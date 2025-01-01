use crate::symbols::operands::immediate::Immediate;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{fail, map_res, opt, peek};
use nom::error::{context, ErrorKind, ParseError, VerboseError};
use nom::Err;
use nom::IResult;
use crate::parsing::basic::{address, character, condition, define, definition, immediate, label, offset, opcode, port, register};
use crate::parsing::helper::{leading_ws, next_instruction, next_token, Res};
use crate::symbols::instruction::Instruction;
use crate::symbols::opcodes::Opcode::*;
use crate::symbols::operands::Operand;
use crate::symbols::operands::Operand::{Address, Character, Condition, Definition, Label, Offset, Port, Register};

pub fn parse_instruction(input: &str) -> IResult<&str, Instruction, VerboseError<&str>> {
    trace!("parse_instruction current input: <{:?}...>", input.chars().take(20).collect::<String>());
    let (rest, opcode) = leading_ws(opcode)(input)?;
    match opcode {
        NOP | HLT | RET => {
            let (rest, _) = next_instruction(rest)?;
            Ok((rest, {
                trace!("Found instruction: {opcode}");
                Instruction::new(opcode)
            }))
        }
        ADD | SUB | NOR | AND | XOR => {
            let (rest, _) = next_token(rest)?;
            let (rest, a) = register(rest)?;
            let (rest, _) = next_token(rest)?;
            let (rest, b) = register(rest)?;
            let (rest, _) = next_token(rest)?;
            let (rest, c) = register(rest)?;
            let (rest, _) = next_instruction(rest)?;
            Ok((rest, {
                trace!("Found instruction: {} {}, {}, {}", opcode, a, b, c);
                let mut temp = Instruction::new(opcode);
                temp.add_register(a);
                temp.add_register(b);
                temp.add_register(c);
                temp
            }))
        },
        RSH | CMP | MOV | LSH | NOT | NEG => {
            let (rest, _) = next_token(rest)?;
            let (rest, a) = register(rest)?;
            let (rest, _) = next_token(rest)?;
            let (rest, b) = register(rest)?;
            let (rest, _) = next_instruction(rest)?;
            Ok((rest, {
                trace!("Found instruction: {} {}, {}", opcode, a, b);
                let mut temp = Instruction::new(opcode);
                temp.add_register(a);
                temp.add_register(b);
                temp
            }))
        },
        INC | DEC => {
            let (rest, _) = next_token(rest)?;
            let (rest, a) = register(rest)?;
            let (rest, _) = next_instruction(rest)?;
            Ok((rest, {
                trace!("Found instruction: {} {}", opcode, a);
                let mut temp = Instruction::new(opcode);
                temp.add_register(a);
                temp
            }))
        }
        LDI | ADI => {
            let (rest, _) = next_token(rest)?;
            let (rest, a) = register(rest)?;
            let (rest, _) = next_token(rest)?;
            let (rest, b) = operand(rest)?;
            let (rest, _) = next_instruction(rest)?;
            match b {
                Operand::Immediate(imm) => {
                    Ok((rest, {
                        trace!("Found instruction: {} {}, {}", opcode, a, imm);
                        let mut temp = Instruction::new(opcode);
                        temp.add_register(a);
                        temp.add_immediate(imm);
                        temp
                    }))
                }
                Definition(def) => {
                    Ok((rest, {
                        trace!("Found instruction: {} {}, {}", opcode, a, def.name);
                        let mut temp = Instruction::new(opcode);
                        temp.add_register(a);
                        temp.add_definition(def);
                        temp
                    }))
                }
                Port(port) => {
                    Ok((rest, {
                        trace!("Found instruction: {} {}, {}", opcode, a, port);
                        let mut temp = Instruction::new(opcode);
                        temp.add_register(a);
                        temp.add_port(port);
                        temp
                    }))
                }
                _ => {
                    error!("Error: Unexpected operand for {opcode}");
                    context("Instruction (Unexpected Operand)", fail)(rest)
                },
            }
        },
        JMP | CAL => {
            let (rest, _) = next_token(rest)?;
            let (rest, a) = operand(rest)?;
            let (rest, _) = next_instruction(rest)?;
            match a {
                Offset(offset) => {
                    Ok((rest, {
                        trace!("Found instruction: {} {}", opcode, offset);
                        let mut temp = Instruction::new(opcode);
                        temp.add_offset(offset);
                        temp
                    }))
                }
                Address(addr) => {
                    Ok((rest, {
                        trace!("Found instruction: {} {}", opcode, addr);
                        let mut temp = Instruction::new(opcode);
                        temp.add_address(addr);
                        temp
                    }))
                }
                _ => {
                    error!("Error: Unexpected operand for {opcode}");
                    context("Instruction (Unexpected Operand)", fail)(rest)
                },
            }
        },
        BRH => {
            let (rest, _) = next_token(rest)?;
            let (rest, a) = condition(rest)?;
            let (rest, _) = next_token(rest)?;
            let (rest, b) = operand(rest)?;
            let (rest, _) = next_instruction(rest)?;
            match b {
                Offset(offset) => {
                    Ok((rest, {
                        trace!("Found instruction: {} {}, {}", opcode, a, offset);
                        let mut temp = Instruction::new(opcode);
                        temp.add_condition(a);
                        temp.add_offset(offset);
                        temp
                    }))
                }
                Address(addr) => {
                    Ok((rest, {
                        trace!("Found instruction: {} {}, {}", opcode, a, addr);
                        let mut temp = Instruction::new(opcode);
                        temp.add_condition(a);
                        temp.add_address(addr);
                        temp
                    }))
                }
                _ => {
                    error!("Error: Unexpected operand for {opcode}");
                    context("Instruction (Unexpected Operand)", fail)(rest)
                },
            }
        },
        LOD | STR => {
            let (rest, _) = next_token(rest)?;
            let (rest, a) = register(rest)?;
            let (rest, _) = next_token(rest)?;
            let (rest, b) = register(rest)?;
            let (rest, next) = opt(peek(tag(",")))(rest)?;
            match next {
                Some(_) => {
                    let (rest, _) = next_token(rest)?;
                    let (rest, offset_value) = immediate(rest)?;
                    let (rest, _) = next_instruction(rest)?;
                    Ok((rest, {
                        trace!("Found instruction: {} {}, {}, {}", opcode, a, b, offset_value);
                        let mut temp = Instruction::new(opcode);
                        temp.add_register(a);
                        temp.add_register(b);
                        temp.add_immediate(offset_value);
                        temp
                    }))
                }
                None => {
                    let (rest, _) = next_token(rest)?;
                    let (rest, imm) = opt(immediate)(rest)?;
                    let (rest, _) = next_instruction(rest)?;
                    match imm {
                        Some(o) => {
                            Ok((rest, {
                                trace!("Found instruction: {} {}, {}, {}", opcode, a, b, o);
                                let mut temp = Instruction::new(opcode);
                                temp.add_register(a);
                                temp.add_register(b);
                                temp.add_immediate(o);
                                temp
                            }))
                        }
                        None => {
                            Ok((rest, {
                                trace!("Found instruction: {} {}, {}", opcode, a, b);
                                let mut temp = Instruction::new(opcode);
                                temp.add_register(a);
                                temp.add_register(b);
                                temp
                            }))
                        }
                    }
                }
            }
        }
        _ => {
            error!("Error: Invalid opcode (How the fuck?)");
            context("Instruction (Invalid Opcode)", fail)(rest)
        }
    }
}

pub fn parse_labels(input: &str) -> IResult<&str, Instruction, VerboseError<&str>> {
    trace!("parse_labels current input: <{:?}>", input.chars().take(20).collect::<String>());
    let (rest, label) = opt(label)(input)?;
    if label.is_some() {
        Ok((rest, {
            trace!("Found label: {}", label.unwrap());
            let mut temp = Instruction::new(_Label);
            temp.add_label(label.unwrap().to_string());
            temp
        }))
    } else {
        Err(Err::Error(VerboseError::from_error_kind(input, ErrorKind::Tag)))
    }
}

pub fn parse_definitions(input: &str) -> IResult<&str, Instruction, VerboseError<&str>> {
    trace!("parse_definitions current input: <{:?}>", input.chars().take(20).collect::<String>());
    let (rest, define) = opt(define)(input)?;
    if define.is_some() {
        let define = define.unwrap();
        Ok((rest, {
            trace!("Found definition: {}", define);
            let mut temp = Instruction::new(_Definition);
            temp.add_definition(define);
            temp
        }))
    } else {
        Err(Err::Error(VerboseError::from_error_kind(input, ErrorKind::Tag)))
    }
}

pub fn operand(input: &str) -> Res<&str, Operand> {
    trace!("operand current input: <{:?}>", input.chars().take(20).collect::<String>());
    let (rest, operand) = alt((
        map_res(register, |r| Ok::<Operand, VerboseError<&str>>(Register(r))),
        map_res(condition, |c| Ok::<Operand, VerboseError<&str>>(Condition(c))),
        map_res(immediate, |i| Ok::<Operand, VerboseError<&str>>(Operand::Immediate(i))),
        map_res(label, |l| Ok::<Operand, VerboseError<&str>>(Label(l.to_string()))),
        map_res(offset, |o| Ok::<Operand, VerboseError<&str>>(Offset(o))),
        map_res(definition, |d| Ok::<Operand, VerboseError<&str>>(Definition(d))),
        map_res(port, |p| Ok::<Operand, VerboseError<&str>>(Port(p))),
        map_res(address, |a| Ok::<Operand, VerboseError<&str>>(Address(a))),
        map_res(character, |c| Ok::<Operand, VerboseError<&str>>(Character(c)))
    ))(input)?;
    Ok((rest, operand))
}
