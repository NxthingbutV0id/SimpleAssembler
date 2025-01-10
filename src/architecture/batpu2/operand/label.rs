use std::fmt::Display;
use crate::architecture::batpu2::instruction::Instruction;
use crate::architecture::batpu2::operand::immediate::Address;

#[derive(Debug, Clone, PartialEq)]
pub struct Label {
    pub name: String,
    address: Option<Address>,
}

impl Label {
    pub fn new(name: String) -> Self {
        Self {
            name,
            address: None
        }
    }

    pub fn set_address(&mut self, instruction: &Instruction) {
        self.address = instruction.location.clone();
    }

    pub fn get_address(&self) -> Option<Address> {
        self.address.clone()
    }
}

impl Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, ".{}", self.name)
    }
}