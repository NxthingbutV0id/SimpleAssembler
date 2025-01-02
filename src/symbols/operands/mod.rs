pub mod immediate;
pub mod register;
pub mod address;
pub mod label;
pub mod definition;
pub mod port;
pub mod condition;
pub mod offset;

use std::fmt::Display;
use crate::symbols::operands::address::Address;
use crate::symbols::operands::condition::Condition;
use crate::symbols::operands::definition::Definition;
use crate::symbols::operands::immediate::Immediate;
use crate::symbols::operands::label::Label;
use crate::symbols::operands::offset::Nybble;
use crate::symbols::operands::port::Port;
use crate::symbols::operands::register::Register;

#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    Register(Register),
    Immediate(Immediate),
    Condition(Condition),
    Address(Address),
    Label(Label),
    Name(String),
    Definition(Definition),
    Port(Port),
    Character(char),
    Offset(Nybble)
}

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Operand::Register(reg) => write!(f, "{}", *reg),
            Operand::Immediate(i) => write!(f, "{}", *i),
            Operand::Condition(cond) => write!(f, "{}", *cond),
            Operand::Address(addr) => write!(f, "{}", *addr),
            Operand::Label(label) => write!(f, "{}", label),
            Operand::Name(name) => write!(f, "{}", name),
            Operand::Definition(def) => write!(f, "{}", def),
            Operand::Port(port) => write!(f, "{}", port),
            Operand::Character(c) => write!(f, "'{}' (0x{:02X})", c, *c as u8),
            Operand::Offset(off) => write!(f, "{}", off)
        }
    }
}

impl From<Register> for Operand {
    fn from(reg: Register) -> Self {
        Operand::Register(reg)
    }
}

impl From<Immediate> for Operand {
    fn from(imm: Immediate) -> Self {
        Operand::Immediate(imm)
    }
}

impl From<Address> for Operand {
    fn from(addr: Address) -> Self {
        Operand::Address(addr)
    }
}

impl From<Condition> for Operand {
    fn from(condition: Condition) -> Self {
        Operand::Condition(condition)
    }
}

impl From<Label> for Operand {
    fn from(offset: Label) -> Self {
        Operand::Label(offset)
    }
}

impl From<Definition> for Operand {
    fn from(def: Definition) -> Self {
        Operand::Definition(def)
    }
}

impl From<Port> for Operand {
    fn from(port: Port) -> Self {
        Operand::Port(port)
    }
}

impl From<char> for Operand {
    fn from(c: char) -> Self {
        Operand::Character(c)
    }
}

impl From<&str> for Operand {
    fn from(label: &str) -> Self {
        Operand::Name(label.to_string())
    }
}

impl From<Nybble> for Operand {
    fn from(n: Nybble) -> Self {
        Operand::Offset(n)
    }
}