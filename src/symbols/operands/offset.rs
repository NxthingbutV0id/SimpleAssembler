use std::fmt::Display;
use crate::symbols::instruction::Instruction;

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
        write!(f, ".{}", self.name)
    }
}