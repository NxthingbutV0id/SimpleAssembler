use std::str::FromStr;
use nom::{
    error::context,
    branch::alt,
    bytes::complete::{ tag, tag_no_case, take_while1 },
    character::complete::{ one_of, multispace1, digit1, space1 },
    combinator::{opt, fail, recognize},
    sequence::{ delimited, pair },
};
use crate::parsing::{
    KEYWORDS,
    helper::*
};
use crate::symbols::{
    opcodes::*,
};
use crate::symbols::operands::address::Address;
use crate::symbols::operands::condition::Condition;
use crate::symbols::operands::definition::Definition;
use crate::symbols::operands::port::Port;
use crate::symbols::operands::immediate::Immediate;
use crate::symbols::operands::offset::Offset;
use crate::symbols::operands::register::Register;

pub fn character(input: &str) -> Res<&str, char> {
    let allowed_chars =
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890-=!@#$%^&*()_+[]\\{}|;':\",./<>?`~ ";
    context(
        "Character",
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

pub fn port(input: &str) -> Res<&str, Port> {
    let (rest, port_name) = identifier(input)?;
    let port = Port::from_str(port_name);
    match port {
        Ok(p) => Ok((rest, p)),
        Err(_) => {
            context("Port (Invalid name)", fail)(input)
        }
    }
}

pub fn define(input: &str) -> Res<&str, Definition> {
    let (rest, _declaration) = tag("define")(input)?;
    let (rest, _) = space1(rest)?;
    let (rest, name) = identifier(rest)?;
    
    if KEYWORDS.contains(&name.to_lowercase().as_str()) {
        return context("Definition (Invalid name)", fail)(input);
    }
    
    let (rest, _) = space1(rest)?;
    let (rest, value) = immediate(rest)?;
    let mut def = Definition::new(name);
    def.value = Some(value);
    Ok((rest, def))
}

pub fn definition(input: &str) -> Res<&str, Definition> {
    let (rest, name) = identifier(input)?;
    if KEYWORDS.contains(&name.to_lowercase().as_str()) {
        return context("Definition (Invalid name)", fail)(input);
    }
    Ok((rest, Definition::new(name)))
}

pub fn condition(input: &str) -> Res<&str, Condition> {
    let (rest, cond) = take_while1(is_condition)(input)?;
    match Condition::from_str(cond) {
        Ok(c) => Ok((rest, c)),
        Err(_) => {
            context("Condition (Invalid)", fail)(input)
        }
    }
}

pub fn offset(input: &str) -> Res<&str, Offset> {
    let (rest, _) = context("Offset", tag("."))(input)?;
    let (rest, name) = identifier(rest)?;
    Ok((rest, Offset::new(name.to_string())))
}

pub fn address(input: &str) -> Res<&str, Address> {
    let (rest, value) = context("Address", hexadecimal::<u16>)(input)?;
    
    match Address::new(value) { 
        Some(addr) => Ok((rest, addr)),
        None => {
            context("Address (Value out of bounds)", fail)(rest)
        }
    }
}

pub fn immediate(input: &str) -> Res<&str, Immediate> {
    let (rest, signed) = opt(one_of("+-"))(input)?;
    let negative: bool = signed.unwrap_or('+') == '-';
    let (rest, imm) = number::<i16>(rest)?;
    let imm = if negative {-imm} else {imm};
    
    match Immediate::new(imm) { 
        Some(i) => Ok((rest, i)),
        None => {
            context("Immediate (Value out of bounds)", fail)(rest)
        }
    }
}

pub fn register(input: &str) -> Res<&str, Register> {
    let (rest, reg) = context(
        "Register", 
        recognize(
            pair(
                tag_no_case("r"),
                digit1
            )
        )
    )(input)?;
    
    let reg = Register::from_str(reg);
    
    match reg {
        Ok(r) => Ok((rest, r)),
        Err(_) => {
            context("Register (Invalid)", fail)(rest)
        }
    }
}

pub fn opcode(input: &str) -> Res<&str, Opcode> {
    let (rest, op) = identifier(input)?;
    let op = Opcode::from_str(op);
    
    match op {
        Ok(o) => Ok((rest, o)),
        Err(_) => {
            context("Opcode (Invalid)", fail)(input)
        }
    }
}

pub fn label(input: &str) -> Res<&str, &str> {
    context(
        "Label",
        delimited(
            leading_ws_nonl(tag(".")),
            identifier,
            multispace1
        )
    )(input)
}