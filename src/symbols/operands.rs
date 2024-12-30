use std::fmt::Display;
use std::str::FromStr;
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
    R0, R1, R2, R3,
    R4, R5, R6, R7,
    R8, R9, R10, R11,
    R12, R13, R14, R15
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

impl Offset {
    pub fn new(name: String) -> Self {
        Offset {
            name,
            binding: None,
            offset: None
        }
    }
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

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "r{}", *self as u8)
    }
}

impl TryFrom<u8> for Register {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Register::R0),
            1 => Ok(Register::R1),
            2 => Ok(Register::R2),
            3 => Ok(Register::R3),
            4 => Ok(Register::R4),
            5 => Ok(Register::R5),
            6 => Ok(Register::R6),
            7 => Ok(Register::R7),
            8 => Ok(Register::R8),
            9 => Ok(Register::R9),
            10 => Ok(Register::R10),
            11 => Ok(Register::R11),
            12 => Ok(Register::R12),
            13 => Ok(Register::R13),
            14 => Ok(Register::R14),
            15 => Ok(Register::R15),
            _ => Err(())
        }
    }
}

impl FromStr for Port {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pixel_x" => Ok(Port::PixelX),
            "pixel_y" => Ok(Port::PixelY),
            "draw_pixel" => Ok(Port::DrawPixel),
            "clear_pixel" => Ok(Port::ClearPixel),
            "load_pixel" => Ok(Port::LoadPixel),
            "buffer_screen" => Ok(Port::BufferScreen),
            "clear_screen_buffer" => Ok(Port::ClearScreenBuffer),
            "write_char" => Ok(Port::WriteChar),
            "buffer_chars" => Ok(Port::BufferChars),
            "clear_chars_buffer" => Ok(Port::ClearCharsBuffer),
            "show_number" => Ok(Port::ShowNumber),
            "clear_number" => Ok(Port::ClearNumber),
            "signed_mode" => Ok(Port::SignedMode),
            "unsigned_mode" => Ok(Port::UnsignedMode),
            "rng" => Ok(Port::RNG),
            "controller_input" => Ok(Port::ControllerInput),
            _ => Err(())
        }
    }
}

impl FromStr for Condition {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "zs" | "eq" | "=" | "z" | "zero" => Ok(Condition::ZS),
            "zc" | "ne" | "!=" | "nz" | "notzero" => Ok(Condition::ZC),
            "cs" | "ge" | ">=" | "c" | "carry" => Ok(Condition::CS),
            "cc" | "lt" | "<" | "nc" | "notcarry" => Ok(Condition::CC),
            _ => Err(())
        }
    }
}