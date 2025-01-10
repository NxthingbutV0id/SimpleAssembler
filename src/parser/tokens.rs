use std::str::FromStr;
use nom::{
    error::{
        context,
    },
    branch::{
        alt
    },
    bytes::complete::{
        tag,
        tag_no_case,
        take_while1
    },
    character::complete::{
        digit1,
        one_of,
        space1,
        multispace1
    },
    combinator::{
        cut,
        eof,
        opt,
        recognize,
        fail
    },
    sequence::{
        delimited,
        pair
    },
    IResult
};
use crate::architecture::batpu2::opcode::Opcode;
use crate::architecture::batpu2::operand::{
    immediate::*,
    label::Label,
    port::Port,
    register::Register,
    condition::Condition,
    definition::Definition,
};
use crate::parser::helpers::*;


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
    let (rest, _declaration) = tag_no_case("define")(input)?;
    let (rest, _) = cut(space1)(rest)?;
    let (rest, name) = cut(identifier)(rest)?;
    let (rest, _) = cut(space1)(rest)?;
    trace!("Skipped whitespace after definition name");
    let (rest, signed) = opt(one_of("+-"))(rest)?;
    let negative: bool = signed.unwrap_or('+') == '-';
    let (rest, imm) = cut(number::<i16>)(rest)?;
    let value = if negative {-imm} else {imm};

    trace!("Found definition value: {}", value);
    match Definition::new_def(name, value) {
        Some(d) => Ok((rest, d)),
        None => {
            context("Definition (Invalid name)", fail)(input)
        }
    }
}

pub fn definition(input: &str) -> Res<&str, Definition> {
    let (rest, name) = identifier(input)?;
    match Definition::new_opr(name) {
        Some(def) => Ok((rest, def)),
        None => {
            context("Definition (Invalid name)", fail)(input)
        }
    }
}

pub fn condition(input: &str) -> Res<&str, Condition> {
    let (rest, cond) = take_while1(is_condition)(input)?;
    match Condition::from_str(cond) {
        Ok(c) => Ok((rest, c)),
        Err(_) => {
            cut(context("Condition (Invalid)", fail))(input)
        }
    }
}

pub fn label_usage(input: &str) -> Res<&str, Label> {
    let (rest, _) = context("Label Usage", tag("."))(input)?;
    let (rest, name) = identifier(rest)?;
    Ok((rest, Label::new(name.to_string())))
}

pub fn offset(input: &str) -> Res<&str, Offset> {
    let (rest, signed) = opt(one_of("+-"))(input)?;
    let negative: bool = signed.unwrap_or('+') == '-';
    let (rest, imm) = decimal::<i8>(rest)?;
    let imm = if negative {-imm} else {imm};

    match Offset::new(imm) {
        Some(i) => Ok((rest, i)),
        None => {
            context("Offset (Value out of bounds)", fail)(rest)
        }
    }
}

pub fn address(input: &str) -> Res<&str, Address> {
    let (rest, value) = context("Address", hexadecimal::<u16>)(input)?;

    match Address::new(value) {
        Some(addr) => Ok((rest, addr)),
        None => {
            eprintln!("Address out of bounds: {}", value);
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
    let (rest, reg) = cut(context(
        "Register (Expected register here)",
        recognize(
            pair(
                tag_no_case("r"),
                digit1
            )
        )
    ))(input)?;

    let reg = Register::from_str(reg);

    match reg {
        Ok(r) => Ok((rest, r)),
        Err(_) => {
            cut(context("Register (Invalid)", fail))(rest)
        }
    }
}

pub fn opcode(input: &str) -> Res<&str, Opcode> {
    let (rest, op) = identifier(input)?;

    match Opcode::from_str(op) {
        Ok(o) => {
            trace!("Parsed {} opcode", o);
            Ok((rest, o))
        },
        Err(_) => {
            cut(context("Opcode (Invalid)", fail))(input)
        }
    }
}

pub fn label_define(input: &str) -> Res<&str, &str> {
    context(
        "Label",
        delimited(
            tag("."),
            identifier,
            alt((eof, multispace1))
        )
    )(input)
}