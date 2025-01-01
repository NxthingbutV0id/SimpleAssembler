use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Condition {
    Z1, // Z == 1
    Z0, // Z == 0
    C1, // C == 1
    C0  // C == 0
}

impl FromStr for Condition {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "zs" | "eq" | "=" | "z" | "zero" => Ok(Condition::Z1),
            "zc" | "ne" | "!=" | "nz" | "notzero" => Ok(Condition::Z0),
            "cs" | "ge" | ">=" | "c" | "carry" => Ok(Condition::C1),
            "cc" | "lt" | "<" | "nc" | "notcarry" => Ok(Condition::C0),
            _ => Err(())
        }
    }
}

impl Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Condition::Z1 => write!(f, "eq"),
            Condition::Z0 => write!(f, "ne"),
            Condition::C1 => write!(f, "ge"),
            Condition::C0 => write!(f, "lt")
        }
    }
}