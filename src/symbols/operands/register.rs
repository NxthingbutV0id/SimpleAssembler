use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Register {
    R0, R1, R2, R3,
    R4, R5, R6, R7,
    R8, R9, R10, R11,
    R12, R13, R14, R15
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "r{}", *self as u8)
    }
}

impl FromStr for Register {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "r0" => Ok(Register::R0),
            "r1" => Ok(Register::R1),
            "r2" => Ok(Register::R2),
            "r3" => Ok(Register::R3),
            "r4" => Ok(Register::R4),
            "r5" => Ok(Register::R5),
            "r6" => Ok(Register::R6),
            "r7" => Ok(Register::R7),
            "r8" => Ok(Register::R8),
            "r9" => Ok(Register::R9),
            "r10" => Ok(Register::R10),
            "r11" => Ok(Register::R11),
            "r12" => Ok(Register::R12),
            "r13" => Ok(Register::R13),
            "r14" => Ok(Register::R14),
            "r15" => Ok(Register::R15),
            _ => Err(())
        }
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