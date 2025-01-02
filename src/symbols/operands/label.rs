use std::fmt::Display;
use crate::symbols::instruction::Instruction;
use crate::symbols::operands::address::Address;

#[derive(Debug, Clone, PartialEq)]
pub struct Label {
    pub name: String,
    pub binding: Option<Instruction>,
    pub offset: Option<Address>
}

impl Label {
    pub fn new(name: String) -> Self {
        Label {
            name,
            binding: None,
            offset: None
        }
    }
}

impl Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, ".{}", self.name)
    }
}