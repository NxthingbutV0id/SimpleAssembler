use std::fmt::Display;
use crate::architecture::batpu2::operand::condition::Condition;
use crate::architecture::batpu2::operand::definition::Definition;
use crate::architecture::batpu2::operand::immediate::{Address, Immediate, Offset};
use crate::architecture::batpu2::operand::label::Label;
use crate::architecture::batpu2::operand::port::Port;
use crate::architecture::batpu2::operand::register::Register;

pub mod condition;
pub mod definition;
pub mod label;
pub mod immediate;
pub mod register;
pub mod port;

#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    /// Used in math operations
    Reg(Register),
    /// Used in immediate operations
    Imm(Immediate),
    /// Used in jump and branch operations
    Addr(Address),
    /// Used in load and store operations
    Offset(Offset),
    /// Used in branch operations
    Cond(Condition),
    /// Used in jump and branch operations
    Label(Label),
    /// Used in immediate operations
    Char(char),
    /// Used in define operations or as an immediate value
    Def(Definition),
    /// Used in immediate operations as constants
    Port(Port),
    /// Used as a label definition
    Name(String)
}

impl From<Register> for Operand {
    fn from(reg: Register) -> Self {
        Operand::Reg(reg)
    }
}

impl From<Immediate> for Operand {
    fn from(imm: Immediate) -> Self {
        Operand::Imm(imm)
    }
}

impl From<Address> for Operand {
    fn from(addr: Address) -> Self {
        Operand::Addr(addr)
    }
}

impl From<Condition> for Operand {
    fn from(condition: Condition) -> Self {
        Operand::Cond(condition)
    }
}

impl From<Label> for Operand {
    fn from(label: Label) -> Self {
        Operand::Label(label)
    }
}

impl From<&str> for Operand {
    fn from(name: &str) -> Self {
        Operand::Name(name.to_string())
    }
}

impl From<Definition> for Operand {
    fn from(def: Definition) -> Self {
        Operand::Def(def)
    }
}

impl From<Port> for Operand {
    fn from(port: Port) -> Self {
        Operand::Port(port)
    }
}

impl From<char> for Operand {
    fn from(c: char) -> Self {
        Operand::Char(c)
    }
}

impl From<Offset> for Operand {
    fn from(n: Offset) -> Self {
        Operand::Offset(n)
    }
}

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Operand::Reg(reg) => write!(f, "{}", *reg),
            Operand::Imm(i) => write!(f, "{}", *i),
            Operand::Cond(cond) => write!(f, "{}", *cond),
            Operand::Addr(addr) => write!(f, "{}", *addr),
            Operand::Label(label) => write!(f, "{}", label),
            Operand::Name(name) => write!(f, "{}", name),
            Operand::Def(def) => write!(f, "{}", def),
            Operand::Port(port) => write!(f, "{}", port),
            Operand::Char(c) => write!(f, "'{}' (0x{:02X})", c, *c as u8),
            Operand::Offset(off) => write!(f, "{}", off)
        }
    }
}