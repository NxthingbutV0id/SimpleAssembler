use std::fmt::Display;
use crate::symbols::definitions::{Definition, Port};
use crate::symbols::instruction::Instruction;

#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    Register(Register),
    Immediate(i16),
    Condition(Condition),
    Address(u16),
    Offset(Offset),
    Label(String),
    Definition(Definition),
    Port(Port),
    Character(char)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Condition {
    ZS, ZC, CS, CC
}

#[derive(Debug, Clone, PartialEq)]
pub struct Offset {
    pub name: String,
    pub binding: Option<Instruction>,
    pub offset: Option<u16>
}

impl Display for Offset {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Operand::Register(reg) => write!(f, "r{}", *reg as u8),
            Operand::Immediate(i) => write!(f, "{}", *i),
            Operand::Condition(cond) => {
                match cond {
                    Condition::CC => write!(f, "lt"),
                    Condition::CS => write!(f, "ge"),
                    Condition::ZC => write!(f, "ne"),
                    Condition::ZS => write!(f, "eq")
                }
            },
            Operand::Address(addr) => write!(f, "0x{:04X}", *addr),
            Operand::Offset(offset) => write!(f, "{}", offset),
            Operand::Label(label) => write!(f, "{}", label),
            Operand::Definition(def) => write!(f, "{}", def),
            Operand::Port(port) => write!(f, "{}", port),
            Operand::Character(c) => write!(f, "'{}' (0x{:02X})", c, *c as u8)
        }
    }
}