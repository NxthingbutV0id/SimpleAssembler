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
        match s {
            "r0" | "R0" | "0" => Ok(Register::R0),
            "r1" | "R1" | "1" => Ok(Register::R1),
            "r2" | "R2" | "2" => Ok(Register::R2),
            "r3" | "R3" | "3" => Ok(Register::R3),
            "r4" | "R4" | "4" => Ok(Register::R4),
            "r5" | "R5" | "5" => Ok(Register::R5),
            "r6" | "R6" | "6" => Ok(Register::R6),
            "r7" | "R7" | "7" => Ok(Register::R7),
            "r8" | "R8" | "8" => Ok(Register::R8),
            "r9" | "R9" | "9" => Ok(Register::R9),
            "r10" | "R10" | "10" => Ok(Register::R10),
            "r11" | "R11" | "11" => Ok(Register::R11),
            "r12" | "R12" | "12" => Ok(Register::R12),
            "r13" | "R13" | "13" => Ok(Register::R13),
            "r14" | "R14" | "14" => Ok(Register::R14),
            "r15" | "R15" | "15" => Ok(Register::R15),
            _ => Err(())
        }
    }
}