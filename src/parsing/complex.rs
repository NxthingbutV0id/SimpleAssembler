use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{cut, fail, map_res, opt, peek};
use nom::error::{context, VerboseError};
use crate::parsing::basic::{offset, address, character, condition, define, definition, immediate, label_define, label_usage, opcode, port, register};
use crate::parsing::helper::{leading_ws, next_instruction, next_token, Res};
use crate::symbols::instruction::Instruction;
use crate::symbols::opcodes::Opcode;
use crate::symbols::operands::Operand;
use crate::symbols::operands::register::Register;

fn no_operands(input: &str, opcode: Opcode) -> Res<&str, Instruction> {
    let (rest, _) = next_instruction(input)?;
    Ok((rest, {
        trace!("Found instruction: {opcode}");
        Instruction::new(opcode)
    }))
}

fn three_operands(input: &str, opcode: Opcode) -> Res<&str, Instruction> {
    let (rest, _) = next_token(input)?;
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
}

fn two_operands(input: &str, opcode: Opcode) -> Res<&str, Instruction> {
    let (rest, _) = next_token(input)?;
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
}

fn one_operand(input: &str, opcode: Opcode) -> Res<&str, Instruction> {
    let (rest, _) = next_token(input)?;
    let (rest, a) = register(rest)?;
    let (rest, _) = next_instruction(rest)?;
    Ok((rest, {
        trace!("Found instruction: {} {}", opcode, a);
        let mut temp = Instruction::new(opcode);
        temp.add_register(a);
        temp
    }))
}

fn immediate_instructions(input: &str, opcode: Opcode) -> Res<&str, Instruction> {
    let (rest, _) = next_token(input)?;
    let (rest, a) = register(rest)?;
    let (rest, _) = next_token(rest)?;
    let (rest, b) = peek(operand_imm)(rest)?;
    match b {
        Operand::Immediate(_) => {
            let (rest, imm) = immediate(rest)?;
            let (rest, _) = next_instruction(rest)?;
            Ok((rest, {
                trace!("Found instruction: {} {}, {}", opcode, a, imm);
                let mut temp = Instruction::new(opcode);
                temp.add_register(a);
                temp.add_immediate(imm);
                temp
            }))
        }
        Operand::Definition(_) => {
            let (rest, def) = definition(rest)?;
            let (rest, _) = next_instruction(rest)?;
            Ok((rest, {
                trace!("Found instruction: {} {}, {}", opcode, a, def.name);
                let mut temp = Instruction::new(opcode);
                temp.add_register(a);
                temp.add_definition(def);
                temp
            }))
        }
        Operand::Port(_) => {
            let (rest, p) = port(rest)?;
            let (rest, _) = next_instruction(rest)?;
            Ok((rest, {
                trace!("Found instruction: {} {}, {}", opcode, a, p);
                let mut temp = Instruction::new(opcode);
                temp.add_register(a);
                temp.add_port(p);
                temp
            }))
        }
        Operand::Character(_) => {
            let (rest, ch) = character(rest)?;
            let (rest, _) = next_instruction(rest)?;
            Ok((rest, {
                trace!("Found instruction: {} {}, {}", opcode, a, ch);
                let mut temp = Instruction::new(opcode);
                temp.add_register(a);
                temp.add_character(ch);
                temp
            }))
        },
        _ => {
            error!("Error: Unexpected operand for {opcode}: {b:?}");
            context("Instruction (Unexpected Operand)", fail)(rest)
        },
    }
}

fn address_instructions(input: &str, opcode: Opcode) -> Res<&str, Instruction> {
    let (rest, _) = next_token(input)?;
    trace!("skipped ahead: <{:?}...>", rest.chars().take(20).collect::<String>());
    let (rest, a) = operand_imm(rest)?;
    trace!("parsed operand: <{:?}...>", rest.chars().take(20).collect::<String>());
    let (rest, _) = next_instruction(rest)?;
    trace!("skipped to next instruction: <{:?}...>", rest.chars().take(20).collect::<String>());
    match a {
        Operand::Label(label) => {
            Ok((rest, {
                trace!("Found instruction: {} {}", opcode, label);
                let mut temp = Instruction::new(opcode);
                temp.add_label(label);
                temp
            }))
        }
        Operand::Address(addr) => {
            Ok((rest, {
                trace!("Found instruction: {} {}", opcode, addr);
                let mut temp = Instruction::new(opcode);
                temp.add_address(addr);
                temp
            }))
        }
        _ => {
            error!("Error: Unexpected operand for {opcode}: {a:?}");
            context("Instruction (Unexpected Operand)", fail)(rest)
        },
    }
}

fn branch_instruction(input: &str, opcode: Opcode) -> Res<&str, Instruction> {
    let (rest, _) = next_token(input)?;
    let (rest, a) = condition(rest)?;
    let (rest, _) = next_token(rest)?;
    let (rest, b) = operand_imm(rest)?;
    let (rest, _) = next_instruction(rest)?;
    match b {
        Operand::Label(label) => {
            Ok((rest, {
                trace!("Found instruction: {} {}, {}", opcode, a, label);
                let mut temp = Instruction::new(opcode);
                temp.add_condition(a);
                temp.add_label(label);
                temp
            }))
        }
        Operand::Address(addr) => {
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
}

/// You put a comma, therefore, there must be a next value
fn there_must_be_a_next_value(input: &str, opcode: Opcode, a: Register, b: Register) -> Res<&str, Instruction> {
    let (rest, _) = next_token(input)?;
    let (rest, offset_value) = operand_off(rest)?;
    let (rest, _) = next_instruction(rest)?;
    there_is_a_next_value(rest, opcode, a, b, offset_value)
}

/// You didn't put a comma, but there could still be a next value
fn there_could_be_a_next_value(input: &str, opcode: Opcode, a: Register, b: Register) -> Res<&str, Instruction> {
    let (rest, test) = opt(next_token)(input)?;
    match test {
        Some(_) => {
            trace!("skipped ahead: <{:?}...>", rest.chars().take(20).collect::<String>());
            let (rest, oper) = opt(operand_off)(rest)?;
            let (rest, _) = next_instruction(rest)?;
            match oper {
                Some(o) => there_is_a_next_value(rest, opcode, a, b, o),
                None => there_is_not_a_next_value(rest, opcode, a, b),
            }
        }
        None => there_is_not_a_next_value(rest, opcode, a, b)
    }
}

/// The most deeply nested piece of shit I've ever written
fn load_and_store(input: &str, opcode: Opcode) -> Res<&str, Instruction> {
    let (rest, _) = next_token(input)?;
    trace!("skipped ahead: <{:?}...>", rest.chars().take(20).collect::<String>());
    let (rest, a) = register(rest)?;
    trace!("parsed register: <{:?}...>", rest.chars().take(20).collect::<String>());
    let (rest, _) = next_token(rest)?;
    trace!("skipped ahead: <{:?}...>", rest.chars().take(20).collect::<String>());
    let (rest, b) = register(rest)?;
    trace!("parsed register: <{:?}...>", rest.chars().take(20).collect::<String>());
    let (rest, next) = opt(peek(tag(",")))(rest)?;
    trace!("peeked ahead: <{:?}...>", rest.chars().take(20).collect::<String>());
    match next {
        Some(_) => there_must_be_a_next_value(rest, opcode, a, b),
        None => there_could_be_a_next_value(rest, opcode, a, b),
    }
}


/// Holy shit, there was a value there!!!
fn there_is_a_next_value(input: &str, opcode: Opcode, a: Register, b: Register, o: Operand) -> Res<&str, Instruction> {
    match o {
        Operand::Offset(i) => {
            Ok((input, {
                trace!("Found instruction: {} {}, {}, {}", opcode, a, b, i);
                let mut temp = Instruction::new(opcode);
                temp.add_register(a);
                temp.add_register(b);
                temp.add_offset(i);
                temp
            }))
        }
        Operand::Definition(d) => {
            Ok((input, {
                trace!("Found instruction: {} {}, {}, {}", opcode, a, b, d);
                let mut temp = Instruction::new(opcode);
                temp.add_register(a);
                temp.add_register(b);
                temp.add_definition(d);
                temp
            }))
        }
        _ => {
            // Wrong value >:(
            error!("Error: Unexpected operand for {opcode}: {o:?}");
            context("Instruction (Unexpected Operand)", fail)(input)
        },
    }
}

/// There was no value there after all :(
fn there_is_not_a_next_value(input: &str, opcode: Opcode, a: Register, b: Register) -> Res<&str, Instruction> {
    Ok((input, {
        trace!("Found instruction: {} {}, {}", opcode, a, b);
        let mut temp = Instruction::new(opcode);
        temp.add_register(a);
        temp.add_register(b);
        temp
    }))
}

pub fn parse_instruction(input: &str) -> Res<&str, Instruction> {
    trace!("parse_instruction current input: <{:?}...>", input.chars().take(20).collect::<String>());
    let (rest, opcode) = leading_ws(opcode)(input)?;
    trace!("{} parsed, remaining: <{:?}...>", opcode, rest.chars().take(20).collect::<String>());
    match opcode {
        Opcode::NOP | Opcode::HLT | Opcode::RET => no_operands(rest, opcode),
        Opcode::ADD | Opcode::SUB | Opcode::NOR | Opcode::AND | Opcode::XOR => three_operands(rest, opcode),
        Opcode::RSH | Opcode::CMP | Opcode::MOV | Opcode::LSH | Opcode::NOT | Opcode::NEG => two_operands(rest, opcode),
        Opcode::INC | Opcode::DEC => one_operand(rest, opcode),
        Opcode::LDI | Opcode::ADI => immediate_instructions(rest, opcode),
        Opcode::JMP | Opcode::CAL => address_instructions(rest, opcode),
        Opcode::BRH => branch_instruction(rest, opcode),
        Opcode::LOD | Opcode::STR => load_and_store(rest, opcode),
        _ => {
            error!("Error: Invalid opcode (How the fuck?)");
            cut(context("Instruction (Invalid Opcode)", fail))(rest)
        }
    }
}

pub fn parse_labels(input: &str) -> Res<&str, Instruction> {
    trace!("attempting to parse label: <{:?}>", input.chars().take(20).collect::<String>());
    let (rest, label) = label_define(input)?;
    Ok((rest, {
        trace!("Found label: {}", label);
        let mut temp = Instruction::new(Opcode::_Label);
        temp.add_label_name(label.to_string());
        temp
    }))
}

pub fn parse_definitions(input: &str) -> Res<&str, Instruction> {
    trace!("attempting to parse definitions: <{:?}>", input.chars().take(40).collect::<String>());
    let (rest, define) = opt(define)(input)?;
    match define {
        Some(d) => {
            Ok((rest, {
                trace!("Found definition: {}", d);
                let mut temp = Instruction::new(Opcode::_Definition);
                temp.add_definition(d);
                temp
            }))
        }
        None => {
            context("Definitions (Not found)", fail)(rest)
        }
    }
}

pub fn operand_off(input: &str) -> Res<&str, Operand> {
    use Operand as O;
    type Verbose = VerboseError<&'static str>;
    trace!("operand (offset) current input: <{:?}>", input.chars().take(20).collect::<String>());
    let (rest, operand) = alt((
        map_res(offset, |o| Ok::<O, Verbose>(O::Offset(o))),
        map_res(label_usage, |o| Ok::<O, Verbose>(O::Label(o))),
        map_res(definition, |d| Ok::<O, Verbose>(O::Definition(d))),
        map_res(port, |p| Ok::<O, Verbose>(O::Port(p))),
        map_res(address, |a| Ok::<O, Verbose>(O::Address(a))),
        map_res(character, |c| Ok::<O, Verbose>(O::Character(c)))
    ))(input)?;
    Ok((rest, operand))
}

pub fn operand_imm(input: &str) -> Res<&str, Operand> {
    use Operand as O;
    type Verbose = VerboseError<&'static str>;
    trace!("operand (imm) current input: <{:?}>", input.chars().take(20).collect::<String>());
    let (rest, operand) = cut(alt((
        map_res(immediate, |i| Ok::<O, Verbose>(O::Immediate(i))),
        map_res(label_usage, |o| Ok::<O, Verbose>(O::Label(o))),
        map_res(definition, |d| Ok::<O, Verbose>(O::Definition(d))),
        map_res(port, |p| Ok::<O, Verbose>(O::Port(p))),
        map_res(address, |a| Ok::<O, Verbose>(O::Address(a))),
        map_res(character, |c| Ok::<O, Verbose>(O::Character(c)))
    )))(input)?;
    Ok((rest, operand))
}

