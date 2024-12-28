use std::str::FromStr;
use log::error;
use nom::*;
use nom::{
    error::{Error, ErrorKind},
    branch::alt,
    bytes::complete::{ tag, take_while1 },
    character::complete::{ alphanumeric1, one_of },
    combinator::{ opt, map_res },
    sequence::{ pair, preceded, delimited },
};
use nom::sequence::terminated;
use crate::parsing::{
    KEYWORDS,
    helper::{alternative, alternative_no_case, binary, decimal, hexadecimal, ws}
};
use crate::symbols::{
    opcodes::*,
    opcodes::Opcode::*,
    operands::{Operand, Offset, Condition, Register},
    instruction::*
};
use crate::symbols::definitions::*;

pub(crate) fn parse_labels(input: &str) -> IResult<&str, Instruction> {
    let (rest, label) = opt(parse_label)(input)?;
    if label.is_some() {
        Ok((rest, {
            let label = Operand::Label(label.unwrap().to_string());
            let mut temp = Instruction::new(_Label);
            temp.add_operand(label);
            temp
        }))
    } else {
        Err(Err::Error(Error::new(input, ErrorKind::Tag)))
    }
}

pub(crate) fn parse_definitions(input: &str) -> IResult<&str, Instruction> {
    let (rest, define) = opt(parse_definition)(input)?;
    if define.is_some() {
        let define = define.unwrap();
        Ok((rest, {
            let mut temp = Instruction::new(_Definition);
            temp.add_operand(Operand::Definition(define));
            temp
        }))
    } else {
        Err(Err::Error(Error::new(input, ErrorKind::Tag)))
    }
}

pub(crate) fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (rest, opcode) = parse_opcode(input)
        .expect("Failed to parse opcode");
    match opcode {
        NOP | HLT | RET => {
            Ok((rest, Instruction::new(opcode)))
        }
        ADD | SUB | NOR | AND | XOR => {
            let (rest, a) = parse_register(rest)
                .expect("Failed to parse register A");
            let (rest, b) = parse_register(rest)
                .expect("Failed to parse register B");
            let (rest, c) = parse_register(rest)
                .expect("Failed to parse register C");
            Ok((rest, {
                let mut temp = Instruction::new(opcode);
                temp.add_operand(Operand::Register(a));
                temp.add_operand(Operand::Register(b));
                temp.add_operand(Operand::Register(c));
                temp
            }))
        },
        RSH | CMP | MOV | LSH | NOT | NEG => {
            let (rest, a) = parse_register(rest)
                .expect("Failed to parse register A");
            let (rest, c) = parse_register(rest)
                .expect("Failed to parse register B");
            Ok((rest, {
                let mut temp = Instruction::new(opcode);
                temp.add_operand(Operand::Register(a));
                temp.add_operand(Operand::Register(c));
                temp
            }))
        },
        INC | DEC => {
            let (rest, a) = parse_register(rest)
                .expect("Failed to parse register");
            Ok((rest, {
                let mut temp = Instruction::new(opcode);
                temp.add_operand(Operand::Register(a));
                temp
            }))
        }
        LDI | ADI => {
            let (rest, a) = parse_register(rest)
                .expect("Failed to parse register");
            let (rest, imm) = opt(parse_immediate)(rest)?;
            if let Some(imm) = imm {
                return Ok((rest, {
                    let mut temp = Instruction::new(opcode);
                    temp.add_operand(Operand::Register(a));
                    temp.add_operand(Operand::Immediate(imm));
                    temp
                }));
            }
            let (rest, port) = opt(parse_port)(rest)?;
            if let Some(port) = port {
                return Ok((rest, {
                    let mut temp = Instruction::new(opcode);
                    temp.add_operand(Operand::Register(a));
                    temp.add_operand(Operand::Port(port));
                    temp
                }));
            }
            let (rest, def) = opt(parse_definition_usage)(rest)?;
            if let Some(def) = def {
                return Ok((rest, {
                    let mut temp = Instruction::new(opcode);
                    temp.add_operand(Operand::Register(a));
                    temp.add_operand(Operand::Definition(def));
                    temp
                }));
            }
            let (rest, character) = opt(parse_character)(rest)?;
            if let Some(character) = character {
                return Ok((rest, {
                    let mut temp = Instruction::new(opcode);
                    temp.add_operand(Operand::Register(a));
                    temp.add_operand(Operand::Character(character));
                    temp
                }));
            }
            error!("Error: Missing operand on {opcode} instructions");
            Err(Err::Error(Error::new(rest, ErrorKind::Tag)))
        },
        JMP | CAL => {
            let (rest, address) = opt(parse_address)(rest)?;
            let (rest, offset) = opt(parse_offset)(rest)?;
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
                Err(Err::Error(Error::new(rest, ErrorKind::Tag)))
            }
        },
        BRH => {
            let (rest, cond) = parse_condition(rest)
                .expect("Failed to parse condition");
            let (rest, address) = opt(parse_address)(rest)?;
            let (rest, offset) = opt(parse_offset)(rest)?;
            if let Some(address) = address {
                Ok((rest, {
                    let mut temp = Instruction::new(opcode);
                    temp.add_operand(Operand::Condition(cond));
                    temp.add_operand(Operand::Address(address));
                    temp
                }))
            } else if let Some(offset) = offset {
                Ok((rest, {
                    let mut temp = Instruction::new(opcode);
                    temp.add_operand(Operand::Condition(cond));
                    temp.add_operand(Operand::Offset(offset));
                    temp
                }))
            } else {
                error!("Error: Missing address or label on brh instruction");
                Err(Err::Error(Error::new(rest, ErrorKind::Tag)))
            }
        },
        LOD | STR => {
            let (rest, a) = parse_register(rest)
                .expect("Failed to parse register A");
            let (rest, b) = parse_register(rest)
                .expect("Failed to parse register B");
            let (rest, imm) = opt(parse_immediate)(rest)?;
            if let Some(imm) = imm {
                return Ok((rest, {
                    let mut temp = Instruction::new(opcode);
                    temp.add_operand(Operand::Register(a));
                    temp.add_operand(Operand::Register(b));
                    temp.add_operand(Operand::Immediate(imm));
                    temp
                }));
            }
            let (rest, offset) = opt(parse_offset)(rest)?;
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
        _ => {
            error!("Error: How in the fuck????");
            Err(Err::Error(Error::new(rest, ErrorKind::Tag)))
        }
    }
}

fn parse_character(input: &str) -> IResult<&str, char> {
    let allowed_chars =
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890-=!@#$%^&*()_+[]\\{}|;':\",./<>?`~ ";
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
    ))(input)
}

fn parse_port(input: &str) -> IResult<&str, Port>{
    let ports = &KEYWORDS[56..];
    let (rest, s) = alternative(input, ports)?;
    match s {
        "pixel_x" => Ok((rest, Port::PixelX)),
        "pixel_y" => Ok((rest, Port::PixelY)),
        "draw_pixel" => Ok((rest, Port::DrawPixel)),
        "clear_pixel" => Ok((rest, Port::ClearPixel)),
        "load_pixel" => Ok((rest, Port::LoadPixel)),
        "buffer_screen" => Ok((rest, Port::BufferScreen)),
        "clear_screen_buffer" => Ok((rest, Port::ClearScreenBuffer)),
        "write_char" => Ok((rest, Port::WriteChar)),
        "buffer_chars" => Ok((rest, Port::BufferChars)),
        "clear_chars_buffer" => Ok((rest, Port::ClearCharsBuffer)),
        "show_number" => Ok((rest, Port::ShowNumber)),
        "clear_number" => Ok((rest, Port::ClearNumber)),
        "signed_mode" => Ok((rest, Port::SignedMode)),
        "unsigned_mode" => Ok((rest, Port::UnsignedMode)),
        "rng" => Ok((rest, Port::RNG)),
        "controller_input" => Ok((rest, Port::ControllerInput)),
        _ => Err(Err::Error(Error::new(rest, ErrorKind::Tag)))
    }
}

fn parse_definition(input: &str) -> IResult<&str, Definition> {
    map_res(
        preceded(
            ws(tag(KEYWORDS[0])),
            pair(
                ws(alphanumeric1),
                parse_immediate
            )
        ),
        |(name, value)| {
            if KEYWORDS.contains(&name) || name.starts_with(".") {
                error!("Reserved name: {}, Constants cannot be labels or keywords", name);
                Err(())
            } else {
                Ok({
                    let mut temp = Definition::new(name);
                    temp.set_value(value);
                    temp
                })
            }
        }
    )(input)
}

fn parse_definition_usage(input: &str) -> IResult<&str, Definition> {
    map_res(
        ws(take_while1(|c: char| c.is_alphanumeric() || c == '_')),
        |name| {
            if KEYWORDS.contains(&name) || name.starts_with(".") {
                error!("Something went horribly wrong if you got here...");
                Err(())
            } else {
                Ok(Definition::new(name))
            }
        }
    )(input)
}

fn parse_condition(input: &str) -> IResult<&str, Condition> {
    let conds = &KEYWORDS[40..56];
    let (rest, s) = terminated(
        ws(|s| alternative_no_case(s, conds)),
        opt(ws(tag(",")))
    )(input)?;
    match s.to_lowercase().as_str() {
        "zs" | "eq" | "=" | "z" | "zero" => Ok((rest, Condition::ZS)),
        "zc" | "ne" | "!=" | "nz" | "notzero" => Ok((rest, Condition::ZC)),
        "cs" | "ge" | ">=" | "c" | "carry" => Ok((rest, Condition::CS)),
        "cc" | "lt" | "<" | "nc" | "notcarry"=> Ok((rest, Condition::CC)),
        _ => Err(Err::Error(Error::new(rest, ErrorKind::Tag)))
    }
}

fn parse_offset(input: &str) -> IResult<&str, Offset> {
    map_res(
        preceded(
            ws(tag(".")),
            take_while1(|c: char| c.is_alphanumeric() || c == '_')
        ),
        |s| {
            if s.starts_with(".") {
                error!("Error: too many dots in label");
                return Err(());
            }
            Ok(Offset {
                name: s.to_string(),
                binding: None,
                offset: None
            })
        }
    )(input)
}

fn parse_address(input: &str) -> IResult<&str, u16> {
    alt((
        map_res(
            ws(hexadecimal),
            |s| u16::from_str_radix(s, 16)
        ),
        map_res(
            ws(binary),
            |s| u16::from_str_radix(s, 2)
        ),
        map_res(
            ws(decimal),
            u16::from_str
        )
    ))(input)
}

fn parse_immediate(input: &str) -> IResult<&str, i16> {
    let (rest, signed) = opt(alt((tag("-"), tag("+"))))(input)?;
    let x = alt((
        map_res(
            ws(hexadecimal),
            |s| {
                if signed.is_some() && signed.unwrap() == "-" {
                    i16::from_str_radix(s, 16).map(|x| -x)
                } else {
                    i16::from_str_radix(s, 16)
                }
            }
        ),
        map_res(
            ws(binary),
            |s| {
                if signed.is_some() && signed.unwrap() == "-" {
                    i16::from_str_radix(s, 2).map(|x| -x)
                } else {
                    i16::from_str_radix(s, 2)
                }
            }
        ),
        map_res(
            ws(decimal),
            |s| {
                if signed.is_some() && signed.unwrap() == "-" {
                    i16::from_str(s).map(|x| -x)
                } else {
                    i16::from_str(s)
                }
            }
        )
    ))(rest); x
}

fn parse_register(input: &str) -> IResult<&str, Register> {
    let mut registers: [&str; 16] = [""; 16];
    for i in 0..16 {
        registers[i] = KEYWORDS[24 + i];
    }
    registers.reverse();
    let (rest, s) =
        terminated(
            ws(move |s| alternative_no_case(s, &registers)),
            opt(ws(tag(",")))
        )(input)?;
    match s.to_lowercase().as_str() {
        "r0" => Ok((rest, Register::R0)),
        "r1" => Ok((rest, Register::R1)),
        "r2" => Ok((rest, Register::R2)),
        "r3" => Ok((rest, Register::R3)),
        "r4" => Ok((rest, Register::R4)),
        "r5" => Ok((rest, Register::R5)),
        "r6" => Ok((rest, Register::R6)),
        "r7" => Ok((rest, Register::R7)),
        "r8" => Ok((rest, Register::R8)),
        "r9" => Ok((rest, Register::R9)),
        "r10" => Ok((rest, Register::R10)),
        "r11" => Ok((rest, Register::R11)),
        "r12" => Ok((rest, Register::R12)),
        "r13" => Ok((rest, Register::R13)),
        "r14" => Ok((rest, Register::R14)),
        "r15" => Ok((rest, Register::R15)),
        _ => Err(Err::Error(Error::new(rest, ErrorKind::Tag)))
    }
}

fn parse_opcode(input: &str) -> IResult<&str, Opcode> {
    let opcodes = &KEYWORDS[1..24];
    let (rest, s) = alternative_no_case(input, opcodes)?;
    match s.to_lowercase().as_str() {
        "nop" => Ok((rest, NOP)),
        "hlt" => Ok((rest, HLT)),
        "add" => Ok((rest, ADD)),
        "sub" => Ok((rest, SUB)),
        "nor" => Ok((rest, NOR)),
        "and" => Ok((rest, AND)),
        "xor" => Ok((rest, XOR)),
        "rsh" => Ok((rest, RSH)),
        "ldi" => Ok((rest, LDI)),
        "adi" => Ok((rest, ADI)),
        "jmp" => Ok((rest, JMP)),
        "brh" => Ok((rest, BRH)),
        "cal" => Ok((rest, CAL)),
        "ret" => Ok((rest, RET)),
        "lod" => Ok((rest, LOD)),
        "str" => Ok((rest, STR)),
        "cmp" => Ok((rest, CMP)),
        "mov" => Ok((rest, MOV)),
        "lsh" => Ok((rest, LSH)),
        "inc" => Ok((rest, INC)),
        "dec" => Ok((rest, DEC)),
        "not" => Ok((rest, NOT)),
        "neg" => Ok((rest, NEG)),
        _ => Err(Err::Error(Error::new(rest, ErrorKind::Tag)))
    }
}

fn parse_label(input: &str) -> IResult<&str, &str> {
    preceded(
        ws(tag(".")),
        take_while1(|c: char| c.is_alphanumeric() || c == '_')
    )(input)
}